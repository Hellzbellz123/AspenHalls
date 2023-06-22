use bevy::prelude::Vec2;
use bevy_rapier2d::prelude::Group;

/// this translates too same folder as executable
pub const APP_SETTINGS_PATH: &str = "./config.toml";
/// timestep for game / physics
pub const TIMESTEP: f32 = 1. / 144.;
/// Z axis for physics interactions
pub const ACTOR_PHYSICS_Z_INDEX: f32 = 8.0;
/// Z axis for sprites/entities to be positioned on
pub const ACTOR_Z_INDEX: f32 = 8.0;
/// games tile size as const for easy use
pub const TILE_SIZE: Vec2 = Vec2 { x: 32.0, y: 32.0 };
/// actor size
pub const ACTOR_SIZE: Vec2 = Vec2::new(TILE_SIZE.x, TILE_SIZE.y * 2.0);
/// bullet speed
pub const BULLET_SPEED_MODIFIER: f32 = 100.0;

/// physics layer that player exists on.
pub const PLAYER_LAYER: bevy_rapier2d::geometry::Group = Group::GROUP_32;
/// physics layer that player projectile exists on
pub const PLAYER_PROJECTILE_LAYER: bevy_rapier2d::geometry::Group = Group::GROUP_30;
/// things the player can collide with
pub const WORLD_COLLIDER_LAYER: bevy_rapier2d::geometry::Group = Group::GROUP_32;
