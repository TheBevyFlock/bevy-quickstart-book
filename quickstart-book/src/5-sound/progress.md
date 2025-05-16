# Progress Report

## What You've learned

- Playing a sound reacting to an action by the user
  - Loading audio assets of the [`AudioSource`](https://docs.rs/bevy/0.16.0/bevy/audio/struct.AudioSource.html) asset type
  - And playing them by spawning the [`AudioPlayer`](https://docs.rs/bevy/0.16.0/bevy/audio/struct.AudioPlayer.html) component
  - Controlling playback settings with the [`PlaybackSettings`](https://docs.rs/bevy/0.16.0/bevy/audio/struct.PlaybackSettings.html) component
- How events work
  - With [`App::add_event`](https://docs.rs/bevy/0.16.0/bevy/app/struct.App.html#method.add_event) to register an event
  - Then [`EventWriter`](https://docs.rs/bevy/0.16.0/bevy/ecs/event/struct.EventWriter.html) to send events
  - And [`EventReader`](https://docs.rs/bevy/0.16.0/bevy/ecs/event/struct.EventReader.html) to iterate on them
- Playing a background music

## Going Further

This workshop uses wav files as they are easier to generate from tools. In a released game, I would recommend another format, mostly ogg, as it has better compression.
