use bevy::prelude::*;
use kayak_ui::bevy::BevyKayakUIPlugin;

mod menu_widgets;
mod menus;

use crate::{
    game::GameStage,
    ui::menus::{
        main_menu::{
            destroy_menu, exit_system, play_button_event, startup, AppExitEvent, PlayButtonEvent,
        },
        pause_menu::listen_for_pause_event,
    },
};

//builds menus for vanillacoffee, both ingame and main menu
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugin(BevyKayakUIPlugin)
            .add_event::<PlayButtonEvent>()
            .add_event::<AppExitEvent>()
            .add_system(play_button_event)
            .add_system(exit_system)
            .add_system_set(SystemSet::on_enter(GameStage::Menu).with_system(startup))
            .add_system_set(SystemSet::on_exit(GameStage::Menu).with_system(destroy_menu))
            .add_system_set(
                SystemSet::on_update(GameStage::Playing).with_system(listen_for_pause_event),
            );
    }
}