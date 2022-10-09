pub mod actions;
pub mod utilities;

use bevy::{
    prelude::{Bundle, Name},
    sprite::SpriteSheetBundle,
};

use crate::actors::{
    animation::{AnimState, AnimationSheet},
    components::Aggroable,
    ActorState, RigidBodyBundle,
};

use super::Enemy;

#[derive(Bundle)]
pub struct SkeletonBundle {
    pub name: Name,
    pub actortype: Enemy,
    pub aggroable: Aggroable,
    pub actorstate: ActorState,
    pub animation_state: AnimState,
    pub available_animations: AnimationSheet,
    #[bundle]
    pub sprite: SpriteSheetBundle,
    #[bundle]
    pub rigidbody: RigidBodyBundle,
}
