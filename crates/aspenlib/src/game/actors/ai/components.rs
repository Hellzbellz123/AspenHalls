use bevy::prelude::*;
use big_brain::prelude::{ActionBuilder, ScorerBuilder};

/// enemies chase scorer
#[derive(Component, Default, Clone, Debug, Reflect, ScorerBuilder)]
pub struct ChaseScore;

/// enemies shoot scorer
#[derive(Component, Default, Clone, Debug, Reflect, ScorerBuilder)]
pub struct AttackScore;

/// enemies wander scorer
#[derive(Component, Default, Clone, Debug, Reflect, ScorerBuilder)]
pub struct WanderScore;

/// enemies that can chase
#[derive(Component, Default, Clone, Debug, Reflect)]
pub struct AIChaseConfig {
    /// how far they will chase from
    pub aggro_distance: f32,
    /// how far they will chase from spawn
    pub chase_distance: f32,
}

/// enemies that can wander
#[derive(Component, Default, Clone, Debug, Reflect)]
pub struct AIWanderConfig {
    /// how far can we wander from spawn
    pub wander_distance: f32,
    /// wander too here
    pub wander_target: Option<Vec2>,
    /// stay close too here
    pub spawn_position: Option<Vec2>,
}

/// enemies with this will shoot
/// holds attack state. how often?, can shoot??, should shoot? target range?
#[derive(Component, Default, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct AIShootConfig {
    /// ai shoot range
    pub find_target_range: f32,
    /// timer for shooting
    pub timer: Timer,
    /// wether should shoot
    pub should_shoot: bool,
    /// is player close enough too shoot
    pub can_shoot: bool,
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
#[derive(Debug, Component, Deref, DerefMut, Reflect, Clone, Copy, PartialEq, Eq)]
pub struct ActorType(pub Type);

/// type of actor
#[derive(Debug, Reflect, Copy, Clone, PartialEq, Eq)]
pub enum Type {
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
