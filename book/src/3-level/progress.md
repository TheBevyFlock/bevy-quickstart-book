# Progress Report

## What You've learned

- Loading a custom asset file
  - Creating a custom asset by defining a struct deriving the [`Asset`](https://docs.rs/bevy/0.16.0/bevy/asset/trait.Asset.html) trait
  - And implementing the [`AssetLoader`](https://docs.rs/bevy/0.16.0/bevy/asset/trait.AssetLoader.html) trait to load a file into this struct
- Getting an asset
  - Using the [`Assets<T>`](https://docs.rs/bevy/0.16.0/bevy/asset/struct.Assets.html) resource

## Going Further

Assets can be hot-reloaded. This can be useful during development, to be able to quickly change the level without recompiling and restarting the game.

- It needs to enable a feature on Bevy: `file_watcher`
- Check if the asset changed, then despawn the level and respawn it from the updated file
