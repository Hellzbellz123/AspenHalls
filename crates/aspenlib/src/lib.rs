#![feature(lint_reasons)]
#![feature(let_chains)]
#![doc = r"
AspenHalls, My video game.
A Dungeon Crawler in the vibes of 'Into The Gungeon' or 'Soul-knight'
"]

/// general component store
mod bundles;
/// things related too `command_console`
mod console;
/// general consts file, if it gets used more than
/// twice it should be here
mod consts;

/// Debug and Development related functions
mod debug;
/// actual game plugin, ui and all "game" functionality
mod game;
/// Holds all Asset Collections and handles loading them
/// also holds fail state
mod loading;
/// misc util functions that cant find a place
mod utilities;

use crate::{
    game::{combat::SameUserDataFilter, DungeonFloor},
    loading::assets::AspenInitHandles,
};
use bevy::prelude::*;

pub use loading::config::*;

/// application stages
pub enum ApplicationStage {
    // TODO: impl this  stuff
    /// load client resources
    LoadingClient, // --> BootingApp
    /// start client
    StartingGame, // --> LoadingApp
    /// succesfully started client
    GameRunning, // --> add gamestate here
    /// Failed too load required assets
    ClientFailed, // --> FailedLoadInit / FailedLoadMenu
}

/// what part of the game we are at
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, States, Resource, Reflect)]
pub enum GameStage {
    /// no actor related logic, just the main menu
    #[default]
    NotStarted,
    /// select character, buy weapons
    Prepare,
    /// crawling has 1 value. the dungeon Level
    Crawling(DungeonFloor),
}

/// main game state loop
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, States, Resource, Reflect)]
pub enum AppState {
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
    /// game failed to load an init asset. fatal error
    FailedLoadInit,
    /// game failed too load default pack
    FailedLoadMenu,
}

// TODO:
// NOTE FIRST PART DONE
//Convert items and weapon definitions too ron assets in packs/$PACK/definitions and gamedata/custom (for custom user content) from the game folder.
// add a system that takes these definitions and then adds them too the game, items that should ONLY be spawned OR placed in game
// world WILL NOT have a [LOOT] component/tag listed in the definitions, Items that should be obtainable in a play through should
// have the [Loot] component/tag and should be added too a "leveled list" (skyrim) like system

/// main app fn, configures app loop with logging, then
/// then loads settings from config.toml and adds
/// general game plugins
pub fn start_app(cfg_file: ConfigFile) -> App {
    println!("Hello World!!");
    let mut vanillacoffee = loading::config::create_configured_app(cfg_file);

    // add third party plugins
    vanillacoffee
        .add_plugins((
            bevy_mod_picking::DefaultPickingPlugins,
            bevy_ecs_ldtk::LdtkPlugin,
            bevy_framepace::FramepacePlugin,
            bevy_prototype_lyon::prelude::ShapePlugin,
            bevy_rapier2d::plugin::RapierPhysicsPlugin::<SameUserDataFilter>::pixels_per_meter(
                32.0,
            ),
        ))
        .insert_resource(bevy_rapier2d::prelude::RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        });

    vanillacoffee.add_plugins((
        loading::AppLoadingPlugin,
        console::QuakeConPlugin,
        game::AspenHallsPlugin,
    ));

    #[cfg(feature = "develop")]
    vanillacoffee.add_plugins(debug::debug_plugin::DebugPlugin);

    vanillacoffee.add_systems(
        Update,
        (utilities::set_window_icon
            .run_if(resource_exists::<AspenInitHandles>.and_then(run_once())),),
    );

    vanillacoffee
}
