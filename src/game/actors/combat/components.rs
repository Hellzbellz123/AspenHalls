use bevy::{
    prelude::{Bundle, Component, Deref, DerefMut, Entity, Name, ReflectComponent, Vec3},
    reflect::Reflect,
    sprite::{SpriteBundle, SpriteSheetBundle},
    transform::TransformBundle,
    utils::hashbrown::HashMap,
};

use bevy_rapier2d::prelude::{Collider, CollisionGroups};

use crate::{bundles::RigidBodyBundle, game::actors::spawners::components::WeaponType};

/// new type around f32, for damage
#[derive(Debug, Component, Deref, DerefMut, Reflect)]
pub struct Damage(pub f32);

/// bundle for spawning weapons
#[derive(Bundle)]
pub struct WeaponBundle {
    /// weapon name
    pub name: Name,
    /// weapon stored slot
    pub tag: Weapon,
    /// weapon type
    pub weapon_type: WeaponType,
    /// weapon stats
    pub weapon_stats: WeaponStats,
    /// damage type
    pub damage_type: DamageType,
    /// sprite for weapon
    pub sprite: SpriteSheetBundle,
    /// weapon physics
    pub rigidbody_bundle: RigidBodyBundle,
}

/// collider tag for weapons
#[derive(Debug, Component)]
pub struct WeaponColliderTag;

/// weapon collider
#[derive(Bundle)]
pub struct WeaponColliderBundle {
    /// collider name
    pub name: Name,
    /// collider tag
    pub tag: WeaponColliderTag,
    /// collider shape
    pub collider: Collider,
    /// collision groups
    pub collision_groups: CollisionGroups,
    /// collider transform
    pub transform_bundle: TransformBundle,
}

/// tag for easy query on bullet endpoint
#[derive(Debug, Component)]
pub struct BarrelPointTag;

/// location where bullet spawns on weapon
#[derive(Bundle)]
pub struct WeaponBarrelEndPoint {
    /// barrel endpoint name
    pub name: Name,
    /// barrel point tag
    pub tag: BarrelPointTag,
    /// barrel point visual
    pub sprite: SpriteBundle,
}

#[derive(Debug, Clone, Copy, Component, Default, Reflect)]
#[reflect(Component)]
/// things with this are weapons
/// 
/// 
pub struct Weapon {
    /// slot weapon is currently in, None if not attached to an actor
    pub holder_slot: Option<WeaponSlots>,
    /// actor holding this weapon
    pub holder: Option<Entity>,
}

/// inserted to currently drawn weapon
#[derive(Debug, Clone, Copy, Component, Reflect, Default)]
#[reflect(Component)]
pub struct CurrentlySelectedWeapon;

/// type of damage
#[derive(Debug, Clone, Copy, Component, Reflect, Default)]
#[reflect(Component)]
pub enum DamageType {
    #[default]
    /// physical ranged
    KineticRanged,
    /// physical melee
    KineticMelee,
}

/// weapon stats
#[derive(Debug, Clone, Copy, Component, Reflect, Default)]
#[reflect(Component)]
pub struct WeaponStats {
    /// where bullet spawns on weapon
    pub barrel_offset: Vec3,
    /// amount of damage bullet does
    pub damage: f32,
    /// how often too spawn bullet
    pub attack_speed: f32,
    /// how fast bullet travels
    pub bullet_speed: f32,
    /// how large is projectile
    pub projectile_size: f32,
}

/// weapon slots that can be filled
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Reflect, Default)]
pub enum WeaponSlots {
    #[reflect(default)]
    #[default]
    /// first slot
    Slot1,
    /// second slot
    Slot2,
    /// third slot
    Slot3,
    /// fourth slot
    Slot4,
}

#[derive(Debug, Clone, Component, Reflect, Default)]
#[reflect(Component)]
/// actors `WeaponSocket`, holds weapons in `WeaponSlots` and currently drawn weapon
pub struct WeaponSocket {
    /// hashmap with weapon slots and entities that are in those slots
    pub weapon_slots: HashMap<WeaponSlots, Option<Entity>>,
    /// weapon that should be visible from weapon slots hashmap
    pub drawn_slot: Option<WeaponSlots>,
}
