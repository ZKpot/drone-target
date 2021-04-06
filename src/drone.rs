use super::{ physics, Action, };

use rapier3d::{
    dynamics::{ RigidBodyBuilder, BodyStatus, },
    na::{ Vector3, Isometry3, geometry::UnitQuaternion, },
};

use dotrix::{
    components:: { Model, },
    services::{ Assets, World, Camera, Input, },
    math::{ Point3, Vec3, Quat, },
    renderer::transform::Transform,
    ecs::{ Mut, Const, },
};

use std::f32::consts::PI;

pub struct Stats {
    pub is_player: bool,
    pub charge: f32,            // drone battery state of charge (0-100%)
    pub strike_charge: f32,     // energy to be used when strike is activated (0-100%)
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            is_player: false,
            charge: 0.0,
            strike_charge: 0.0
        }
    }
}

const D_CHARGE:        f32 = 0.05;
const D_ACC_CHARGE:    f32 = 0.25;
const D_STRIKE_CHARGE: f32 = 0.5;
const MAX_CHARGE:      f32 = 100.0;

pub fn control(
    world: Mut<World>,
    mut bodies: Mut<physics::BodiesService>,
    input: Const<Input>,
    mut camera: Mut<Camera>,
) {
    // Query player entity
    let query = world.query::<(&mut Model, &mut physics::RigidBody, &mut Stats)>();

    // this loop will run only once, because Player component is assigned to only one entity
    for (model, rigid_body, stats) in query {

        let body = bodies.get_mut(rigid_body.handle).unwrap();
        let postion = body.position().translation;

        //TO DO: rething dw1 and dw2 usage
        let dw1 = UnitQuaternion::from_euler_angles(0.0, 0.0, -PI/2.0);
        let rotation = body.position().rotation * dw1.inverse();

        if stats.is_player {
            let target_xz_angle = camera.xz_angle;
            let target_y_angle = camera.y_angle;

            //TO DO: rething PI/2.0 shift
            let target_rotation = UnitQuaternion::from_euler_angles(
                0.0,
                -target_xz_angle,
                PI/2.0 - target_y_angle
            );

            let delta_rotation = target_rotation * rotation.inverse();
            let delta_axis = match delta_rotation.axis() {
                Some(x) => Vector3::new(
                    x.into_inner().data[0],
                    x.into_inner().data[1],
                    x.into_inner().data[2],
                ),
                None    => Vector3::new(0.0, 0.0, 0.0),
            };

            let delta_angle = delta_rotation.angle();

            let rotation_euler = rotation.euler_angles();

            let fwd = Vector3::new(
                -rotation_euler.2.sin() * rotation_euler.1.cos(),
                rotation_euler.1.sin(),
                -rotation_euler.2.cos() * rotation_euler.1.cos(),
            );

            let side = Vector3::new(
                (-PI/2.0 + rotation_euler.2).sin(),
                0.0,
                (-PI/2.0 + rotation_euler.2).cos()
            );

            let spd = if input.is_action_hold(Action::Accelerate) & (stats.charge >= D_ACC_CHARGE) {
                stats.charge = stats.charge - D_ACC_CHARGE;
                5.0
            } else {
                1.0
            };

            if input.is_action_hold(Action::MoveForward) {
                body.apply_force(fwd * spd, true);
            };
            if input.is_action_hold(Action::MoveBackward) {
                body.apply_force(-fwd * spd, true);
            };
            if input.is_action_hold(Action::MoveLeft) {
                body.apply_force(side * spd, true);
            };
            if input.is_action_hold(Action::MoveRight) {
                body.apply_force(-side * spd, true);
            };

            body.apply_torque(delta_axis * delta_angle * 50.0, true);

            // make camera following the player
            camera.target = Point3::new(postion.x, postion.y, postion.z);
            camera.set_view();

            stats.charge = stats.charge + D_CHARGE;

            if input.is_action_hold(Action::Strike) & (stats.strike_charge < stats.charge)  {
                stats.strike_charge = stats.strike_charge + D_STRIKE_CHARGE;
            };

            if input.is_action_deactivated(Action::Strike) {
                body.apply_impulse(fwd * stats.strike_charge, true);
                stats.charge = stats.charge - stats.strike_charge;
                stats.strike_charge = 0.0;
            };

            stats.charge = stats.charge.min(MAX_CHARGE);
            stats.strike_charge = stats.strike_charge.min(stats.charge);

            println!("{} {}", stats.charge, stats.strike_charge);
        }

        // apply translation to the model
        model.transform.translate.x = postion.x;
        model.transform.translate.y = postion.y;
        model.transform.translate.z = postion.z;

        //TO DO: rething dw1 and dw2 usage
        let dw2 = UnitQuaternion::from_euler_angles(0.0, 0.0, PI/2.0);
        let rot = rotation * dw2.inverse();

        // apply rotation to the model
        model.transform.rotate = Quat::new(
            rot.into_inner().w,
            rot.into_inner().j,
            rot.into_inner().k,
            rot.into_inner().i,
        );
    }
}

pub fn spawn(
    world: &mut World,
    assets: &mut Assets,
    bodies: &mut physics::BodiesService,
    position: Point3,
    is_player: bool,
) {
    let texture = assets.register("drone::texture");
    let mesh = assets.register("drone::mesh");

    let transform = Transform {
        translate: Vec3::new(position.x, position.y, position.z),
        ..Default::default()
    };

    let rigid_body = RigidBodyBuilder::new(BodyStatus::Dynamic)
        .position(Isometry3::new(
            Vector3::new(position.x, position.y, position.z),
            Vector3::new(0.0, 0.0, 0.0))
        )
        .mass(0.1)
        .principal_angular_inertia(Vector3::new(1.0, 1.0, 1.0))
        .angular_damping(10.0)
        .linear_damping(1.0)
        .build();

    // spawn model in the world
    world.spawn(Some(
        (
            Model { mesh, texture, transform, ..Default::default() },
            physics::RigidBody::new(bodies.insert(rigid_body)),
            Stats{ is_player, ..Default::default() },
        ),
    ));
}
