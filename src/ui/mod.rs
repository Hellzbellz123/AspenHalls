use bevy::prelude::*;

use crate::game::GameStage;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameStage::Menu).with_system(ui_setup));
    }
}

fn ui_setup() {
    info!("setting up UI");
}
