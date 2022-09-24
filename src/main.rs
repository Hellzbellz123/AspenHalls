// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy::render::texture::ImageSettings;
use bevy::DefaultPlugins;

// use dev_tools::{debug_dirs, DebugPlugin};
use game::TimeInfo;
use heron::PhysicsLayer;
use loading::AssetLoadPlugin;
use splashscreen::SplashPlugin;

pub mod action_manager;
pub mod audio;
pub mod actors;
pub mod game;
pub mod game_world;
pub mod loading;
pub mod splashscreen;
pub mod ui;
pub mod utilities;

// #[cfg(feature="dev")]
mod dev_tools;

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
        .add_startup_system(utilities::set_window_icon)
        .add_plugins(DefaultPlugins)
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
        .add_startup_system(utilities::set_window_icon)
        .add_plugins(DefaultPlugins)
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

// struct GamePlugin;
// impl Plugin for GamePlugin {
//     fn build(&self, app: &mut App) {
//         app.add_plugin(MenuPlugin)
//             .add_plugin(ActionsPlugin)
//             .add_plugin(InternalAudioPlugin)
//             .add_plugin(MapSystem)
//             .add_plugin(PlayerPlugin)
//             .add_plugin(GraphicsPlugin)
//             .add_system_set(SystemSet::on_enter(GameStage::Playing).with_system(setup_time_state));
//     }
// }

// pub fn setup_time_state(mut timeinfo: ResMut<TimeInfo>) {
//     *timeinfo = TimeInfo {
//         time_step: 1.0,
//         game_paused: false,
//         pause_menu: false,
//     }
// }
