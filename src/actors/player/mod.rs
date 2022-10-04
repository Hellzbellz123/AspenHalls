use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::{
    action_manager::bindings::PlayerInput,
    actors::{
        player::{
            movement::{camera_movement_system, player_movement_system, player_sprint},
            utilities::spawn_player,
        },
        RigidBodyBundle,
    },
    game::GameStage,
};

use crate::actors::animation::{AnimState, FacingDirection};

use super::ActorState;

mod movement;
mod utilities;

#[derive(Bundle)]
pub struct PlayerBundle {
    name: Name,
    pub player_state: ActorState,
    pub player_animationstate: AnimState,
    #[bundle]
    rigidbody: RigidBodyBundle,
    // This bundle must be added to your player entity
    // (or whatever else you wish to control)
    #[bundle]
    pub player_input_map: PlayerInput,
    #[bundle]
    pub player_sprite_sheet: SpriteSheetBundle,
}

pub struct PlayerPlugin;
/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameStage::Playing).with_system(spawn_player))
            .add_system_set(
                SystemSet::on_update(GameStage::Playing)
                    .with_system(player_movement_system)
                    .with_system(camera_movement_system)
                    .with_system(player_sprint),
            );
    }
}
