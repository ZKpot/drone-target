use super::{ physics, };

use rapier3d::{
    dynamics::{ RigidBodyBuilder, BodyStatus, },
    na::{ Vector3, Isometry3, },
};

use dotrix::{
    components:: { Model, },
    ecs::{ Mut, Const, },
    services::{ Assets, World, Input, },
    math::{ Point3, Vec3, },
    renderer::transform::Transform,
};

pub struct Stats {
    pub isPlayer: bool,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            isPlayer: false,
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
            Vector3::y())
        )
        .mass(0.1)
        .build();

    // spawn model in the world
    world.spawn(Some(
        (
            Model { mesh, texture, transform, ..Default::default() },
            physics::RigidBody::new(bodies.bodies.insert(rigid_body)),
            stats,
        ),
    ));
}
