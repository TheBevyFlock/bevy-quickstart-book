# Exercises

It's very easy to avoid collisions with the asteroids as they don't move... Let's make this game a bit harder!

## Moving the Asteroids

Make the asteroids move in random directions, at random speeds.

Tips:

- Add information about direction and speed to the `Asteroid` component.
- Add a system to update the position of the asteroids based on their direction and speed.

## Losing the Game

Let's go back to the menu when colliding with an asteroid.

Tips:

- Use the `ResMut<NextState<GameStates>>` system parameter to change the current state on collision

## Explosion Effect on Collision

It's nicer to see what happened before going back to the menu, let's display an explosion and wait a bit.

Tips:

- Load the asset for the explosion effect
- Spawn a sprite at the same `Transform` as the ship, with a `Timer`
  - Use the `Commands` system parameter to spawn the explosion sprite
- Despawn the ship
- Add a new system that will tick the timer
- After the timer is done, despawn it and change state
- Some systems will return errors now as they try to access the player transform
  - Those are the systems handling player control and collisions with asteroids
  - Those systems shouldn't do anything when there isn't a player
  - Instead of using `?` with `single` / `single_mut` when querying for it, use `let Ok(...) = query.single() else { return Ok(()); };`
