use bevy::prelude::{info, Vec2};
use std::{path::Path, thread};

use crate::{
    app_config::{ConfigFile, GeneralSettings, SoundSettings, WindowSettings},
    consts::APP_SETTINGS_PATH,
};

use super::GameDifficulty;

#[must_use]
pub fn load_settings() -> Box<ConfigFile> {
    let settings_path = Path::new(APP_SETTINGS_PATH);
    info!("got settings path");
    let settings_file_path = Box::new(std::fs::read_to_string(settings_path)); //File::open(settings_path);
    info!("loading from std::fs, returns Box<Result<String,Error>>");
    match *settings_file_path {
        // if settings file cant be read cause it doesnt exit, no permissions, or other
        Err(target_settings) => {
            info!(
                "there was an error: {} acessing settings file as: {}",
                target_settings,
                settings_path.display()
            );

            create_default_settings()
        }
        // if settings file can be read
        Ok(target_settings) => {
            let toml_cfg: Box<ConfigFile> =
                match toml::from_str::<ConfigFile>(target_settings.as_str()) {
                    // if malformed settings file, create default
                    Err(toml_cfg) => {
                        info!(
                            "There was an error deserializing `AppSettings`: {} at {}",
                            toml_cfg,
                            settings_path.display()
                        );

                        create_default_settings()
                    }
                    // setting file is not malformed, can be loaded
                    Ok(toml_cfg) => {
                        info!("Game Settings loaded from file succesfully");

                        Box::new(ConfigFile {
                            window_settings: Box::new(WindowSettings {
                                resolution: Vec2 {
                                    x: toml_cfg.window_settings.resolution.x,
                                    y: toml_cfg.window_settings.resolution.y,
                                },
                                vsync: toml_cfg.window_settings.vsync,
                                frame_rate_target: toml_cfg.window_settings.frame_rate_target,
                                fullscreen: toml_cfg.window_settings.fullscreen,
                            }),

                            sound_settings: Box::new(SoundSettings {
                                mastervolume: toml_cfg.sound_settings.mastervolume,
                                ambiencevolume: toml_cfg.sound_settings.ambiencevolume,
                                musicvolume: toml_cfg.sound_settings.musicvolume,
                                soundvolume: toml_cfg.sound_settings.soundvolume,
                            }),

                            general_settings: Box::new(GeneralSettings {
                                camera_zoom: toml_cfg.general_settings.camera_zoom,
                                game_difficulty: toml_cfg.general_settings.game_difficulty,
                            }),
                        })
                    }
                };
            toml_cfg
        }
    }
}

fn create_default_settings() -> Box<ConfigFile> {
    let settings_path = Path::new(APP_SETTINGS_PATH);
    let app_settings = {
        let window_settings = Box::new(WindowSettings {
            resolution: Vec2 {
                x: 1280.0,
                y: 720.0,
            },
            vsync: true,
            frame_rate_target: 60.0,
            fullscreen: false,
        });

        let sound_settings = Box::new(SoundSettings {
            mastervolume: 0.2,
            ambiencevolume: 0.2,
            musicvolume: 0.2,
            soundvolume: 0.2,
        });

        let general_settings = Box::new(GeneralSettings {
            camera_zoom: 1.0,
            game_difficulty: GameDifficulty::Medium,
        });

        Box::new(ConfigFile {
            window_settings,
            sound_settings,
            general_settings,
        })
    };

    save_settings(app_settings.clone(), settings_path);
    app_settings
}

fn save_settings(app_settings: Box<ConfigFile>, settings_path: &Path) {
    info!("Saving AppSettings, this overwrites current settings");
    let serd_cfg = toml::to_string(&app_settings).expect("error converting config to string");
    std::fs::write(settings_path, serd_cfg).expect("couldnt write file");
}
