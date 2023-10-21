use aspen_halls_game::{ConfigFile, WindowSettings, SoundSettings, GeneralSettings, GameDifficulty};
use bevy::prelude::Vec2;
use yew::prelude::*;

fn set_window_title(title: &str) {
    web_sys::window()
        .and_then(|w| w.document())
        .expect("Unable to get DOM")
        .set_title(title);
}

#[function_component(Root)]
fn view() -> Html {
    set_window_title("Aspen Halls");

    html! {
        <> </>
    }
}

fn main() {
    #[cfg(feature = "inspect")]
    wasm_logger::init(
        wasm_logger::Config::new(log::Level::Info)
            // .module_prefix(module_prefix), // .module_prefix("wasm_kill_errors")
                                                    // .module_prefix("game"),
    );
    // Mount the DOM
    yew::Renderer::<Root>::new().render();
    // Start the Bevy App
    log::info!("Starting launcher: WASM");
    let cfg_file = ConfigFile {
        window_settings: WindowSettings { v_sync: true, frame_rate_target: 144.0, full_screen: false, resolution: Vec2::new(1920.0,1080.0), window_scale_override: 1.0 },
        sound_settings: SoundSettings { master_volume: 0.5, ambience_volume: 1.01, music_volume: 1.0, sound_volume: 1.0 },
        general_settings: GeneralSettings { camera_zoom: 3.5, game_difficulty: GameDifficulty::Easy },
    };
    aspen_halls_game::start_app(cfg_file).run();
}
