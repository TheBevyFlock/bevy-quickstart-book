# Background

We'll build a first shader for the background that will displayed some stars.

Let's create a new plugin for it, we'll call it `starfield`!

We want our starfield to be different each time it's loaded, so we will seed it with random values. We also want some kind of parallax effect, where bigger stars in the foreground appear closer than smaller stars in the background. To achieve that we will also pass the current player position to the shader.

## Custom GPU type

First step is to declare the data we'll send to the GPU:

```rust
# extern crate bevy;
# use bevy::{prelude::*, render::render_resource::{AsBindGroup, ShaderRef, ShaderType}};
#[derive(Asset, TypePath, AsBindGroup, ShaderType, Debug, Clone)]
#[uniform(0, StarfieldMaterial)]
pub struct StarfieldMaterial {
    position: Vec2,
    seeds: Vec2,
}

impl<'a> From<&'a StarfieldMaterial> for StarfieldMaterial {
    fn from(material: &'a StarfieldMaterial) -> Self {
        material.clone()
    }
}
```

By deriving the [`AsBindGroup`](https://docs.rs/bevy/0.16.0/bevy/render/render_resource/trait.AsBindGroup.html) trait and annotating with `uniform`, Bevy will be able to know how to transform the data from Rust type to what is expected by the GPU.

You can add the `uniform` annotation on fields, and for most common types Bevy knows out of the box how to convert them in a format understanble by the GPU.

If you want to make available a type you defined, you'll need to derive the [`ShaderType`](https://docs.rs/bevy/0.16.0/bevy/render/render_resource/trait.ShaderType.html) trait on it. Then by using that type in the uniform, Bevy will know how to send data to the GPU.

Here, the data has the same types on CPU or GPU, so we're able to use the same type for both representation. A more complete version would be:

```rust
# extern crate bevy;
# use bevy::{prelude::*, render::render_resource::{AsBindGroup, ShaderRef, ShaderType}};
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
#[uniform(0, StarfieldUniform)]
pub struct StarfieldMaterial {
    position: Vec2,
    seeds: Vec2,
}

#[derive(ShaderType, Debug, Clone)]
pub struct StarfieldUniform {
    position: Vec2,
    seeds: Vec2,
}

impl<'a> From<&'a StarfieldMaterial> for StarfieldUniform {
    fn from(material: &'a StarfieldMaterial) -> Self {
        StarfieldUniform {
            position: material.position,
            seeds: material.seeds,
        }
    }
}
```

`StarfieldMaterial` will be the material used on the CPU, and `StarfieldUniform` the data used on the GPU.

<div class="warning">

In this case, our material is made of two `Vec2`, and will work fine on all platforms. WebGL2 in particular need uniforms to be 16 bytes aligned or will crash.

The two strategies to solve that are padding and packing. Padding is using bigger types than necessary and wasting memory, packing is grouping fields that have separate meaning in a single type.

</div>

## Custom Material

Next is to define the shader that will be used to render the data. This is done by implementing the [`Material2d`](https://docs.rs/bevy/0.16.0/bevy/sprite/trait.Material2d.html) trait:

```rust
# extern crate bevy;
# use bevy::{
#     prelude::*,
#     render::render_resource::{AsBindGroup, ShaderRef},
#     sprite::{AlphaMode2d, Material2d, Material2dPlugin},
# };
# #[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
# pub struct StarfieldMaterial {}
impl Material2d for StarfieldMaterial {
    fn fragment_shader() -> ShaderRef {
        "starfield.wgsl".into()
    }
}
```

The trait has more customisation than used here, and use sane defaults. By just using a string for the fragment shader, Bevy will load the file specified from the asset folder.

This is a basic shader that will display the sprite selected by the `index` from a sprite sheet:

```wgsl
#import bevy_sprite::{
    mesh2d_vertex_output::VertexOutput,
    mesh2d_view_bindings::globals,
}

struct Material {
    coords: vec2<f32>,
    seeds: vec2<f32>,
}

@group(2) @binding(0)
var<uniform> material: Material;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var result = vec3<f32>(0.0, 0.0, 0.0);

    // ...

    return vec4<f32>(result, 1.0);
}
```

Bevy has some extensions to WGSL to allow imports and expose some helpful features.

Variables with the `@group(2)` will match the bind group declared on Rust side.

## Using the Material

Our new material must be added to Bevy before it can be used. This can be done in a plugin:

```rust
# extern crate bevy;
# use bevy::{
#     prelude::*,
#     render::render_resource::{AsBindGroup, ShaderRef, ShaderType},
#     sprite::{Material2d, Material2dPlugin},
# };
# #[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
# pub struct StarfieldMaterial {}
# impl Material2d for StarfieldMaterial {}
# #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
# enum GameState { #[default] Game };
# pub fn setup() {}
# pub fn update_starfield() {}
pub fn starfield_plugin(app: &mut bevy::prelude::App) {
    app.add_plugins(Material2dPlugin::<StarfieldMaterial>::default())
        .add_systems(OnEnter(GameState::Game), setup)
        .add_systems(
            PostUpdate,
            update_starfield.run_if(in_state(GameState::Game)),
        );
}
```

We're adding two systems:

- `setup` which will spawn the material on the background
- `update_starfield` which will send the updated player position to the material

`setup` looks like this:

```rust
# extern crate bevy;
# extern crate rand;
# use bevy::{
#     prelude::*,
#     render::render_resource::{AsBindGroup, ShaderRef, ShaderType},
#     sprite::{Material2d, Material2dPlugin},
# };
# use rand::Rng;
# #[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
# pub struct StarfieldMaterial { position: Vec2, seeds: Vec2 }
# impl Material2d for StarfieldMaterial {}
# #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
# enum GameState { #[default] Game };
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StarfieldMaterial>>,
    windows: Query<&Window>,
) {
    let window = windows.single().unwrap();
    let size = window.width().max(window.height());

    commands.spawn((
        // Apply the material to a square
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(StarfieldMaterial {
            // At start, player position is (0.0, 0.0)
            position: Vec2::ZERO,
            // Seed the material with random values
            seeds: Vec2::new(
                rand::thread_rng().gen_range(0.0..1000.0),
                rand::thread_rng().gen_range(0.0..1000.0),
            ),
        })),
        // Scale up the material so that it covers the whole screen
        Transform::from_scale(Vec3::new(size, size, 1.0)),
        StateScoped(GameState::Game),
    ));
}
```

`update_starfield` will update the `position` field in our material with the current player position, and will also change the material scale in case the window is resized.

```rust
# extern crate bevy;
# extern crate rand;
# use bevy::{
#     prelude::*,
#     render::render_resource::{AsBindGroup, ShaderRef, ShaderType},
#     sprite::{Material2d, Material2dPlugin},
#     window::WindowResized,
# };
# use rand::Rng;
# #[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
# pub struct StarfieldMaterial { position: Vec2, seeds: Vec2 }
# impl Material2d for StarfieldMaterial {}
fn update_starfield(
    mut starfield: Query<(&mut Transform, &MeshMaterial2d<StarfieldMaterial>), Without<Camera2d>>,
    camera: Query<Ref<Transform>, With<Camera2d>>,
    mut materials: ResMut<Assets<StarfieldMaterial>>,
    mut resized: EventReader<WindowResized>,
) {
    // As the camera follows the player, take the camera transform
    let camera_transform = camera.single().unwrap();
    if camera_transform.is_changed() {
        let (mut starfield_transform, material) = starfield.single_mut().unwrap();

        // Change the starfield transform so that it stays in sync with the camera
        starfield_transform.translation = camera_transform.translation.with_z(-2.0);

        // Update the position in the material
        let material = materials.get_mut(&material.0).unwrap();
        material.position = camera_transform.translation.xy();
    }

    if let Some(resized) = resized.read().last() {
        let (mut starfield_transform, _) = starfield.single_mut().unwrap();

        // Window size changed, update the size of the mesh showing the material
        starfield_transform.scale.x = resized.width.max(resized.height);
        starfield_transform.scale.y = resized.width.max(resized.height);
    }
}
```

## Let's Put Some Stars in the Sky!

Right now our shader is just displaying the emptiness of space... everything is black.

To have "stars", we'll want to display some specks of white at some points. There are plenty of examples we can take inspiration on [Shadertoy](https://www.shadertoy.com/results?query=starfield).

We'll first define two "random" functions. They are actually deterministic, which helps with keeping the stars in place.

```wgsl
// Returns a single f32 for a position
fn rand(p: vec2<f32>) -> f32 {
    return fract(sin(dot(p, vec2<f32>(54.90898, 18.233))) * 4337.5453);
}

// Returns two f32 for a position
fn rand2(p: vec2<f32>) -> vec2<f32> {
    let p2 = vec2<f32>(dot(p, vec2<f32>(12.9898, 78.233)), dot(p, vec2<f32>(26.65125, 83.054543)));
    return fract(sin(p2) * 43758.5453);
}
```

Using those two functions, we can create a starfield! This is the most complicated part of the shader, and not really linked to Bevy. It takes the density of stars we want, their size and their brightness, and for each point on screen will return if it's in a star or not.

```wgsl
fn stars(position: vec2<f32>, density: f32, size: f32, brightness: f32) -> f32 {
    let n = position * density;
    let f = floor(n);

    var d = 1.0e10;
    for (var i = -1; i <= 1; i = i + 1) {
        for (var j = -1; j <= 1; j = j + 1) {
            var g = f + vec2<f32>(f32(i), f32(j));
            g = n - g - rand2(g % density) + rand(g);
            g = g / (density * size);
            d = min(d, dot(g, g));
        }
    }

    return brightness * (smoothstep(.95, 1., (1. - sqrt(d))));
}
```

We can now call this function in our `fragment` shader to be able to draw stars!

```wgsl
@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var result = vec3<f32>(0.0, 0.0, 0.0);

    result = result + stars(in.uv, 30.0, 0.025, 0.5);

    return vec4<f32>(result, 1.0);
}
```

It's a start, but very bland for now: stars are just points on screen that don't move.

To create a parallax effect, we want different layers of stars that don't move at the same speed.

```wgsl
    result = result + stars(in.uv - coords / (1000.0 * 1.2), 3.0, 0.025, 2.0);
    result = result + stars(in.uv - coords / (1000.0 * 1.4), 10.0, 0.018, 1.0);
    result = result + stars(in.uv - coords / (1000.0 * 2.0), 30.0, 0.015, 0.5);
```

This will create three layers of stars, with different sizes, and not moving at the same speed relative to the player.
