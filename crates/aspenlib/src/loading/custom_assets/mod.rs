// custom background shader asset for tiled single image
// pub mod background_shader;

pub mod npc_definition;

use bevy::app::Plugin;

/// handles initialization of all custom assets
pub struct AspenAssetsPlugin;

impl Plugin for AspenAssetsPlugin {
    fn build(&self, _app: &mut bevy::prelude::App) {
    }
}
