use bevy::ecs::system::NonSend;
use bevy::window::WindowId;
use winit::window::Icon;
// use bevy::winit::WinitWindows;
// extern crate winapi;

/// # Panics
/// will panic if it cant find a window to attach icons, or the icon is not present
pub fn set_window_icon(
    // we have to use `NonSend` here
    windows: NonSend<bevy::winit::WinitWindows>,
) {
    let primary = windows
        .get_window(WindowId::primary())
        .expect("Couldnt find a window, thats wack");

    // here we use the `image` crate to load our icon data from a png file
    // this is not a very bevy-native solution, but it will do
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("assets/textures/stonercaticon.png")
            .expect("Failed to open icon path: assets/textures/stonercaticon.png")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height)
        .expect("the icon is not the correct size");

    primary.set_window_icon(std::option::Option::Some(icon));
}
