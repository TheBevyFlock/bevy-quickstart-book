// This shader is inspired by Start Nest by Pablo Roman Andrioli:
// https://www.shadertoy.com/view/XlfGRj

#import bevy_sprite::{
    mesh2d_vertex_output::VertexOutput,
    mesh2d_view_bindings::globals,
}

fn rand2(p: vec2<f32>) -> vec2<f32> {
    let p2 = vec2<f32>(dot(p, vec2<f32>(12.9898, 78.233)), dot(p, vec2<f32>(26.65125, 83.054543)));
    return fract(sin(p2) * 43758.5453);
}

fn rand(p: vec2<f32>) -> f32 {
    return fract(sin(dot(p, vec2<f32>(54.90898, 18.233))) * 4337.5453);
}

fn stars(x: vec2<f32>, num_cells: f32, size: f32, br: f32) -> f32 {
    let n = x * num_cells;
    let f = floor(n);

    var d = 1.0e10;
    for (var i = -1; i <= 1; i = i + 1) {
        for (var j = -1; j <= 1; j = j + 1) {
            var g = f + vec2<f32>(f32(i), f32(j));
			g = n - g - rand2(g % num_cells) + rand(g);
            g = g / (num_cells * size);
			d = min(d, dot(g, g));
        }
    }

    return br * (smoothstep(.95, 1., (1. - sqrt(d))));
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
    let coords = vec2<f32>(-material.coords.x, material.coords.y);
    let move_factor = 1000.0;
    let time_factor = 0.05;

    let intensity = clamp(rand(in.uv * globals.time), 0.4, 1.0);

    result = result + stars(in.uv + vec2<f32>(material.seeds.x, 0.0) - coords / (move_factor * 1.2), 3.0, 0.025, 2.0) * vec3<f32>(3.6, 3.6, 3.6) * intensity;
    result = result + stars(in.uv + vec2<f32>(material.seeds.y, 0.0) - coords / (move_factor * 1.4), 10.0, 0.018, 1.0) * vec3<f32>(6.7, 6.7, 6.7) * intensity;
    result = result + stars(in.uv + vec2<f32>(material.seeds.x, material.seeds.y) - coords / (move_factor * 2.0), 30.0, 0.015, 0.5) * vec3<f32>(.75, .75, .75) * intensity;

    return vec4<f32>(result * 6.0, 1.0);
}
