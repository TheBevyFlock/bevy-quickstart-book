# Systems and Schedules

A splash screen needs to display something, so let's show a title in the open window.

```rust,no_run
# extern crate bevy;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Quickstart".into(),
                ..default()
            }),
            ..default()
        }))
        // add a system that executes once at startup
        .add_systems(Startup, display_title)
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
                Text::new("Bevy Quickstart"),
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
}
```

## Schedules

The `Startup` schedule is used for tasks that need to occur only once during application startup.

Other common schedules include `PreUpdate`, `Update`, and `PostUpdate`, along with their fixed counterparts: `FixedPreUpdate`, `FixedUpdate`, and `FixedPostUpdate`.

Systems in the `Update` schedule execute every frame. With vsync enabled, this is typically driven by your screen's refresh rate, commonly 60fps, with some Macs running at 120fps. Systems in the `FixedUpdate` schedule execute at a configurable, fixed frequency, defaulting to 64Hz. Most game logic should occur within these schedules.

`Pre*` and `Post*` schedules are useful for preparation and cleanup/propagation tasks surrounding game logic.

## Systems

Systems are functions whose parameters must implement the [`SystemParam`](https://docs.rs/bevy/0.16.0/bevy/ecs/system/trait.SystemParam.html) trait. These parameters are provided through dependency injection based on their type.

If you want more details on how this works, you can find them here: [Dependency Injection like Bevy Engine from Scratch](https://promethia-27.github.io/dependency_injection_like_bevy_from_scratch/introductions.html)

## Commands

Commands are one way of modifying the game world, without risking to encounter double borrow of the world. You can add, mutate, or remove entities and components. They are not executed straight away, but at sync points between systems.

## Hierarchy

Bevy has the concept of hierarchy, with Parent / Children relationship. This is heavily used in UI for layout, or in animations.

When an entity is a child of another, its position is relative to its parent. It's also possible to remove a complete branch of a hierarchy at once.

The `children!` macro is an helper to reduce boilerplate when spawning an entity with children. It handles automatically the `Children` / `Parent` components, and keeps the code simpler.

## Side note: UI

The startup system in the example above spawns text. It first spawns a node entity, which functions similarly to a `<div>` HTML tag, used to center the text, and then spawns the text itself as a child.

Bevy offers two layout strategies for UI: Flexbox and CSS Grids.
