use super::physics;
use super::Action;
use rapier3d::dynamics::{ RigidBodyHandle, RigidBodyBuilder, BodyStatus, };
use rapier3d::na::{ Vector3, Isometry3, };

use dotrix::{
    assets:: { Id, Texture, Mesh, },
    components:: { Model, },
    ecs::{ Mut, Const, },
    services::{ World, Input },
    math::{ Point3, },
};

pub struct Drone{
    rigid_body_h: RigidBodyHandle,
}

impl Drone {
    pub fn new(position: Point3, bodies: &mut physics::BodiesService) -> Self {
        
        let rigid_body = RigidBodyBuilder::new(BodyStatus::Dynamic)
            .position(Isometry3::new(
                Vector3::new(position.x, position.y, position.z),
                Vector3::y())
            )
            .mass(0.1)
            .build();
        
        Self {
            rigid_body_h: bodies.bodies.insert(rigid_body),
        }
    }
}

pub fn init_drone(
    world: &mut World,
    mesh: Id<Mesh>,
    texture: Id<Texture>,
    drone: Drone,
) {
    // spawn model in the world
    world.spawn(Some(
        (
            Model { mesh, texture, ..Default::default() },
            drone,
        ),
    ));
}

pub fn player_control(
    world: Mut<World>,
    mut bodies: Mut<physics::BodiesService>,
    input: Const<Input>,
) {
    // Query player entity
    let query = world.query::<(&mut Model, &mut Drone)>();

    // this loop will run only once, because Player component is assigned to only one entity
    for (model, drone) in query {

        let rigid_body = bodies.bodies.get_mut(drone.rigid_body_h).unwrap();
        
        if input.is_action_hold(Action::MoveForward) {
            rigid_body.set_linvel(Vector3::y() * 1.0, true);
        };

        let pos = rigid_body.position().translation;
    
        // apply translation
        model.transform.translate.x = pos.x;
        model.transform.translate.y = pos.y;
        model.transform.translate.z = pos.z;       
    }
}