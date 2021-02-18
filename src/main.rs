mod physics;

use dotrix::{
    Dotrix,
    assets:: { Texture, Mesh },
    components:: { SkyBox, Model, Light },
    ecs::{ Mut, RunLevel, System },
    input::{ ActionMapper, Button, Mapper },
    services::{ Assets, Camera, Frame, Input, World },
    systems::{ camera_control, world_renderer },
    renderer::transform::Transform,
    math::{ Vec3, Point3 },
};

fn main() {
    let mapper: Mapper<Action> = Mapper::new();

    Dotrix::application("push-it")
        .with_system(System::from(world_renderer).with(RunLevel::Render))
        .with_system(System::from(startup).with(RunLevel::Startup))
        .with_system(System::from(camera_control))
        .with_system(System::from(physics::physics_system))
        .with_system(System::from(player_control))
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

fn startup(mut world: Mut<World>, mut assets: Mut<Assets>) {
    
    init_skybox(&mut world, &mut assets);
    init_player(&mut world, &mut assets);
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

fn init_player(
    world: &mut World,
    assets: &mut Assets,
) {
    let texture = assets.register::<Texture>("player::texture");
    let mesh = assets.register::<Mesh>("player::mesh");   

    assets.import("assets/player/player.gltf");

    let trans = Transform {
        translate: Vec3::new(0.0, 1.0, 0.0),
        ..Default::default()
    };

    // spawn model in the world
    world.spawn(Some(
        (
            Model { mesh, texture, transform: trans, ..Default::default() },
        ),
    ));
}

fn init_light(world: &mut World) {
    world.spawn(Some((Light::white([25.0, 100.0, 25.0]),)));
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
/// All bindable actions
struct Action;

impl ActionMapper<Action> for Input {
    fn action_mapped(&self, action: Action) -> Option<&Button> {
        let mapper = self.mapper::<Mapper<Action>>();
        mapper.get_button(action)
    }
}

fn player_control(
    world: Mut<World>,
    bodies: Mut<physics::BodiesService>,
) {
    // Query player entity
    let query = world.query::<(&mut Model,)>();

    // this loop will run only once, because Player component is assigned to only one entity
    for (model,) in query {

        let rigid_body = bodies.bodies.get(bodies.handle).unwrap();    

        let pos = rigid_body.position().translation;
    
        // apply translation
        model.transform.translate.x = pos.x;
        model.transform.translate.y = pos.y;
        model.transform.translate.z = pos.z;       
    }
}