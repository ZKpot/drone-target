use rapier3d::{
    dynamics::{ RigidBodyBuilder, BodyStatus, RigidBodySet, },
    geometry::{ ColliderSet, ColliderBuilder, },
};

use dotrix::{
    components:: { Model, },
    services::{ Assets, World, },
    math::{ Point3, Vec3, },
    renderer::transform::Transform,
};

const SCALE: f32 = 5.0;

pub fn spawn(
    world: &mut World,
    assets: &mut Assets,
    bodies: &mut RigidBodySet,
    colliders: &mut ColliderSet,
    position: Point3,
) {
    let texture = assets.register("energy_beam::texture");
    let mesh = assets.register("energy_beam::mesh");

    let transform = Transform {
        translate: Vec3::new(position.x, position.y, position.z),
        scale: Vec3::new(0.57 * SCALE, 0.57 * SCALE, 0.57 * SCALE),
        ..Default::default()
    };

    let rigid_body = RigidBodyBuilder::new(BodyStatus::Static)
        .translation(position.x, position.y, position.z)
        .build();

    let collider = ColliderBuilder::ball(1.0 * SCALE)
        .build();

    let body_handle = bodies.insert(rigid_body);

    colliders.insert(collider, body_handle, bodies);

    world.spawn(Some(
        (
            Model { mesh, texture, transform, ..Default::default() },
            body_handle,
        ),
    ));
}