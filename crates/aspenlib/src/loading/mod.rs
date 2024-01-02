/// holds asset definitions
pub mod assets;
/// holds game configuration structs and functions too define/create/load/save them
pub mod config;
/// extra custom asset useable in game
pub mod custom_assets;
/// splashscreen functions
pub mod splashscreen;
/// registry data
pub mod registry;

use crate::prelude::{engine::*, game::*};

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html/>
pub struct AppAssetLoadingPlugin;

impl Plugin for AppAssetLoadingPlugin {
    // TODO: convert the asset load plugin too handle the selected packs
    // this can probably reside in the config file, the native launcher should be used too select packs
    // mobile and web platforms will not make use of packs at this moment
    fn build(&self, app: &mut App) {
        app.add_plugins(custom_assets::AspenCustomAssetsPlugin);
        app.add_plugins(registry::RegistryPlugin);

        // TODO:
        // make the pack plugin, using bevy_asset_loader and bevy_common_assets

        app.add_loading_state(
            LoadingState::new(AppState::BootingApp)
                .load_collection::<AspenInitHandles>()
                .load_collection::<AspenTouchHandles>()
                .set_standard_dynamic_asset_collection_file_endings(["registry"].to_vec())
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>("init/pack.registry")
                .continue_to_state(AppState::Loading)
                .on_failure_continue_to_state(AppState::FailedLoadInit),
        )
        .add_loading_state(
            LoadingState::new(AppState::Loading)
                .continue_to_state(AppState::StartMenu)
                .on_failure_continue_to_state(AppState::FailedLoadMenu)
                .set_standard_dynamic_asset_collection_file_endings(["registry"].to_vec())
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>(
                    "packs/asha/pack.registry",
                )
                .load_collection::<AspenDefinitionHandles>()
                .load_collection::<AspenMapHandles>()
                .load_collection::<AspenTextureHandles>(),
        );
    }
}
