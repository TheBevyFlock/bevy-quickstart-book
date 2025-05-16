# Custom Asset Format

## Level Format

We'll load the level information from a basic text file. The information we want from it are:

- Width and height of the level
- Number of asteroids to spawn
- Number of lives of the player

## Asset Type

To match the basic level format, we'll use a basic struct that will hold four `u32`s. The struct must derive the [`Asset`](https://docs.rs/bevy/0.16.0/bevy/asset/trait.Asset.html) trait.

```rust
# extern crate bevy;
# use bevy::prelude::*;
#[derive(Asset, TypePath)]
pub struct Level {
    pub width: u32,
    pub height: u32,
    pub asteroids: u32,
    pub lives: u32,
}
```

## Asset Loader

Let's add `thiserror` as a dependency, this will help us when declaring the kind of errors that can happen when loading our file.

```sh
cargo add thiserror
```

To load this format, we'll read the file character by character, then choose the right tile depending on the character. Bevy expects custom asset loader to implement the trait [`AssetLoader`](https://docs.rs/bevy/0.16.0/bevy/asset/trait.AssetLoader.html).

```rust
# extern crate bevy;
# extern crate thiserror;
# use bevy::{asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext}, prelude::*};
# use thiserror::Error;
# #[derive(Asset, TypePath)]
# struct Level {width: u32, height: u32, asteroids: u32, lives: u32}
#[derive(Default)]
struct LevelLoader;

#[derive(Debug, Error)]
enum LevelLoaderError {
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    #[error("Error in file format")]
    FormatError,
}

impl AssetLoader for LevelLoader {
    type Asset = Level;
    type Settings = ();
    type Error = LevelLoaderError;
    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut buf = String::new();
        reader.read_to_string(&mut buf).await?;

        let mut lines = buf.lines();
        Ok(Level {
            width: lines
                .next()
                .and_then(|s| s.parse().ok())
                .ok_or(LevelLoaderError::FormatError)?,
            height: lines
                .next()
                .and_then(|s| s.parse().ok())
                .ok_or(LevelLoaderError::FormatError)?,
            asteroids: lines
                .next()
                .and_then(|s| s.parse().ok())
                .ok_or(LevelLoaderError::FormatError)?,
            lives: lines
                .next()
                .and_then(|s| s.parse().ok())
                .ok_or(LevelLoaderError::FormatError)?,
        })
    }

    fn extensions(&self) -> &[&str] {
        &["bw"]
    }
}
```

## Loading the Level

Custom asset formats and loaders must be initiated in the application with [`App::init_asset`](https://docs.rs/bevy/0.16.0/bevy/app/struct.App.html#method.init_asset) and [`App::init_asset_loader`](https://docs.rs/bevy/0.16.0/bevy/asset/trait.AssetApp.html#tymethod.init_asset_loader). We can wrap that in a plugin.

```rust
# extern crate bevy;
# extern crate thiserror;
# use bevy::{asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext}, prelude::*};
# use thiserror::Error;
# #[derive(Asset, TypePath)]
# struct Level {width: u32, height: u32, asteroids: u32, lives: u32}
# #[derive(Default)]
# struct LevelLoader;
# #[derive(Debug, Error)]
# enum LevelLoaderError {}
# impl AssetLoader for LevelLoader {
#     type Asset = Level;
#     type Settings = ();
#     type Error = LevelLoaderError;
#     async fn load(&self, reader: &mut dyn Reader, _settings: &(), _load_context: &mut LoadContext<'_>) -> Result<Self::Asset, Self::Error> { unimplemented!() }
#     fn extensions(&self) -> &[&str] { &["bw"] }
# }
pub fn level_loader_plugin(app: &mut App) {
    app.init_asset::<Level>().init_asset_loader::<LevelLoader>();
}
```

<div class="warning">

Don't forget to add the new `level_loader_plugin` to the app in the `main.rs` file.

</div>

Now we can load the asset file like the sprites we're already using, and keeping the handle to the loaded level in a resource.

```rust
# extern crate bevy;
# use bevy::{asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext}, prelude::*};
# #[derive(Asset, TypePath)]
# struct Level {width: u32, height: u32, asteroids: u32, lives: u32}
#[derive(Resource)]
pub struct LoadedLevel {
    pub level: Handle<Level>,
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // ...
) {
    commands.insert_resource(LoadedLevel {
        level: asset_server.load("level.bw"),
    });
    // ...
}
```
