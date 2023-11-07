// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(stmt_expr_attributes)]
#![feature(type_ascription)]
#![feature(lint_reasons)]
#![feature(trivial_bounds)]
#![feature(exact_size_is_empty)]
#![feature(fs_try_exists)]
#![doc = r"
Vanilla Coffee, My video game.
it kinda sucks but it'll be finished eventually
A Dungeon Crawler in the vibes of 'Into The Gungeon' or 'Soul-knight'
"]
#![allow(clippy::module_name_repetitions)]
#![warn(
    clippy::missing_docs_in_private_items,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::missing_safety_doc
)]

/// general component store
mod bundles;
/// things related too `command_console`
mod console;
/// general consts file, if it gets used more than
/// twice it should be here
mod consts;
#[cfg(feature = "inspect")]
/// Debug and Development related functions
mod dev_tools;
/// actual game plugin, ui and all "game" functionality
mod game;
/// Holds all Asset Collections and handles loading them
/// also holds fail state
mod loading;
/// misc util functions that cant find a place
mod utilities;

/// TODO: wip
///
/// A.H.P. Aspen Halls Prelude, in the future this can be the only import for mods, no need too manually specify bevy, or other dependency versions
///
/// common imports for all modules, maybe make it specific, ie no wildcards.
///  all modules that aren't plugin should probably be defined here
pub mod ahp;

use ahp::{
    engine::{
        bevy_rapier2d, default, resource_exists, run_once, App, Condition, IntoSystemConfigs,
        Reflect, Resource, States, Update, Vec2,
    },
    game::{ConfigFile, InitAssetHandles},
};

#[cfg(feature = "inspect")]
use ahp::game::inspect::*;

/// main game state loop
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, States, Resource, Reflect)]
pub enum AppStage {
    /// pre loading state before window is shown.
    /// Loads REQUIRED resources
    #[default]
    BootingApp,
    /// assets from game pack loaded during this state
    /// pack configuration from config file is setup here
    Loading,
    /// Main menu is drawn
    /// wait for load-saved-game or new-saved-game
    StartMenu,
    /// playing game, some States are inserted here
    PlayingGame, //(PlaySubStage),
    /// Game Paused in this state, rapier timestep set too 0.0, no physics, ai is also stopped
    PauseMenu,
    /// game failed to load an asset
    FailedLoading,
}

// TODO: Convert items and weapon definitions too ron assets in packs/$PACK/definitions and gamedata/custom (for custom user content) from the game folder.
// add a system that takes these definitions and then adds them too the game, items that should ONLY be spawned OR placed in game
// world WILL NOT have a [LOOT] component/tag listed in the definitions, Items that should be obtainable in a play through should
// have the [Loot] component/tag and should be added too a "leveled list" (skyrim) like system

/// main app fn, configures app loop with logging, then
/// then loads settings from config.toml and adds
/// general game plugins
pub fn start_app(cfg_file: ConfigFile) -> App {
    let mut vanillacoffee = loading::config::create_configured_app(cfg_file);

    // add third party plugins
    vanillacoffee
        .add_plugins((
            bevy_ecs_ldtk::LdtkPlugin,
            belly::prelude::BellyPlugin,
            bevy_framepace::FramepacePlugin,
            bevy_prototype_lyon::prelude::ShapePlugin,
            bevy_rapier2d::plugin::RapierPhysicsPlugin::<bevy_rapier2d::prelude::NoUserData>::pixels_per_meter(32.0),
        ))
        .insert_resource(bevy_rapier2d::prelude::RapierConfiguration {
            gravity:  Vec2::ZERO,
            .. default()
        });

    vanillacoffee.add_plugins((
        ahp::plugins::AppAssetsPlugin,
        ahp::plugins::SplashPlugin,
        ahp::plugins::QuakeConPlugin,
        ahp::plugins::DungeonGamePlugin,
    ));

    #[cfg(feature = "inspect")]
    vanillacoffee.add_plugins(DebugPlugin);

    vanillacoffee.add_systems(
        Update,
        IntoSystemConfigs::run_if(
            utilities::set_window_icon,
            Condition::and_then(resource_exists::<InitAssetHandles>(), run_once()),
        ),
    );

    vanillacoffee
}
