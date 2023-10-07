use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::{Reflect, TypeUuid},
    utils::{BoxedFuture, HashMap},
};

#[cfg(feature = "inspect")]
use bevy_inspector_egui::prelude::*;

#[derive(Debug, Resource, Default, Reflect, TypeUuid)]
#[cfg_attr(feature = "inspect", derive(InspectorOptions))]
#[cfg_attr(feature = "inspect", reflect(InspectorOptions))]
#[uuid = "39cadc56-aa9c-4543-8640-a018b74b5082"]
pub struct LocaleAsset {
    pub value: HashMap<String, String>,
}

impl LocaleAsset {
    #[allow(unused)]
    pub fn get(&self, key: &str) -> Option<&str> {
        self.value.get(key).map(|v| v.as_str())
    }

    pub fn get_default<'a>(&'a self, key: &str, def: &'a str) -> &str {
        self.get(key).unwrap_or(def)
    }
}

#[derive(Default)]
pub struct LocaleAssetLoader;

impl AssetLoader for LocaleAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let mut asset = LocaleAsset::default();
            let content = String::from_utf8(bytes.to_vec())?;
            log::info!("Finished File Read on {:?}", load_context.path().to_str());
            asset.value.clear();
            for line in content.lines() {
                if !line.starts_with('#') {
                    let (key, value) = line.split_once('=').unwrap();
                    asset
                        .value
                        .insert(key.trim().to_string(), value.trim().to_string());
                }
            }
            load_context.set_default_asset(LoadedAsset::new(asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["locale"]
    }
}
