/// player input map
pub mod actions;
mod touch_input;

use bevy::prelude::{App, Plugin};
use leafwing_input_manager::prelude::InputManagerPlugin;

use self::touch_input::TouchInputPlugin;

/// player input plugin
pub struct ActionsPlugin;

// holds default bindings for game
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<actions::Combat>::default());

        app.add_plugins(TouchInputPlugin);
    }
}
