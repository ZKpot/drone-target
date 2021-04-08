use rapier3d:: {
    na::{ Vector3, },
    dynamics::{ CCDSolver, JointSet, RigidBodySet, IntegrationParameters, },
    geometry::{ BroadPhase, NarrowPhase, ColliderSet, },
    pipeline::PhysicsPipeline,
};

use dotrix::{
    ecs::{ Mut, Context, },
};

pub struct Pipeline {
    pipeline: PhysicsPipeline,
    gravity: Vector3<f32>,
}

impl Default for Pipeline {
    fn default() -> Self {
        Self {
            pipeline: PhysicsPipeline::new(),
            gravity: Vector3::new(0.0, 0.0, 0.0),
        }
    }
}

pub fn step(mut context: Context<Pipeline>,
    mut bodies: Mut<RigidBodySet>,
    mut colliders: Mut<ColliderSet>,
    mut joints: Mut<JointSet>,
) {

    let gravity = context.gravity;
    let integration_parameters = IntegrationParameters::default();
    let mut broad_phase = BroadPhase::new();
    let mut narrow_phase = NarrowPhase::new();
    let mut ccd_solver = CCDSolver::new();
    let physics_hooks = ();
    let event_handler = ();

    context.pipeline.step(
        &gravity,
        &integration_parameters,
        &mut broad_phase,
        &mut narrow_phase,
        &mut bodies,
        &mut colliders,
        &mut joints,
        &mut ccd_solver,
        &physics_hooks,
        &event_handler
    );
}
