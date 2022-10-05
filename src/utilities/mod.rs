use bevy::prelude::{App, Plugin};

use self::window_icon::set_window_icon;

pub mod game;
pub mod window_icon;

pub struct UtilitiesPlugin;

impl Plugin for UtilitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(set_window_icon);
    }
}
