use bevy::{
    math::Vec2,
    prelude::{Component, Deref, DerefMut, Entity, ReflectComponent},
    reflect::Reflect,
    time::Timer,
    utils::hashbrown::HashMap,
};
use crate::game::actors::attributes_stats::Damage;

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
    Debug, Copy, Clone, Component, Reflect, Default, Deref, DerefMut, serde::Deserialize, serde::Serialize,
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
