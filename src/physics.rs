use bevy::{
    math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume},
    prelude::*,
};

#[derive(Component)]
pub struct LinearVelocity(pub Vec2);

#[derive(Component)]
pub struct AngularVelocity(pub f32);

#[derive(Component)]
pub struct AngularDamping(pub f32);

#[derive(Component)]
pub struct Collider(pub Aabb2d);

impl Collider {
    pub fn rectangle(width: f32, height: f32) -> Self {
        Self(Aabb2d::new(Vec2::ZERO, Vec2::new(width, height)))
    }
}

#[derive(Component)]
pub struct CollisionEventsEnabled;

#[derive(Event)]
pub struct OnCollisionStart {
    pub collider: Entity,
}

pub fn physics_plugin(app: &mut App) {
    app.add_systems(FixedUpdate, (update_positions, update_rotations));

    app.add_systems(
        PostUpdate,
        (find_collisions).after(TransformSystem::TransformPropagate),
    );

    #[cfg(feature = "debug")]
    app.add_systems(
        PostUpdate,
        debug_draw_colliders.after(TransformSystem::TransformPropagate),
    );
}

fn update_positions(time: Res<Time<Fixed>>, mut query: Query<(&mut Transform, &LinearVelocity)>) {
    let delta = time.delta_secs();
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += (velocity.0 * delta).extend(0.0);
    }
}

fn update_rotations(
    time: Res<Time<Fixed>>,
    mut query: Query<(
        &mut Transform,
        &mut AngularVelocity,
        Option<&AngularDamping>,
    )>,
) {
    let delta = time.delta_secs();
    for (mut transform, mut angular_velocity, damping) in query.iter_mut() {
        let rotation = Quat::from_rotation_z(angular_velocity.0 * delta);
        transform.rotation *= rotation;
        if let Some(damping) = damping {
            angular_velocity.0.smooth_nudge(&0.0, damping.0, delta);
        }
    }
}

fn find_collisions(
    mut commands: Commands,
    colliders: Query<(
        Entity,
        &GlobalTransform,
        &Collider,
        Option<&CollisionEventsEnabled>,
    )>,
) {
    for (entity_a, transform_a, collider_a, events_enabled_a) in colliders.iter() {
        let collider_a = collider_a
            .0
            .translated_by(transform_a.translation().truncate());
        for (entity_b, transform_b, collider_b, events_enabled_b) in colliders.iter() {
            let collider_b = collider_b
                .0
                .translated_by(transform_b.translation().truncate());
            if entity_a != entity_b && collider_a.intersects(&collider_b) {
                if events_enabled_a.is_some() {
                    commands.trigger_targets(OnCollisionStart { collider: entity_b }, entity_a);
                }
                if events_enabled_b.is_some() {
                    commands.trigger_targets(OnCollisionStart { collider: entity_a }, entity_b);
                }
            }
        }
    }
}

#[allow(dead_code)]
fn debug_draw_colliders(mut gizmos: Gizmos, query: Query<(&GlobalTransform, &Collider)>) {
    for (global_transform, collider) in query.iter() {
        let size = collider.0.max - collider.0.min;
        gizmos.rect_2d(
            global_transform.translation().truncate(),
            size,
            bevy::color::palettes::basic::RED,
        );
    }
}
