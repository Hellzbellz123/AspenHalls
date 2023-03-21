use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use super::PlayerActions;

pub struct ActionsPlugin;

// holds default bindings for game
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<PlayerActions>::default());
    }
}

#[derive(Bundle)]
pub struct PlayerInput {
    #[bundle]
    input: InputManagerBundle<PlayerActions>,
}

impl Default for PlayerInput {
    fn default() -> Self {
        use PlayerActions::Move;
        let mut input_map = InputMap::default();
        input_map.set_gamepad(Gamepad { id: 0 });

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

        // equip slot []
        input_map.insert(KeyCode::Key1, PlayerActions::EquipSlot1);
        input_map.insert(KeyCode::Key2, PlayerActions::EquipSlot2);
        input_map.insert(KeyCode::Key3, PlayerActions::EquipSlot3);
        input_map.insert(KeyCode::Key4, PlayerActions::EquipSlot4);

        input_map.insert(KeyCode::Space, PlayerActions::Shoot);
        input_map.insert(MouseButton::Left, PlayerActions::Shoot);

        input_map.insert(KeyCode::F, PlayerActions::Melee);

        input_map.insert(KeyCode::LShift, PlayerActions::Sprint);
        input_map.insert(GamepadButtonType::West, PlayerActions::Sprint);

        input_map.insert(KeyCode::Escape, PlayerActions::Pause);
        input_map.insert(GamepadButtonType::Start, PlayerActions::Pause);

        input_map.insert(KeyCode::Q, PlayerActions::Heal);
        input_map.insert(KeyCode::E, PlayerActions::Interact);

        //debug and misc
        input_map.insert(KeyCode::F1, PlayerActions::DebugF1);
        input_map.insert(KeyCode::F2, PlayerActions::DebugF2);
        input_map.insert(KeyCode::NumpadAdd, PlayerActions::ZoomIn);
        input_map.insert(KeyCode::NumpadSubtract, PlayerActions::ZoomOut);

        Self {
            input: InputManagerBundle::<PlayerActions> {
                input_map,
                ..Default::default()
            },
        }
    }
}
