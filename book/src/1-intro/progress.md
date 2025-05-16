# Progress Report

Let's review what was done: <https://github.com/TheBevyFlock/bevy-quickstart-book/compare/0-zero..05-intro-to-bevy>

## What You've learned

- Bevy dependencies, and its features
  - Disabling default features for build time and size, and for runtime performances
  - Knowing the [list of features](https://docs.rs/bevy/0.16.0/bevy/#cargo-features)
- Application creation and adding Bevy default plugins
  - Creating the [`App`](https://docs.rs/bevy/0.16.0/bevy/app/struct.App.html) struct
  - And adding the [`DefaultPlugins`](https://docs.rs/bevy/0.16.0/bevy/struct.DefaultPlugins.html)
- Schedules and adding systems
  - Adding system with [`App::add_systems`](https://docs.rs/bevy/0.16.0/bevy/app/struct.App.html#method.add_systems)
  - To a [`Schedule`](https://docs.rs/bevy/0.16.0/bevy/ecs/prelude/struct.Schedule.html)
  - From the [list of schedules](https://docs.rs/bevy/0.16.0/bevy/ecs/schedule/trait.ScheduleLabel.html#implementors)
- Basic use of commands and queries
  - The [`Commands`](https://docs.rs/bevy/0.16.0/bevy/ecs/prelude/struct.Commands.html) queue
  - To issue a commanddocs.rs/bevy/0.16.0/
  - And using a [`Query`](https://docs.rs/bevy/0.16.0/bevy/ecs/prelude/struct.Query.html) to access components
- States, and running system only on a state or during state transition
  - Using [`States`](https://docs.rs/bevy/0.16.0/bevy/prelude/trait.States.html) trait
  - And the [`OnEnter`](https://docs.rs/bevy/0.16.0/bevy/state/prelude/struct.OnEnter.html) state transition
  - With the [`NextState`](https://docs.rs/bevy/0.16.0/bevy/prelude/enum.NextState.html) resource
- Code organization with plugins
  - The [`Plugin`](https://docs.rs/bevy/0.16.0/bevy/app/trait.Plugin.html) trait
