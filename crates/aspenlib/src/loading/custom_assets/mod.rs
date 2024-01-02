use crate::loading::custom_assets::actor_definitions::ActorAssetPlugin;
use bevy::app::Plugin;

/// holds aspen halls custom actor asset plugins
pub mod actor_definitions;

/// handles initialization of all custom assets
pub struct AspenCustomAssetsPlugin;

impl Plugin for AspenCustomAssetsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(ActorAssetPlugin);
    }
}
