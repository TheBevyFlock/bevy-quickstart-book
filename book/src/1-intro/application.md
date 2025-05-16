# The Application

The initial goal is to open a window using Bevy!

## Empty Application

Let's start a new project with Bevy

```sh
cargo new game
cd game
```

We can add Bevy 0.16 with the default features enabled:

```sh
cargo add bevy@0.16

Updating crates.io index
  Adding bevy v0.16 to dependencies
         Features as of v0.16.0:
         41 activated features
         68 deactivated features
Updating crates.io index
 Locking 468 packages to latest Rust 1.86.0 compatible versions
```

Bevy exposes a lot of features, 109 for the 0.16! [The full list of features is available in the documentation](https://docs.rs/bevy/0.16.0/bevy/#cargo-features). It is important to disable default features and only enable the ones you need. This will improve performance, compilation time and reduce binary size.

For this workshop, we'll use the following features:

```sh
cargo add bevy@0.16 --no-default-features --features "bevy_asset,bevy_audio,bevy_core_pipeline,bevy_render,bevy_sprite,bevy_state,bevy_text,bevy_ui,bevy_winit,default_font,multi_threaded,bevy_gizmos,wav,png,x11,wayland,webgl2"
```

<div class="warning">

You can add all the dependencies used in this workshop now to avoid recompilations later:

```sh
cargo add bevy@0.16 --no-default-features --features "bevy_asset,bevy_audio,bevy_core_pipeline,bevy_render,bevy_sprite,bevy_state,bevy_text,bevy_ui,bevy_winit,default_font,multi_threaded,bevy_gizmos,wav,png,x11,wayland,webgl2"
cargo add avian2d
cargo add bevy_enhanced_input
cargo add bevy_enoki
cargo add rand@0.8
cargo add thiserror
```

</div>

This is the most basic Bevy application. It will exit immediately upon running and perform no actions.

```rust
# extern crate bevy;
use bevy::prelude::*;

fn main() {
    App::new().run();
}
```

## Default Bevy Plugins

Default plugins are added to handle windowing, rendering, input, audio, and more. This application opens a window and then does nothing.

```rust,no_run
# extern crate bevy;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .run();
}
```

Plugins can be configured; in this example, we set a custom title for the window.

```rust,no_run
# extern crate bevy;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Quickstart Book".into(),
                ..default()
            }),
            ..default()
        }))
        .run();
}
```
