use super::{ physics, drone };

use rapier3d::{
    na::{ Vector3, Isometry3, },
    parry::simba::scalar::SupersetOf,
};

use dotrix::{
    components::{ Model, },
    ecs::{ Mut, Const, },
    services::{ Assets, World, Input, },
    math::{ Point3, },
    input::{ ActionMapper, Button, KeyCode, Mapper, },
};

pub fn control(
    world: Mut<World>,
    mut bodies: Mut<physics::BodiesService>,
    input: Const<Input>,
) {
    // Query player entity
    let query = world.query::<(&mut Model, &mut physics::RigidBody, &mut drone::Stats)>();

    // this loop will run only once, because Player component is assigned to only one entity
    for (model, rigid_body, stats) in query {

        let body = bodies.bodies.get_mut(rigid_body.rigid_body_handle).unwrap();
        
        if stats.isPlayer == true {
            if input.is_action_hold(Action::MoveForward) {
                body.apply_force(Vector3::x() * 1.0, true);
            };
            if input.is_action_hold(Action::MoveBackward) {
                body.apply_force(Vector3::x() *-1.0, true);
            };
            if input.is_action_hold(Action::MoveLeft) {
                body.apply_force(Vector3::z() *-1.0, true);
            };
            if input.is_action_hold(Action::MoveRight) {
                body.apply_force(Vector3::z() * 1.0, true);
            };
        }       

        let pos = body.position().translation;
    
        // apply translation
        model.transform.translate.x = pos.x;
        model.transform.translate.y = pos.y;
        model.transform.translate.z = pos.z;       
    }
}

pub fn spawn(
    world: &mut World,
    assets: &mut Assets,
    bodies: &mut physics::BodiesService,
    position: Point3,
    input: &mut Input,
) {
    drone::spawn(
        world,
        assets,
        bodies,
        position,
        drone::Stats{ isPlayer: true },
    );

    // Map W key to Run Action
    input.mapper_mut::<Mapper<Action>>()
        .set(vec![
            (Action::MoveForward, Button::Key(KeyCode::W)),
            (Action::MoveBackward, Button::Key(KeyCode::S)),
            (Action::MoveLeft, Button::Key(KeyCode::A)),
            (Action::MoveRight, Button::Key(KeyCode::D)),
        ]);
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
// All bindable actions
pub enum Action {
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
}

// Bind Inputs and Actions
impl ActionMapper<Action> for Input {
    fn action_mapped(&self, action: Action) -> Option<&Button> {
        let mapper = self.mapper::<Mapper<Action>>();
        mapper.get_button(action)
    }
}
