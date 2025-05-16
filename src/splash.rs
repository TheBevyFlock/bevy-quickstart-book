use bevy::{core_pipeline::bloom::Bloom, prelude::*};

use crate::{AudioAssets, GameAssets, GameState, LoadedLevel};

pub fn splash_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Splash), (display_title, load_assets))
        .add_systems(Update, switch_to_menu.run_if(in_state(GameState::Splash)));
}

fn display_title(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            hdr: true,
            ..default()
        },
        Bloom::default(),
    ));

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children![
            (
                Text::new("Bevy Quickstart"),
                TextFont {
                    font_size: 130.0,
                    ..default()
                },
            ),
            (
                Text::new("Rust Week 2025"),
                TextFont {
                    font_size: 100.0,
                    ..default()
                },
            )
        ],
        StateScoped(GameState::Splash),
    ));

    commands.insert_resource(SplashScreenTimer(Timer::from_seconds(2.0, TimerMode::Once)));
}

#[derive(Resource)]
struct SplashScreenTimer(Timer);

fn switch_to_menu(
    mut next: ResMut<NextState<GameState>>,
    mut timer: ResMut<SplashScreenTimer>,
    time: Res<Time>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        next.set(GameState::StartMenu);
    }
}

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        player_ship: asset_server.load("textures/playerShip1_green.png"),
        asteroid: asset_server.load("textures/meteorBrown_big1.png"),
        jets: asset_server.load("textures/fire07.png"),
        explosion: asset_server.load("textures/explosion00.png"),
        laser: asset_server.load("textures/laserRed07.png"),
        jet_particles: asset_server.load("particles/jet.particle.ron"),
    });
    commands.insert_resource(LoadedLevel {
        level: asset_server.load("levels/level.bw"),
    });
    commands.insert_resource(AudioAssets {
        laser: asset_server.load("audio/laser.wav"),
        ship_explosion: asset_server.load("audio/ship_explosion.wav"),
        asteroid_explosion: asset_server.load("audio/asteroid_explosion.wav"),
        start: asset_server.load("audio/start.wav"),
        win: asset_server.load("audio/win.wav"),
        lose: asset_server.load("audio/lose.wav"),
        game_loop: asset_server.load("audio/Mission Plausible.wav"),
    });
}
