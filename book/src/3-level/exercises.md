# Exercises

Don't forget to checkout the branch:

```sh
git checkout 07-level-loading
```

Let's review what was changed: <https://github.com/TheBevyFlock/bevy-quickstart-book/compare/06-basic-game..07-level-loading>

## Spawn the correct number of asteroids

Spawn all the asteroids in the level

Tips:

- Find a random position in the arena
- Avoid spawning an asteroid on top of the player
  - Find a random position in the arena
  - Ensure the distance from the center is more than the radius of the player plus a safe margin

## Player can have multiple lives

Don't return to the menu on the first collision. Instead, do it when the player doesn't have any live remaining

Tips:

- Add the number of lives as a resource at the start of the game
- On collision, decrement the number of lives
- After a collision, wait for a few seconds before respawning the player
- You can move the code spawning the player to a separate function to be able to call it either at game start or on respawn
- Decide where to respawn:
  - At the game starting point
  - At the player last position
  - At a random position in the arena
  - The respawn point shouldn't have an asteroid or the player would die again immediately
- If the number of lives is 0, game over

## Display information about the level

Let's display some information about the current game:

- Number of asteroids remaining
- Lives remaining
- Time spent in the level

Tips:

- Start a new plugin `hud` (for heads up display)
- Add a system when entering the `Game` state that will display some text and start a `StopWatch`
  - Add a `StopWatch` as a resource
  - Spawn an entity with a `Text` component
  - Spawn children with a `TextSpan` component. Text spans make it easy to change the style of the text, or to target a specific part for editing
- Add a system that will update the text
  - Target the entity with the `Text` component
  - Use the `TextWriter` system parameter to update the text
  - And tick the stopwatch
