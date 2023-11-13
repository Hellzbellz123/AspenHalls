use crate::ahp::engine::{bevy, Group, Vec2, Vec3};

/// games tile size as const for easy use
pub const TILE_SIZE: Vec2 = Vec2 { x: 32.0, y: 32.0 };

/// Z axis for physics interactions
pub const ACTOR_PHYSICS_Z_INDEX: f32 = 10.0;

/// Z axis for sprites/entities to be positioned on
pub const ACTOR_Z_INDEX: f32 = 10.0;

/// actor scale
pub const ACTOR_SCALE: Vec3 = Vec3 {
    x: 1.0,
    y: 1.0,
    z: 1.0,
};

/// actor size
pub const ACTOR_SIZE: Vec2 = Vec2::new(TILE_SIZE.x / 2.0, TILE_SIZE.y);

/// common actor capsule dimensions
pub const ACTOR_COLLIDER: (bevy::prelude::Vec2, bevy::prelude::Vec2, f32) = (
    Vec2 {
        x: 0.0,
        y: ACTOR_SIZE.y / 3.0,
    },
    Vec2 {
        x: 0.0,
        y: -ACTOR_SIZE.y / 5.0,
    },
    ACTOR_SIZE.x / 2.0,
);

/// bullet speed
pub const BULLET_SPEED_MODIFIER: f32 = 100.0;

#[non_exhaustive]
/// Collision Groups wrapper
/// created for easy use
///```
/// collision_groups: CollisionGroups::new(
///     AspenCollisionLayer::PROJECTILE, <--- Select Membership
///     AspenCollisionLayer::WORLD | AspenCollisionLayer::ACTOR | AspenCollisionLayer::PROJECTILE  <---- bitwise-or the groups you want this member too collide with
///```
pub struct AspenCollisionLayer;

impl AspenCollisionLayer {
    /// entities that provide world collision belong to this group
    pub const WORLD: Group = Group::GROUP_1;
    /// entities that can move belong too this group
    pub const ACTOR: Group = Group::GROUP_2;
    /// entities that are created from weapons belong too this group
    pub const PROJECTILE: Group = Group::GROUP_3;
    /// All possible collision groups
    ///
    /// use as the membership and bitwise-or what you do NOT want too collide with
    pub const EVERYTHING: Group = Group::ALL;
}
