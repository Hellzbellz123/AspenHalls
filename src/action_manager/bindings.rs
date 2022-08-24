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

        // movement
        input_map.insert(
            VirtualDPad {
                up: KeyCode::W.into(),
                down: KeyCode::S.into(),
                left: KeyCode::A.into(),
                right: KeyCode::D.into(),
            },
            Move,
        );
        input_map.insert(
            DualAxis::symmetric(
                GamepadAxisType::LeftStickX,
                GamepadAxisType::LeftStickY,
                0.01, // TODO: this should probably be a game setting in a control menu so should the rest of this technically but w/e this is defaults
            ),
            Move,
        );

        input_map.insert(KeyCode::LShift, GameActions::Dash);
        input_map.insert(GamepadButtonType::RightTrigger2, GameActions::Dash);

        input_map.insert(KeyCode::Escape, GameActions::Pause);
        input_map.insert(GamepadButtonType::Start, GameActions::Pause);

        input_map.set_gamepad(Gamepad { id: 0 });
        Self {
            input: InputManagerBundle::<GameActions> {
                input_map,
                ..Default::default()
            },
        }
    }
}
