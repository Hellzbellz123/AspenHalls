// mod settings;
mod button;
mod menu;

use bevy::prelude::*;
use kayak_ui::{prelude::KayakContextPlugin, widgets::KayakWidgets};

use crate::game::GameStage;

use self::menu::{game_ui, on_game_state_change};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameStage::Menu).with_system(ui_setup))
            .add_plugin(KayakContextPlugin)
            .add_plugin(KayakWidgets)
            .add_system_set(SystemSet::on_enter(GameStage::Menu).with_system(game_ui))
            .add_system(on_game_state_change);
    }
}

fn ui_setup() {
    info!("setting up UI");
}
