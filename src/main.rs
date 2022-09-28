// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(lint_reasons)]

use bevy::prelude::*;
use bevy::render::texture::ImageSettings;
use bevy::DefaultPlugins;

// use dev_tools::{debug_dirs, DebugPlugin};
use game::TimeInfo;
use heron::PhysicsLayer;
use loading::AssetLoadPlugin;
use splashscreen::SplashPlugin;
use utilities::UtilitiesPlugin;

pub mod action_manager;
pub mod actors;
pub mod audio;
pub mod game;
pub mod game_world;
pub mod loading;
pub mod splashscreen;
pub mod ui;
pub mod utilities;

// #[cfg(feature="dev")]
mod dev_tools;

// pub struct AppSettings {
//     sound_settings: SoundSettings,
//     control_settings: PlayerInput,
// }

pub const TILE_SIZE: Vec2 = Vec2 { x: 32.0, y: 32.0 };
pub const PLAYER_SIZE: Vec2 = Vec2::new(TILE_SIZE.x, TILE_SIZE.y * 2.0);

#[derive(PhysicsLayer)]
enum Layer {
    World,
    Player,
    // Enemies,
}

#[cfg(not(feature = "dev"))]
pub fn main() {
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
        .add_plugins(DefaultPlugins)
        .add_plugin(UtilitiesPlugin)
        .add_state(game::GameStage::Loading)
        .add_plugin(AssetLoadPlugin)
        .insert_resource(TimeInfo {
            time_step: 0.0,
            game_paused: true,
            pause_menu: false,
        })
        .add_plugin(SplashPlugin)
        .add_plugin(game::GamePlugin)
        .run();
}

#[cfg(feature = "dev")]
pub fn main() {
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
        .add_plugins(DefaultPlugins)
        .add_plugin(UtilitiesPlugin)
        .add_state(game::GameStage::Loading)
        .add_plugin(AssetLoadPlugin)
        .insert_resource(TimeInfo {
            time_step: 0.0,
            game_paused: true,
            pause_menu: false,
        })
        .add_plugin(SplashPlugin)
        .add_plugin(game::GamePlugin)
        .add_plugin(dev_tools::DebugPlugin)
        .run();
}
