# Exercises

Don't forget to checkout the branch:

```sh
git checkout 06-basic-game
```

Let's review what was changed: <https://github.com/TheBevyFlock/bevy-quickstart-book/compare/05-intro-to-bevy..06-basic-game>

## Displaying an Asteroid

File `meteorBrown_big1.png` has a sprite for an asteroid. Use it instead of the blue box.

Tips:

- Add a new field to the `GameAssets` resource for the asteroid

## Player Sprite Animation

Display engine jets behind the ship when moving forward.

Tips:

- Load the new sprite in `GameAssets`. `fire07.png` is a good sprite for jets
- Spawn the jets as children sprite of the player entity with the `children!` macro
  - Make it invisible with the `Visibility::Hidden` component
  - A good starting position is `Vec3::new(0.0, -40.0, 0.0)`
- Toggle sprite visibility when the ship moves forward, when the player presses the `W` key
  - You can get children of an entity with the `Children` component
  - Add a new query to the `control_player` that can modify the `Visibility` component
  - When `W` is pressed, query the `Visibility` component of the first child of the `Player` entity

## Player Acceleration

In space, there's no friction. Pressing `W` should make the ship accelerate in a direction, and movements should continue after the key is released.

Tips:

- Store the player velocity in a new component
- When the `W` key is pressed, change the velocity
- In a separate system, move the player according to its current velocity
