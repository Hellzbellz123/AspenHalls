// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(stmt_expr_attributes)]
#![feature(type_ascription)]
#![feature(lint_reasons)]
// #![forbid(missing_docs)]
#![allow(clippy::module_name_repetitions)]

use bevy::prelude::{default, Vec2, warn};

use bevy_prototype_lyon::prelude::ShapePlugin;
use bevy_rapier2d::prelude::{NoUserData, RapierConfiguration, RapierPhysicsPlugin};

// use crate::console::VCConsolePlugin;
use crate::app_config::configure_and_build;
use crate::dev_tools::debug_plugin::debug_dump_graphs;

#[cfg(feature = "dev")]
use crate::dev_tools::debug_plugin::DebugPlugin;

/// module holds actors (entitys that have spatialbundles and that can affect gameplay)
pub mod actors;
/// module for all game audio, internal audio plugin handles all sound
pub mod audio;
pub mod components;
/// module holds buttons that can be pressed
pub mod input;
// pub mod console;
mod dev_tools;
pub mod game;
pub mod game_world;
pub mod loading;
// pub mod kayak_ui;
mod app_config;
pub mod consts;
pub mod ui_bevy;
pub mod utilities;

fn main() {
    let mut vanillacoffee = configure_and_build();

    // add third party plugins
    vanillacoffee
        .add_plugin(bevy_framepace::FramepacePlugin)
        .add_plugin(ShapePlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(16.0))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        });

    // add vanillacoffee plugins
    vanillacoffee
        .add_plugin(game::GamePlugin)
        .add_plugin(utilities::UtilitiesPlugin);

    #[cfg(feature = "dev")]
    vanillacoffee.add_plugin(DebugPlugin);

    #[cfg(feature = "dev")]
    debug_dump_graphs(&mut vanillacoffee);
    #[cfg(feature = "dev")]
    warn!("Dumping graphs");

    vanillacoffee.run();
}
