pub mod assets;
pub mod splashscreen;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::game::GameStage;
use crate::loading::assets::{
    AudioHandles, FontHandles, ActorTextureHandles, MapAssetHandles, UiTextureHandles,
};

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html/>

pub struct AssetLoadPlugin;

impl Plugin for AssetLoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(splashscreen::SplashPlugin);
        info!("asset loader init");
        LoadingState::new(GameStage::Loading)
            .with_collection::<FontHandles>()
            .with_collection::<AudioHandles>()
            .with_collection::<ActorTextureHandles>()
            .with_collection::<UiTextureHandles>()
            .with_collection::<MapAssetHandles>()
            .on_failure_continue_to_state(GameStage::FailedLoading)
            .continue_to_state(GameStage::Menu)
            .build(app);
    }
}
