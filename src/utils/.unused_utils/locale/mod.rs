use bevy::prelude::*;

#[cfg(feature = "inspect")]
use bevy_inspector_egui::prelude::*;

use self::asset::LocaleAssetLoader;

mod asset;

pub use self::asset::LocaleAsset;

pub struct LocalePlugin;

impl Plugin for LocalePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LocaleLangs>()
            .add_asset::<LocaleAsset>()
            .init_asset_loader::<LocaleAssetLoader>();
    }
}

#[derive(Clone, Copy, Debug, Resource, Reflect)]
#[cfg_attr(feature = "inspect", derive(InspectorOptions))]
#[cfg_attr(feature = "inspect", reflect(InspectorOptions))]
pub enum LocaleLangs {
    EN = 0,
    ES = 1,
}

impl LocaleLangs {
    pub fn to_name(&self) -> &str {
        match self {
            LocaleLangs::EN => "en-EN",
            LocaleLangs::ES => "es-ES",
        }
    }
}
