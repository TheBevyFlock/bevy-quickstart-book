# Firing Lasers

## Load an Audio Asset

We'll create a new resource to hold the handles to audio assets, and load it in the `load_assets` system.

```rust
# extern crate bevy;
# use bevy::prelude::*;
#[derive(Resource)]
pub struct AudioAssets {
    pub laser: Handle<AudioSource>,
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // ...
) {
    commands.insert_resource(AudioAssets {
        laser: asset_server.load("laser.wav"),
    });
    // ...
}

```

The build-in type for audio is [`AudioSource`](https://docs.rs/bevy/0.16.0/bevy/audio/struct.AudioSource.html).

## Trigger an Event to Play Audio

We'll trigger an event when we want to play audio. For now, that is when the player is starting to jump. To avoid triggering to many events, we should make sure that the player was not already jumping.

We'll start by declaring an event type:

```rust
# extern crate bevy;
# use bevy::prelude::*;
#[derive(Event)]
pub enum AudioStart {
    Laser,
}
```

To send an event, we can use the [`EventWriter`](https://docs.rs/bevy/0.16.0/bevy/ecs/event/struct.EventWriter.html) system parameter:

```rust
# extern crate bevy;
# extern crate bevy_enhanced_input;
# use bevy::prelude::*;
# use bevy_enhanced_input::prelude::*;
# use bevy::prelude::*;
# #[derive(Event)]
# pub enum AudioStart { Laser }
# #[derive(Debug, InputAction)]
# #[input_action(output = bool)]
# struct FireLaser;
fn fire_laser(
    trigger: Trigger<Fired<FireLaser>>,
    mut audio: EventWriter<AudioStart>
    // Other system parameters
) {
    # let target = trigger.target();
    // Other actions to fire a laser

    audio.write(AudioStart::Laser);
}
```

## Play Audio when Receiving the Event

To receive an event, we must use the [`EventReader`](https://docs.rs/bevy/0.16.0/bevy/ecs/event/struct.EventReader.html) system parameter, and by calling [`EventReader::read`](https://docs.rs/bevy/0.16.0/bevy/ecs/event/struct.EventReader.html#method.read) we can iterate over events.

To play audio, we must spawn an entity with the [`AudioPlayer`](https://docs.rs/bevy/0.16.0/bevy/audio/struct.AudioPlayer.html) component that will contain an [`Handle`](https://docs.rs/bevy/0.16.0/bevy/asset/enum.Handle.html) to the [`AudioSource`](https://docs.rs/bevy/0.16.0/bevy/audio/struct.AudioSource.html) asset.

By default, audio entities remain present once the audio is done playing. You can change this behaviour with the component [`PlaybackSettings::DESPAWN`](https://docs.rs/bevy/0.16.0/bevy/audio/struct.PlaybackSettings.html#associatedconstant.DESPAWN).

```rust
# extern crate bevy;
# use bevy::prelude::*;
# #[derive(Event)]
# enum AudioStart {Laser}
# #[derive(Resource)]
# struct AudioAssets { laser: Handle<AudioSource> }
fn play_audio(
    mut commands: Commands,
    mut audio_triggers: EventReader<AudioStart>,
    sound_assets: Res<AudioAssets>,
) {
    for trigger in audio_triggers.read() {
        match trigger {
            AudioStart::Laser => {
                commands.spawn((
                    AudioPlayer::<AudioSource>(sound_assets.laser.clone()),
                    PlaybackSettings::DESPAWN,
                ));
            }
        }
    }
}
```

We'll start a new plugin for all the audio related actions. Unlike events used with triggers and observers, events used with `EventWriter` and `EventReader` must be registered in the application with [`App::add_event`](https://docs.rs/bevy/0.16.0/bevy/app/struct.App.html#method.add_event). The plugin will register the event and add the system.

```rust
# extern crate bevy;
# use bevy::prelude::*;
# #[derive(Event)]
# enum AudioStart {Lasers}
# fn play_audio() {}
pub fn audio_plugin(app: &mut App) {
    app.add_event::<AudioStart>()
        .add_systems(Update, play_audio);
}
```
