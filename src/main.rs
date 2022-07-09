// disable console on windows for release builds
//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use vanillacoffee::*;

use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy_inspector_egui::WorldInspectorPlugin;


pub mod window_icon;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(ClearColor(Color::BLACK)) //rgb(100.0, 100.0, 100.0)
        .insert_resource(WindowDescriptor {
            width: 1200.,
            height: 800.,
            title: "Project Kira".to_string(), // ToDo
            ..Default::default()
        })
        // .insert_resource(bevy::log::LogSettings {
        //     level: bevy::log::Level::DEBUG,
        //     filter: "naga=off".to_string(),
        //     // filter: "off".to_string()
        //     ..Default::default()
        // })
        .add_startup_system(window_icon::set_window_icon)
        .add_plugin(WorldInspectorPlugin::new())
        .add_state(GameState::Loading)

        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .run();
}
