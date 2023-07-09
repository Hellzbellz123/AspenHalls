use bevy::prelude::{App, Plugin};
use big_brain::BigBrainPlugin;

/// all functionality for artifical intellegince on actors is stored here
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

/// all npcs in the game, along with player and spawners for said npcs
pub struct ActorPlugin;

impl Plugin for ActorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BigBrainPlugin)
            .add_plugin(spawners::SpawnerPlugin)
            .add_plugin(animation::AnimationPlugin)
            .add_plugin(player::PlayerPlugin)
            .add_plugin(combat::ActorWeaponPlugin)
            .add_plugin(enemies::EnemyPlugin)
            .add_plugin(ai::AIPlugin);
    }
}
