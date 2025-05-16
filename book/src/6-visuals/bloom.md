# Bloom

Bloom is another way to improve how our game looks. It's very easy to enable it, and we can follow Bevy example for that: [2D Bloom](https://bevyengine.org/examples/2d-rendering/bloom-2d/).

## Enable bloom

When spawning our `Camera2d` in the `display_title` system, we'll need to add a few components for bloom:

```rust
# extern crate bevy;
# use bevy::{core_pipeline::bloom::Bloom, prelude::*};
fn display_title(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            hdr: true,
            ..default()
        },
        Bloom::default(),
    ));

    // ...
}
```

And that's it! Bloom is enabled.

But by itself that isn't enough to see a change on screen, for that we need to do something to our _colors_.

## Blooming Laser!

A good candidate for bloom is our laser. To do that, when spawning the `Sprite` component with the handle to the image, we'll also provide a color. To have a bloom effect, the color should bigger value on some channels than `1.0`. As our laser is red, let's try `Color::srgb(5.0, 1.0, 1.0)` which should emit a red light.

```rust
# extern crate bevy;
# use bevy::prelude::*;
# #[derive(Resource)]
# struct GameAssets { laser: Handle<Image> }
# fn system(mut commands: Commands, game_assets: Res<GameAssets>) {
commands
    .spawn((
        Sprite {
            image: game_assets.laser.clone(),
            color: Color::srgb(5.0, 1.0, 1.0),
            ..default()
        },
        // ...
    ));
# }
```
