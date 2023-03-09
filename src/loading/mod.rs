pub mod assets;
pub mod splashscreen;

use bevy::prelude::*;
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};
use bevy_asset_loader::standard_dynamic_asset::StandardDynamicAssetCollection;

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
        app.add_plugin(splashscreen::SplashPlugin);
        app.add_loading_state(
            LoadingState::new(GameStage::Loading)
                .set_standard_dynamic_asset_collection_file_endings(["ron", "assets"].to_vec())
                .on_failure_continue_to_state(GameStage::FailedLoading)
                .continue_to_state(GameStage::StartMenu),
        )
        .add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
            GameStage::Loading,
            "registery.assets",
        )
        .add_collection_to_loading_state::<_, ActorTextureHandles>(GameStage::Loading)
        .add_collection_to_loading_state::<_, FontHandles>(GameStage::Loading)
        .add_collection_to_loading_state::<_, AudioHandles>(GameStage::Loading)
        .add_collection_to_loading_state::<_, UiTextureHandles>(GameStage::Loading)
        .add_collection_to_loading_state::<_, MapAssetHandles>(GameStage::Loading);
    }
}
