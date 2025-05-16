# Exercises

Don't forget to checkout the branch:

```sh
git checkout 08-player-actions
```

Let's review what was changed: <https://github.com/TheBevyFlock/bevy-quickstart-book/compare/07-level-loading..08-player-actions>

## Idiomatic Ship Collision Detection

Now we can remove the `collision` system, and check for asteroid collisions on our ship with an observer!

Tips:

- Remove the existing `collision` system from the `Update` schedule
- Add the `CollisionEventsEnabled` component to the ship entity
- Change the previous `collision` system to be an observer that will trigger on `Trigger<OnCollisionStart>`

## Can Destroy Asteroids

To destroy asteroid, we need to be able to fire lasers!

Tips:

- New action to fire lasers
- New sprite for lasers
- Spawn a laser when the action is fired
  - With a `Sprite`
  - With a `RigidBody`
  - With a `Collider`
  - With a `LinearVelocity`
  - With `CollisionEventsEnabled`
- Despawn lasers after a certain time
- Observe collisions between asteroids and lasers
- Despawn asteroid and laser when they collide

## Detect when all Asteroids are Destroyed

Switch to a win screen when all asteroids are destroyed

Tips:

- Add a new state `GameState::Won`
- Copy the menu and change the text and all the states used
- Add a system with a query on asteroids
- When there are no asteroids anymore, change state
