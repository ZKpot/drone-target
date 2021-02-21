use rapier3d:: {
    na::{ Vector3, },
    dynamics::{ JointSet, RigidBodySet, IntegrationParameters, RigidBodyHandle, },
    geometry::{ BroadPhase, NarrowPhase, ColliderSet, },
    pipeline::PhysicsPipeline,
};

use dotrix::{
    ecs::{ Mut, Context, },
};

/// Component
pub struct RigidBody {
    pub rigid_body_handle: RigidBodyHandle,
}

impl RigidBody {
    pub fn new(rigid_body_handle: RigidBodyHandle) -> Self {        
        Self {
            rigid_body_handle,
        }
    }
}

pub struct BodiesService {   
    pub bodies: RigidBodySet,
}

impl Default for BodiesService {
    fn default() -> Self {
        Self {         
            bodies: RigidBodySet::new(),
        }        
    }
}

pub struct CollidersService {   
    colliders: ColliderSet,
}

impl Default for CollidersService {
    fn default() -> Self {
        Self {         
            colliders: ColliderSet::new(),
        }        
    }
}

pub struct JointsService {   
    joints: JointSet,
}

impl Default for JointsService {
    fn default() -> Self {
        Self {         
            joints: JointSet::new(),
        }        
    }
}

pub struct PipelineContext {
    pipeline: PhysicsPipeline,
    gravity: Vector3<f32>,
}

impl Default for PipelineContext {
    fn default() -> Self {
        Self {
            pipeline: PhysicsPipeline::new(),
            gravity: Vector3::new(0.0, -0.5, 0.0),
        }
    }
}

pub fn system(mut ppl_ctx: Context<PipelineContext>,
    mut bodies: Mut<BodiesService>,
    mut colliders: Mut<CollidersService>,
    mut joints: Mut<JointsService>,
) {
    
    let gravity = ppl_ctx.gravity;
    let integration_parameters = IntegrationParameters::default();
    let mut broad_phase = BroadPhase::new();
    let mut narrow_phase = NarrowPhase::new();
    let event_handler = ();
    
    ppl_ctx.pipeline.step(
        &gravity,
        &integration_parameters,
        &mut broad_phase,
        &mut narrow_phase,
        &mut bodies.bodies,
        &mut colliders.colliders,
        &mut joints.joints,
        None,
        None,
        &event_handler
    );
}
