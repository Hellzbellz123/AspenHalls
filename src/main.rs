// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(stmt_expr_attributes)]
#![feature(type_ascription)]
#![feature(lint_reasons)]
#![feature(trivial_bounds)]
#![feature(exact_size_is_empty)]

#![doc = r"
Vanilla Coffee, My video game.
it kinda sucks but it'll be finished eventually
A Dungeon Crawler in the vibes of Into The Gungeon
"]
// #![doc = include_str!("../README.md")]
#![allow(clippy::module_name_repetitions)]
#![warn(
    clippy::missing_docs_in_private_items,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::missing_safety_doc
)]

use crate::launch_config::app_with_logging;
use bevy::prelude::{default, Vec2};
use bevy_rapier2d::prelude::{NoUserData, RapierConfiguration};

#[cfg(feature = "dev")]
use crate::dev_tools::debug_plugin::DebugPlugin;
#[cfg(feature = "dev")]
use tracing_log::log::warn;

/// holds app settings logic and systems
mod launch_config;
/// general component store
mod bundles;
/// things related too `command_console`
mod console;
/// general consts file, if it gets used more than
/// twice it should be here
mod consts;
/// Debug and Development related functions
mod dev_tools;
/// actual game plugin, ui and all "game" functionality
mod game;
/// Holds all Asset Collections and handles loading them
/// also holds fail state
mod loading;
/// misc util functions that cant find a place
mod utilities;

// TODO: Convert items and weapon definitions too ron assets in gamedata/definitions and gamedata/custom (for custom user content) from the game folder.
// add a system that takes these definitions and then adds them too the game, items that should ONLY be spawned OR placed in game
// world WILL NOT have a [LOOT] component/tag listed in the definitions, Items that should be obtainable in a play through should
// have the [Loot] component/tag and should be added too a "leveled list" (skyrim) like system

/// main app fn, configures app loop with logging, then
/// then loads settings from config.toml and adds
/// general game plugins
fn main() {
    let mut vanillacoffee = app_with_logging();

    // add third party plugins
    vanillacoffee
        .add_plugins((
            bevy_ecs_ldtk::LdtkPlugin,
            belly::prelude::BellyPlugin,
            bevy_framepace::FramepacePlugin,
            bevy_prototype_lyon::prelude::ShapePlugin,
            bevy_rapier2d::plugin::RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(16.0),
        ))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        });

    // add vanillacoffee stuff
    vanillacoffee.add_state::<game::AppStage>().add_plugins((
        loading::AssetLoadPlugin,
        console::QuakeConPlugin,
        game::GamePlugin,
        utilities::UtilitiesPlugin,
    ));

    #[cfg(feature = "dev")]
    vanillacoffee.add_plugins(DebugPlugin);

    vanillacoffee.run();
}
