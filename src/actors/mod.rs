use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use heron::prelude::*;

use crate::actors::animation::FacingDirection;

pub mod animation;
pub mod enemies;
pub mod player;

#[derive(Bundle)]
pub struct RigidBodyBundle {
    rigidbody: RigidBody,
    collisionlayers: CollisionLayers,
    rconstraints: RotationConstraints,
    physicsmat: PhysicMaterial,
    velocity: Velocity,
}

#[derive(Component, Default, Reflect, Inspectable)]
#[reflect(Component)]
pub struct ActorState {
    //stores actor information, all actors have this
    pub target_positon: Option<Vec2>,
    pub speed: f32,
    pub sprint_available: bool,
    pub facing: FacingDirection,
    pub just_moved: bool,
}
