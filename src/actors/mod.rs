use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use heron::prelude::*;

use crate::actors::animation::FacingDirection;

pub mod animation;
pub mod enemies;
pub mod player;

#[derive(Bundle)] //bundle for ease of use
pub struct RigidBodyBundle {
    pub rigidbody: RigidBody,
    pub collision_layers: CollisionLayers,
    pub rconstraints: RotationConstraints,
    // pub physicsmat: PhysicMaterial,
    pub velocity: Velocity,
}

#[derive(Component, Default, Reflect, Inspectable)]
#[reflect(Component)]
pub struct ActorState {
    //stores actor information, all actors have this
    pub speed: f32,
    pub sprint_available: bool,
    pub facing: FacingDirection,
    pub just_moved: bool,
}
