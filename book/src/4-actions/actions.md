# Action Mapper

Goal of the game is to destroy all the asteroids.

Let's give the player a way to do that! But first let's refactor how we handle the player's actions so that it's easier to extend.

## Using an Input Manager

First step is to add the new dependency to our project

```sh
cargo add bevy_enhanced_input
```

And, as is customary with Bevy plugins, we need to add the plugin to our application. In our main function, we can add it with the physics plugin:

```rust
# extern crate bevy;
# extern crate avian2d;
# extern crate bevy_enhanced_input;
# use avian2d::{PhysicsPlugins, prelude::Gravity};
# use bevy::prelude::*;
# use bevy_enhanced_input::EnhancedInputPlugin;
fn main() {
    App::new()
        // ...
        .add_plugins((PhysicsPlugins::default(), EnhancedInputPlugin))
        // ...
# ;
}
```

In our `game_plugin`, we will now remove the `control_player` system.

First step to start using input mapping is to enable an input context. This is useful in more complex games where control schemes change depending on the current game mode. Here we will have only one input context to control the ship.

```rust
# extern crate bevy;
# extern crate bevy_enhanced_input;
# use bevy::prelude::*;
# use bevy_enhanced_input::prelude::*;
# fn display_level() {}
# fn collision() {}
# fn tick_explosion() {}
# #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
# enum GameState {
#     #[default]
#     Splash,
#     StartMenu,
#     Game,
# }
#[derive(InputContext)]
struct ShipController;

pub fn game_plugin(app: &mut App) {
    app.add_input_context::<ShipController>()
        .add_systems(OnEnter(GameState::Game), display_level)
        .add_systems(
            Update,
            (collision, tick_explosion).run_if(in_state(GameState::Game)),
        );
}
```

## First Action: Ship Rotation

We can now declare our actions! Let's start with how to rotate the ship:

```rust
# extern crate bevy;
# extern crate avian2d;
# extern crate bevy_enhanced_input;
# use avian2d::{PhysicsPlugins, prelude::Gravity};
# use bevy::prelude::*;
# use bevy_enhanced_input::prelude::*;
#[derive(Debug, InputAction)]
#[input_action(output = f32)]
struct Rotate;
```

This is an `InputAction` that will return a `f32`, whose sign will give us the direction in which the ship will rotate.

For this action to be triggered, we need to add an a `Actions` component to our ship. In the function `spawn_player`, we'll create it and add it to the other components:

```rust
# extern crate bevy;
# extern crate avian2d;
# extern crate bevy_enhanced_input;
# use avian2d::{PhysicsPlugins, prelude::Gravity};
# use bevy::prelude::*;
# use bevy_enhanced_input::prelude::*;
# #[derive(Debug, InputAction)]
# #[input_action(output = f32)]
# struct Rotate;
# struct GameAssets;
# #[derive(InputContext)]
# struct ShipController;
fn spawn_player(commands: &mut Commands, game_assets: &GameAssets) {
    let mut actions = Actions::<ShipController>::default();

    actions.bind::<Rotate>().to(Bidirectional {
        positive: KeyCode::KeyA,
        negative: KeyCode::KeyD,
    });

    commands
        .spawn((
            // The other components
            actions,
        ));
}
```

## Observers

To react to the input action, we use a system that takes a [`Trigger`](https://docs.rs/bevy/0.16.0/bevy/ecs/observer/struct.Trigger.html) as a system parameter, plus any other parameter needed. Those systems are known as [`Observer`s](https://docs.rs/bevy/latest/bevy/ecs/observer/struct.Observer.html).

```rust
# extern crate bevy;
# extern crate avian2d;
# extern crate bevy_enhanced_input;
# use avian2d::prelude::*;
# use bevy::prelude::*;
# use bevy_enhanced_input::prelude::*;
# #[derive(Debug, InputAction)]
# #[input_action(output = f32)]
# struct Rotate;
fn rotate(
    trigger: Trigger<Fired<Rotate>>,
    mut player: Query<&mut AngularVelocity>,
    time: Res<Time>,
) -> Result {
    let fixed_rate = 0.2;
    let delta = time.delta().as_secs_f32();
    let rate = fixed_rate / (1.0 / (60.0 * delta));
    let mut angular_velocity = player.get_mut(trigger.target())?;
    angular_velocity.0 += trigger.value.signum() * rate;

    Ok(())
}
```

The `Trigger<Fired<Rotate>>` system parameter means that this system will run every time a `Fired<Rotate>` event is triggered. `bevy_enhanced_input` sends different events for an action, for example `Started<Rotate>`, `Fired<Rotate>` and `Completed<Rotate>`. In this case we want to react as long as the `Rotate` action happens.

We will attach this observer to our ship entity:

```rust
# extern crate bevy;
# extern crate avian2d;
# extern crate bevy_enhanced_input;
# use avian2d::prelude::*;
# use bevy::prelude::*;
# use bevy_enhanced_input::prelude::*;
# #[derive(Debug, InputAction)]
# #[input_action(output = f32)]
# struct Rotate;
# struct GameAssets;
# fn rotate(trigger: Trigger<Fired<Rotate>>) -> Result {Ok(())}
fn spawn_player(commands: &mut Commands, game_assets: &GameAssets) {
    commands
        .spawn((
            // all the components
        ))
        .observe(rotate);
}
```

## Second action: Thrust

We need another action for thrust, this time its output should be just a boolean: is there thrust or not.

```rust
# extern crate bevy;
# extern crate avian2d;
# extern crate bevy_enhanced_input;
# use avian2d::prelude::*;
# use bevy::prelude::*;
# use bevy_enhanced_input::prelude::*;
#[derive(Debug, InputAction)]
#[input_action(output = bool)]
struct Thrust;
```

We need to react when thrust is fired, adding linear velocity to the ship, and once it's finished, to remove the jets. Let's create our two systems for that:

```rust
# extern crate bevy;
# extern crate avian2d;
# extern crate bevy_enhanced_input;
# use avian2d::prelude::*;
# use bevy::prelude::*;
# use bevy_enhanced_input::prelude::*;
# #[derive(Debug, InputAction)]
# #[input_action(output = bool)]
# struct Thrust;
fn thrust(
    trigger: Trigger<Fired<Thrust>>,
    mut player: Query<(&Transform, &mut LinearVelocity, &Children)>,
    mut visibility: Query<&mut Visibility>,
) -> Result {
    let (transform, mut linear_velocity, children) = player.get_mut(trigger.target())?;
    linear_velocity.0 += transform.local_y().xy() * 2.0;
    linear_velocity.0 = linear_velocity.0.clamp_length_max(200.0);
    visibility
        .get_mut(children[0])?
        .set_if_neq(Visibility::Visible);
    Ok(())
}

fn thrust_stop(
    trigger: Trigger<Completed<Thrust>>,
    player: Query<&Children>,
    mut visibility: Query<&mut Visibility>,
) -> Result {
    let children = player.get(trigger.target())?;

    visibility
        .get_mut(children[0])?
        .set_if_neq(Visibility::Hidden);

    Ok(())
}
```

And we're ready to define when this action will be executed, and to add the observers to our ship:

```rust
# extern crate bevy;
# extern crate avian2d;
# extern crate bevy_enhanced_input;
# use avian2d::prelude::*;
# use bevy::prelude::*;
# use bevy_enhanced_input::prelude::*;
# #[derive(Debug, InputAction)]
# #[input_action(output = f32)]
# struct Rotate;
# #[derive(Debug, InputAction)]
# #[input_action(output = bool)]
# struct Thrust;
# fn rotate(trigger: Trigger<Fired<Thrust>>) -> Result {Ok(())}
# fn thrust(trigger: Trigger<Fired<Thrust>>) -> Result {Ok(())}
# fn thrust_stop(trigger: Trigger<Completed<Thrust>>) -> Result {Ok(())}
# struct GameAssets;
# #[derive(InputContext)]
# struct ShipController;
fn spawn_player(commands: &mut Commands, game_assets: &GameAssets) {
    let mut actions = Actions::<ShipController>::default();

    actions.bind::<Rotate>().to(Bidirectional {
        positive: KeyCode::KeyA,
        negative: KeyCode::KeyD,
    });
    actions.bind::<Thrust>().to(KeyCode::KeyW);

    commands
        .spawn((
            // the other components
            actions,
        ))
        .observe(rotate)
        .observe(thrust)
        .observe(thrust_stop);
}
```

## Triggers

All the events we are using are triggered by `bevy_enhanced_input`, so we're just reacting to them. It's also possible to trigger your own events, this can be done through commands.

## Scope of Observers

They can be global with [`Commands::trigger`](https://docs.rs/bevy/0.16.0/bevy/ecs/prelude/struct.Commands.html#method.trigger) and [`App::add_observer`](https://docs.rs/bevy/0.16.0/bevy/app/struct.App.html#method.add_observer), or specific to an entity with [`EntityCommands::trigger`](https://docs.rs/bevy/0.16.0/bevy/ecs/prelude/struct.EntityCommands.html#method.trigger) and [`EntityCommands::observe`](https://docs.rs/bevy/0.16.0/bevy/ecs/prelude/struct.EntityCommands.html#method.observe).
