use aspen_lib::ahp::aspen_lib as asha;
use aspen_lib::ahp::engine as bevy;
use std::path::Path;

/// this translates too same folder as executable
pub const APP_SETTINGS_PATH: &str = "./config.toml";

fn main() {
    bevy::info!("Starting launcher: Native");
    let cfg_file: asha::ConfigFile = load_settings();
    aspen_lib::start_app(cfg_file).run();
}

/// loads app settings from `consts::APP_SETTINGS_PATH` and returns a boxed config file
pub fn load_settings() -> asha::ConfigFile {
    let settings_path = Path::new(APP_SETTINGS_PATH);
    bevy::info!("loading config file from filesystem @ {:?}", settings_path);
    let target_settings = match std::fs::read_to_string(settings_path) {
        // if settings file cant be read cause it doesn't exist, no permissions, or other
        Err(error) => {
            eprintln!(
                "There was an error: {} accessing settings file at: {}",
                error,
                settings_path.display()
            );
            return asha::ConfigFile::default();
        }
        // if settings file can be read
        Ok(target_settings) => target_settings,
    };

    match toml::from_str::<asha::ConfigFile>(target_settings.as_str()) {
        // if malformed settings file, create default
        Err(error) => {
            eprintln!(
                "There was an error deserializing `AppSettings`: {} at {}",
                error,
                settings_path.display()
            );
            asha::ConfigFile::default()
        }
        // setting file is not malformed, can be loaded
        Ok(cfg) => {
            println!("Game Settings loaded from file successfully");
            asha::ConfigFile {
                window_settings: asha::WindowSettings {
                    resolution: bevy::Vec2 {
                        x: cfg.window_settings.resolution.x,
                        y: cfg.window_settings.resolution.y,
                    },
                    v_sync: cfg.window_settings.v_sync,
                    frame_rate_target: cfg.window_settings.frame_rate_target,
                    full_screen: cfg.window_settings.full_screen,
                    window_scale_override: cfg.window_settings.window_scale_override,
                },

                sound_settings: asha::SoundSettings {
                    master_volume: cfg.sound_settings.master_volume,
                    ambience_volume: cfg.sound_settings.ambience_volume,
                    music_volume: cfg.sound_settings.music_volume,
                    sound_volume: cfg.sound_settings.sound_volume,
                },

                general_settings: asha::GeneralSettings {
                    camera_zoom: cfg.general_settings.camera_zoom,
                    game_difficulty: cfg.general_settings.game_difficulty,
                },
                render_settings: asha::RenderSettings {
                    msaa: cfg.render_settings.msaa,
                },
            }
        }
    }
}
