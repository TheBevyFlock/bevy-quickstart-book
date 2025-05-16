# Progress Report

## What You've learned

- Defining a custom material
  - With the [`AsBindGroup`](https://docs.rs/bevy/0.16.0/bevy/render/render_resource/trait.AsBindGroup.html) derive and its attributes to handle data transfer to the GPU
  - Implementing the [`Material2d`](https://docs.rs/bevy/0.16.0/bevy/sprite/trait.Material2d.html) trait to define the shader
  - And some basic WGSL
- And using that material
  - Adding it to the app with the [`Material2dPlugin`](https://docs.rs/bevy/0.16.0/bevy/sprite/struct.Material2dPlugin.html)
  - With the [`Mesh2d`](https://docs.rs/bevy/0.16.0/bevy/prelude/struct.Mesh2d.html) component to define the shape
  - And the [`MeshMaterial2d`](https://docs.rs/bevy/0.16.0/bevy/prelude/struct.MeshMaterial2d.html) component to define the material
- Enabling bloom
  - How to enable bloom on the camera
  - How to have a color display a bloom effect by going higher than `1.0` on one of the color channel
- And getting a little more effects with particles
  - Using a third party plugin, `bevy_enoki`

## Going Further

Shaders and rendering is a _very big_ domain. You can start by reading the [Book of Shaders](https://thebookofshaders.com) and the [Learn WGPU](https://sotrh.github.io/learn-wgpu/) tutorial.
