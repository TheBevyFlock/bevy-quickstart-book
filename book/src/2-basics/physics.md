# Physics

## Third party crates

In the following sections we'll add basic movement and collision detection.

Game physics is a deep subject, and for more complicated games you will want to use an existing solution instead of reinventing it yourself. Bevy currently doesn't have a built-in physics solution, but there are community crates you can use:

<p align="center">
    <br />
    <br />
    <a href="https://github.com/Jondolf/avian">
        <img src="https://raw.githubusercontent.com/Jondolf/avian/avian/assets/branding/logo.svg" />
    </a>
    <br />
    <br />
    <br />
    <a href="https://rapier.rs/">
        <img src="https://rapier.rs/img/rapier_logo_color_textpath.svg" />
    </a>
    <br />
    <br />
    <br />
</p>

With that out of the way, let's add more asteroids and handle collisions!

## More Asteroids

We'll spawn 4 asteroids, at fixed positions for now.

```rust,no_run
# extern crate bevy;
# use bevy::prelude::*;
# #[derive(Component)]
# struct Player;
# #[derive(Component)]
# struct Asteroid;
# #[derive(Resource)]
# struct GameAssets {
#     asteroid: Handle<Image>,
# }
# #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
# enum GameState {
#     #[default]
#     Game,
# }
fn display_level(mut commands: Commands, game_assets: Res<GameAssets>) {
    // Same player spawning

    // Asteroids spawning
    for (x, y) in [(1., 1.), (-1., 1.), (-1., -1.), (1., -1.)] {
        commands.spawn((
            Sprite::from_image(game_assets.asteroid.clone()),
            Transform::from_xyz(300.0 * x, 200.0 * y, 0.0),
            Asteroid,
            StateScoped(GameState::Game),
        ));
    }
}
```

## Collisions

One of the easiest way to test collisions is to assume that everything is round, and then check that the distance between two objects is less than the sum of their radii. This is a close enough approximation that works well in our case. Another basic shape that is often used for collision detection is `AABB` (for Axis-Aligned Bounding Box, so a rectangle).

Let's get the position of the player, and check the distance with every asteroid.

```rust,no_run
# extern crate bevy;
# use bevy::prelude::*;
# #[derive(Component)]
# struct Player;
# #[derive(Component)]
# struct Asteroid;
fn collision(
    asteroids: Query<&Transform, With<Asteroid>>,
    player: Query<&Transform, With<Player>>,
) -> Result {
    let player_radius = 40.0;
    let asteroid_radius = 50.0;
    let player_transform = player.single()?;
    for asteroid_transform in &asteroids {
        let distance = asteroid_transform
            .translation
            .distance(player_transform.translation);
        if distance < (asteroid_radius + player_radius) {
            println!("Collision detected!");
        }
    }

    Ok(())
}
```

<div class="warning">

Don't forget to add the new `collision` system to the `game_plugin`, on `Update` in the `GameState::Game` state.

</div>

## Gizmos

An easy way to debug what is happening on screen are gizmos. They make it possible to draw simple shapes on screen, like circles or rectangles.

We'll draw circles around the different objects, with their radius.

```rust,no_run
# extern crate bevy;
# use bevy::prelude::*;
# #[derive(Component)]
# struct Player;
# #[derive(Component)]
# struct Asteroid;
fn collision(
    asteroids: Query<&Transform, With<Asteroid>>,
    player: Query<&Transform, With<Player>>,
    mut gizmos: Gizmos,
) -> Result {
    let player_radius = 40.0;
    let asteroid_radius = 50.0;
    let player_transform = player.single()?;
    gizmos.circle_2d(
        player_transform.translation.xy(),
        player_radius,
        Color::linear_rgb(1.0, 0.0, 0.0),
    );
    for asteroid_transform in &asteroids {
        gizmos.circle_2d(
            asteroid_transform.translation.xy(),
            asteroid_radius,
            Color::linear_rgb(0.0, 0.0, 1.0),
        );
        let distance = asteroid_transform
            .translation
            .distance(player_transform.translation);
        if distance < (asteroid_radius + player_radius) {
            println!("Collision detected!");
        }
    }

    Ok(())
}
```

<div class="warning">

It's often useful to add a `debug` feature to your game, and put things like debug drawing with gizmos behind it!

</div>
