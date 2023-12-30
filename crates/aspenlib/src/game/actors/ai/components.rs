use bevy::prelude::*;
use big_brain::prelude::{ActionBuilder, ScorerBuilder};
use clap::ValueEnum;

/// enemies chase scorer
#[derive(Component, Default, Clone, Debug, Reflect, ScorerBuilder)]
pub struct ChaseScorer;

/// enemies shoot scorer
#[derive(Component, Default, Clone, Debug, Reflect, ScorerBuilder)]
pub struct AttackScorer;

/// enemies wander scorer
#[derive(Component, Default, Clone, Debug, Reflect, ScorerBuilder)]
pub struct WanderScore;

/// actor combat ai cfg
#[derive(Component, Default, Clone, Debug, Reflect, serde::Deserialize,)]
pub struct AICombatConfig {
        /// when ai will consider chasing
        pub chase_start: i32,
        /// max distance from spawn ai will chase
        pub chase_end: i32,
        /// shoot distance
        pub shoot_range: i32,
        /// if enemy is inside this characters personal_space, move backward
        pub personal_space: i32,
        /// scared health
        pub runaway_hp: f32,
}

/// enemies with this will shoot
/// holds attack state. how often?, can shoot??, should shoot? target range?
#[derive(Component, Default, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct AIShootConfig {
    /// ai shoot range
    pub find_target_range: i32,
    /// timer for shooting
    pub timer: Timer,
    /// wether should shoot
    pub should_shoot: bool,
    /// is player close enough too shoot
    pub can_shoot: bool,
}

/// enemies that can wander
#[derive(Component, Default, Clone, Debug, Reflect, serde::Deserialize)]
pub struct AIWanderConfig {
    /// how far can we wander from spawn
    pub wander_distance: i32,
    /// wander too here
    pub wander_target: Option<Vec2>,
    /// stay close too here
    pub spawn_position: Option<Vec2>,
}

/// enemies with this tag are shooting a target
#[derive(Component, Default, Clone, Debug, Reflect, ActionBuilder)]
pub struct AIShootAction;

/// enemies with this tag are chasing a target
#[derive(Component, Default, Clone, Debug, Reflect, ActionBuilder)]
pub struct AIChaseAction;

/// enemies with this tag are wandering
#[derive(Component, Default, Clone, Debug, Reflect, ActionBuilder)]
pub struct AIWanderAction;


// TODO: change this too be enum of enums
// yeet Hero, Weapon from ActorType
// add Hero too NpcType
// add ObjectType with Weapon/Armor/Trinket
// rename Npc -> Character and NpcType -> CharacterType / item -> Object
/// actors function in game
#[derive(Debug, Component, Reflect, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum ActorType {
    /// actor is an npc
    /// - NpcType decides freinds/enemies
    Npc(NpcType),
    /// character controlled by human
    Hero,
    // TODO: merge weapon / item
    /// used by npc/player too attack
    Weapon,
    /// is an item, can be equipped
    Item,
    // TODO: add Object or some other name for chests/switches world interactables etc
}

impl ActorType {
    /// checks if actor is a creep, ignores ai level
    pub fn is_creep(&self) -> bool {
        match self {
            ActorType::Npc(a) => match a {
                NpcType::Creep => true,
                NpcType::Boss => false,
                NpcType::Minion => false,
                NpcType::Critter => false,
                NpcType::Friendly => false,
            },
            _ => false,
        }
    }

    /// checks if actor is a hero
    pub fn is_hero(&self) -> bool {
        match self {
            ActorType::Hero => true,
            ActorType::Item => false,
            ActorType::Npc(_) => false,
            ActorType::Weapon => false,
        }
    }
}

/// type of actor
#[derive(Debug, Reflect, Copy, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum NpcType {
    /// - final enemy of dungeon level
    /// - hostile too all npcs
    Boss,
    /// - generic enemy for dungeon levels
    /// - passive too creep
    Creep,
    /// - runs away from creeps
    /// - passive too self and freindly
    Critter,
    /// passive too player
    Friendly,
    /// player pet
    Minion,
}

#[derive(Debug, Reflect, Copy, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum AiType {
    Stupid,
    Boss,
    Critter,
    PlayerPet,
    FollowerHero,
}
