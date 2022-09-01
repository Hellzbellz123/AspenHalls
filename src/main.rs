// disable console on windows for release builds
//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy::render::texture::ImageSettings;
use bevy::DefaultPlugins;

use dev_tools::DebugPlugin;
use game::TimeInfo;
use loading::LoadingPlugin;
use splashscreen::SplashPlugin;

pub mod action_manager;
pub mod audio;
pub mod characters;
mod dev_tools;
pub mod game;
mod game_world;
pub mod loading;
pub mod splashscreen;
pub mod ui;
pub mod utilities;

pub const TILE_SIZE: Vec2 = Vec2 { x: 32.0, y: 32.0 };

pub fn main() {
    utilities::debugdir();
    App::new()
        // .insert_resource(Msaa { samples: 0 })
        .insert_resource(ClearColor(Color::BLACK)) //rgb(100.0, 100.0, 100.0)
        .insert_resource(ImageSettings::default_nearest())
        .insert_resource(WindowDescriptor {
            width: 1920.,
            height: 1080.,
            title: "Project Kira".to_string(), // ToDo
            ..Default::default()
        })
        .add_startup_system(utilities::set_window_icon)
        .add_plugins(DefaultPlugins)
        .add_state(game::GameStage::Loading)
        .add_plugin(LoadingPlugin)
        .insert_resource(TimeInfo {
            time_step: 0.0,
            game_paused: true,
            pause_menu: false,
        })
        .add_plugin(SplashPlugin)
        .add_plugin(game::GamePlugin)
        .add_plugin(DebugPlugin)
        .run();
}
