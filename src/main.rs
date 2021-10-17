mod physics;
mod drone;
mod beam;
mod settings;

use rapier3d;

use dotrix::prelude::*;

use dotrix::{
    Assets,
    Camera,
    CubeMap,
    Color,
    Input,
    World,
    Pipeline,

    sky::{ skybox, SkyBox, },
    pbr::{ self, Light, },
    input::{ ActionMapper, Button, KeyCode, Mapper, },
    camera,
    math::{ Point3, Vec3 },
};

fn main() {
    Dotrix::application("drone-target")
        .with(System::from(startup))
        .with(System::from(settings::startup))

        .with(System::from(settings::update))
        .with(System::from(camera::control))
        .with(System::from(physics::step))
        .with(System::from(drone::control))
        .with(System::from(beam::gravity))

        .with(Service::from(rapier3d::dynamics::RigidBodySet::new()))
        .with(Service::from(rapier3d::geometry::ColliderSet::new()))
        .with(Service::from(rapier3d::dynamics::JointSet::new()))
        .with(Service::from(rapier3d::geometry::BroadPhase::new()))
        .with(Service::from(rapier3d::geometry::NarrowPhase::new()))
        .with(Service::from(rapier3d::dynamics::CCDSolver::new()))
        .with(Service::from(settings::Settings::default()))

        .with(skybox::extension)
        .with(pbr::extension)

        .run();
}

fn startup(
    mut world: Mut<World>,
    mut camera: Mut<Camera>,
    mut assets: Mut<Assets>,
    mut bodies: Mut<rapier3d::dynamics::RigidBodySet>,
    mut colliders: Mut<rapier3d::geometry::ColliderSet>,
    mut input: Mut<Input>,
) {
    input.set_mapper(Box::new(Mapper::<Action>::new()));
    init_camera(&mut camera);
    init_skybox(&mut world, &mut assets);
    init_light(&mut world);
    init_world(&mut world, &mut assets, &mut bodies, &mut colliders);
    init_drones(&mut world, &mut assets, &mut bodies, &mut colliders);
    init_controls(&mut input);
}

fn init_camera(camera: &mut Camera) {
    camera.y_angle = 0.0;
    camera.xz_angle = 0.0;
    camera.target = Point3::new(0.0, 2.0, 0.0);
    camera.distance = 10.0;
}

fn init_skybox(
    world: &mut World,
    assets: &mut Assets,
) {
    // The skybox cubemap was downloaded from https://opengameart.org/content/elyvisions-skyboxes
    // These files were licensed as CC-BY 3.0 Unported on 2012/11/7
    assets.import("assets/skybox/skybox_right.png");
    assets.import("assets/skybox/skybox_left.png");
    assets.import("assets/skybox/skybox_top.png");
    assets.import("assets/skybox/skybox_bottom.png");
    assets.import("assets/skybox/skybox_front.png");
    assets.import("assets/skybox/skybox_back.png");

    // Spawn skybox
    world.spawn(Some((
        SkyBox {
            view_range: 500.0,
            ..Default::default()
        },
        CubeMap {
            right: assets.register("skybox_right"),
            left: assets.register("skybox_left"),
            top: assets.register("skybox_top"),
            bottom: assets.register("skybox_bottom"),
            back: assets.register("skybox_back"),
            front: assets.register("skybox_front"),
            ..Default::default()
        },
        Pipeline::default()
    )));
}

fn init_world(
    world: &mut World,
    assets: &mut Assets,
    bodies: &mut rapier3d::dynamics::RigidBodySet,
    colliders: &mut rapier3d::geometry::ColliderSet,
) {
    assets.import("assets/energy_beam/energy_beam.gltf");

    beam::spawn(
        world,
        assets,
        bodies,
        colliders,
        Point3::new(0.0, 0.0, 0.0),
    );
}

fn init_drones(
    world: &mut World,
    assets: &mut Assets,
    bodies: &mut rapier3d::dynamics::RigidBodySet,
    colliders: &mut rapier3d::geometry::ColliderSet,
) {
    assets.import("assets/drone/drone.gltf");

    drone::spawn(
        world,
        assets,
        bodies,
        colliders,
        Point3::new(10.0, 0.0, 0.0),
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
            colliders,
            Point3::new(positions[i][0], positions[i][1], positions[i][2]),
            false,
        );
    }
}

fn init_light(world: &mut World) {
    world.spawn(Some((
        Light::Simple {
            position: Vec3::new(200.0, 0.0, 200.0),
            color: Color::white(),
            intensity: 0.8,
            enabled: true,
        },
    )));

    world.spawn(Some((
        Light::Simple {
            position: Vec3::new(-200.0, 50.0, 100.0),
            color: Color::white(),
            intensity: 0.8,
            enabled: true,
        },
    )));

    world.spawn(Some((
        Light::Simple {
            position: Vec3::new(100.0, -50.0, -200.0),
            color: Color::white(),
            intensity: 0.8,
            enabled: true,
        },
    )));
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
            (Action::Menu, Button::Key(KeyCode::Escape)),
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
    Menu,
}

// Bind Inputs and Actions
impl ActionMapper<Action> for Input {
    fn action_mapped(&self, action: Action) -> Option<&Button> {
        let mapper = self.mapper::<Mapper<Action>>();
        mapper.get_button(action)
    }
}
