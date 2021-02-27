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
    pub handle: RigidBodyHandle,
}

impl RigidBody {
    pub fn new(handle: RigidBodyHandle) -> Self {
        Self {
            handle,
        }
    }
}

pub struct BodiesService {
    bodies: RigidBodySet,
}

impl Default for BodiesService {
    fn default() -> Self {
        Self {
            bodies: RigidBodySet::new(),
        }
    }
}

impl BodiesService {
    #[inline(always)]
    pub fn insert(&mut self, rigid_body:  rapier3d::dynamics::RigidBody) -> RigidBodyHandle {
        self.bodies.insert(rigid_body)
    }

    #[inline(always)]
    pub fn get_mut(&mut self, handle: RigidBodyHandle) -> Option<&mut rapier3d::dynamics::RigidBody> {
        self.bodies.get_mut(handle)
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
            gravity: Vector3::new(0.0, 0.0, 0.0),
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
