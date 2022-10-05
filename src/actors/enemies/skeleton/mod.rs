use bevy::{
    prelude::{Bundle, Name},
    sprite::SpriteSheetBundle,
};

use crate::actors::{animation::AnimState, ActorState, RigidBodyBundle};

use super::Enemy;

#[derive(Bundle)]
pub struct SkeletonBundle {
    pub name: Name,
    pub actortype: Enemy,
    pub actorstate: ActorState,
    pub animation_state: AnimState,
    #[bundle]
    pub sprite: SpriteSheetBundle,
    #[bundle]
    pub rigidbody: RigidBodyBundle,
}
