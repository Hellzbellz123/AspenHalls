#![allow(unused)]
use std::{
    collections::VecDeque,
    iter::Sum,
    ops::{Add, Mul},
    time::Duration,
};

use bevy::{
    ecs::{bundle::Bundle, component::Component, entity::Entity, reflect::ReflectComponent},
    log::warn,
    prelude::{Added, Changed, Children, HierarchyQueryExt, Parent, Query, With},
    reflect::Reflect,
};

use crate::consts::TILE_SIZE;

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
/// updates actor stats if equipment on player changes
pub fn sync_character_stats(
    mut characters_with_changed_children: Query<(Entity, &mut CharacterStats), Changed<Children>>,
    mut added_stats: Query<(Entity, &mut CharacterStats), Added<CharacterStats>>,
    equipment: Query<&EquipmentStats, With<Parent>>,
    children: Query<&Children>,
) {
    for (character, mut stats) in &mut characters_with_changed_children {
        let equipment_total_attrs: Vec<Attributes> = children
            .iter_descendants(character)
            .filter_map(|f| {
                if equipment.get(f).is_ok() {
                    Some(equipment.get(f).unwrap().calculated)
                } else {
                    None
                }
            })
            .collect();
        let current_equipment_amount = equipment_total_attrs.len() as u32;

        if stats.equipment_amount != current_equipment_amount {
            stats.equipment_amount = current_equipment_amount;
            let equipment_attrs_total: Attributes = equipment_total_attrs.into_iter().sum();

            stats.equipment = equipment_attrs_total;
            stats.current = stats.equipment + stats.base;
        }
    }

    for (_actor_ent, mut actor_stat) in &mut added_stats {
        if actor_stat.attrs().is_all_zero() || actor_stat.get_base_attrs().is_all_zero() {
            warn!("actor was added without base stats or calculated current stats");
            actor_stat.current = Attributes::CREEP_DEFAULT;
            actor_stat.base = Attributes::CREEP_DEFAULT;
            actor_stat.added = Attributes::ZERO;
            actor_stat.equipment = Attributes::ZERO;
        }
    }
}

/// stats related components for characters
#[derive(Debug, Bundle, Clone, Reflect)]
pub struct CharacterStatBundle {
    /// attributes thi character has
    stats: CharacterStats,
    /// list of all effects applied too character
    effects: EffectQueue,
    /// list of all damage too be applied too character
    damage: DamageQueue,
}

//TODO: track id of "effect"
/// modifiers too player stats
/// can be spell modifiers, curses, pretty much anything
#[derive(Debug, Component, Clone, Reflect)]
#[reflect(Component)]
pub struct EffectQueue {
    /// list of positive effects applied too character
    pub buffs: VecDeque<Effect>,
    /// list of negative effects applied too character
    pub debuffs: VecDeque<Effect>,
    /// attributes applied from this effect queue
    pub current: Attributes,
    /// total amount of effects this character has
    pub amount: u32,
    /// maximum amount of effects this character can have
    pub max: u32,
}

/// damage list that can be applied too the character
#[derive(Debug, Component, Clone, Reflect, Default)]
#[reflect(Component)]
pub struct DamageQueue {
    /// instances of damage applied too character
    instances: VecDeque<Damage>,
    /// does this damage queue accept damage
    immune: bool,
}

//TODO: make a "BuffQueue" of Buff
/// stats for npcs or players
#[derive(Debug, Component, Reflect, Clone, Copy)]
pub struct CharacterStats {
    /// current actor health
    pub health: f32,
    /// current actor mana
    pub mana: f32,
    /// elemental buffer values
    element_buffer: ElementalBuffers,
    /// number of equipments on actor
    equipment_amount: u32,
    /// final stats calculated from equipment, spawned and base
    current: Attributes,
    /// attributes collected from equipment
    equipment: Attributes,
    /// attributes assigned at spawn
    base: Attributes,
    /// attributes added from levels/perks/rewards/buffs
    added: Attributes,
}

//TODO: reuse "BuffQueue" of Buff for weapon upgrade system
/// stats for equipment or items
#[derive(Debug, Component, Reflect, Clone, Copy)]
#[reflect(Component)]
pub struct EquipmentStats {
    /// amount of upgrades too this equipment
    pub upgrade_amount: u32,
    /// final stats calculated from base equipment stat values and equipment upgrades value
    calculated: Attributes,
    /// attributes assigned at spawn
    spawned: Attributes,
    /// attributes added from levels/perks/rewards/buffs
    upgrades: Attributes,
}

/// projectile data
#[derive(Debug, Component, Clone, Copy, Reflect, serde::Deserialize, serde::Serialize)]
pub struct ProjectileStats {
    /// did this projectile originate from a player
    pub entity_that_shot: Entity,
    /// damage from this projectile
    pub damage: Damage,
}

/// damage amounts
#[derive(Debug, Clone, Copy, Default, PartialEq, Reflect, serde::Deserialize, serde::Serialize)]
pub struct Damage {
    /// damage too apply
    pub physical: PhysicalDamage,
    /// damage applied too elemental buffer
    pub elemental: ElementalEffect,
}

/// damage applied directly too characters health
#[derive(Debug, Clone, Copy, PartialEq, Default, Reflect, serde::Deserialize, serde::Serialize)]
pub struct PhysicalDamage(pub f32);

//TODO:
// create buffer for enemies that is filled up by elemental damage
// apply unique effects too characters when elemental buffer is filled
// earth: character should be slowed
// fire: character should burn for a bit
// water: idek
// air: character should randomly get a burst of uncontrollable speed
/// different elemental buffer damage amounts that attacks/actions can apply
#[derive(Debug, Clone, Copy, PartialEq, Reflect, Default, serde::Deserialize, serde::Serialize)]
pub enum ElementalEffect {
    /// no elemental effect
    #[default]
    None,
    /// attack effects earth buffer
    Earth(f32),
    /// attack effects fire buffer
    Fire(f32),
    /// attack effects water buffer
    Water(f32),
    /// attack effects air buffer
    Air(f32),
}

/// resistances too different attack special effects
#[derive(Debug, Clone, Copy, PartialEq, Reflect, serde::Deserialize, serde::Serialize)]
pub struct ElementalBuffers {
    /// max earth element before effect
    earth: f32,
    /// max fire element before effect
    fire: f32,
    /// max water element before effect
    water: f32,
    /// max air element before effect
    air: f32,
    /// how fast the buffer recharges
    repair_rate: f32,
}

/// modifier applied too player stats
#[derive(Debug, Clone, Reflect, PartialEq)]
pub struct Effect {
    /// duration of this effect
    duration: EffectDuration,
    /// what this effect does
    apply_type: EffectType,
}

/// effects value and how it is applied
#[derive(Debug, Clone, PartialEq, Reflect)]
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

/// how long should this effect last
#[derive(Debug, Clone, Hash, PartialEq, Eq, Reflect)]
enum EffectDuration {
    /// effect never wears off unless removed or entitity dies
    Infinite,
    /// effects value is applied over total duration
    OverTime(Duration),
    /// all of this effect is applied instantly
    Instant,
}

/// stats updated from equipment and "base stats"
#[derive(Debug, Reflect, Component, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct Attributes {
    /// how much damage can this actor take
    pub max_hp: f32,
    /// also called energy. used for special attacks
    pub max_mana: f32,
    /// calculates damage
    pub strength: i32,
    /// calculates speed
    pub agility: i32,
    /// calculates mana
    pub intelligence: i32,
    /// armor amount. just a damage reduction
    pub armor: i32,
    /// damage recovery rate per second
    pub hp_regen: f32,
    /// recovery rate per second
    pub mana_regen: f32,
    /// how fast can this actor move
    pub base_speed: f32,
    /// unarmed range. not applied too ranged weapons
    pub range: f32,
    /// unarmed attack speed, also effects reload speed
    pub arm_speed: f32,
    /// unarmed damage, not applied too weapons
    pub damage: Damage,
}

impl Default for EffectQueue {
    fn default() -> Self {
        Self {
            buffs: VecDeque::with_capacity(15),
            debuffs: VecDeque::with_capacity(15),
            amount: 0,
            max: 15,
            current: Attributes::ZERO,
        }
    }
}

impl DamageQueue {
    /// default empty damage queue
    pub const DEFAULT: Self = Self {
        instances: VecDeque::new(),
        immune: false,
    };

    /// returns an iterator over the damage in the queue
    pub fn iter_queue(&self) -> std::collections::vec_deque::Iter<'_, Damage> {
        self.instances.iter()
    }

    /// add damage too queue
    pub fn push_damage(&mut self, damage: Damage) {
        self.instances.push_front(damage);
    }

    /// sets empty damage queue
    pub fn empty_queue(&mut self) {
        self.instances = VecDeque::new();
    }

    /// sets immunity too damage
    pub fn set_immune(&mut self, immune: bool) {
        self.immune = immune;
    }
}

impl Default for ElementalBuffers {
    fn default() -> Self {
        Self {
            earth: 500.0,
            fire: 500.0,
            water: 500.0,
            air: 500.0,
            repair_rate: 10.0,
        }
    }
}

impl CharacterStatBundle {
    /// creates character stat bundle from passed attributes
    pub fn from_attrs(attrs: Attributes) -> Self {
        Self {
            stats: CharacterStats::from_attrs(attrs, None),
            ..Default::default()
        }
    }
}

impl Default for CharacterStatBundle {
    fn default() -> Self {
        Self {
            stats: CharacterStats::from_attrs(Attributes::CREEP_DEFAULT, None),
            effects: EffectQueue::default(),
            damage: DamageQueue::DEFAULT,
        }
    }
}

impl EquipmentStats {
    /// get upgrade total
    pub const fn get_upgrade_amount(&self) -> u32 {
        self.upgrade_amount
    }

    /// set upgrade total
    pub fn set_upgrade_amount(&mut self, amt: u32) {
        self.upgrade_amount = amt;
    }

    /// returns ref too final stat values
    pub const fn attrs(&self) -> &Attributes {
        &self.calculated
    }

    /// returns the values this actor was spawned with
    pub const fn get_base_attrs(&self) -> &Attributes {
        &self.spawned
    }

    /// returns the values of this actors equipment
    pub const fn get_equipment_attrs(&self) -> &Attributes {
        &self.upgrades
    }

    /// creates `ActorStats` with boosted base attributes and calculated final attributes
    pub fn from_attrs(base: Attributes, extra: Option<Attributes>) -> Self {
        let new = base.add(extra.unwrap_or(Attributes::ZERO));
        Self {
            upgrade_amount: 0,
            calculated: new,
            spawned: new,
            upgrades: Attributes::ZERO,
        }
    }
}

impl CharacterStats {
    /// get current health
    pub const fn get_current_health(&self) -> f32 {
        self.health
    }

    /// gets current equipment total
    pub const fn get_equipment_amount(&self) -> u32 {
        self.equipment_amount
    }

    /// set health too amount
    pub fn set_health(&mut self, amt: f32) {
        self.health = amt;
    }

    /// remove damamge value from total health
    pub fn apply_damage(&mut self, amt: Damage) {
        self.health -= amt.physical.0;
        match amt.elemental {
            ElementalEffect::None => {}
            ElementalEffect::Earth(v) => self.element_buffer.earth -= v,
            ElementalEffect::Fire(v) => self.element_buffer.fire -= v,
            ElementalEffect::Water(v) => self.element_buffer.water -= v,
            ElementalEffect::Air(v) => self.element_buffer.air -= v,
        }
    }

    /// set equpment amount
    pub fn set_equipment_amount(&mut self, amt: u32) {
        self.equipment_amount = amt;
    }

    /// returns ref too final stat values
    pub const fn attrs(&self) -> &Attributes {
        &self.current
    }

    /// returns the values this actor was spawned with
    pub const fn get_base_attrs(&self) -> &Attributes {
        &self.base
    }

    /// returns the values of this actors equipment
    pub const fn get_equipment_attrs(&self) -> &Attributes {
        &self.equipment
    }

    /// creates `ActorStats` with boosted base attributes and calculated final attributes
    pub fn from_attrs(base: Attributes, extra: Option<Attributes>) -> Self {
        let new = base.add(extra.unwrap_or(Attributes::ZERO));
        Self {
            health: new.max_hp,
            mana: new.max_mana,
            equipment_amount: 0,
            base: new,
            current: new,
            equipment: Attributes::ZERO,
            added: Attributes::ZERO,
            element_buffer: ElementalBuffers::default(),
        }
    }
}

impl Default for CharacterStats {
    /// creates default `ActorStats` with calculated values
    fn default() -> Self {
        let attrs = Attributes::CREEP_DEFAULT;

        Self {
            health: attrs.max_hp,
            mana: attrs.max_mana,
            current: attrs,
            base: attrs,
            added: Attributes::ZERO,
            equipment: Attributes::ZERO,
            equipment_amount: 0,
            element_buffer: ElementalBuffers::default(),
        }
    }
}

impl Attributes {
    /// attributes all set too `1`
    pub const ONE: Self = Self {
        max_hp: 1.0,
        hp_regen: 1.0,
        max_mana: 1.0,
        mana_regen: 1.0,
        base_speed: 1.0,
        strength: 1,
        agility: 1,
        intelligence: 1,
        damage: Damage {
            physical: PhysicalDamage(1.0),
            elemental: ElementalEffect::None,
        },
        range: 1.0,
        arm_speed: 1.0,
        armor: 1,
    };

    /// attributes all set too `0`.
    pub const ZERO: Self = Self {
        max_hp: 0.0,
        hp_regen: 0.0,
        max_mana: 0.0,
        mana_regen: 0.0,
        base_speed: 0.0,
        strength: 0,
        agility: 0,
        intelligence: 0,
        damage: Damage {
            physical: PhysicalDamage(0.0),
            elemental: ElementalEffect::None,
        },
        range: 0.0,
        arm_speed: 0.0,
        armor: 0,
    };

    /// default attributes for hero actors
    pub const WEAPON_DEFAULT: Self = Self {
        max_hp: 10.0,
        hp_regen: 0.5,
        max_mana: 20.0,
        mana_regen: 0.5,
        base_speed: 10.0,
        strength: 2,
        agility: 2,
        intelligence: 6,
        damage: Damage {
            physical: PhysicalDamage(40.0),
            elemental: ElementalEffect::None,
        },
        range: (TILE_SIZE * 1.5),
        arm_speed: 0.4,
        armor: 2,
    };

    /// default attributes for hero actors
    pub const HERO_DEFAULT: Self = Self {
        max_hp: 200.0,
        hp_regen: 10.0,
        max_mana: 200.0,
        mana_regen: 10.0,
        base_speed: 120.0,
        strength: 10,
        agility: 10,
        intelligence: 10,
        damage: Damage {
            physical: PhysicalDamage(5.0),
            elemental: ElementalEffect::None,
        },
        range: (TILE_SIZE * 1.5),
        arm_speed: 1.0,
        armor: 10,
    };

    /// default attributes for trash mob actors
    pub const CREEP_DEFAULT: Self = Self {
        max_hp: 75.0,
        hp_regen: 2.5,
        max_mana: 50.0,
        mana_regen: 4.0,
        base_speed: 90.0,
        strength: 10,
        agility: 10,
        intelligence: 10,
        damage: Damage {
            physical: PhysicalDamage(5.0),
            elemental: ElementalEffect::None,
        },
        range: (TILE_SIZE * 1.5),
        arm_speed: 1.0,
        armor: 10,
    };

    /// default attributes for "elite" mob actors
    pub const ELITE_DEFAULT: Self = Self {
        max_hp: 150.0,
        hp_regen: 6.0,
        max_mana: 120.0,
        mana_regen: 6.0,
        base_speed: 110.0,
        strength: 10,
        agility: 10,
        intelligence: 10,
        damage: Damage {
            physical: PhysicalDamage(5.0),
            elemental: ElementalEffect::None,
        },
        range: (TILE_SIZE * 1.5),
        arm_speed: 1.0,
        armor: 10,
    };

    /// default attributes for boss mob actors
    pub const BOSS_DEFAULT: Self = Self {
        max_hp: 600.0,
        hp_regen: 5.5,
        max_mana: 500.0,
        mana_regen: 20.0,
        base_speed: 140.0,
        strength: 10,
        agility: 10,
        intelligence: 10,
        damage: Damage {
            physical: PhysicalDamage(5.0),
            elemental: ElementalEffect::None,
        },
        range: (TILE_SIZE * 1.5),
        arm_speed: 1.0,
        armor: 10,
    };

    /// multiplies all attributes by passed value
    fn scale(scale: i32) -> Self {
        let scale_integer = scale;
        let scale_float = scale as f32;
        Self {
            max_hp: (100 * scale_integer) as f32,
            hp_regen: 5.0 * scale_float,
            max_mana: (200 * scale_integer) as f32,
            mana_regen: 10.0 * scale_float,
            base_speed: 100.0 * scale_float,
            strength: 10 * scale_integer,
            agility: 10 * scale_integer,
            intelligence: 10 * scale_integer,
            damage: Damage {
                physical: PhysicalDamage(5.0 * scale_float),
                elemental: ElementalEffect::None,
            },
            range: (TILE_SIZE * (1.5 * scale_float)),
            arm_speed: 1.0 * scale_float,
            armor: 10 * scale_integer,
        }
    }

    /// checks if all values in `self` are == 07
    fn is_all_zero(&self) -> bool {
        self.max_hp == 0.0
            && self.hp_regen == 0.0
            && self.max_mana == 0.0
            && self.mana_regen == 0.0
            && self.base_speed == 0.0
            && self.strength == 0
            && self.agility == 0
            && self.intelligence == 0
            // TODO: how should damage be handled?
            && self.damage == Damage { physical: PhysicalDamage(0.0), elemental: ElementalEffect::None }
            && self.range == 0.0
            && self.arm_speed == 0.0
            && self.armor == 0
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
            base_speed: self.base_speed + rhs.base_speed,
            strength: self.strength + rhs.strength,
            agility: self.agility + rhs.agility,
            intelligence: self.intelligence + rhs.intelligence,
            // TODO: how should damage be handled?
            damage: self.damage,
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
            base_speed: self.base_speed * rhs.base_speed,
            strength: self.strength * rhs.strength,
            agility: self.agility * rhs.agility,
            intelligence: self.intelligence * rhs.intelligence,
            // TODO: how should damage be handled?
            damage: self.damage,
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
            base_speed: acc.base_speed + f.base_speed,
            strength: acc.strength + f.strength,
            agility: acc.agility + f.agility,
            intelligence: acc.intelligence + f.intelligence,
            // TODO: how should damage be handled?
            damage: acc.damage,
            range: acc.range + f.range,
            arm_speed: acc.arm_speed + f.arm_speed,
            armor: acc.armor + f.armor,
        })
    }
}
