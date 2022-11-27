use bevy::prelude::{App, Plugin};

/// holds game consts and other settings related stuffs
pub mod game;

/// change window icon/title
pub mod window;

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
