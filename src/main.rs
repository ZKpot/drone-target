use dotrix::{
    Dotrix,
    assets:: {Texture, Mesh},
    components:: {SkyBox, Model, Light},
    ecs::{ Mut, RunLevel, System },
    input::{ ActionMapper, Button, Mapper },
    services::{ Assets, Camera, Frame, Input, World },
    systems::{ camera_control, world_renderer },
    renderer::transform::Transform,
    math::{ Vec3, Point3, },
};

fn main() {
    let mapper: Mapper<Action> = Mapper::new();

    Dotrix::application("SkyBox Example")
        .with_system(System::from(world_renderer).with(RunLevel::Render))
        .with_system(System::from(startup).with(RunLevel::Startup))
        .with_system(System::from(camera_control))
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
        .run();

}

fn startup(mut world: Mut<World>, mut assets: Mut<Assets>) {
    
    init_skybox(&mut world, &mut assets);
    init_terrain(&mut world, &mut assets);
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

fn init_terrain(
    world: &mut World,
    assets: &mut Assets,
) {
    // Generate terrain mesh like this:
    //   0   1
    // 0 +---+---+---> x
    //   | / | / |
    // 1 +---+---+
    //   | / | / |
    //   +---+---+
    //   |
    //   z

    let size = 64;
    let mut positions = Vec::with_capacity(3 * 2 * size * size);
    let mut uvs = Vec::new();
    for x in 0..size {
        let x0 = x as f32;
        let x1 = x0 + 1.0;
        for z in 0..size {
            let z0 = z as f32;
            let z1 = z0 + 1.0;
            // Add vertices
            positions.push([x0, 0.0, z0]);
            positions.push([x0, 0.0, z1]);
            positions.push([x1, 0.0, z0]);
            positions.push([x1, 0.0, z0]);
            positions.push([x0, 0.0, z1]);
            positions.push([x1, 0.0, z1]);
            // Add texture vertices
            uvs.push([0.0, 0.0]);
            uvs.push([0.0, 1.0]);
            uvs.push([1.0, 0.0]);
            uvs.push([1.0, 0.0]);
            uvs.push([0.0, 1.0]);
            uvs.push([1.0, 1.0]);
        }
    }

    let mut mesh = Mesh {
        positions,
        uvs: Some(uvs),
        ..Default::default()
    };
    // Calculate mesh normals
    mesh.calculate();

    // Store mesh and get its ID
    let mesh = assets.store(mesh, "terrain");

    // import terrain texture and get its ID
    assets.import("assets/terrain/terrain.png");
    let texture = assets.register("terrain");

    // Center terrain tile at coordinate system center (0.0, 0.0, 0.0) by moving the tile on a
    // half of its size by X and Z axis
    let shift = (size / 2) as f32;
    let transform = Transform {
        translate: Vec3::new(-shift, 0.0, -shift),
        ..Default::default()
    };

    // Spawn terrain in the world
    world.spawn(Some(
        (Model { mesh, texture, transform, ..Default::default() },)
    ));
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