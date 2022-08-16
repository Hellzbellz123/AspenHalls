// disable console on windows for release builds
//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy::DefaultPlugins;

pub mod action_manager;
pub mod audio;
pub mod characters;
pub mod game;
pub mod loading;
pub mod splashscreen;
// pub mod ui;
mod dev_tools;
pub mod utilities;

pub fn main() {
    #[cfg(target_os = "windows")]
    utilities::debugdirwindows();
    App::new()
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(ClearColor(Color::BLACK)) //rgb(100.0, 100.0, 100.0)
        .insert_resource(WindowDescriptor {
            width: 1200.,
            height: 800.,
            title: "Project Kira".to_string(), // ToDo
            ..Default::default()
        })
        .add_startup_system(utilities::set_window_icon)
        .add_plugins(DefaultPlugins)
        .add_state(game::GameStage::Loading)
        .add_plugin(game::GamePlugin)
        .run();
}
