use bevy::{
    ecs::system::NonSend,
    prelude::{warn, Entity, Query},
    window::Window,
};
use winit::window::Icon;

/// # Panics
/// will panic if it cant find a window to attach icons, or the icon is not present
pub fn set_window_icon(
    // we have to use `NonSend` here
    window_query: Query<Entity, &Window>,
    windows: NonSend<bevy::winit::WinitWindows>,
) {
    if let Ok(main_window) = window_query.get_single() {
        let Some(winit_window) = windows.get_window(main_window) else {
            warn!("NO WINDOW TOO SET ICON");
            return;
        };

        // here we use the `image` crate to load our icon data from a png file
        // this is not a very bevy-native solution, but it will do
        let (icon_rgba, icon_width, icon_height) = {
            let image = match image::open("assets/favicon.png") {
                Ok(img) => img.into_rgba8(),
                Err(e) => {
                    warn!("couldnt load window icon: {}", e);
                    return;
                }
            };
            let (width, height) = image.dimensions();
            let rgba = image.into_raw();
            (rgba, width, height)
        };

        let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height)
            .expect("the icon is not the correct size");

        winit_window.set_window_icon(Some(icon));
    }
}
