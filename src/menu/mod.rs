use bevy::prelude::*;
use kayak_ui::bevy::BevyKayakUIPlugin;

use crate::menu::main_menu::{destroy, play_button_event, startup};
use crate::{GameState, PlayButtonEvent};

use self::pause_menu::listen_for_pause_event;

pub(crate) mod main_menu;
pub(crate) mod menu_widgets;
pub(crate) mod pause_menu;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugin(BevyKayakUIPlugin)
            .add_event::<PlayButtonEvent>()
            .add_system(play_button_event)
            .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(startup))
            .add_system_set(SystemSet::on_exit(GameState::Menu).with_system(destroy))
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(listen_for_pause_event),
            );
    }
}
