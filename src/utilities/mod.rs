use bevy::prelude::*;
use chrono::Utc;
use std::path::Path;

pub mod game;
pub mod window;

use self::game::AppSettings;
use crate::audio::SoundSettings;

/// holds general game utilities
/// not particularly related to gameplay
pub struct UtilitiesPlugin;

impl Plugin for UtilitiesPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "dev")]
        app.add_system(window::set_debug_title);

        app.add_startup_system(window::set_window_icon);
    }
}

pub fn despawn_with<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in to_despawn.iter() {
        info!("despawning entity recursively: {:#?}", entity);
        commands.entity(entity).despawn_recursive();
    }
}

pub fn load_settings() -> AppSettings {
    let settings_path = Path::new("./config.toml");
    let target_settings = std::fs::read_to_string(settings_path); //File::open(settings_path);

    match target_settings {
        Ok(_) => {
            println!("{}", append_info("Game Settings loaded succesfully"));
            let toml_cfg: AppSettings = toml::from_str(target_settings.unwrap().as_str())
                .expect("error parsing config file");
            AppSettings {
                sound_settings: toml_cfg.sound_settings,
                resolution: toml_cfg.resolution,
                camera_zoom: toml_cfg.camera_zoom,
            }
        }
        Err(_) => {
            println!(
                "{} {:?}, target file: {:?}",
                append_info("there was an error reading app settings:"),
                target_settings,
                settings_path,
            );

            let app_settings = AppSettings {
                camera_zoom: 1.0,
                resolution: Vec2 {
                    x: 1200.0,
                    y: 900.0,
                },
                sound_settings: SoundSettings {
                    mastervolume: 1.0,
                    ambiencevolume: 1.0,
                    musicvolume: 1.0,
                    soundvolume: 1.0,
                },
            };

            let serial_cfg =
                toml::to_string(&app_settings).expect("error converting config to string");
            std::fs::write(settings_path, serial_cfg).expect("couldnt write file");

            app_settings
        }
    }
}

/// custom wrapper over format that outputs current time in rfc2822 and a green INFO tag
// #[allow(unused_variables)]
pub fn append_info(content: &str) -> String {
    let time = Utc::now();
    format!("{} \x1b[32mINFO\x1b[0m {content}", time.to_rfc2822())
}
