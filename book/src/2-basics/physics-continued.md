# Physics continued


## Asteroid Movements

Asteroids are the easiest to do! First remove the `inertia` system, and the fields of the `Asteroid` component, as that will now be handled by the physics engine.

When spawning an asteroid, we'll need to add the following components:

- `Collider`
- `LinearVelocity`
- `AngularVelocity`

And that's it! As a bonus, now asteroids will bounce off each other.

```rust
# extern crate bevy;
# extern crate rand;
# use std::f32::consts::TAU;
# use bevy::prelude::*;
# use crate::rand::Rng;
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
# #[derive(Component)]
# pub struct LinearVelocity(pub Vec2);
# #[derive(Component)]
# pub struct AngularVelocity(pub f32);
# #[derive(Component)]
# pub struct Collider(pub f32);
fn display_level(mut commands: Commands, game_assets: Res<GameAssets>) {
    // Same player spawning

    let mut rng = rand::thread_rng();
    for (x, y) in [(1., 1.), (-1., 1.), (-1., -1.), (1., -1.)] {
        commands.spawn((
            Sprite::from_image(game_assets.asteroid.clone()),
            Transform::from_xyz(300.0 * x, 200.0 * y, 0.0),
            Collider(50.0),
            LinearVelocity(Vec2::from_angle(rng.gen_range(0.0..TAU)) * rng.gen_range(10.0..100.0)),
            AngularVelocity(rng.gen_range(-1.5..1.5)),
            Asteroid,
            StateScoped(GameState::Game),
        ));
    }
}
```

## Ship Movements

Ship movements are a bit more complicated. As it doesn't have fixed linear and angular velocities, we'll need to change them when reacting to user input.

First, we'll add some components when spawning the ship entity:

- `Collider`

Another component we'll add is `AngularDamping`. As the ship is in space, once it's rotating it shouldn't slow down by itself, but that isn't very pleasant to control. Adding damping means that it will stop rotating by itself.

```rust
# extern crate bevy;
# use bevy::prelude::*;
# #[derive(Component)]
# struct Player;
# #[derive(Resource)]
# struct GameAssets {
#     player_ship: Handle<Image>,
#     jets: Handle<Image>,
# }
# #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
# enum GameState {
#     #[default]
#     Game,
# }
# #[derive(Component)]
# pub struct LinearVelocity(pub Vec2);
# #[derive(Component)]
# pub struct AngularVelocity(pub f32);
# #[derive(Component)]
# pub struct AngularDamping(pub f32);
# #[derive(Component)]
# pub struct Collider(pub f32);
fn display_level(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands.spawn((
        Sprite::from_image(game_assets.player_ship.clone()),
        Collider(40.0),
        LinearVelocity(Vec3::ZERO),
        AngularVelocity(0.0),
        AngularDamping(5.0),
        Player,
        StateScoped(GameState::Game),
        children![(
            Sprite::from_image(game_assets.jets.clone()),
            Transform::from_xyz(0.0, -40.0, -1.0),
            Visibility::Hidden,
        )],
    ));

    // Same asteroids spawning
}
```

And when reacting to user input, we'll modify the `AngularVelocity` and `LinearVelocity` components. One thing to keep in mind is to set a maximum `LinearVelocity` or the ship could accelerate forever and reach an uncontrollable speed.

```rust
# extern crate bevy;
# use bevy::prelude::*;
# #[derive(Component)]
# struct Player;
# #[derive(Component)]
# pub struct LinearVelocity(pub Vec2);
# #[derive(Component)]
# pub struct AngularVelocity(pub f32);
fn control_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player: Query<
        (
            &Transform,
            &mut AngularVelocity,
            &mut LinearVelocity,
            &Children,
        ),
        With<Player>,
    >,
    mut visibility: Query<&mut Visibility>,
) -> Result {
    let Ok((player_transform, mut angular_velocity, mut linear_velocity, children)) = player.single_mut()
    else {
        // No player at the moment, skip control logic
        return Ok(());
    };
    if keyboard_input.pressed(KeyCode::KeyA) {
        angular_velocity.0 += 0.2;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        angular_velocity.0 -= 0.2;
    }
    if keyboard_input.pressed(KeyCode::KeyW) {
        let forward = player_transform.local_y();
        linear_velocity.0 += forward.xy() * 2.0;
        linear_velocity.0 = linear_velocity.0.clamp_length_max(300.0);
        *visibility.get_mut(children[0])? = Visibility::Visible;
    } else {
        visibility
            .get_mut(children[0])?
            .set_if_neq(Visibility::Hidden);
    }
    Ok(())
}
```

With that done, we can now remove the `move_player` system!

## Collisions

TODO

```
# extern crate bevy;
# use bevy::prelude::*;
# #[derive(Component)]
# struct Player;
# #[derive(Component)]
# struct Explosion(Timer);
# #[derive(Resource)]
# struct GameAssets {
#     explosion: Handle<Image>,
# }
# #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
# enum GameState {
#     #[default]
#     Game,
# }
fn collision(
    collisions: Collisions,
    player: Query<(&Transform, Entity), With<Player>>,
    mut commands: Commands,
    game_assets: Res<GameAssets>,
) -> Result {
    let Ok((transform, entity)) = player.single() else {
        return Ok(());
    };

    if collisions.collisions_with(entity).next().is_some() {
        commands.spawn((
            Sprite::from_image(game_assets.explosion.clone()),
            (*transform).with_scale(Vec3::splat(0.2)),
            Explosion(Timer::from_seconds(1.0, TimerMode::Once)),
            StateScoped(GameState::Game),
        ));
        commands.entity(entity).despawn();
    }

    Ok(())
}
```
