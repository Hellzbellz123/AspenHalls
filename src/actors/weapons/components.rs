use bevy::{
    prelude::{Bundle, Component, Entity, Handle, Name, ReflectComponent, SpatialBundle, Vec2},
    reflect::{FromReflect, Reflect},
    sprite::{TextureAtlas, TextureAtlasSprite},
    transform::TransformBundle,
    utils::hashbrown::HashMap,
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
pub struct CurrentlyDrawnWeapon;

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
    pub barreloffset: Vec2,
    pub damage: f32,
    pub speed: f32,
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
