use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use super::bindings::GameActions;

pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<GameActions>::default());
    }
}

pub struct GameActionSettings {
    input: InputMap<GameActions>,
}

impl Default for GameActionSettings {
    fn default() -> Self {
        let mut input = InputMap::default();
        input
            .insert(KeyCode::W, GameActions::Up)
            .insert(KeyCode::S, GameActions::Down)
            .insert(KeyCode::A, GameActions::Left)
            .insert(KeyCode::D, GameActions::Right);
        Self { input }
    }
}
