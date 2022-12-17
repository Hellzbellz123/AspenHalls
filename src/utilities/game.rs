use bevy::prelude::{FromWorld, Resource, SystemLabel, Vec2, World};
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::prelude::Group;
use serde::{Deserialize, Serialize};

use crate::audio::SoundSettings;

/// timestep for game / physics
pub const TIMESTEP: f32 = 1. / 144.;
/// Z axis for physics interactions
pub const ACTOR_PHYSICS_Z_INDEX: f32 = 5.0;
/// Z axis for sprites/entities to be positioned on
pub const ACTOR_Z_INDEX: f32 = 8.0;
/// games tile size as const for easy use
pub const TILE_SIZE: Vec2 = Vec2 { x: 32.0, y: 32.0 };
/// actor size
pub const ACTOR_SIZE: Vec2 = Vec2::new(TILE_SIZE.x, TILE_SIZE.y * 2.0);
/// max amount of enemy actors
pub const MAX_ENEMIES: usize = 20;
/// bullet speed
pub const BULLET_SPEED_MODIFIER: f32 = 100.0;

/// const for easier usage
pub const PLAYER_LAYER: bevy_rapier2d::geometry::Group = Group::GROUP_32;
/// const
pub const PLAYER_PROJECTILE_LAYER: bevy_rapier2d::geometry::Group = Group::GROUP_30;
/// const
pub const WORLD_COLLIDER_LAYER: bevy_rapier2d::geometry::Group = Group::GROUP_32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemLabel)]
///labels for system ordering
pub enum SystemLabels {
    /// spawn label for systems that query things that might not exist
    Spawn,
    /// insert settings resources
    InitSettings,
    /// modify those settings resources
    UpdateSettings,
    /// everything that handles input
    Input,
    /// everything that updates player state
    Player,
    /// everything that moves things (works with transforms)
    Movement,
    /// systems that update the world map
    Map,
}

/// all game settings in metastruct
/// make sure tables are AFTER single fields
#[derive(Inspectable, Resource, Serialize, Deserialize, Copy, Clone)]
pub struct AppSettings {
    /// enable vsync if true
    pub vsync: bool,
    /// framerate
    pub frame_rate_target: f64,
    /// camera zooom
    pub camera_zoom: f32,
    /// display resolution
    pub resolution: Vec2,
    /// volume settings
    pub sound_settings: SoundSettings, //this last for now ig
}

//TODO: default app settings if its a setting it goes here, move this too settings plugin
impl FromWorld for AppSettings {
    fn from_world(_: &mut World) -> Self {
        AppSettings {
            sound_settings: SoundSettings {
                mastervolume: 0.2,
                ambiencevolume: 0.5,
                musicvolume: 0.2,
                soundvolume: 0.5,
            },
            resolution: Vec2 {
                x: 1200.0,
                y: 800.0,
            },
            camera_zoom: 1.0,
            vsync: true,
            frame_rate_target: 144.0,
        }
    }
}

// #[must_use]
// pub fn is_player(layers: CollisionLayers) -> bool {
//     //determines if entity is in player collision layer
//     layers.contains_group(PhysicsLayers::Player)
//         && !layers.contains_group(PhysicsLayers::Enemy)
//         && !layers.contains_group(PhysicsLayers::Sensor)
//         && !layers.contains_group(PhysicsLayers::World)
//     // && !layers.contains_group(PhysicsLayers::Projectile)
// }

// #[must_use]
// pub fn is_enemy(layers: CollisionLayers) -> bool {
//     //determines if entity is in enemy collision layer
//     layers.contains_group(PhysicsLayers::Enemy)
//         && !layers.contains_group(PhysicsLayers::Player)
//         && !layers.contains_group(PhysicsLayers::Sensor)
//         && !layers.contains_group(PhysicsLayers::World)
//     // && !layers.contains_group(PhysicsLayers::Projectile)
// }

// #[must_use]
// pub fn is_sensor(layers: CollisionLayers) -> bool {
//     layers.contains_group(PhysicsLayers::Sensor)
//         && !layers.contains_group(PhysicsLayers::Player)
//         && !layers.contains_group(PhysicsLayers::World)
//         && !layers.contains_group(PhysicsLayers::Enemy)
//     // && !layers.contains_group(PhysicsLayers::Projectile)
// }

// #[must_use]
// pub fn is_wall(layers: CollisionLayers) -> bool {
//     layers.contains_group(PhysicsLayers::World)
//         && !layers.contains_group(PhysicsLayers::Player)
//         && !layers.contains_group(PhysicsLayers::Enemy)
//         && !layers.contains_group(PhysicsLayers::Enemy)
//     // && !layers.contains_group(PhysicsLayers::Projectile)
// }

// #[derive(PhysicsLayer, Inspectable)]
// pub enum PhysicsLayers {
//     World,
//     Player,
//     Enemy,
//     Sensor,
//     PlayerAttack,
//     EnemyAttack,
// }

// impl PhysicsLayers {
//     #[must_use]
//     pub fn layers(&self) -> CollisionLayers {
//         match self {
//             PhysicsLayers::Player => CollisionLayers::none()
//                 .with_group(PhysicsLayers::Player)
//                 .with_masks(vec![
//                     PhysicsLayers::Enemy,
//                     PhysicsLayers::Sensor,
//                     PhysicsLayers::Player,
//                     PhysicsLayers::World,
//                 ]),
//             PhysicsLayers::Enemy => CollisionLayers::none()
//                 .with_group(PhysicsLayers::Enemy)
//                 .with_masks(vec![
//                     PhysicsLayers::Player,
//                     PhysicsLayers::Enemy,
//                     PhysicsLayers::World,
//                 ]),
//             PhysicsLayers::World => CollisionLayers::none()
//                 .with_group(PhysicsLayers::World)
//                 .with_masks(vec![PhysicsLayers::Player, PhysicsLayers::Enemy]),
//             PhysicsLayers::Sensor => CollisionLayers::none()
//                 .with_group(PhysicsLayers::Sensor)
//                 .with_masks(vec![PhysicsLayers::Player]),
//             PhysicsLayers::PlayerAttack => CollisionLayers::none()
//                 .with_group(PhysicsLayers::PlayerAttack)
//                 .with_masks(vec![PhysicsLayers::Enemy, PhysicsLayers::World]),
//             PhysicsLayers::EnemyAttack => CollisionLayers::none()
//                 .with_group(PhysicsLayers::EnemyAttack)
//                 .with_masks(vec![PhysicsLayers::Player, PhysicsLayers::World]),
//         }
//     }
// }
