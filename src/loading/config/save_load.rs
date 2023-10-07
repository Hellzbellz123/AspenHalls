use bevy::prelude::{info, Vec2};
use std::path::Path;

use crate::{
    loading::config::{ConfigFile, GeneralSettings, SoundSettings, WindowSettings},
    consts,
};

/// returns a default `ConfigFile`
pub fn create_default_settings() -> ConfigFile {
    let app_settings: ConfigFile = ConfigFile::default();
    app_settings
}

/// saves a `ConfigFile` at `&Path`
/// will panic if config file cant be deserialized into a string,
/// or if it cant write a file
pub fn save_settings(app_settings: &ConfigFile, settings_path: &Path) {
    info!("Saving AppSettings, this overwrites current settings");
    let serde_cfg = toml::to_string(&app_settings).expect("error converting config to string");
    std::fs::write(settings_path, serde_cfg).expect("couldn't write file");
}
