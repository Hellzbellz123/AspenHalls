use bevy::prelude::*;

use bevy_mouse_tracking_plugin::prelude::MousePosPlugin;
use leafwing_input_manager::prelude::*;

use super::actions::PlayerBindables;

pub struct ActionsPlugin;

// holds default bindings for game

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<PlayerBindables>::default())
            .add_plugin(MousePosPlugin);
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
        input_map.insert(KeyCode::Key1, PlayerBindables::EquipSlot1);
        input_map.insert(KeyCode::Key2, PlayerBindables::EquipSlot2);
        input_map.insert(KeyCode::Key3, PlayerBindables::EquipSlot3);
        input_map.insert(KeyCode::Key4, PlayerBindables::EquipSlot4);

        input_map.insert(KeyCode::Space, PlayerBindables::Shoot);
        input_map.insert(KeyCode::F, PlayerBindables::Melee);

        input_map.insert(KeyCode::LShift, PlayerBindables::Sprint);
        input_map.insert(GamepadButtonType::West, PlayerBindables::Sprint);

        input_map.insert(KeyCode::Escape, PlayerBindables::Pause);
        input_map.insert(GamepadButtonType::Start, PlayerBindables::Pause);

        input_map.insert(KeyCode::Q, PlayerBindables::Heal);

        //debug and misc
        input_map.insert(KeyCode::F1, PlayerBindables::DebugF1);
        input_map.insert(KeyCode::F2, PlayerBindables::DebugF2);
        input_map.insert(KeyCode::NumpadAdd, PlayerBindables::ZoomIn);
        input_map.insert(KeyCode::NumpadSubtract, PlayerBindables::ZoomOut);

        Self {
            input: InputManagerBundle::<PlayerBindables> {
                input_map,
                ..Default::default()
            },
        }
    }
}
