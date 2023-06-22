pub mod actions;

use bevy::prelude::{App, Plugin};
use leafwing_input_manager::prelude::InputManagerPlugin;

pub struct ActionsPlugin;

// holds default bindings for game
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<actions::Combat>::default());
    }
}
