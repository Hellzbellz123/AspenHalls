/// holds asset definitions
pub mod assets;
/// holds game configuration structs and functions too define/create/load/save them
pub mod config;
/// extra custom asset useable in game
pub mod custom_assets;
/// splashscreen functions
pub mod splashscreen;

use crate::ahp::{engine::*, game::*};
/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html/>
pub struct AppAssetsPlugin;

impl Plugin for AppAssetsPlugin {
    // TODO: convert the asset load plugin too handle the selected packs
    // this can probably reside in the config file, the native launcher should be used too select packs
    // mobile and web platforms will not make use of packs at this moment
    fn build(&self, app: &mut App) {
        info!("asset loader init");
        app.add_loading_state(
            LoadingState::new(AppState::Loading)
                .set_standard_dynamic_asset_collection_file_endings(["registry"].to_vec())
                .continue_to_state(AppState::StartMenu)
                .on_failure_continue_to_state(AppState::StartMenu),
        )
        .add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
            AppState::Loading,
            "packs/asha/pack.registry",
        )
        .add_collection_to_loading_state::<_, ActorTextureHandles>(AppState::Loading)
        .add_collection_to_loading_state::<_, AudioHandles>(AppState::Loading)
        .add_collection_to_loading_state::<_, MapAssetHandles>(AppState::Loading)
        .add_collection_to_loading_state::<_, SingleTileTextureHandles>(AppState::Loading);
    }
}
