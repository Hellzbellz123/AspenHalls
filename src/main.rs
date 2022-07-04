// disable console on windows for release builds
//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::{App, ClearColor, Color, Msaa, WindowDescriptor};
use bevy::DefaultPlugins;
use bevy_inspector_egui::WorldInspectorPlugin;
use vanillacoffee::GamePlugin;

pub mod window_icon;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .insert_resource(WindowDescriptor {
            width: 1200.,
            height: 800.,
            title: "Project Kira".to_string(), // ToDo
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(window_icon::set_window_icon)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(GamePlugin)
        .run();
}
