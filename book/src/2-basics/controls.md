# Controlling With Input

We'll control our player with the `A` and `D` keys on the keyboard to turn, and `W` for thrust.

Let's start by handling rotation:

```rust,no_run
# extern crate bevy;
# use bevy::prelude::*;
# use std::f32::consts::FRAC_PI_8;
# #[derive(Component)]
# struct Player;
fn control_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) -> Result {
    let mut player_transform = player.single_mut()?;

    let fixed_rotation_rate = 0.2;
    let rotation_rate = fixed_rotation_rate / (1.0 / (60.0 * time.delta().as_secs_f32()));

    if keyboard_input.pressed(KeyCode::KeyA) {
        player_transform.rotate_z(rotation_rate);
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        player_transform.rotate_z(-rotation_rate);
    }
    Ok(())
}
```

<div class="warning">

Don't forget to add the new `control_player` system to the `game_plugin`, on `FixedUpdate` in the `GameState::Game` state.

</div>

## Keyboard controls

Bevy exposes a resource `ButtonInput<KeyCode>` that can be used in a system. [`KeyCode`](https://docs.rs/bevy/0.16.0/bevy/input/keyboard/enum.KeyCode.html) lists all the keys available on a standard US QWERTY keyboard. They ignore the layout of the user keyboard. This is useful in games to be able to react to the key at the same position no matter the layout.

If you want to handle text input, you should use [`KeyboardInput`](https://docs.rs/bevy/0.16.0/bevy/input/keyboard/struct.KeyboardInput.html) instead and use either the logical key (one for each key press) or the actual text (present only when it would add some text, with deadkeys / modifiers applied).

The same `ButtonInput` interface is used for other kind of button input: [`ButtonInput<GamepadButton>`](https://docs.rs/bevy/0.16.0/bevy/input/gamepad/enum.GamepadButton.html) for gamepads, [`ButtonInput<MouseButton>`](https://docs.rs/bevy/0.16.0/bevy/input/mouse/enum.MouseButton.html) for mice.

## Modifying transforms

The `Transform` component controls where an object is in the game world. Modifying it moves the object.

In 2D, the world is Y-up, X-right and Z is out of the screen. This means that to rotate something on screen, it has to be along the Z-axis.

The front of the ship in the image is towards the top, so forward is the Y-axis

Bevy exposes helper methods to manipulate the `Transform`:

- [`Transform::translation`](https://docs.rs/bevy/0.16.0/bevy/prelude/struct.Transform.html#structfield.translation) to change the position
- [`Transform::rotate_z()`](https://docs.rs/bevy/0.16.0/bevy/prelude/struct.Transform.html#method.rotate_z) to rotate
- [`Transform::local_y()`](https://docs.rs/bevy/0.16.0/bevy/prelude/struct.Transform.html#method.local_y) to get a given direction according to an object

As Bevy doesn't specialize for 2D, `Transform` has all the needed part for 3D and can be a bit hard to use in 2D.

## Time Delta

TODO

## Error handling in systems

Bevy systems can return a [`Result`](https://docs.rs/bevy/0.16.0/bevy/ecs/error/type.Result.html) (an alias to `Result<(), BevyError>`) to be able to use error handling, like `?`.

By default, a system that returns an error will cause a panic logging the error. It's possible to change this default behaviour by changing the [`GLOBAL_ERROR_HANDLER`](https://docs.rs/bevy/0.16.0/bevy/ecs/error/static.GLOBAL_ERROR_HANDLER.html).
