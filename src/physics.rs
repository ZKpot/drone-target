use rapier3d::na::{Vector3, Isometry3};
use rapier3d::dynamics::{JointSet, RigidBodySet, IntegrationParameters, BodyStatus, RigidBodyBuilder, RigidBodyHandle};
use rapier3d::geometry::{BroadPhase, NarrowPhase, ColliderSet};
use rapier3d::pipeline::PhysicsPipeline;

use dotrix::{
    ecs::{ Mut, Context },
};

pub struct BodiesService {   
    pub bodies: RigidBodySet,
    pub handle: RigidBodyHandle,
}

impl Default for BodiesService {
    fn default() -> Self {
        let rigid_body = RigidBodyBuilder::new(BodyStatus::Dynamic)
            .position(Isometry3::new(Vector3::new(1.0, 3.0, 2.0), Vector3::y() * 0.4))
            .linvel(0.0, 15.0, 0.0)
            .mass(0.1)
            .build();
            
        let mut body_set = RigidBodySet::new();
        let handle = body_set.insert(rigid_body);

        Self {         
            bodies: body_set,
            handle: handle,
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

/* 
pub struct BroadPhaseContext {
    broad_phase: BroadPhase,
}

impl Default for BroadPhaseContext {
    fn default() -> Self {
        Self {
            broad_phase: BroadPhase::new(),
        }
    }
}

pub struct NarrowPhaseContext {
    narrow_phase: NarrowPhase,
}

impl Default for NarrowPhaseContext {
    fn default() -> Self {
        Self {
            narrow_phase: NarrowPhase::new(),
        }
    }
}
*/

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
    
    let gravity = Vector3::new(0.0, -9.81, 0.0);
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