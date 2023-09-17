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
        wasm_logger::Config::new(log::Level::Info), // .module_prefix("wasm_kill_errors")
                                                    // .module_prefix("game"),
    );
    // Mount the DOM
    yew::Renderer::<Root>::new().render();
    // Start the Bevy App
    log::info!("Starting launcher: WASM");
    aspen_halls_game::app(false).run();
}
