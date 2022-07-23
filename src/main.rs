// disable console on windows for release builds
//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy_inspector_egui::Inspectable;

use vanillacoffee::*;

use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy_inspector_egui::WorldInspectorPlugin;

pub mod utilities;

#[derive(Inspectable, Component)]
struct InspectableType;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
struct ReflectedType;

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
        .add_startup_system(utilities::set_window_icon)
        .add_state(GameState::Loading)
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .register_type::<ReflectedType>()
        .run();
}
