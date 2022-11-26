use bevy::prelude::*;

use crate::{
    action_manager::bindings::PlayerInput,
    actors::player::{
        movement::{camera_movement_system, player_movement_system, player_sprint},
        utilities::spawn_player,
    },
    components::actors::{
        animation::{AnimState, AnimationSheet},
        bundles::RigidBodyBundle,
        general::{ActorState, CombatStats, DefenseStats, Player},
    },
    game::GameStage,
    utilities::game::SystemLabels,
};

pub mod attack;
mod movement;
pub mod utilities;

#[derive(Bundle)]
pub struct PlayerBundle {
    name: Name,
    pub player: Player,
    pub player_state: ActorState,
    pub player_animationstate: AnimState,
    pub available_animations: AnimationSheet,
    pub combat_stats: CombatStats,
    pub defense_stats: DefenseStats,
    #[bundle]
    rigidbody: RigidBodyBundle,
    #[bundle]
    pub player_sprite_sheet: SpriteSheetBundle,
    // This bundle must be added to your player entity
    // (or whatever else you wish to control)
    #[bundle]
    pub player_input_map: PlayerInput,
}

pub struct PlayerPlugin;
/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameStage::Playing)
                .with_system(spawn_player.label(SystemLabels::Spawn)),
        )
        .add_system_set(
            SystemSet::on_update(GameStage::Playing)
                .with_system(player_movement_system)
                .with_system(camera_movement_system)
                .with_system(player_sprint),
        );
    }
}
