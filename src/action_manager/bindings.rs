use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use super::actions::GameActions;

pub struct ActionsPlugin;

// holds default bindings for game

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<GameActions>::default());
    }
}

#[derive(Bundle)]
pub struct PlayerInput {
    #[bundle]
    input: InputManagerBundle<GameActions>,
}

impl Default for PlayerInput {
    fn default() -> Self {
        use GameActions::*;

        let mut input_map = InputMap::default();

        // basic movement
        input_map.insert(KeyCode::W, Up);
        input_map.insert(KeyCode::S, Down);
        input_map.insert(KeyCode::A, Left);
        input_map.insert(KeyCode::D, Right);

        input_map.insert(KeyCode::E, GameActions::Dash);
        input_map.insert(GamepadButtonType::RightTrigger2, GameActions::Dash);

        input_map.insert(KeyCode::Return, GameActions::Pause);
        input_map.insert(GamepadButtonType::Start, GameActions::Pause);

        input_map.set_gamepad(Gamepad(0));
        Self {
            input: InputManagerBundle::<GameActions> {
                input_map,
                ..Default::default()
            },
        }
    }
}
