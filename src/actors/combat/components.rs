use bevy::{
    prelude::{Bundle, Component, Entity, Handle, Name, ReflectComponent, SpatialBundle, Vec3},
    reflect::{FromReflect, Reflect},
    sprite::{SpriteBundle, TextureAtlas, TextureAtlasSprite},
    transform::TransformBundle,
    utils::hashbrown::HashMap,
};

use bevy_rapier2d::prelude::{Collider, CollisionGroups};

use crate::components::actors::{bundles::RigidBodyBundle, spawners::WeaponType};

#[derive(Debug, Component)]
pub struct Damaged {
    pub amount: f32,
}

#[derive(Debug, Component)]
pub struct Destroyed;

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

#[derive(Debug, Component)]
pub struct WeaponColliderTag;

#[derive(Bundle)]
pub struct WeaponColliderBundle {
    pub name: Name,
    pub tag: WeaponColliderTag,
    pub collider: Collider,
    pub cgroups: CollisionGroups,
    pub transformbundle: TransformBundle,
}

#[derive(Debug, Component)]
pub struct BarrelPointTag;

#[derive(Bundle)]
pub struct WeaponBarrelEndPoint {
    pub name: Name,
    pub tag: BarrelPointTag,
    pub sprite: SpriteBundle,
}

#[derive(Debug, Clone, Copy, Component, Default, Reflect)]
#[reflect(Component)]
pub struct WeaponTag {
    /// weapon slot weapon is currently in, None if not attached to player
    pub stored_weapon_slot: Option<WeaponSlots>,
    /// weapons parent
    pub parent: Option<Entity>,
}

/// inserted to currently drawn weapon
#[derive(Debug, Clone, Copy, Component, Reflect, Default)]
#[reflect(Component)]
pub struct CurrentlySelectedWeapon;

#[derive(Debug, Clone, Copy, Component, Reflect, Default)]
#[reflect(Component)]
pub enum DamageType {
    #[default]
    KineticRanged,
    KineticMelee,
}

#[derive(Debug, Clone, Copy, Component, Reflect, Default)]
#[reflect(Component)]
pub struct WeaponStats {
    pub barreloffset: Vec3,
    pub damage: f32,
    pub firing_speed: f32,
    pub bullet_speed: f32,
    pub projectile_size: f32,
}

#[derive(Debug, Clone, Copy, Component, Hash, PartialEq, Eq, Reflect, FromReflect, Default)]
#[reflect(Component)]
pub enum WeaponSlots {
    #[reflect(default)]
    #[default]
    Slot1,
    Slot2,
    Slot3,
    Slot4,
}

#[derive(Debug, Clone, Component, Reflect, Default)]
#[reflect(Component)]
pub struct WeaponSocket {
    /// hashmap with weapon slots and entitys that are in those slots
    pub weapon_slots: HashMap<WeaponSlots, Option<Entity>>,
    /// weapon that should be visible from weaponslots hashmap
    pub drawn_slot: WeaponSlots,
}
