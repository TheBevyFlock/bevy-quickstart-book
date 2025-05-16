use std::{any::TypeId, marker::PhantomData};

use bevy::{platform::collections::HashMap, prelude::*};

pub fn input_plugin(app: &mut bevy::prelude::App) {
    app.add_systems(Update, trigger_actions);
}

#[derive(Debug, Deref, Event)]
pub struct Started<T>(PhantomData<fn(T)>);

#[derive(Debug, Deref, Event)]
pub struct Running<T>(PhantomData<fn(T)>);

#[derive(Debug, Deref, Event)]
pub struct Ended<T>(PhantomData<fn(T)>);

#[derive(Debug)]
struct ActionBinding {
    triggers: Vec<KeyCode>,
    trigger_action: fn(&mut Commands, ActionState, Entity),
}

fn trigger_action<A: 'static>(commands: &mut Commands, state: ActionState, entity: Entity) {
    let mut entity = commands.entity(entity);
    match state {
        ActionState::JustPressed => {
            entity.trigger(Started::<A>(PhantomData));
        }
        ActionState::Pressed => {
            entity.trigger(Running::<A>(PhantomData));
        }
        ActionState::JustReleased => {
            entity.trigger(Ended::<A>(PhantomData));
        }
    }
}

#[derive(Debug, Default, Component)]
pub struct ActionMappings {
    mappings: HashMap<TypeId, ActionBinding>,
}

impl ActionMappings {
    pub fn bind<T: std::any::Any>(&mut self, keys: impl IntoIterator<Item = KeyCode>) {
        let id = std::any::TypeId::of::<T>();
        self.mappings
            .entry(id)
            .or_insert_with(|| ActionBinding {
                triggers: vec![],
                trigger_action: trigger_action::<T>,
            })
            .triggers
            .extend(keys);
    }
}

#[derive(Debug, Clone, Copy)]
enum ActionState {
    JustPressed,
    Pressed,
    JustReleased,
}

fn trigger_actions(
    mut commands: Commands,
    input_events: Res<ButtonInput<KeyCode>>,
    action_query: Query<(Entity, &ActionMappings)>,
) {
    for (entity, mappings) in &action_query {
        for action in mappings.mappings.values() {
            let inputs = action.triggers.iter().copied();
            if input_events.any_just_pressed(inputs.clone()) {
                (action.trigger_action)(&mut commands, ActionState::JustPressed, entity)
            } else if input_events.any_pressed(inputs.clone()) {
                (action.trigger_action)(&mut commands, ActionState::Pressed, entity)
            } else if input_events.any_just_released(inputs) {
                (action.trigger_action)(&mut commands, ActionState::JustReleased, entity)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::AtomicBool;

    use bevy::prelude::*;

    use crate::input::{Ended, Running};

    use super::{ActionMappings, Started, input_plugin};

    struct Jump;

    #[test]
    fn check_triggered() {
        let mut app = App::new();
        app.add_plugins(input_plugin);

        let mut actions = ActionMappings::default();
        actions.bind::<Jump>([KeyCode::Space]);

        static IS_JUMPING: AtomicBool = AtomicBool::new(false);

        app.world_mut()
            .spawn((actions,))
            .observe(|_trigger: Trigger<Started<Jump>>| {
                IS_JUMPING.store(true, std::sync::atomic::Ordering::Release);
            })
            .observe(|_trigger: Trigger<Running<Jump>>| {
                IS_JUMPING.store(true, std::sync::atomic::Ordering::Release);
            })
            .observe(|_trigger: Trigger<Ended<Jump>>| {
                IS_JUMPING.store(false, std::sync::atomic::Ordering::Release);
            });

        let mut input = ButtonInput::<KeyCode>::default();
        input.press(KeyCode::Space);
        app.insert_resource(input);

        app.update();

        // verify that we are pressing the button
        assert!(IS_JUMPING.load(std::sync::atomic::Ordering::Acquire));

        IS_JUMPING.store(false, std::sync::atomic::Ordering::Release);

        {
            let mut inputs = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
            inputs.clear();
        }

        app.update();

        // verify that we are _STILL_ pressing the button
        assert!(IS_JUMPING.load(std::sync::atomic::Ordering::Acquire));

        {
            let mut inputs = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
            inputs.clear();
            inputs.release_all();
        }

        app.update();

        // verify that we are no longer pressing the button
        assert!(!IS_JUMPING.load(std::sync::atomic::Ordering::Acquire))
    }
}
