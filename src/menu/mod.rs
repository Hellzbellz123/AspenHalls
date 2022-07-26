use bevy::prelude::*;
use kayak_ui::bevy::BevyKayakUIPlugin;

use crate::menu::main_menu::{destroy, play_button_event, startup};
use crate::{GameStage, PlayButtonEvent};

use self::main_menu::OptionsButtonEvent;
use self::pause_menu::listen_for_pause_event;

pub(crate) mod main_menu;
pub(crate) mod menu_widgets;
pub(crate) mod pause_menu;

//builds menus for vanillacoffee, both ingame and main menu
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugin(BevyKayakUIPlugin)
            .add_event::<PlayButtonEvent>()
            .add_event::<OptionsButtonEvent>()
            .add_system(play_button_event)
            .add_system_set(SystemSet::on_enter(GameStage::Menu).with_system(startup))
            .add_system_set(SystemSet::on_exit(GameStage::Menu).with_system(destroy))
            .add_system_set(
                SystemSet::on_update(GameStage::Playing).with_system(listen_for_pause_event),
            );
    }
}
