use bevy::prelude::{info, Vec2};
use std::path::Path;

use crate::{
    launch_config::{ConfigFile, GeneralSettings, SoundSettings, WindowSettings},
    consts,
};

/// loads app settings from `consts::APP_SETTINGS_PATH` and returns a boxed config file
pub fn load_settings() -> ConfigFile {
    let settings_path = Path::new(consts::APP_SETTINGS_PATH);
    info!("loading config file from filesystem @ {:?}", settings_path);
    let target_settings = match std::fs::read_to_string(settings_path) {
        // if settings file cant be read cause it doesn't exist, no permissions, or other
        Err(error) => {
            info!(
                "There was an error: {} accessing settings file at: {}",
                error,
                settings_path.display()
            );
            return create_default_settings();
        }
        // if settings file can be read
        Ok(target_settings) => target_settings,
    };

    match toml::from_str::<ConfigFile>(target_settings.as_str()) {
        // if malformed settings file, create default
        Err(error) => {
            info!(
                "There was an error deserializing `AppSettings`: {} at {}",
                error,
                settings_path.display()
            );
            create_default_settings()
        }
        // setting file is not malformed, can be loaded
        Ok(cfg) => {
            info!("Game Settings loaded from file successfully");
            ConfigFile {
                window_settings: WindowSettings {
                    resolution: Vec2 {
                        x: cfg.window_settings.resolution.x,
                        y: cfg.window_settings.resolution.y,
                    },
                    v_sync: cfg.window_settings.v_sync,
                    frame_rate_target: cfg.window_settings.frame_rate_target,
                    full_screen: cfg.window_settings.full_screen,
                },

                sound_settings: SoundSettings {
                    master_volume: cfg.sound_settings.master_volume,
                    ambience_volume: cfg.sound_settings.ambience_volume,
                    music_volume: cfg.sound_settings.music_volume,
                    sound_volume: cfg.sound_settings.sound_volume,
                },

                general_settings: GeneralSettings {
                    camera_zoom: cfg.general_settings.camera_zoom,
                    game_difficulty: cfg.general_settings.game_difficulty,
                },
            }
        }
    }
}

/// creates a default settings file and saves it `consts::APP_SETTINGS_PATH`
fn create_default_settings() -> ConfigFile {
    let settings_path = Path::new(consts::APP_SETTINGS_PATH);
    let app_settings: ConfigFile = ConfigFile::default();
    save_settings(&app_settings, settings_path);
    app_settings
}

/// saves a `Box<ConfigFile>` at `&Path`
/// will panic if config file cant be deserialized into a string,
/// or if it cant write a file
fn save_settings(app_settings: &ConfigFile, settings_path: &Path) {
    info!("Saving AppSettings, this overwrites current settings");
    let serde_cfg = toml::to_string(&app_settings).expect("error converting config to string");
    std::fs::write(settings_path, serde_cfg).expect("couldn't write file");
}
