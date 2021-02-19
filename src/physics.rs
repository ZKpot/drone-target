use rapier3d::na::{ Vector3, };
use rapier3d::dynamics::{ 
    JointSet,
    RigidBodySet,
    IntegrationParameters,
};
use rapier3d::geometry::{ BroadPhase, NarrowPhase, ColliderSet };
use rapier3d::pipeline::PhysicsPipeline;

use dotrix::{
    ecs::{ Mut, Context },
};

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
}

impl Default for PipelineContext {
    fn default() -> Self {
        Self {
            pipeline: PhysicsPipeline::new(),
        }
    }
}

pub fn physics_system(mut ppl_ctx: Context<PipelineContext>,
                      mut bodies: Mut<BodiesService>,
                      mut colliders: Mut<CollidersService>,
                      mut joints: Mut<JointsService>,
                     ) {
    
    let gravity = Vector3::new(0.0, -0.05, 0.0);
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