use bevy::prelude::*;
use big_brain::prelude::{ActionBuilder, ScorerBuilder};

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
#[derive(Component, Default, Clone, Debug, Reflect)]
pub struct AICombatConfig {
        /// when ai will consider chasing
        pub chase_start: i32,
        /// max distance from spawn ai will chase
        pub chase_end: i32,
        /// shoot distance
        pub shoot_range: i32,
        /// if enemy is inside this ai's personal space, move backward
        pub personal_space: i32,
        /// scared health
        pub runaway_hp: f32,
}


/// enemies that can chase
#[derive(Component, Default, Clone, Debug, Reflect)]
pub struct AIChaseConfig {
    /// when ai will consider chasing
    pub aggro_distance: i32,
    /// max distance from spawn ai will chase
    pub max_chase_distance: i32,
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
#[derive(Component, Default, Clone, Debug, Reflect)]
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

/// marks actor as enemy
#[derive(Component, Default, Clone, Debug, Reflect)]
pub struct Enemy {
    // TODO: make this updated for all enemies globally with a raycast and use it
    /// does raycast too player hit any objects other than the player?
    pub can_see_player: bool,
}

/// faction enemy belongs too
#[derive(Debug, Component, Reflect, Clone, Copy, PartialEq, Eq, serde::Deserialize)]
pub enum ActorType {
    /// actor is an npc
    ///
    /// faction decides who it attacks
    Npc(Faction),
    /// is an item, can be equipped
    Item,
}
// (pub Faction);

/// type of actor
#[derive(Debug, Reflect, Copy, Clone, PartialEq, Eq, serde::Deserialize)]
pub enum Faction {
    /// passive too enemy and neutral
    Enemy,
    /// fear enemy and player
    Neutral,
    /// fear enemy, passive too player
    Friendly,
    /// enemy will attack
    Player,
    /// weapons and items are both actors, but can be equipped or used and are parented too holding enemy
    Item,
}

// #[derive(Component, Clone, Reflect)]
// pub enum AIEnemy {
//     Skeleton,
//     Slime,
// }
