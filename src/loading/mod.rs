/// holds asset definitions
pub mod assets;
/// splashscreen functions
pub mod splashscreen;

use bevy::prelude::*;
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};
use bevy_asset_loader::standard_dynamic_asset::StandardDynamicAssetCollection;

use crate::game::AppStage;
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
        app.add_plugins(splashscreen::SplashPlugin);
        app.add_loading_state(
            LoadingState::new(AppStage::Loading)
                .set_standard_dynamic_asset_collection_file_endings(["ron", "assets"].to_vec())
                .on_failure_continue_to_state(AppStage::FailedLoading)
                .continue_to_state(AppStage::StartMenu),
        )
        .add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
            AppStage::Loading,
            "registry.assets",
        )
        .add_collection_to_loading_state::<_, ActorTextureHandles>(AppStage::Loading)
        .add_collection_to_loading_state::<_, FontHandles>(AppStage::Loading)
        .add_collection_to_loading_state::<_, AudioHandles>(AppStage::Loading)
        .add_collection_to_loading_state::<_, UiTextureHandles>(AppStage::Loading)
        .add_collection_to_loading_state::<_, MapAssetHandles>(AppStage::Loading);
    }
}
