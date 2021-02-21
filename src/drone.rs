use super::{ physics, Action, };

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

pub fn spawn(
    world: &mut World,
    assets: &mut Assets,
    bodies: &mut physics::BodiesService,
    position: Point3,
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
        ),
    ));
}

pub fn player_control(
    world: Mut<World>,
    mut bodies: Mut<physics::BodiesService>,
    input: Const<Input>,
) {
    // Query player entity
    let query = world.query::<(&mut Model, &mut physics::RigidBody)>();

    // this loop will run only once, because Player component is assigned to only one entity
    for (model, rigid_body) in query {

        let body = bodies.bodies.get_mut(rigid_body.rigid_body_handle).unwrap();
        
        if input.is_action_hold(Action::MoveForward) {
            body.set_linvel(Vector3::y() * 1.0, true);
        };

        let pos = body.position().translation;
    
        // apply translation
        model.transform.translate.x = pos.x;
        model.transform.translate.y = pos.y;
        model.transform.translate.z = pos.z;       
    }
}
