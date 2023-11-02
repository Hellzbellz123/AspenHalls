use bevy::prelude::{App, Plugin, SystemSet};

/// all functionality for artificial intelligence on actors is stored here
pub mod ai;
/// holds animation functionality for actors plugin
pub mod animation;
/// game combat functionality
pub mod combat;
/// shared actor components
pub mod components;
/// holds enemy functionality
pub mod enemies;
/// holds player information and functions
pub mod player;
/// holds spawner info
pub mod spawners;

/// system set for ordering actor systems
#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub enum ActorSystemsSet {
    /// systems that use velocity are in this set, this set is ordered
    /// AFTER any systems that modify velocity in update
    UseVelocity,
}

/// all Characters in the game, along with spawners for spawn able characters
// TODO: make actors "configurable". load actor types from $PACK/definitions/$ACTORTYPE/ and add them too a database.
// use this database for "available actors" when spawning
pub struct ActorPlugin;

impl Plugin for ActorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            spawners::SpawnerPlugin,
            animation::AnimationPlugin,
            player::PlayerPlugin,
            combat::ActorWeaponPlugin,
            enemies::EnemyPlugin,
            ai::AIPlugin,
        ));
    }
}
