pub mod assets;
pub mod splashscreen;

use bevy::prelude::*;
use bevy_asset_loader::prelude::LoadingState;
use bevy_asset_loader::standard_dynamic_asset::StandardDynamicAssetCollection;
use bevy_common_assets::ron::RonAssetPlugin;


use crate::game::GameStage;
use crate::loading::assets::{
    ActorTextureHandles, AudioHandles, FontHandles, MapAssetHandles, UiTextureHandles,
};

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html/>

pub struct AssetLoadPlugin;

impl Plugin for AssetLoadPlugin {
    fn build(&self, app: &mut App) {
        info!("asset loader init");
        app.add_plugin(RonAssetPlugin::<StandardDynamicAssetCollection>::new(&["ron", "assets"]));
        app.add_plugin(splashscreen::SplashPlugin);
        LoadingState::new(GameStage::Loading)
            .with_dynamic_collections::<StandardDynamicAssetCollection>(vec!["registery.assets"])
            .with_collection::<ActorTextureHandles>()
            .with_collection::<FontHandles>()
            .with_collection::<AudioHandles>()
            .with_collection::<UiTextureHandles>()
            .with_collection::<MapAssetHandles>()
            .on_failure_continue_to_state(GameStage::FailedLoading)
            .continue_to_state(GameStage::StartMenu)
            .build(app);
    }
}
