use bevy::{
    prelude::{Bundle, Component, Entity, Handle, Name, SpatialBundle},
    sprite::{TextureAtlas, TextureAtlasSprite},
    transform::TransformBundle,
};
use bevy_rapier2d::prelude::{Collider, CollisionGroups};

use crate::components::actors::{bundles::RigidBodyBundle, spawners::WeaponType};

#[derive(Bundle)]
pub struct WeaponBundle {
    pub name: Name,
    pub tag: WeaponTag,
    pub weapontype: WeaponType,
    pub weaponstats: WeaponStats,
    pub damagetype: DamageType,
    pub sprite: TextureAtlasSprite,
    pub texture: Handle<TextureAtlas>,
    pub spatial: SpatialBundle,
    pub rigidbodybundle: RigidBodyBundle,
}

#[derive(Bundle)]
pub struct WeaponColliderBundle {
    pub name: Name,
    pub collider: Collider,
    pub cgroups: CollisionGroups,
    pub transformbundle: TransformBundle,
}

#[derive(Debug, Clone, Copy, Component)]
pub struct WeaponTag {
    /// weapon slot weapon is currently in, None if not attached to player
    pub stored_weapon_slot: Option<i8>,
    /// weapons parent
    pub parent: Option<Entity>,
}

/// inserted to currently drawn weapon
#[derive(Debug, Clone, Copy, Component)]
pub struct CurrentlyDrawnWeapon;

#[derive(Debug, Clone, Copy, Component)]
pub enum DamageType {
    KineticRanged,
    KineticMelee,
}

#[derive(Debug, Clone, Copy, Component)]
pub struct WeaponStats {
    pub damage: f32,
    pub speed: f32,
}

#[derive(Debug, Clone, Copy, Component)]
pub struct WeaponSocket {
    pub currently_equipped: Option<i8>,
    pub weapon_slots: i8,
    pub attached_weapon: Option<Entity>,
}
