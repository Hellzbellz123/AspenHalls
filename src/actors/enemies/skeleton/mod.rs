pub mod actions;
pub mod utilities;

use bevy::{
    prelude::{Bundle, Name},
    sprite::SpriteSheetBundle,
};

use crate::components::actors::{
    ai::AIEnemy,
    animation::{AnimState, AnimationSheet},
    bundles::RigidBodyBundle,
    general::ActorState,
};

#[derive(Bundle)]
pub struct SkeletonBundle {
    pub name: Name,
    pub actortype: AIEnemy,
    pub actorstate: ActorState,
    pub animation_state: AnimState,
    pub available_animations: AnimationSheet,
    #[bundle]
    pub sprite: SpriteSheetBundle,
    #[bundle]
    pub rigidbody: RigidBodyBundle,
}
