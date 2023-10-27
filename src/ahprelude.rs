#![allow(unused_imports)]

// TODO: convert all source files too use ahprelude
// prelude then items not in prelude that are still needed in multiple spots.
// if import is only used in one file it can stay in that file ig
pub(crate) use bevy::prelude::*;

pub(crate) use rand::prelude::*;

pub use crate::loading::{
    assets::{ActorTextureHandles, MapAssetHandles, SingleTileTextureHandles},
    config::{
        ConfigFile, DifficultyScales, GameDifficulty, GeneralSettings, SoundSettings,
        WindowSettings,
    },
};

#[cfg(feature = "inspect")]
pub(crate) use crate::dev_tools::debug_plugin::DebugPlugin;
