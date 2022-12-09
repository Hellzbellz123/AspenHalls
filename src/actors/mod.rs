use bevy::prelude::{App, Plugin};
use big_brain::BigBrainPlugin;

pub mod ai;
/// holds animation plugin
pub mod animation;
/// holds enemies
pub mod enemies;
/// holds player information and functions
pub mod player;
/// holds spawner info
pub mod spawners;
pub mod weapons;

/// all npcs in the game, along with player and spawners
pub struct ActorPlugin;

impl Plugin for ActorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BigBrainPlugin)
            .add_plugin(spawners::SpawnerPlugin)
            .add_plugin(animation::AnimationPlugin)
            .add_plugin(player::PlayerPlugin)
            .add_plugin(weapons::WeaponPlugin)
            .add_plugin(enemies::EnemyPlugin)
            .add_plugin(ai::AIPlugin);
    }
}
