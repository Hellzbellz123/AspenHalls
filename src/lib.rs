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
A Dungeon Crawler in the vibes of 'Into The Gungeon' or 'Soulknight'
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
/// Debug and Development related functions
mod dev_tools;
/// actual game plugin, ui and all "game" functionality
mod game;
/// Holds all Asset Collections and handles loading them
/// also holds fail state
mod loading;
/// misc util functions that cant find a place
mod utilities;

/// TODO: common imports for all modules, maybe make it specific, ie no wildcards. all modules that aren't plugin should probably be defined here
pub mod ahprelude;

pub use ahprelude::*;
// pub use loading::config::{
//     create_configured_app, ConfigFile, GameDifficulty, GeneralSettings, SoundSettings,
//     WindowSettings,
// };
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
            bevy_egui::EguiPlugin,
            bevy_ecs_ldtk::LdtkPlugin,
            belly::prelude::BellyPlugin,
            bevy_framepace::FramepacePlugin,
            bevy_prototype_lyon::prelude::ShapePlugin,
            bevy_rapier2d::plugin::RapierPhysicsPlugin::<bevy_rapier2d::prelude::NoUserData>::pixels_per_meter(32.0),
        ))
        .insert_resource(bevy_rapier2d::prelude::RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        });

    // add vanillacoffee stuff
    vanillacoffee.add_state::<game::AppStage>();
    vanillacoffee.add_plugins((
        loading::AppAssetsPlugin,
        console::QuakeConPlugin,
        game::GamePlugin,
    ));

    #[cfg(feature = "inspect")]
    vanillacoffee.add_plugins(DebugPlugin);

    vanillacoffee.add_systems(
        ahprelude::Update,
        (utilities::set_window_icon)
            .run_if(resource_exists::<SingleTileTextureHandles>().and_then(run_once())),
    );

    vanillacoffee
}
