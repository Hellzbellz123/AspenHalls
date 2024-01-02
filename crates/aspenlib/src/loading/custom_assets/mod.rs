use bevy::app::Plugin;
use crate::loading::custom_assets::actor_definitions::ActorAssetPlugin;

pub mod actor_definitions;

/// handles initialization of all custom assets
pub struct AspenCustomAssetsPlugin;

impl Plugin for AspenCustomAssetsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(ActorAssetPlugin);
    }
}
