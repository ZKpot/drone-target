mod physics;
mod drone;

use dotrix::{
    Dotrix,
    assets:: { Texture, },
    components:: { SkyBox, Light, },
    ecs::{ Mut, RunLevel, System, },
    input::{ ActionMapper, Button, KeyCode, Mapper, },
    services::{ Assets, Camera, Frame, Input, World, },
    systems::{ camera_control, world_renderer, },
    math::{ Point3, },
};

fn main() {
    let mapper: Mapper<Action> = Mapper::new();

    Dotrix::application("push-it")
        .with_system(System::from(world_renderer).with(RunLevel::Render))
        .with_system(System::from(startup).with(RunLevel::Startup))
        .with_system(System::from(camera_control))
        .with_system(System::from(physics::system))
        .with_system(System::from(drone::player_control))
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
    init_players(&mut world, &mut assets, &mut bodies, &mut input);
    init_light(&mut world);
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

fn init_players(
    world: &mut World,
    assets: &mut Assets,
    bodies: &mut physics::BodiesService,
    input: &mut Input,
) {
    assets.import("assets/player/player.gltf");

    drone::spawn(
        world,
        assets,
        bodies,
        Point3::new(0.0, 2.0, 0.0),
    );

    // Map W key to Run Action
    input.mapper_mut::<Mapper<Action>>()
        .set(vec![
            (Action::MoveForward, Button::Key(KeyCode::W)),
        ]);
}

fn init_light(world: &mut World) {
    world.spawn(Some((Light::white([25.0, 100.0, 25.0]),)));
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
// All bindable actions
enum Action {
    MoveForward,
}

// Bind Inputs and Actions
impl ActionMapper<Action> for Input {
    fn action_mapped(&self, action: Action) -> Option<&Button> {
        let mapper = self.mapper::<Mapper<Action>>();
        mapper.get_button(action)
    }
}