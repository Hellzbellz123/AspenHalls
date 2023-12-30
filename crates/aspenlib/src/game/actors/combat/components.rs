use bevy::{
    math::Vec2,
    prelude::{Bundle, Component, Deref, DerefMut, Entity, Name, ReflectComponent},
    reflect::Reflect,
    sprite::SpriteSheetBundle,
    time::Timer,
    transform::TransformBundle,
    utils::hashbrown::HashMap,
};

use bevy_rapier2d::prelude::{Collider, CollisionGroups};

use crate::{
    bundles::RigidBodyBundle,
    game::actors::attributes_stats::{Damage, ElementalEffect, EquipmentStats, PhysicalDamage},
};

/// bundle for spawning weapons
#[derive(Bundle, Reflect, Clone)]
pub struct WeaponBundle {
    /// weapon name
    pub name: Name,
    /// weapon stored slot
    pub holder: WeaponHolder,
    /// weapons function when used
    pub damage: AttackDamage,
    /// how this weapon attacks, along with data for attack
    pub weapon_type: WeaponForm,
    /// stats applied too holder
    pub stats: EquipmentStats,
    /// sprite for weapon
    #[reflect(ignore)]
    pub sprite: SpriteSheetBundle,
    /// weapon physics
    #[reflect(ignore)]
    pub rigidbody_bundle: RigidBodyBundle,
    // /// weapon stats
    // pub weapon_stats: WeaponStats,
}

impl std::fmt::Debug for WeaponBundle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WeaponBundle")
            .field("name", &self.name)
            .field("holder", &self.holder)
            .field("damage", &self.damage)
            .field("weapon_type", &self.weapon_type)
            .field("stats", &self.stats)
            .field("sprite", &self.sprite.texture_atlas)
            .field("rigidbody_bundle", &self.rigidbody_bundle.rigidbody)
            .finish()
    }
}

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

/// collider tag for weapons
#[derive(Debug, Component)]
pub struct WeaponColliderTag;

/// entity that holds this weapon, and the slot that it is in
#[derive(Debug, Clone, Copy, Component, Default, Reflect, Deref, DerefMut)]
#[reflect(Component)]
pub struct WeaponHolder(pub Option<(Entity, WeaponSlots)>);

/// inserted to currently drawn weapon
#[derive(Debug, Clone, Copy, Component, Reflect, Default)]
#[reflect(Component)]
pub struct CurrentlyDrawnWeapon;

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

/// characters available weapon slots and currently drawn weapon slot
#[derive(Debug, Clone, Component, Reflect, Default)]
#[reflect(Component)]
pub struct WeaponSocket {
    /// hashmap with weapon slots and entities that are in those slots
    pub weapon_slots: HashMap<WeaponSlots, Option<Entity>>,
    /// weapon that should be visible from weapon slots hashmap
    pub drawn_slot: Option<WeaponSlots>,
}

#[derive(
    Debug, Clone, Component, Reflect, Default, Deref, DerefMut, serde::Deserialize, serde::Serialize,
)]
#[reflect(Component)]
pub struct AttackDamage(pub Damage);

#[derive(
    Debug, Clone, Copy, PartialEq, Component, Reflect, serde::Deserialize, serde::Serialize,
)]
#[reflect(Component)]
pub enum WeaponForm {
    /// ball and chain
    ///
    /// extends from then orbits around character
    Flail {
        ball_size: f32,
        chain_lenght: f32,
        extend_time: f32,
    },

    /// sword/dagger/stabby thing
    Blade {
        /// how long is sword
        length: f32,
        /// arc of swing
        swing_arc: f32,
        /// how long 1 swing takes
        swing_speed: f32,
    },

    Gun {
        /// projectile velocity
        projectile_speed: f32,
        /// projectile size
        projectile_size: f32,
        /// where projectile spawns
        barrel_end: Vec2,
        /// how many shots before reload
        ammo_amount: i32,
        /// how long reload takes
        reload_time: f32,
        /// how long between shots
        fire_rate: f32,
    },
}

impl Default for WeaponForm {
    fn default() -> Self {
        Self::Gun {
            projectile_speed: 100.0,
            projectile_size: 20.0,
            barrel_end: Vec2 { x: 0.0, y: 0.0 },
            ammo_amount: 10,
            reload_time: 0.5,
            fire_rate: 1.0,
        }
    }
}

#[derive(Debug, Clone, Default, Reflect, serde::Deserialize, serde::Serialize)]
pub struct WeaponTimers {
    /// time between weapon attacks
    pub attack_timer: Timer,
    /// time between weapon reloads / charge refills
    pub refill_timer: Timer,
    /// timer for max weapon attack time
    ///
    /// basically weapon heat
    pub duration_timer: Timer,
}

// // TODO: YEET
// /// tag for easy query on bullet endpoint
// #[derive(Debug, Component)]
// pub struct BarrelPointTag;

// // TODO: YEET
// /// location where bullet spawns on weapon
// #[derive(Bundle)]
// pub struct WeaponBarrelEndPoint {
//     /// barrel endpoint name
//     pub name: Name,
//     /// barrel point tag
//     pub tag: BarrelPointTag,
//     /// barrel point visual
//     pub sprite: SpriteBundle,
// }

// /// weapon type
// #[derive(Debug, Clone, Copy, Component, Reflect, serde::Deserialize, serde::Serialize)]
// #[reflect(Component)]
// pub enum WeaponType {
//     Melee(DamageType),
//     Ranged(DamageType)
// }

// impl Default for WeaponType {
//     fn default() -> Self {
//         WeaponType::Ranged(DamageType::Kinetic)
//     }
// }

// #[derive(Debug, Clone, Copy, Reflect, serde::Deserialize, serde::Serialize)]
// pub enum DamageType {
//     Kinetic,
//     //TODO: this should be different
//     // either an enum with data or split into different enums
//     Elemental,
// }

// /// weapon stats
// #[derive(Debug, Clone, Copy, Component, Reflect, Default, serde::Deserialize, serde::Serialize)]
// #[reflect(Component)]
// pub struct WeaponStats {
//     pub weapon_type: WeaponType,
//     /// amount of damage bullet does
//     pub damage: f32,
//     /// where bullet spawns on weapon
//     pub barrel_offset: Vec3,
//     /// how often too spawn bullet
//     pub attack_speed: f32,
//     /// how fast bullet travels
//     pub bullet_speed: f32,
//     /// how large is projectile
//     pub projectile_size: f32,
// }

// /// type of damage
// #[derive(Debug, Clone, Copy, Reflect, Default)]
// pub enum DamageType {
//     #[default]
//     /// physical ranged
//     KineticRanged,
//     /// physical melee
//     KineticMelee,
// }
