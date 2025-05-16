# 🎁 Particles

Let's add some particles! They are a good effect to help make a game look nicer, and can be easy to add.

Bevy doesn't have first party support for particles, but there are at least two third party plugins that provide that:

- [bevy_hanabi](https://crates.io/crates/bevy_hanabi)
- [bevy_enoki](https://crates.io/crates/bevy_enoki)

bevy_hanabi uses the GPU through compute shaders, while bevy_enoki does it all on the CPU. In our case, as we want our game to work on Wasm in WebGL2 where compute shaders are not available, we'll use bevy_enoki.

## bevy_enoki Setup

Let's add the plugin to our project:

```sh
cargo add bevy_enoki
```

And the plugin to our app:

```rust,no_run
# extern crate bevy;
# extern crate avian2d;
# extern crate bevy_enhanced_input;
# extern crate bevy_enoki;
# use avian2d::{PhysicsPlugins, prelude::Gravity};
# use bevy::prelude::*;
# use bevy_enoki::EnokiPlugin;
# use bevy_enhanced_input::EnhancedInputPlugin;
fn main() {
    App::new()
        // ...
#        .add_plugins(DefaultPlugins)
        .add_plugins((PhysicsPlugins::default(), EnhancedInputPlugin, EnokiPlugin))
        // ...
# ;
}
```

## Creating a Particle Effect for our Ship Jet

bevy_enoki particle effects are declared through an [`Particle2dEffect`](https://docs.rs/bevy_enoki/0.4.0/bevy_enoki/struct.Particle2dEffect.html) asset. The easiest way to do that is through a ron configuration file.

In the file `assets/jet.particle.ron`, add the following content:

```ron
(
    spawn_rate: 0.1,
    spawn_amount: 1,
    emission_shape: Point,
    lifetime: (1.0, 0.0),
    linear_speed: Some((100, 0.1)),
    direction: Some(((0, -1), 0.1)),
    scale: Some((3., 1.)),
    color: Some((red: 3.0, green: 3.0, blue: 0.0, alpha: 1.0)),
)
```

You don't need to define all the fields, only the one that you want to set with e different value than the default one.

If you look at the color we defined, it will be a yellow that will have a bloom effect.

We can now load that file in our `GameAssets` struct, in a field `jet` of type `Handle<Particle2dEffect>`.

We'll load it as a "sibling" to our jet `Sprite`. Like the jet which is set to hidden when no thrust is applied, the particle effect will be set as inactive:

```rust
# extern crate bevy;
# extern crate bevy_enoki;
# use bevy::prelude::*;
# use bevy_enoki::prelude::*;
# #[derive(Resource)]
# struct GameAssets{ player_ship: Handle<Image>, jet_particles: Handle<Particle2dEffect> }
fn spawn_player(commands: &mut Commands, game_assets: &GameAssets) {
    // Actions setup

    commands
        .spawn((
            Sprite::from_image(game_assets.player_ship.clone()),
            // Rest of the components of the ship
            children![
                (
                    // Components for the jet sprite
                ),
                (
                    ParticleSpawner::default(),
                    ParticleSpawnerState {
                        active: false,
                        ..default()
                    },
                    ParticleEffectHandle(game_assets.jet_particles.clone()),
                    Transform::from_xyz(0.0, -40.0, 0.0),
                )
            ],
        ))
    // ...
# ;
}
```

And we'll need to enable the particle effect in the `thrust` system, and disable it in the `thrust_stop` system.

```rust
# extern crate bevy;
# extern crate avian2d;
# extern crate bevy_enhanced_input;
# extern crate bevy_enoki;
# use avian2d::prelude::*;
# use bevy::prelude::*;
# use bevy_enoki::prelude::*;
# use bevy_enhanced_input::prelude::*;
# #[derive(Debug, InputAction)]
# #[input_action(output = bool)]
# struct Thrust;
fn thrust(
    trigger: Trigger<Fired<Thrust>>,
    mut player: Query<(&Transform, &mut LinearVelocity, &Children)>,
    mut visibility: Query<&mut Visibility>,
    mut particle_state: Query<&mut ParticleSpawnerState>,
) -> Result {
    let (transform, mut linear_velocity, children) = player.get_mut(trigger.target())?;
    linear_velocity.0 += transform.local_y().xy() * 2.0;
    linear_velocity.0 = linear_velocity.0.clamp_length_max(300.0);

    // Make jet sprite visible
    visibility
        .get_mut(children[0])?
        .set_if_neq(Visibility::Visible);

    // Make jet particles active
    particle_state
        .get_mut(children[1])?
        .map_unchanged(|s| &mut s.active)
        .set_if_neq(true);

    Ok(())
}
```

And similarly in the `thrust_stop` system:

```rust
# extern crate bevy;
# extern crate bevy_enhanced_input;
# extern crate bevy_enoki;
# use bevy::prelude::*;
# use bevy_enoki::prelude::*;
# use bevy_enhanced_input::prelude::*;
# #[derive(Debug, InputAction)]
# #[input_action(output = bool)]
# struct Thrust;
fn thrust_stop(
    trigger: Trigger<Completed<Thrust>>,
    player: Query<&Children>,
    mut visibility: Query<&mut Visibility>,
    mut particle_state: Query<&mut ParticleSpawnerState>,
) -> Result {
    let Ok(children) = player.get(trigger.target()) else {
        return Ok(());
    };

    // Make the jet sprite hidden
    visibility
        .get_mut(children[0])?
        .set_if_neq(Visibility::Hidden);

    // Make the jet particle inactive
    particle_state
        .get_mut(children[1])?
        .map_unchanged(|s| &mut s.active)
        .set_if_neq(false);

    Ok(())
}
```
