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

// supported resolutions
// const RESOLUTIONS: [(f32, f32); 28] = [
//     // Common Resolutions (as mentioned before)
//     (0682.00, 0512.00), // (1024, 0768)  found in: 4:3
//     (0682.00, 0512.00), // (2048, 1536)  found in: 4:3
//     (0720.00, 1280.00), // (1080, 1920)  found in: 16:9
//     (0757.33, 0426.67), // (0640, 1136) rotated   found in: iPhone
//     (0853.33, 0480.00), // (0720, 1280)  found in: 16:9
//     (0889.33, 0500.00), // (0750, 1334) rotated   found in: iPhone 6, 7, 8
//     (0910.00, 0512.00), // (1366, 0768)  found in: 16:9
//     (0960.00, 0600.00), // (1440, 0900)  found in: 16:10
//     (0995.00, 0695.00), // (1668, 2388) rotated   found in: iPad Pro 11"
//     (1120.00, 0700.00), // (1680, 1050)  found in: 16:10
//     (1138.67, 0853.00), // (2048, 2732)  found in: iPad Pro 12.9"
//     (1194.67, 0552.00), // (0828, 1792) rotated   found in: iPhone
//     (1280.00, 0720.00), // (1920, 1080)  found in: 16:9
//     (1280.00, 0800.00), // (1920, 1200)  found in: 16:10
//     (1600.00, 0900.00), // (3840, 2160)  found in: 16:9
//     (1600.00, 0900.00), // (7680, 4320)  found in: 16:9
//     (1624.00, 0750.00), // (1125, 2436) rotated   found in: iPhone
//     (1706.00, 0960.00), // (2560, 1440)  found in: 16:9
//     (1706.00, 0960.00), // (5120, 2880)  found in: 16:9
//     (1706.67, 0960.00), // (1440, 2560)  found in: 16:9
//     (1792.00, 0828.00), // (1242, 2688) rotated   found in: iPhone XS Max
//     (1973.33, 0960.00), // (1440, 2960)  found in: 18.5:9
//     // Additional Less Common Resolutions
//     (0853.33, 0360.00), // (2560.0, 1080.0)  found in: 21:9 Ultrawide
//     (1066.67, 0450.00), // (5120.0, 2160.0)  found in: 21:9 Ultrawide
//     (1146.67, 0480.00), // (3440.0, 1440.0)  found in: 21:9 Ultrawide
//     (1253.33, 0705.00), // (6016.0, 3384.0)  found in: 6K UHD
//     (1600.00, 0300.00), // (7680, 1440)      found in: triple monitor
//     (2400.00, 0450.00), // (11520.0, 2160.0) found in: Triple Monitor// Found in: Triple Monitor setups
// ];
