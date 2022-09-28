use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use super::actions::PlayerBindables;

pub struct ActionsPlugin;

// holds default bindings for game

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<PlayerBindables>::default());
    }
}

#[derive(Bundle)]
pub struct PlayerInput {
    #[bundle]
    input: InputManagerBundle<PlayerBindables>,
}

impl Default for PlayerInput {
    fn default() -> Self {
        use PlayerBindables::Move;

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

        input_map.insert(KeyCode::LShift, PlayerBindables::Dash);
        input_map.insert(GamepadButtonType::West, PlayerBindables::Dash);

        input_map.insert(KeyCode::Escape, PlayerBindables::Pause);
        input_map.insert(GamepadButtonType::Start, PlayerBindables::Pause);

        input_map.insert(KeyCode::Q, PlayerBindables::Heal);

        input_map.set_gamepad(Gamepad { id: 0 });
        Self {
            input: InputManagerBundle::<PlayerBindables> {
                input_map,
                ..Default::default()
            },
        }
    }
}
