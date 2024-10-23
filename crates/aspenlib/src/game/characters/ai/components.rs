use bevy::prelude::*;
use big_brain::prelude::{ActionBuilder, ScorerBuilder};

/// enemies chase scorer
#[derive(Debug, Clone, Default, Reflect, Component, ScorerBuilder)]
#[reflect(Component)]
pub struct ChaseScorer;

/// enemies shoot scorer
#[derive(Debug, Clone, Default, Reflect, Component, ScorerBuilder)]
#[reflect(Component)]
pub struct AttackScorer;

/// enemies wander scorer
#[derive(Debug, Clone, Default, Reflect, Component, ScorerBuilder)]
#[reflect(Component)]
pub struct WanderScore;

/// actor combat ai cfg
#[derive(Debug, Clone, Default, Reflect, Component, serde::Deserialize)]
#[reflect(Component)]
pub struct AICombatAggroConfig {
    /// when ai will consider chasing
    pub chase_start: i32,
    /// max distance from spawn ai will chase
    pub chase_end: i32,
    /// shoot distance
    pub shoot_range: i32,
    /// if enemy is inside this characters `personal_space`, move backward
    pub personal_space: i32,
    /// scared health
    pub runaway_hp: f32,
}

/// enemies with this will shoot
/// holds attack state. how often?, can shoot??, should shoot? target range?
#[derive(Component, Default, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct AIAutoShootConfig {
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
#[reflect(Component)]
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
#[reflect(Component)]
pub struct AIShootAction;

/// enemies with this tag are chasing a target
#[derive(Component, Default, Clone, Debug, Reflect, ActionBuilder)]
#[reflect(Component)]
pub struct AIChaseAction;

/// enemies with this tag are wandering
#[derive(Component, Default, Clone, Debug, Reflect, ActionBuilder)]
#[reflect(Component)]
pub struct AIWanderAction;

// TODO: move ai config too this enum, each ai type gets a scorer that determines its actions using config data held inside AiType, AiType is defined inside character_definition

/// type of ai this ai wanting character should get
#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Eq,
    serde::Deserialize,
    serde::Serialize,
    Reflect,
    Component,
)]
#[reflect(Component)]
pub enum AiType {
    /// not very smart ai
    #[default]
    Stupid,
    /// boss ai
    Boss,
    /// critter ai
    Critter,
    /// player pet
    PlayerPet,
    /// hero player can hire
    FollowerHero,
}
