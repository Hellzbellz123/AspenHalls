use bevy::prelude::{Vec2, Vec3};
use bevy_rapier2d::prelude::Group;

/// this translates too same folder as executable
pub const APP_SETTINGS_PATH: &str = "./config.toml";
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

/// physics layer that player exists on.
pub const PLAYER_LAYER: bevy_rapier2d::geometry::Group = Group::GROUP_32;
/// physics layer that player projectile exists on
pub const PLAYER_PROJECTILE_LAYER: bevy_rapier2d::geometry::Group = Group::GROUP_30;
