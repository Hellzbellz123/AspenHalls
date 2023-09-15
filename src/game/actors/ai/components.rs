use bevy::prelude::*;
use big_brain::prelude::{ActionBuilder, ScorerBuilder};

/// enemies chase scorer
#[derive(Component, Default, Clone, Debug, Reflect, ScorerBuilder)]
pub struct AggroScore;

/// enemies shoot scorer
#[derive(Component, Default, Clone, Debug, Reflect, ScorerBuilder)]
pub struct AttackScore;

/// enemies wander scorer
#[derive(Component, Default, Clone, Debug, Reflect, ScorerBuilder)]
pub struct WanderScore;

/// enemies that can chase
#[derive(Component, Default, Clone, Debug, Reflect)]
pub struct AICanAggro {
    /// how far they can chase from
    pub aggro_distance: f32,
}

/// enemies that can wander
#[derive(Component, Default, Clone, Debug, Reflect)]
pub struct AICanWander {
    /// wander too here
    pub wander_target: Option<Vec2>,
    /// stay close too here
    pub spawn_position: Option<Vec2>,
}

/// enemies that can shoot
#[derive(Component, Default, Clone, Debug, Reflect)]
pub struct AICanShoot {
    /// how far can ai shoot
    pub shoot_range: f32,
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

/// enemy attack state, how often and whether can shoot
#[derive(Component, Default, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct AIAttackState {
    /// timer for shooting
    pub timer: Timer,
    /// wether should shoot
    pub should_shoot: bool,
    /// is player close enough too shoot
    pub is_near: bool,
}

/// marks actor as enemy
#[derive(Component, Default, Clone, Debug, Reflect)]
pub struct Enemy;

/// faction enemy belongs too
#[derive(Component, Deref, DerefMut, Reflect)]
pub struct ActorType(pub Faction);

/// who enemy is friends with
#[derive(Reflect)]
pub enum Faction {
    /// passive too enemy and neutral
    Enemy,
    /// fear enemy and player
    Neutral,
    /// fear enemy, passive too player
    Friendly,
    /// enemy will attack
    Player,
}

// #[derive(Component, Clone, Reflect)]
// pub enum AIEnemy {
//     Skeleton,
//     Slime,
// }
