use bevy::{prelude::Name, sprite::SpriteSheetBundle};

use crate::actors::{animation::AnimState, RigidBodyBundle};

struct SkeletonBundle {
    name: Name,
    sprite: SpriteSheetBundle,
    rigidbody: RigidBodyBundle,
    animation_state: AnimState,
}
