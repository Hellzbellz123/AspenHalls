use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::prelude::{
    Collider, ColliderMassProperties, Damping, Friction, LockedAxes, Restitution, RigidBody,
    Velocity,
};

use crate::actors::animation::FacingDirection;

pub mod animation;
pub mod components;
pub mod enemies;
pub mod player;
pub mod spawners;

#[derive(Bundle)] //bundle for ease of use
pub struct RigidBodyBundle {
    rigidbody: RigidBody,
    velocity: Velocity,
    friction: Friction,
    howbouncy: Restitution,
    massprop: ColliderMassProperties,
    rotationlocks: LockedAxes,
    dampingprop: Damping,
}

#[derive(Bundle)]
pub struct ActorColliderBundle {
    transform_bundle: TransformBundle,
    collider: Collider,
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

//     rigidbody: heron::RigidBody::Dynamic,
//     velocity: Velocity::default(),
//     rconstraints: RotationConstraints::lock(),
//     collision_layers: CollisionLayers::all_masks::<PhysicsLayers>()
//         .with_group(PhysicsLayers::Player),
//     physicsmat: PhysicMaterial {
//         restitution: 0.1,
//         density: 1.0,
//         friction: 0.5,
//     }, //PhysicsLayers::Player.layers()
// },
