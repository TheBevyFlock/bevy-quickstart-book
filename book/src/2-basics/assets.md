# Using Assets

Let's improve on those blocks of colors! We'll start by loading a spritesheet for the player

## Loading Assets

Loading assets is asynchronous, and returns an [`Handle`](https://docs.rs/bevy/0.16.0/bevy/asset/enum.Handle.html) to its data. By adding a system to our splash screen, we ensure it happens as early as possible.

```rust,no_run
# extern crate bevy;
# use bevy::prelude::*;
#[derive(Resource)]
struct GameAssets {
    player_ship: Handle<Image>,
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(GameAssets {
        player_ship: asset_server.load("playerShip1_green.png"),
    });
}
```

<div class="warning">

Don't forget to add the new `load_assets` system to the `splash_plugin`, when entering the `GameState::Splash` state.

</div>

## Displaying Those Assets

Now that we have a texture atlas, we can use it to display a sprite for our player instead of a block of red.

```rust,no_run
# extern crate bevy;
# use bevy::prelude::*;
# #[derive(Resource)]
# struct GameAssets {
#     player_ship: Handle<Image>,
# }
# #[derive(Component)]
# struct Player;
# #[derive(Component)]
# struct Ground;
# #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
# enum GameState { #[default] Game }
fn display_level(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands.spawn((
        Sprite::from_image(game_assets.player_ship.clone()),
        Player,
        StateScoped(GameState::Game),
    ));

    commands.spawn((
        Sprite::from_color(Color::linear_rgb(0.0, 1.0, 0.0), Vec2::new(1000.0, 80.0)),
        Transform::from_xyz(300.0, -200.0, 0.0),
        Ground,
        StateScoped(GameState::Game),
    ));
}
```
