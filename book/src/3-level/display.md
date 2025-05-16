# Displaying the Level

Loading an asset is an asynchronous process. As it involves file or network access, it doesn't happen immediately. This is why the asset server is returning an [`Handle`](https://docs.rs/bevy/0.16.0/bevy/asset/enum.Handle.html) instead of the data.

Accessing the data from the [`Assets<T>`](https://docs.rs/bevy/0.16.0/bevy/asset/struct.Assets.html) resource returns an `Option<T>` as the data may not be present yet. In our case, we're using the 2 second delay of the splash screen to be sure that assets are done loading, so we can `unwrap()` the `Option`.

```rust
# extern crate bevy;
# use bevy::prelude::*;
# #[derive(Asset, TypePath)]
# struct Level {width: u32, height: u32, asteroids: u32, lives: u32}
# #[derive(Resource)]
# struct GameAssets {
# }
# #[derive(Resource)]
# pub struct LoadedLevel { pub level: Handle<Level> }
# #[derive(Component)]
# struct Asteroid;
# #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
# enum GameState { #[default] Game }
fn display_level(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    loaded_level: Res<LoadedLevel>,
    levels: Res<Assets<Level>>,
) {
    let level = levels.get(&loaded_level.level).unwrap();

    // do something with the level
}
```

<div class="warning">

Waiting for some time is not a good general solution to ensure assets are loaded: the actual delay will depend on the number of asset, the disk and CPU of the player.

There are different ways to go around that: you can [poll assets](https://docs.rs/bevy/latest/bevy/asset/struct.Assets.html#method.get) in a system to check that they are available, you can wait for [asset events](https://docs.rs/bevy/latest/bevy/asset/enum.AssetEvent.html) or you
can [use a guard when loading assets](https://docs.rs/bevy/latest/bevy/asset/struct.AssetServer.html#method.load_acquire).

A third party plugin that handles asset loading through states is [bevy_asset_loader](https://crates.io/crates/bevy_asset_loader).

</div>
