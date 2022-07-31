use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::{
    action_manager::bindings::PlayerInput,
    characters::player::{player_movement::*, player_utils::spawn_player},
    game::GameStage,
};

mod player_movement;
mod player_utils;

#[derive(Component, Inspectable, Reflect)]
pub struct PlayerComponent {
    //stores important player data
    pub speed: f32,
    pub sprint_available: bool,
}

// trunk-ignore(clippy/forget_non_drop)
#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: PlayerComponent,
    // This bundle must be added to your player entity
    // (or whatever else you wish to control)
    #[bundle]
    pub pinput_map: PlayerInput,
    #[bundle]
    pub psprite: SpriteBundle,
}

pub struct PlayerPlugin;
/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameStage::Playing)
                .with_system(spawn_player)
                .with_system(spawn_camera),
        )
        .add_system_set(
            SystemSet::on_update(GameStage::Playing)
                .with_system(player_movement_system)
                .with_system(player_sprint),
        );
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
