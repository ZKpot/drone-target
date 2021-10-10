use rapier3d::{
    dynamics::{ RigidBodyBuilder, BodyStatus, RigidBodySet, RigidBodyHandle },
    geometry::{ ColliderSet, ColliderBuilder, },
};

use dotrix::{
    Transform,
    Pipeline,
    pbr:: { Model, Material, },
    services::{ Assets, World, },
    math::{ Point3, Vec3, },
    ecs::{ Mut, },
};

// beam size
const SCALE: f32 = 5.0;

pub struct Stats {
    pub gravity_radius:    f32,
    pub gravity_max_force: f32,

    pub radius_near:   f32,
    pub radius_medium: f32,
    pub radius_far:    f32,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            gravity_radius:    125.0,
            gravity_max_force:   2.5,

            radius_near:    25.0,
            radius_medium:  75.0,
            radius_far:    125.0,
        }
    }
}

pub fn gravity(
    world: Mut<World>,
    mut bodies: Mut<RigidBodySet>,
) {
    // Query the beams
    let beams_query =
        world.query::<(&mut RigidBodyHandle, &mut Stats)>();

    for (beam_rigid_body, beam_stats) in beams_query {

        let beam_body = bodies.get_mut(*beam_rigid_body).unwrap();
        let beam_position = beam_body.position().translation;

        // Query all rigid bodies
        let objects_query = world.query::<(&mut RigidBodyHandle, )>();

        for (rigid_body, ) in objects_query {

            let body = bodies.get_mut(*rigid_body).unwrap();
            let position = body.position().translation;

            let distance = nalgebra::distance(
                &nalgebra::Point3::new(position.x, position.y, position.z,),
                &nalgebra::Point3::new(
                    beam_position.x, beam_position.y, beam_position.z)
            );

            if distance < beam_stats.gravity_radius {
                let gravity_force = beam_stats.gravity_max_force *
                    (1.0 - distance/beam_stats.gravity_radius);

                let direction =
                    (beam_position.vector - position.vector).normalize();

                body.apply_force(direction * gravity_force, true);
            }



        }
    }
}

pub fn spawn(
    world: &mut World,
    assets: &mut Assets,
    bodies: &mut RigidBodySet,
    colliders: &mut ColliderSet,
    position: Point3,
) {
    let texture = assets.register("energy_beam::texture");
    let mesh = assets.register("energy_beam::mesh");

    let rigid_body = RigidBodyBuilder::new(BodyStatus::Static)
        .translation(position.x, position.y, position.z)
        .build();

    let collider = ColliderBuilder::ball(1.0 * SCALE)
        .build();

    let body_handle = bodies.insert(rigid_body);

    colliders.insert(collider, body_handle, bodies);

    world.spawn(Some((
        Model::from(mesh),
        Material {
            texture,
            ..Default::default()
        },
        Transform {
            translate: Vec3::new(position.x, position.y, position.z),
            scale: Vec3::new(0.57 * SCALE, 0.57 * SCALE, 0.57 * SCALE),
            ..Default::default()
        },
        body_handle,
        Stats { ..Default::default() },
        Pipeline::default(),
    )));
}