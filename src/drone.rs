use super::{ Action, ToExile};
use super::settings;

use rapier3d::{
    dynamics::{
        RigidBodyBuilder, BodyStatus, RigidBodySet, RigidBodyHandle, JointSet,
    },
    geometry::{ ColliderSet, ColliderBuilder, },
    na::{ Vector3, geometry::UnitQuaternion, },
    na,
};

use dotrix::{
    Transform,
    Pipeline,
    pbr:: { Model, Material, },
    services::{ Assets, World, Camera, Input, },
    math::{ Point3, Vec3, Quat, },
    ecs::{ Mut, Const, Entity, },
};

use std::f32::consts::PI;

use crate::beam;

#[derive(Debug)]
pub struct Stats {
    pub is_player:     bool,
    pub charge:        f32,  // drone battery state of charge (0-100%)
    pub strike_charge: f32,  // energy to be used when strike is activated (0-100%)
    pub health:        f32,
    pub x:             f32,
    pub y:             f32,
    pub z:             f32,
    pub dist_to_beam:  f32
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            is_player:     false,
            charge:          0.0,
            strike_charge:   0.0,
            health:        100.0,
            x:               0.0,
            y:               0.0,
            z:               0.0,
            dist_to_beam:    0.0
        }
    }
}

const D_CHARGE:        f32 = 0.5;
const D_HEALTH:        f32 = 0.2;
const D_MOVE_CHARGE:   f32 = 0.05;
const D_ACC_CHARGE:    f32 = 0.25;
const D_STRIKE_CHARGE: f32 = 0.5;
const MAX_CHARGE:      f32 = 100.0;
const VELO_MIN:        f32 = 10.0;

pub fn control(
    world: Const<World>,
    mut bodies: Mut<RigidBodySet>,
    input: Const<Input>,
    mut camera: Mut<Camera>,
    settings: Const<settings::Settings>,
    mut to_exile: Mut<ToExile>,
) {
    // Query drone entities
    let query = world.query::<(
        &Entity, &mut Transform, &mut RigidBodyHandle, &mut Stats
    )>();

    for (entity, transform, rigid_body, stats) in query {

        let body = bodies.get_mut(*rigid_body).unwrap();
        let position = body.position().translation;

        //TO DO: rethink dw1 and dw2 usage
        let dw1 = UnitQuaternion::from_euler_angles(0.0, 0.0, -PI/2.0);
        let rotation = body.position().rotation * dw1.inverse();

        if stats.is_player {
            let target_xz_angle = camera.xz_angle;
            let target_y_angle = camera.y_angle;

            //TO DO: rethink PI/2.0 shift
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
                10.0
            } else {
                1.0
            };

            let mut dir = Vector3::new(0.0, 0.0, 0.0);

            if input.is_action_hold(Action::MoveForward) {
                dir = dir + fwd;
            };
            if input.is_action_hold(Action::MoveBackward) {
                dir = dir - fwd;
            };
            if input.is_action_hold(Action::MoveLeft) {
                dir = dir + side;
            };
            if input.is_action_hold(Action::MoveRight) {
                dir = dir - side;
            };

            let velo = *body.linvel();

            if (dir != Vector3::new(0.0, 0.0, 0.0)) & (stats.charge >= D_MOVE_CHARGE)  {
                dir = dir.normalize();

                // compensate movement in other directions
                if velo != Vector3::new(0.0, 0.0, 0.0) {
                    let comp = velo.normalize().dot(&dir)/dir.dot(&dir)*dir;
                    dir = dir - (velo.normalize() - comp);
                    dir = dir.normalize();
                }

                body.apply_force(dir * spd, true);

                stats.charge = stats.charge - D_MOVE_CHARGE;
            }

            // drag force to limit acceleration
            let speed = (velo.dot(&velo)).sqrt() - VELO_MIN;
            if speed > 0.0 {
                let f_drag = -(0.1*speed + 0.002 * speed.powf(2.0)) * velo.normalize();
                body.apply_force(f_drag, true);
            }

            body.apply_torque(delta_axis * delta_angle * 50.0, true);

            // make camera following the player
            camera.target = Point3::new(position.x, position.y, position.z);

            if input.is_action_hold(Action::Strike) & (stats.strike_charge < stats.charge)  {
                stats.strike_charge = stats.strike_charge + D_STRIKE_CHARGE;
            };

            if input.is_action_deactivated(Action::Strike) {
                body.apply_impulse(fwd * stats.strike_charge * 2.0, true);
                stats.charge = stats.charge - stats.strike_charge;
                stats.strike_charge = 0.0;
            };

            stats.charge = stats.charge.min(MAX_CHARGE);
            stats.strike_charge = stats.strike_charge.min(stats.charge);

        }

        // interaction with beams
        let beams_query =
            world.query::<(&mut RigidBodyHandle, &mut beam::Stats)>();

        for (beam_rigid_body, beam_stats) in beams_query {
            let beam_body = bodies.get_mut(*beam_rigid_body).unwrap();
            let beam_position = beam_body.position().translation;

            let distance = na::distance(
                &na::Point3::new(position.x, position.y, position.z,),
                &na::Point3::new(
                    beam_position.x, beam_position.y, beam_position.x)
            );

            if distance < beam_stats.radius_near {
                stats.charge = stats.charge + D_CHARGE;
                stats.health = stats.health - D_HEALTH;
            } else if distance < beam_stats.radius_medium {
                stats.charge = stats.charge + D_CHARGE / 10.0;
            } else if distance > beam_stats.radius_far {
                stats.health = stats.health - D_HEALTH;
            }

            stats.dist_to_beam = distance;
        }

        //god mode
        if stats.is_player & settings.god_mode {
            stats.health = 100.0;
        }

        // despawn
        if stats.health <= 0.0 {
            to_exile.entity_list.push(*entity);
        }

        // apply translation to the model
        transform.translate.x = position.x;
        transform.translate.y = position.y;
        transform.translate.z = position.z;

        stats.x = position.x;
        stats.y = position.y;
        stats.z = position.z;

        //TO DO: rething dw1 and dw2 usage
        let dw2 = UnitQuaternion::from_euler_angles(0.0, 0.0, PI/2.0);
        let rot = rotation * dw2.inverse();

        // apply rotation to the model
        transform.rotate = Quat::new(
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
    bodies: &mut RigidBodySet,
    colliders: &mut ColliderSet,
    position: Point3,
    is_player: bool,
) {
    let texture = assets.register("drone::texture");
    let mesh = assets.register("drone::mesh");

    let rigid_body = RigidBodyBuilder::new(BodyStatus::Dynamic)
        .translation(position.x, position.y, position.z)
        .angular_damping(40.0)
        .additional_principal_angular_inertia(Vector3::new(0.2, 0.2, 0.2))
        .linear_damping(0.0)
        .build();

    let collider = ColliderBuilder::ball(1.0)
        .density(0.02)
        .build();

    let body_handle = bodies.insert(rigid_body);

    colliders.insert(collider, body_handle, bodies);

    world.spawn(Some((
        Model::from(mesh),
        Material {
            texture,
            ..Default::default()
        },
        Transform {
            translate: Vec3::new(position.x, position.y, position.z),
            scale: Vec3::new(1.18, 1.18, 1.18),
            ..Default::default()
        },
        body_handle,
        Stats{ is_player, ..Default::default() },
        Pipeline::default(),
    )));
}

pub fn exile(
    world: Const<World>,
    to_exile: Const<ToExile>,
    mut bodies: Mut<RigidBodySet>,
    mut colliders: Mut<ColliderSet>,
    mut joints: Mut<JointSet>,
) {

    // Query drone entities
    let query = world.query::<(
        &Entity, &mut RigidBodyHandle, &mut Stats
    )>();

    for (entity, rigid_body, _) in query {

        for i in 0..to_exile.entity_list.len() {
            if entity == &to_exile.entity_list[i] {
                bodies.remove(*rigid_body, &mut colliders, &mut joints);
                break;
            }
        }
    }
}
