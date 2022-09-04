pub mod assets;
pub mod setuptextureatlas;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::game::GameStage;
use crate::loading::assets::{AudioHandles, FontHandles, RexTextureHandles, UiTextureHandles};

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html/>

pub struct AssetLoadPlugin;

impl Plugin for AssetLoadPlugin {
    fn build(&self, app: &mut App) {
        info!("asset loader init");
        LoadingState::new(GameStage::Loading)
            .with_collection::<FontHandles>()
            .with_collection::<AudioHandles>()
            .with_collection::<RexTextureHandles>()
            .with_collection::<UiTextureHandles>()
            .continue_to_state(GameStage::Splash)
            .build(app);
    }
}
