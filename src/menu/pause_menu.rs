use bevy::prelude::{Commands, Res, ResMut, *};
use kayak_ui::bevy::{FontMapping, ImageManager};

use leafwing_input_manager::prelude::ActionState;

use crate::action_manager::actions::GameActions;
use crate::loading::{FontAssets, UiTextureAssets};
use crate::player::Player;

pub fn listen_for_pause_event(
    input_query: Query<&ActionState<GameActions>, With<Player>>,
    commands: Commands,
    font_assets: Res<FontAssets>,
    ui_assets: Res<UiTextureAssets>,
    image_manager: ResMut<ImageManager>,
    font_mapping: ResMut<FontMapping>,
) {
    let action_state = input_query.single();

    if action_state.just_pressed(GameActions::Pause) {
        info!("pause button pressed");
        spawn_menu(
            commands,
            font_assets,
            ui_assets,
            image_manager,
            font_mapping,
        )
    }
}

fn spawn_menu(
    commands: Commands,
    font_assets: Res<FontAssets>,
    ui_assets: Res<UiTextureAssets>,
    image_manager: ResMut<ImageManager>,
    font_mapping: ResMut<FontMapping>,
) {
    info!("Spawning pause menu and pausing game")
}
