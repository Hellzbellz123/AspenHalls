use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    ecs::{entity::Entities, system::NonSend},
    prelude::{warn, Entity, Query, Res, ResMut, State, With},
    window::{PrimaryWindow, Window},
};
use winit::window::Icon;

use crate::game::GameStage;
// use bevy::winit::WinitWindows;
// extern crate winapi;

/// # Panics
/// will panic if it cant find a window to attach icons, or the icon is not present
pub fn set_window_icon(
    // we have to use `NonSend` here
    windowquery: Query<Entity, &Window>,
    windows: NonSend<bevy::winit::WinitWindows>,
) {
    if let Ok(main_window) = windowquery.get_single() {
        let Some(winit_window) = windows.get_window(main_window) else {
            warn!("NO WINDOW TOO SET ICON");
            return;
        };

        // here we use the `image` crate to load our icon data from a png file
        // this is not a very bevy-native solution, but it will do
        let (icon_rgba, icon_width, icon_height) = {
            let image = image::open("gamedata/assets/ico/stonercaticon.png")
                .expect("Failed to open icon path: assets/ico/stonercaticon.png")
                .into_rgba8();
            let (width, height) = image.dimensions();
            let rgba = image.into_raw();
            (rgba, width, height)
        };

        let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height)
            .expect("the icon is not the correct size");

        winit_window.set_window_icon(Some(icon))
    }
}

/// # Panics
/// will panic if their is no window, will also panic if the bevy diagnostics plugin isnt added
/// This system will then change the title during execution
pub fn set_debug_title(
    entities: &Entities,
    diagnostics: Res<Diagnostics>,
    state: ResMut<State<GameStage>>,
    windows: NonSend<bevy::winit::WinitWindows>,
    main_window: Query<(Entity, &mut Window), With<PrimaryWindow>>,
) {
    let title = format!(
        "Avg. FPS: {:.02} | Entity Count: {:?} | CurrentState: {:?}",
        diagnostics
            .get(FrameTimeDiagnosticsPlugin::FPS)
            .unwrap()
            .average()
            .unwrap_or(666.66),
        (entities.len()),
        state
    );

    if let Ok(main_window) = main_window.get_single() {
        let Some(winit_window) = windows.get_window(main_window.0) else {
            warn!("NO WINDOW TOO CHANGE TITLE");
            return;
        };
        winit_window.set_title(&title)
    }
}
