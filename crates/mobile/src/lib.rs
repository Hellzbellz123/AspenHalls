#![doc = r"
    mobile library too be used by platform specific apps
    currently only targets android, should be expanded too ios mobile
"]

use aspenlib::{
    ConfigFile, GameDifficulty, GeneralSettings, RenderSettings, SoundSettings, WindowSettings,
};
use bevy::{math::Vec2, prelude::bevy_main};

#[bevy_main]
fn main() {
    let config = ConfigFile {
        log_filter: Some("Info,wgpu=error,naga=error".to_string()),
        window_settings: WindowSettings {
            v_sync: true,
            frame_rate_target: 144.0,
            full_screen: true,
            resolution: Vec2 {
                x: 1920.0,
                y: 1080.0,
            },
            window_scale_override: 1.75,
        },
        sound_settings: SoundSettings {
            master_volume: 1.0,
            ambience_volume: 0.5,
            music_volume: 0.5,
            sound_volume: 0.5,
        },
        general_settings: GeneralSettings {
            camera_zoom: 3.5,
            game_difficulty: GameDifficulty::Medium,
        },
        render_settings: RenderSettings { msaa: false },
    };

    println!("Starting launcher: Mobile");
    aspenlib::start_app(config).run();
}
