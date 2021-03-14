use super::{ physics, };

use rapier3d::{
    dynamics::{ RigidBodyBuilder, BodyStatus, },
    na::{ Vector3, Isometry3, },
};

use dotrix::{
    components:: { Model, },
    services::{ Assets, World, },
    math::{ Point3, Vec3, },
    renderer::transform::Transform,
};

pub struct Stats {
    pub is_player: bool,
    pub charge: f32,            // drone battery state of charge (0-100%)
    pub strike_charge: f32,     // energy to be used when strike is activated (0-100%)
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            is_player: false,
            charge: 0.0,
            strike_charge: 0.0
        }
    }
}

pub fn spawn(
    world: &mut World,
    assets: &mut Assets,
    bodies: &mut physics::BodiesService,
    position: Point3,
    stats: Stats,
) {
    let texture = assets.register("player::texture");
    let mesh = assets.register("player::mesh");

    let transform = Transform {
        translate: Vec3::new(position.x, position.y, position.z),
        ..Default::default()
    };

    let rigid_body = RigidBodyBuilder::new(BodyStatus::Dynamic)
        .position(Isometry3::new(
            Vector3::new(position.x, position.y, position.z),
            Vector3::new(0.0, 0.0, 0.0))
        )
        .mass(0.1)
        .principal_angular_inertia(Vector3::new(1.0, 1.0, 1.0))
        .angular_damping(10.0)
        .linear_damping(1.0)
        .build();

    // spawn model in the world
    world.spawn(Some(
        (
            Model { mesh, texture, transform, ..Default::default() },
            physics::RigidBody::new(bodies.insert(rigid_body)),
            stats,
        ),
    ));
}
