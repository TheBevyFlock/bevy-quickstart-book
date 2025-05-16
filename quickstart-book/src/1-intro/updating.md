# Updating the World

A key characteristic of a splash screen is that it doesn't stay forever. Let's remove the title after two seconds.

```rust,no_run
# extern crate bevy;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Workshop".into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, display_title)
        .add_systems(Update, remove_title)
        .run();
}

fn display_title(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children![
            (
                Text::new("Bevy Workshop"),
                TextFont {
                    font_size: 130.0,
                    ..default()
                },
            ),
            (
                Text::new("Rust Week 2025"),
                TextFont {
                    font_size: 100.0,
                    ..default()
                },
            )
        ],
    ));

    commands.insert_resource(SplashScreenTimer(Timer::from_seconds(2.0, TimerMode::Once)));
}

#[derive(Resource)]
struct SplashScreenTimer(Timer);

fn remove_title(
    time: Res<Time>,
    mut timer: ResMut<SplashScreenTimer>,
    mut commands: Commands,
    nodes: Query<Entity, With<Node>>
) {
    if timer.0.tick(time.delta()).just_finished() {
        for entity in &nodes {
            commands.entity(entity).despawn();
        }
    }
}
```

## Resources

[`Resources`](https://docs.rs/bevy/0.16.0/bevy/ecs/prelude/trait.Resource.html) are used to store singletons in the world, based on their type.

Here, we're adding a resource `SplashScreenTimer` that simply holds a `Timer`.

## Queries

[`Queries`](https://docs.rs/bevy/0.16.0/bevy/ecs/prelude/struct.Query.html) are used to access entities and their components in the world and can be filtered.

In the `remove_title` system, we're using a `Query` that requests access only to the [`Entity`](https://docs.rs/bevy/0.16.0/bevy/ecs/entity/struct.Entity.html), filtering on the component [`Node`](https://docs.rs/bevy/0.16.0/bevy/prelude/struct.Node.html), which is a basic component shared among all UI elements.

## Mutable vs. Immutable Access

The `remove_title` system accesses two resources:

- [`Time`](https://docs.rs/bevy/0.16.0/bevy/prelude/struct.Time.html), provided by Bevy, in an immutable way
- `SplashScreenTimer`, our custom resource, in a mutable way; the timer in this resource will be ticked, so we need to modify it

As the world continues to hold ownership of data, systems have access to references. Only one system accessing a given part of the world mutably can run at a time. Systems that access different parts mutably, or the same parts immutably, can run in parallel.
