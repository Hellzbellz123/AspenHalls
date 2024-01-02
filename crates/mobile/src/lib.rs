#![doc = r"
    mobile library too be used by platform specific apps
    currently only targets android, should be expanded too ios mobile
"]

use aspenlib::prelude::engine::{self as engine, bevy};
use aspenlib::prelude::game as asha;

#[bevy::prelude::bevy_main]
fn main() {
    let config = asha::ConfigFile {
        log_filter: Some("Info,wgpu=error,naga=error".to_string()),
        window_settings: asha::WindowSettings {
            v_sync: true,
            frame_rate_target: 144.0,
            full_screen: true,
            resolution: engine::Vec2 {
                x: 1920.0,
                y: 1080.0,
            },
            window_scale_override: 1.75,
        },
        sound_settings: asha::SoundSettings {
            master_volume: 1.0,
            ambience_volume: 0.5,
            music_volume: 0.5,
            sound_volume: 0.5,
        },
        general_settings: asha::GeneralSettings {
            camera_zoom: 3.5,
            game_difficulty: asha::GameDifficulty::Medium,
        },
        render_settings: asha::RenderSettings { msaa: false },
    };

    println!("Starting launcher: Mobile");
    aspenlib::start_app(config).run();
}
