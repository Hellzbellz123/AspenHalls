use bevy::{
    math::Vec2,
    prelude::{Component, Deref, DerefMut, Entity, ReflectComponent},
    reflect::Reflect,
    time::Timer,
    utils::hashbrown::HashMap,
};

use crate::game::{attributes_stats::Damage, characters::components::WeaponSlot};

/// current ammo count for weapons clip
#[derive(Debug, Clone, Copy, Component, Default, Reflect, Deref, DerefMut)]
#[reflect(Component)]
pub struct CurrentAmmo {
    /// how much ammo this weapon currently has
    #[deref]
    pub current: u32,
    /// maximum amount of ammo weapon can have
    pub max: u32,
}

/// entity that holds this weapon, and the slot that it is in
#[derive(Debug, Clone, Copy, Component, Default, Reflect, Deref, DerefMut)]
#[reflect(Component)]
pub struct WeaponHolder(pub Option<(WeaponSlot, Entity)>);

/// inserted to currently drawn weapon
#[derive(Debug, Clone, Copy, Component, Reflect, Default)]
#[reflect(Component)]
pub struct CurrentlyDrawnWeapon;

/// characters available weapon slots and currently drawn weapon slot
#[derive(Debug, Clone, Component, Reflect, Default)]
#[reflect(Component)]
pub struct WeaponCarrier {
    /// hashmap with weapon slots and entities that are in those slots
    pub weapon_slots: HashMap<WeaponSlot, Option<Entity>>,
    /// weapon that should be visible from weapon slots hashmap
    pub drawn_slot: Option<WeaponSlot>,
}

/// damage that weapon attacks can do
#[derive(
    Debug,
    Copy,
    Clone,
    Component,
    Reflect,
    Default,
    Deref,
    DerefMut,
    serde::Deserialize,
    serde::Serialize,
)]
#[reflect(Component)]
pub struct AttackDamage(pub Damage);

/// information describing how a weapon attacks and the paramaters for attack
#[derive(
    Debug, Clone, Copy, PartialEq, Component, Reflect, serde::Deserialize, serde::Serialize,
)]
#[reflect(Component)]
pub enum WeaponDescriptor {
    // /// ball and chain
    // ///
    // /// extends from then orbits around character
    // Flail {
    //     /// how large is flail end
    //     ball_size: f32,
    //     /// how far from character can flail be
    //     chain_lenght: f32,
    //     /// how long is flail away from character
    //     extend_time: f32,
    // },

    // /// sword/dagger/stabby thing
    // Blade(Blade),
    /// shoots projectiles from a clip, reloads self usually
    Gun(GunCfg),
}

/// encapsulated data for swing style weapons
#[derive(Debug, Clone, Copy, Reflect, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct BladeCfg {
    /// how long is sword
    length: f32,
    /// arc of swing
    swing_arc: f32,
    /// how long 1 swing takes
    swing_speed: f32,
}

// new type for blade weapon attack type
// single swing (swing_arc, swing_speed)
// list of swings (Vec<(f32, f32)>)
// others?

/// encapsulated gun data for for enum variants
#[derive(Debug, Clone, Copy, Reflect, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct GunCfg {
    /// projectile velocity
    pub projectile_speed: f32,
    /// projectile size
    pub projectile_size: f32,
    /// where projectile spawns
    pub barrel_end: Vec2,
    /// how many shots before reload
    pub max_ammo: u32,
    /// how long reload takes
    pub reload_time: f32,
    /// how long between shots
    pub fire_rate: f32,
}

impl Default for WeaponDescriptor {
    fn default() -> Self {
        Self::Gun(GunCfg {
            projectile_speed: 100.0,
            projectile_size: 20.0,
            barrel_end: Vec2 { x: 0.0, y: 0.0 },
            max_ammo: 10,
            reload_time: 0.5,
            fire_rate: 1.0,
        })
    }
}

/// timers used for weapon attacks
#[derive(Debug, Clone, Default, Reflect, Component, serde::Deserialize, serde::Serialize)]
pub struct WeaponTimers {
    /// time between weapon attacks
    pub attack: Timer,
    /// time between weapon reloads / charge full
    pub refill: Timer,
    /// timer for max weapon attack time
    ///
    /// basically weapon heat
    pub duration: Timer,
}
