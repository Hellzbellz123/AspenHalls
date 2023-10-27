use bevy::prelude::*;
use std::path::Path;

/// this translates too same folder as executable
pub const APP_SETTINGS_PATH: &str = "./config.toml";

fn main() {
    info!("Starting launcher: Native");
    let cfg_file: aspen_halls_game::ConfigFile = load_settings();
    aspen_halls_game::start_app(cfg_file).run();
}

/// loads app settings from `consts::APP_SETTINGS_PATH` and returns a boxed config file
pub fn load_settings() -> aspen_halls_game::ConfigFile {
    let settings_path = Path::new(APP_SETTINGS_PATH);
    info!("loading config file from filesystem @ {:?}", settings_path);
    let target_settings = match std::fs::read_to_string(settings_path) {
        // if settings file cant be read cause it doesn't exist, no permissions, or other
        Err(error) => {
            eprintln!(
                "There was an error: {} accessing settings file at: {}",
                error,
                settings_path.display()
            );
            return aspen_halls_game::ConfigFile::default();
        }
        // if settings file can be read
        Ok(target_settings) => target_settings,
    };

    match toml::from_str::<aspen_halls_game::ConfigFile>(target_settings.as_str()) {
        // if malformed settings file, create default
        Err(error) => {
            eprintln!(
                "There was an error deserializing `AppSettings`: {} at {}",
                error,
                settings_path.display()
            );
            aspen_halls_game::ConfigFile::default()
        }
        // setting file is not malformed, can be loaded
        Ok(cfg) => {
            println!("Game Settings loaded from file successfully");
            aspen_halls_game::ConfigFile {
                window_settings: aspen_halls_game::WindowSettings {
                    resolution: Vec2 {
                        x: cfg.window_settings.resolution.x,
                        y: cfg.window_settings.resolution.y,
                    },
                    v_sync: cfg.window_settings.v_sync,
                    frame_rate_target: cfg.window_settings.frame_rate_target,
                    full_screen: cfg.window_settings.full_screen,
                    window_scale_override: cfg.window_settings.window_scale_override,
                },

                sound_settings: aspen_halls_game::SoundSettings {
                    master_volume: cfg.sound_settings.master_volume,
                    ambience_volume: cfg.sound_settings.ambience_volume,
                    music_volume: cfg.sound_settings.music_volume,
                    sound_volume: cfg.sound_settings.sound_volume,
                },

                general_settings: aspen_halls_game::GeneralSettings {
                    camera_zoom: cfg.general_settings.camera_zoom,
                    game_difficulty: cfg.general_settings.game_difficulty,
                },
            }
        }
    }
}
