use bevy::{
    ecs::{component::Component, entity::Entity},
    log::warn,
    prelude::{Added, Changed, Children, Parent, Query, With},
};

use std::{
    collections::VecDeque,
    iter::Sum,
    ops::{Add, Mul},
    time::Duration,
};

use crate::ahp::game::TILE_SIZE;

/// how many points of health per stamina point
pub const HP_PER_STAMINA: i32 = 4;
/// how many points of speed per agility point
pub const SPEED_PER_AGILITY: i32 = 4;
/// how many points of damage per strength point
pub const DAMAGE_PER_STREGNTH: i32 = 4;
/// how many points of mana per intelligence point
pub const MANA_PER_INTELLIGENCE: i32 = 4;

//TODO: figure out if this and hp and buffs and damage should be seperate components or not
// intuition says that the buff and damage queues should be seperate components;

//TODO: track id of "effect"
/// modifiers too player stats
/// can be spell modifiers, curses, pretty much anything
#[derive(Debug)]
pub struct StatEffects(VecDeque<Effect>);

#[derive(Debug)]
pub enum EffectType {
    // TODO: impl buff modifiers.
    // this is multiplication
    /// SIGNED percentage modifiers. +5% -10% +11.5% etc
    PercentModifier(f32),
    /// SIGNED values added too stat.
    ///
    ///  ex. `+100 speed` `-10 hp` `+500 range`
    ValueModifier(f32),
}

#[derive(Debug, Default)]
enum EffectDuration {
    Infinite,
    DamageOverTime(Duration),
    #[default]
    Instant,
}

#[derive(Debug)]
pub struct Effect {
    /// duration of this effect
    duration: EffectDuration,
    /// what this effect does
    apply_type: EffectType,
}

pub use bevy::prelude::HierarchyQueryExt;

/// updates actor stats if equipment on player changes
pub fn sync_actor_stats(
    mut actors_with_new_equipment: Query<(Entity, &mut ActorStats), Changed<Children>>,
    mut added_stats: Query<(Entity, &mut ActorStats), Added<ActorStats>>,
    equipment: Query<&EquipmentStats, With<Parent>>,
    children: Query<&Children>,
) {
    for (actor_ent, mut actor_stat) in &mut actors_with_new_equipment {
        let child_equipment_attrs: Vec<Attributes> = children
            .iter_descendants(actor_ent)
            .filter_map(|f| Some(equipment.get(f).unwrap().calculated))
            // .filter(|f| { equipment.get(*f).is_ok() })
            // .map(|f| {equipment.get(f).unwrap().calculated})
            .collect();
        let current_equipment_amount = child_equipment_attrs.len() as i32;

        if actor_stat.equipment_amount != current_equipment_amount {
            actor_stat.equipment_amount = current_equipment_amount;
            let equipment_attrs_total: Attributes = child_equipment_attrs.into_iter().sum();

            actor_stat.equipment = equipment_attrs_total;
        }
        actor_stat.current = actor_stat.equipment + actor_stat.base;
    }

    for (actor_ent, mut actor_stat) in &mut added_stats {
        if actor_stat.current().all_zero() || actor_stat.base().all_zero() {
            warn!("actor was added without base stats or calculated current stats");
            actor_stat.current = Attributes::CREEP_DEFAULT;
            actor_stat.base = Attributes::CREEP_DEFAULT;
            actor_stat.added = Attributes::ZERO;
            actor_stat.equipment = Attributes::ZERO;
        }
    }
}

/// stats for npcs or players
#[derive(Debug, Component, Clone, Copy)]
pub struct ActorStats {
    /// current actor health
    pub health: f32,
    /// equipment amount
    pub equipment_amount: i32,
    /// final stats calculated from equipment, spawned and base
    current: Attributes,
    /// attributes collected from equipment
    equipment: Attributes,
    /// attributes assigned at spawn
    base: Attributes,
    /// attributes added from levels/perks/rewards/buffs
    added: Attributes,
    //TODO: make a "BuffQueue" of Buff
}

/// stats for equipment or items
#[derive(Debug, Component, Clone, Copy)]
pub struct EquipmentStats {
    /// equipment amount
    pub upgrade_amount: i32,
    /// final stats calculated from equipment, spawned and base
    calculated: Attributes,
    /// attributes collected from equipment
    equipment: Attributes,
    /// attributes assigned at spawn
    spawned: Attributes,
    /// attributes added from levels/perks/rewards/buffs
    added: Attributes,
    //TODO: reuse "BuffQueue" of Buff for weapon upgrade system
}

impl ActorStats {
    /// returns ref too final stat values
    pub const fn current(&self) -> &Attributes {
        &self.current
    }
    /// returns the values this actor was spawned with
    pub const fn base(&self) -> &Attributes {
        &self.base
    }
    /// returns the values of this actors equipment
    pub const fn equipment(&self) -> &Attributes {
        &self.equipment
    }

    /// creates `ActorStats` with uncalculated final attributes
    pub fn new(base: Attributes, extra: Option<Attributes>) -> Self {
        let new = base.add(extra.unwrap_or(Attributes::ZERO));
        Self {
            health: new.max_hp,
            equipment_amount: 0,
            base: new,
            equipment: Attributes::ZERO,
            added: Attributes::ZERO,
            current: new,
        }
    }

    /// creates `ActorStats` with calculated values from passed `Attributes`
    pub const fn from_attrs(attrs: Attributes) -> Self {
        Self {
            health: attrs.max_hp,
            equipment_amount: 0,
            current: attrs,
            base: attrs,
            added: Attributes::ZERO,
            equipment: Attributes::ZERO,
        }
    }
}

impl Default for ActorStats {
    /// creates default `ActorStats` with calculated values
    fn default() -> Self {
        let attrs = Attributes::CREEP_DEFAULT;

        Self {
            health: attrs.max_hp,
            current: attrs,
            base: attrs,
            added: Attributes::ZERO,
            equipment: Attributes::ZERO,
            equipment_amount: 0,
        }
    }
}

/// stats updated from equipment and "base stats"
#[derive(Debug, Component, Clone, Copy)]
pub struct Attributes {
    /// how much damage can this actor take
    max_hp: f32,
    /// damage recovery rate per second
    hp_regen: f32,
    /// also called energy. used for special attacks
    max_mana: f32,
    /// recovery rate per second
    mana_regen: f32,
    /// how fast can this actor move
    move_speed: f32,
    /// calculates damage
    strength: i32,
    /// calculates speed
    agility: i32,
    /// calculates mana
    intelligence: i32,
    /// unarmed damage, not applied too weapons
    damage: f32,
    /// unarmed range. not applied too ranged weapons
    range: f32,
    /// unarmed attack speed, also effects reload speed
    arm_speed: f32,
    /// armor amount. just a damage reduction
    armor: f32,
}

impl Attributes {
    /// attributes all set too `1`
    pub const ONE: Self = Self {
        max_hp: 1.0,
        hp_regen: 1.0,
        max_mana: 1.0,
        mana_regen: 1.0,
        move_speed: 1.0,
        strength: 1,
        agility: 1,
        intelligence: 1,
        damage: 1.0,
        range: 1.0,
        arm_speed: 1.0,
        armor: 1.0,
    };

    /// attributes all set too `0`.
    pub const ZERO: Self = Self {
        max_hp: 0.0,
        hp_regen: 0.0,
        max_mana: 0.0,
        mana_regen: 0.0,
        move_speed: 0.0,
        strength: 0,
        agility: 0,
        intelligence: 0,
        damage: 0.0,
        range: 0.0,
        arm_speed: 0.0,
        armor: 0.0,
    };

    /// default attributes for hero actors
    pub const HERO_DEFAULT: Self = Self {
        max_hp: 200.0,
        hp_regen: 10.0,
        max_mana: 200.0,
        mana_regen: 10.0,
        move_speed: 120.0,
        strength: 10,
        agility: 10,
        intelligence: 10,
        damage: 5.0,
        range: (TILE_SIZE.x * 1.5),
        arm_speed: 1.0,
        armor: 10.0,
    };

    /// default attributes for trash mob actors
    pub const CREEP_DEFAULT: Self = Self {
        max_hp: 75.0,
        hp_regen: 2.5,
        max_mana: 50.0,
        mana_regen: 4.0,
        move_speed: 90.0,
        strength: 10,
        agility: 10,
        intelligence: 10,
        damage: 5.0,
        range: (TILE_SIZE.x * 1.5),
        arm_speed: 1.0,
        armor: 10.0,
    };

    /// default attributes for "elite" mob actors
    pub const ELITE_DEFAULT: Self = Self {
        max_hp: 150.0,
        hp_regen: 6.0,
        max_mana: 120.0,
        mana_regen: 6.0,
        move_speed: 110.0,
        strength: 10,
        agility: 10,
        intelligence: 10,
        damage: 5.0,
        range: (TILE_SIZE.x * 1.5),
        arm_speed: 1.0,
        armor: 10.0,
    };

    /// default attributes for boss mob actors
    pub const BOSS_DEFAULT: Self = Self {
        max_hp: 600.0,
        hp_regen: 5.5,
        max_mana: 500.0,
        mana_regen: 20.0,
        move_speed: 140.0,
        strength: 10,
        agility: 10,
        intelligence: 10,
        damage: 5.0,
        range: (TILE_SIZE.x * 1.5),
        arm_speed: 1.0,
        armor: 10.0,
    };

    /// multiplies all attributes by passed value
    fn scale(scale: i32) -> Self {
        let scale_i32 = scale;
        let scale_f32 = scale as f32;
        Self {
            max_hp: 100.0 * scale_f32,
            hp_regen: 5.0 * scale_f32,
            max_mana: 200.0 * scale_f32,
            mana_regen: 10.0 * scale_f32,
            move_speed: 100.0 * scale_f32,
            strength: 10 * scale_i32,
            agility: 10 * scale_i32,
            intelligence: 10 * scale_i32,
            damage: 5.0 * scale_f32,
            range: (TILE_SIZE.x * (1.5 * scale_f32)),
            arm_speed: 1.0 * scale_f32,
            armor: 10.0 * scale_f32,
        }
    }

    /// checks if all values in `self` are == 0
    fn all_zero(&self) -> bool {
        self.max_hp == 0.0
            && self.hp_regen == 0.0
            && self.max_mana == 0.0
            && self.mana_regen == 0.0
            && self.move_speed == 0.0
            && self.strength == 0
            && self.agility == 0
            && self.intelligence == 0
            && self.damage == 0.0
            && self.range == 0.0
            && self.arm_speed == 0.0
            && self.armor == 0.0
    }
}

impl Add for Attributes {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            max_hp: self.max_hp + rhs.max_hp,
            hp_regen: self.hp_regen + rhs.hp_regen,
            max_mana: self.max_mana + rhs.max_mana,
            mana_regen: self.mana_regen + rhs.mana_regen,
            move_speed: self.move_speed + rhs.move_speed,
            strength: self.strength + rhs.strength,
            agility: self.agility + rhs.agility,
            intelligence: self.intelligence + rhs.intelligence,
            damage: self.damage + rhs.damage,
            range: self.range + rhs.range,
            arm_speed: self.arm_speed + rhs.arm_speed,
            armor: self.armor + rhs.armor,
        }
    }
}

impl Mul for Attributes {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            max_hp: self.max_hp * rhs.max_hp,
            hp_regen: self.hp_regen * rhs.hp_regen,
            max_mana: self.max_mana * rhs.max_mana,
            mana_regen: self.mana_regen * rhs.mana_regen,
            move_speed: self.move_speed * rhs.move_speed,
            strength: self.strength * rhs.strength,
            agility: self.agility * rhs.agility,
            intelligence: self.intelligence * rhs.intelligence,
            damage: self.damage * rhs.damage,
            range: self.range * rhs.range,
            arm_speed: self.arm_speed * rhs.arm_speed,
            armor: self.armor * rhs.armor,
        }
    }
}

impl Sum for Attributes {
    /// sums iterator of `Attributes` and returns a clone of the sum
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |acc, f| Self {
            max_hp: acc.max_hp + f.max_hp,
            hp_regen: acc.hp_regen + f.hp_regen,
            max_mana: acc.max_mana + f.max_mana,
            mana_regen: acc.mana_regen + f.mana_regen,
            move_speed: acc.move_speed + f.move_speed,
            strength: acc.strength + f.strength,
            agility: acc.agility + f.agility,
            intelligence: acc.intelligence + f.intelligence,
            damage: acc.damage + f.damage,
            range: acc.range + f.range,
            arm_speed: acc.arm_speed + f.arm_speed,
            armor: acc.armor + f.armor,
        })
    }
}
