# Visual Effects

Enhance your game's visual appeal with effects. This is often achieved using shaders, which are programs that run on the GPU. The preferred language for writing them in Bevy is the [WebGPU Shading Language](https://www.w3.org/TR/WGSL/), which is translated as needed by the platform on which the application is running.

Bevy provides several abstractions for rendering:

- Directly using images, colors, or texture atlases, which is what we've been doing so far. The shaders are built into Bevy, optimized for performance at the expense of customization.
- Custom materials, which we'll explore in this section. For 2D, you'll need to implement the [`Material2d`](https://docs.rs/bevy/0.16.0/bevy/sprite/trait.Material2d.html) trait.
- Lower-level abstractions, offering complete control over the entire rendering pipeline. This won't be covered in this workshop.

Switch to the branch:

```sh
git checkout 10-visual-effects
```
