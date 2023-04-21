// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(stmt_expr_attributes)]
#![feature(type_ascription)]
#![feature(lint_reasons)]
#![feature(trivial_bounds)]
// #![forbid(missing_docs)]
#![allow(clippy::module_name_repetitions)]

use bevy::prelude::{default, warn, Vec2};

use bevy_prototype_lyon::prelude::ShapePlugin;
use bevy_rapier2d::prelude::{NoUserData, RapierConfiguration, RapierPhysicsPlugin};

use crate::app_config::configure_and_build;
use crate::dev_tools::debug_plugin::debug_dump_graphs;

#[cfg(feature = "dev")]
use crate::dev_tools::debug_plugin::DebugPlugin;

/// module holds actors (entitys that have spatialbundles and that can affect gameplay)
pub mod actors;
/// module for all game audio, internal audio plugin handles all sound
pub mod audio;
pub mod components;
pub mod console;
mod dev_tools;
pub mod game;
pub mod game_world;
/// module holds buttons that can be pressed
pub mod input;
pub mod loading;
// pub mod kayak_ui;
mod app_config;
pub mod consts;
pub mod ui_bevy;
pub mod utilities;

// TODO: Convert items and weapon definitions too ron assets in gamedata/definitions and gamedata/custom (for custom user content) from the game folder.
// add a system that takes these definitions and then adds them too the game, items that should ONLY be spawned OR placed in game
// world WILL NOT have a [LOOT] component/tag listed in the definitions, Items that should be obtainable in a playthrough should
// have the [Loot] component/tag and should be added too a "leveled list" (skyrim) like system

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

    // add vanillacoffee stuff
    vanillacoffee
        .add_state::<game::GameStage>()
        .add_plugin(loading::AssetLoadPlugin)
        .add_plugin(console::QuakeConPlugin)
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
