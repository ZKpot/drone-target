mod physics;
mod drone;
mod player;

use dotrix::{
    Dotrix,
    assets:: { Texture, },
    components:: { SkyBox, Light, Model },
    ecs::{ Mut, RunLevel, System, },
    input::{ ActionMapper, Button, KeyCode, Mapper, },
    services::{ Assets, Camera, Frame, Input, World, },
    systems::{ camera_control, world_renderer, },
    math::{ Point3, Vec3, },
    renderer::transform::Transform,
};

fn main() {
    let mapper: Mapper<player::Action> = Mapper::new();

    Dotrix::application("push-it")
        .with_system(System::from(world_renderer).with(RunLevel::Render))
        .with_system(System::from(startup).with(RunLevel::Startup))
        .with_system(System::from(camera_control))
        .with_system(System::from(physics::system))
        .with_system(System::from(player::control))
        .with_service(Assets::new())
        .with_service(Frame::new())
        .with_service(
            Camera {
                distance: 10.0,
                y_angle: 0.5,
                xz_angle: 0.0,                
                target: Point3::new(0.0, 2.0, 0.0),
                ..Default::default()
            }
        )
        .with_service(World::new())
        .with_service(Input::new(Box::new(mapper)))
        .with_service(physics::BodiesService::default())
        .with_service(physics::CollidersService::default())
        .with_service(physics::JointsService::default())
        .run();
}

fn startup(
    mut world: Mut<World>,
    mut assets: Mut<Assets>,
    mut bodies: Mut<physics::BodiesService>,
    mut input: Mut<Input>,
) {
    init_skybox(&mut world, &mut assets);
    init_light(&mut world);
    init_drones(&mut world, &mut assets, &mut bodies, &mut input);
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
    bodies: &mut physics::BodiesService,
    input: &mut Input,
) {
    assets.import("assets/player/player.gltf");

    player::spawn(
        world,
        assets,
        bodies,
        Point3::new(0.0, 2.0, 0.0),
        input,
    );

    drone::spawn(
        world,
        assets,
        bodies,
        Point3::new(0.0, 0.0, 0.0),
        drone::Stats::default(),
    );
}

fn init_light(world: &mut World) {
    world.spawn(Some((Light::white([25.0, 100.0, 25.0]),)));
}
