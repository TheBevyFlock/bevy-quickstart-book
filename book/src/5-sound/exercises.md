# Exercises

Don't forget to checkout the branch:

```sh
git checkout 09-sound-effects
```

Let's review what was changed: <https://github.com/TheBevyFlock/bevy-quickstart-book/compare/08-player-actions..09-sound-effects>

## Explosions

Add sound for asteroid and ship explosions.

Tips:

- You can use [chiptone](https://sfbgames.itch.io/chiptone) or [jsfxr](https://sfxr.me) to quickly try new sound effects

## Other Events

Add sound for game start, winning and losing.

Tips:

- You can use [chiptone](https://sfbgames.itch.io/chiptone) or [jsfxr](https://sfxr.me) to quickly try new sound effects

## Background Music

Add a background music

Tips:

- You can use [`PlaybackSettings::LOOP`](https://docs.rs/bevy/0.16.0/bevy/audio/struct.PlaybackSettings.html#associatedconstant.LOOP) to play a looping audio
- You can set the volume lower than the default with [`PlaybackSettings::with_volume`](https://docs.rs/bevy/0.16.0/bevy/audio/struct.PlaybackSettings.html#method.with_volume)
