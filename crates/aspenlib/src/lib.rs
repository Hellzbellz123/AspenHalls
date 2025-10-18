#![feature(trivial_bounds)]
#![doc = r"
AspenHalls, My video game.
A Dungeon Crawler in the vibes of 'Into The Gungeon' or 'Soul-knight'
"]

#![allow(
    unused,
    elided_lifetimes_in_paths,
    elided_lifetimes_in_associated_constant,
    mismatched_lifetime_syntaxes,
)]

/// general component store
mod bundles;
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

use crate::loading::assets::AspenInitHandles;
use avian2d::prelude::Gravity;
use bevy::prelude::*;

pub use asha_macros::{on_enter, register_types};
pub use bevy::color::palettes::css as colors;
pub use loading::config::*;

/// application stages
#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default, Reflect)]
pub enum AppStage {
    /// load required client resources and abort if we cant load them
    #[default]
    Loading, // --> BootingApp
    /// start client and display window
    Starting, // --> LoadingApp
    /// succesfully started client and running update loop
    Running, // --> add gamestate here
    /// Failed too load required assets
    Failed, // --> FailedLoadInit / FailedLoadMenu
}

/// what part of the game we are at
#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default, Reflect)]
#[source(AppStage = AppStage::Running)]
pub enum GameStage {
    #[default]
    /// showing start menu for game
    StartMenu,
    /// choose character
    SelectCharacter,
    /// game systems running
    PlayingGame,
    /// game systems paused
    PausedGame,
}

/// run condition that checks if controllable player should exist
pub fn playing_game() -> impl FnMut(Option<Res<State<GameStage>>>) -> bool + Clone {
    move |current_state: Option<Res<State<GameStage>>>| {
        current_state.map_or(false, |current_state| {
            *current_state == GameStage::PlayingGame
        })
    }
}

// TODO:
// NOTE FIRST PART DONE
// Convert items and weapon definitions too ron assets in packs/$PACK/definitions and gamedata/custom (for custom user content) from the game folder.
// add a system that takes these definitions and then adds them too the game, items that should ONLY be spawned OR placed in game
// world WILL NOT have a [LOOT] component/tag listed in the definitions, Items that should be obtainable in a play through should
// have the [Loot] component/tag and should be added too a "leveled list" (skyrim) like system

// TODO: use standard system ordering across lib.
// document all cases for why non standard
// run system based on conditions
// systems should have querys with "unreachable" panics
// handle system trigger in run condition and do 'some thing' inside system
// panic should error "'some assumtion' failed"

/// main app fn, configures app loop with logging, then
/// then loads settings from config.toml and adds
/// general game plugins
pub fn start_app(cfg_file: ConfigFile) -> App {
    println!("Hello World!!");
    let mut vanillacoffee = loading::config::create_configured_app(cfg_file);

    // add third party plugins
    vanillacoffee.add_plugins((
        bevy_ecs_ldtk::LdtkPlugin,
        bevy_framepace::FramepacePlugin,
        avian2d::PhysicsPlugins::default().with_length_unit(32.0),
    ));

    vanillacoffee.insert_resource(Gravity(Vec2::ZERO));

    vanillacoffee.add_systems(
        Update,
        (utilities::set_window_icon
            .run_if(resource_exists::<AspenInitHandles>.and(run_once)),),
    );

    vanillacoffee.add_systems(OnEnter(AppStage::Starting), start_app_functionality);

    vanillacoffee.add_plugins((loading::AppLoadingPlugin, game::AspenHallsPlugin));

    vanillacoffee.add_plugins(dev_tools::AspenDevToolsPlugin);

    vanillacoffee
}

// TODO: track pack loading and only continue when base game packs are initialized
/// start app when all resources have finished loading
fn start_app_functionality(mut cmds: Commands) {
    cmds.insert_resource(NextState::Pending(AppStage::Running));
}
