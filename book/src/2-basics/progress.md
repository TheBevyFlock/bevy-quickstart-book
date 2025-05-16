# Progress Report

## What You've learned

- Loading sprites and displaying them
  - With the [`AssetServer::load`](https://docs.rs/bevy/0.16.0/bevy/asset/struct.AssetServer.html#method.load) function
  - By adding the [`Sprite`](https://docs.rs/bevy/0.16.0/bevy/prelude/struct.Sprite.html) component
- Defining components
  - With [required components](https://docs.rs/bevy/0.16.0/bevy/ecs/component/trait.Component.html#required-components) to simplify adding related components
  - And using Zero Sized Types as tag components to filter entities in queries
- Handling user input
  - Reading the [`ButtonInput<T>`](https://docs.rs/bevy/0.16.0/bevy/input/struct.ButtonInput.html) resource
  - For the input [`KeyCode`](https://docs.rs/bevy/0.16.0/bevy/input/keyboard/enum.KeyCode.html)
- Writing more complex queries, and updating components
  - The [`With`](https://docs.rs/bevy/0.16.0/bevy/ecs/prelude/struct.With.html) and [`Without`](https://docs.rs/bevy/0.16.0/bevy/ecs/prelude/struct.Without.html) query filters
  - And using [`&mut`](https://docs.rs/bevy/0.16.0/bevy/ecs/change_detection/struct.Mut.html) to query data mutably
- Error handling in systems
  - Using [`Result`](https://docs.rs/bevy/0.16.0/bevy/ecs/error/type.Result.html) to handle errors in systems
  - Setting the [global error handler](https://docs.rs/bevy/0.16.0/bevy/ecs/error/static.GLOBAL_ERROR_HANDLER.html) to something else than panic
- Third Party Plugins
  - Explore [community assets](https://bevyengine.org/assets/#assets)
  - Integrate a third party plugin
