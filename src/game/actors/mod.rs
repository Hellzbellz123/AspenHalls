use bevy::prelude::{App, Plugin};

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

/// all Characters in the game, along with spawners for spawn able characters
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
