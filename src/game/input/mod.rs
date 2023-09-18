/// player input map
pub mod actions;

use bevy::prelude::{App, Plugin};
use leafwing_input_manager::prelude::InputManagerPlugin;

/// player input plugin
pub struct ActionsPlugin;

// holds default bindings for game
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<actions::Combat>::default());
    }
}
