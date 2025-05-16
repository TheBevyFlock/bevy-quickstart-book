use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef, ShaderType},
    sprite::{Material2d, Material2dPlugin},
    window::WindowResized,
};
use rand::Rng;

use crate::GameState;

pub fn starfield_plugin(app: &mut bevy::prelude::App) {
    app.add_plugins(Material2dPlugin::<StarfieldMaterial>::default())
        .add_systems(OnEnter(GameState::Game), setup)
        .add_systems(
            PostUpdate,
            update_starfield.run_if(in_state(GameState::Game)),
        );
}

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

impl Material2d for StarfieldMaterial {
    fn fragment_shader() -> ShaderRef {
        ShaderRef::Path("starfield.wgsl".into())
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StarfieldMaterial>>,
    windows: Query<&Window>,
) {
    let window = windows.single().unwrap();
    let size = window.width().max(window.height());

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(StarfieldMaterial {
            position: Vec2::ZERO,
            seeds: Vec2::new(
                rand::thread_rng().gen_range(0.0..1000.0),
                rand::thread_rng().gen_range(0.0..1000.0),
            ),
        })),
        Transform::from_scale(Vec3::new(size, size, 1.0)),
        StateScoped(GameState::Game),
    ));
}

fn update_starfield(
    mut starfield: Query<(&mut Transform, &MeshMaterial2d<StarfieldMaterial>), Without<Camera2d>>,
    camera: Query<Ref<Transform>, With<Camera2d>>,
    mut materials: ResMut<Assets<StarfieldMaterial>>,
    mut resized: EventReader<WindowResized>,
) {
    let camera_transform = camera.single().unwrap();
    if camera_transform.is_changed() {
        let (mut starfield_transform, material) = starfield.single_mut().unwrap();

        starfield_transform.translation = camera_transform.translation.with_z(-2.0);

        let material = materials.get_mut(&material.0).unwrap();
        material.position = camera_transform.translation.xy();
    }

    if let Some(resized) = resized.read().last() {
        let (mut starfield_transform, _) = starfield.single_mut().unwrap();

        starfield_transform.scale.x = resized.width.max(resized.height);
        starfield_transform.scale.y = resized.width.max(resized.height);
    }
}
