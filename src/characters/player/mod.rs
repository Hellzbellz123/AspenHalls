use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::{
    action_manager::bindings::PlayerInput,
    characters::player::{
        player_animation::FacingDirection,
        player_movement::*,
        player_utils::spawn_player, //animate_sprite,
    },
    game::GameStage,
};

use self::player_animation::FrameAnimation;

pub mod player_animation;
mod player_movement;
mod player_utils;

#[derive(Component, Default, Reflect, Inspectable)]
#[reflect(Component)]
pub struct PlayerComponent {
    //stores important player data
    pub speed: f32,
    pub sprint_available: bool,
    pub facing: FacingDirection,
    pub just_moved: bool,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    name: Name,
    pub player_data: PlayerComponent,
    // This bundle must be added to your player entity
    // (or whatever else you wish to control)
    pub player_animations: FrameAnimation,
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
                    // .with_system(animate_sprite)
                    .with_system(player_movement_system)
                    .with_system(player_sprint),
            );
    }
}
