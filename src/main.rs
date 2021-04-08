mod physics;
mod drone;
use rapier3d;

use dotrix::{
    Dotrix,
    assets:: { Texture, },
    components:: { SkyBox, Light },
    ecs::{ Mut, RunLevel, System, },
    input::{ ActionMapper, Button, KeyCode, Mapper, },
    services::{ Assets, Camera, Frame, Input, World, },
    systems::{ camera_control, world_renderer, },
    math::{ Point3, },
};

fn main() {
    let mapper: Mapper<Action> = Mapper::new();

    Dotrix::application("drone-target")
        .with_system(System::from(world_renderer).with(RunLevel::Render))
        .with_system(System::from(startup).with(RunLevel::Startup))
        .with_system(System::from(camera_control))
        .with_system(System::from(physics::step))
        .with_system(System::from(drone::control))
        .with_service(Assets::new())
        .with_service(Frame::new())
        .with_service(
            Camera {
                distance: 10.0,
                y_angle: 0.0,
                xz_angle: 0.0,
                target: Point3::new(0.0, 2.0, 0.0),
                ..Default::default()
            }
        )
        .with_service(World::new())
        .with_service(Input::new(Box::new(mapper)))
        .with_service(rapier3d::dynamics::RigidBodySet::new())
        .with_service(rapier3d::geometry::ColliderSet::new())
        .with_service(rapier3d::dynamics::JointSet::new())
        .run();
}

fn startup(
    mut world: Mut<World>,
    mut assets: Mut<Assets>,
    mut bodies: Mut<rapier3d::dynamics::RigidBodySet>,
    mut input: Mut<Input>,
) {
    init_skybox(&mut world, &mut assets);
    init_light(&mut world);
    init_drones(&mut world, &mut assets, &mut bodies);
    init_controls(&mut input);
}

fn init_skybox(
    world: &mut World,
    assets: &mut Assets,
) {
    let primary_texture = [
        assets.register::<Texture>("skybox_right"),
        assets.register::<Texture>("skybox_left"),
        assets.register::<Texture>("skybox_top"),
        assets.register::<Texture>("skybox_bottom"),
        assets.register::<Texture>("skybox_back"),
        assets.register::<Texture>("skybox_front"),
    ];

    // The skybox cubemap was downloaded from https://opengameart.org/content/elyvisions-skyboxes
    // These files were licensed as CC-BY 3.0 Unported on 2012/11/7
    assets.import("assets/skybox/skybox_right.png");
    assets.import("assets/skybox/skybox_left.png");
    assets.import("assets/skybox/skybox_top.png");
    assets.import("assets/skybox/skybox_bottom.png");
    assets.import("assets/skybox/skybox_front.png");
    assets.import("assets/skybox/skybox_back.png");

    world.spawn(vec![
        (SkyBox { primary_texture, ..Default::default() },),
    ]);
}

fn init_drones(
    world: &mut World,
    assets: &mut Assets,
    bodies: &mut rapier3d::dynamics::RigidBodySet,
) {
    assets.import("assets/drone/drone.gltf");

    drone::spawn(
        world,
        assets,
        bodies,
        Point3::new(0.0, 2.0, 0.0),
        true,
    );

    let positions: [[f32; 3]; 20] = [
        [ 80.0,  10.0, -90.0],
        [-50.0,  20.0,  30.0],
        [100.0, -50.0, -40.0],
        [  0.0, -25.0,  20.0],
        [ 15.0,  35.0,  -2.0],
        [-90.0, -85.0,  10.0],
        [-45.0,  25.0, -95.0],
        [-80.0, -10.0,  90.0],
        [ 50.0, -20.0, -30.0],
        [-95.0,  50.0,  40.0],
        [ 10.0,  25.0, -20.0],
        [-15.0, -35.0,   2.0],
        [ 90.0,  85.0, -10.0],
        [ 45.0, -25.0,  95.0],
        [ 80.0, -10.0, -90.0],
        [ 50.0,  20.0, -30.0],
        [100.0,  50.0, -40.0],
        [  0.0, -25.0, -20.0],
        [-15.0,  35.0,   2.0],
        [-90.0,  85.0,  10.0],
    ];

    for i in 0..positions.len() {
        println!("{}", i);
        drone::spawn(
            world,
            assets,
            bodies,
            Point3::new(positions[i][0], positions[i][1], positions[i][2]),
            false,
        );
    }


}

fn init_light(world: &mut World) {
    world.spawn(Some((Light::white([25.0, 100.0, 25.0]),)));
}

fn init_controls(input: &mut Input) {
    // Map W key to Run Action
    input.mapper_mut::<Mapper<Action>>()
        .set(vec![
            (Action::MoveForward, Button::Key(KeyCode::W)),
            (Action::MoveBackward, Button::Key(KeyCode::S)),
            (Action::MoveLeft, Button::Key(KeyCode::A)),
            (Action::MoveRight, Button::Key(KeyCode::D)),
            (Action::Accelerate, Button::Key(KeyCode::LShift)),
            (Action::Strike, Button::MouseLeft),
        ]);
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
// All bindable actions
pub enum Action {
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
    Accelerate,
    Strike,
}

// Bind Inputs and Actions
impl ActionMapper<Action> for Input {
    fn action_mapped(&self, action: Action) -> Option<&Button> {
        let mapper = self.mapper::<Mapper<Action>>();
        mapper.get_button(action)
    }
}
