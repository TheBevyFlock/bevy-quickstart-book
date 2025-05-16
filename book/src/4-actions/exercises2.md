# Exercises

It's possible for the player to go off-screen, which makes the game harder to control. Let's do some camera work!

## Make the camera follow the player

Translate the camera along with the player ship.

Tips:

- Get the player ship `Transform`
- Move the camera `Transform` to the same translation

## Closest Asteroid Indicator

Let's make it easier to find the last few asteroids!

Find the closest asteroid, and display an indicator of its direction.

Tips:

- Iterate over all asteroids, and find the one closest to the player ship
- Find its direction
- Display an indicator (Gizmos have an helpful `arrow_2d` function)

## Make it easier to finish

Once you're done to the last few asteroids, it can be a bit boring to hunt the last few ones. Let's make it even easier for the player.

If the closest asteroid is farther away than some distance, send the asteroid towards the player.

Tips:

- Check the distance between the ship and the closest asteroid
- If it's too far away, send the asteroid towards the ship
