use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Bundle)]
pub struct PlayerBindings {
    #[bundle]
    input: InputManagerBundle<Combat>,
}

impl Default for PlayerBindings {
    fn default() -> Self {
        use Combat::Move;
        let mut input_map = InputMap::default();
        input_map.set_gamepad(Gamepad { id: 0 });

        // movement
        input_map.insert(
            VirtualDPad {
                up: QwertyScanCode::W.into(), //KeyCode::W.into(),
                down: QwertyScanCode::S.into(),
                left: QwertyScanCode::A.into(),
                right: QwertyScanCode::D.into(),
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
        input_map.insert(KeyCode::Key1, Combat::EquipSlot1);
        input_map.insert(KeyCode::Key2, Combat::EquipSlot2);
        input_map.insert(KeyCode::Key3, Combat::EquipSlot3);
        input_map.insert(KeyCode::Key4, Combat::EquipSlot4);

        input_map.insert(KeyCode::Space, Combat::Shoot);
        input_map.insert(MouseButton::Left, Combat::Shoot);

        input_map.insert(KeyCode::F, Combat::Melee);

        input_map.insert(KeyCode::LShift, Combat::Sprint);
        input_map.insert(GamepadButtonType::West, Combat::Sprint);

        input_map.insert(KeyCode::Escape, Combat::Pause);
        input_map.insert(GamepadButtonType::Start, Combat::Pause);

        input_map.insert(KeyCode::Q, Combat::Heal);
        input_map.insert(KeyCode::E, Combat::Interact);

        //debug and misc
        input_map.insert(KeyCode::F1, Combat::DebugF1);
        input_map.insert(KeyCode::F2, Combat::DebugF2);
        input_map.insert(KeyCode::NumpadAdd, Combat::ZoomIn);
        input_map.insert(KeyCode::NumpadSubtract, Combat::ZoomOut);

        Self {
            input: InputManagerBundle::<Combat> {
                input_map,
                ..Default::default()
            },
        }
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Combat {
    /// Vec2: input from keyboard is collected via VirtualDPad, gamepad via DualAxis
    ///
    /// WASD for keyboard, LeftJoystick For mouse
    Move,
    /// Shift for keyboard,
    Sprint,
    /// Space for keyboard
    Shoot,
    /// F for keyboard
    Melee,
    /// Q for keyboard
    Heal,
    /// E for keyboard
    Interact,

    //equip weapons
    /// 1 for keyboard
    EquipSlot1,
    /// 2 for keyboard
    EquipSlot2,
    /// 3 for keyboard
    EquipSlot3,
    /// 4 for keyboard
    EquipSlot4,

    /// Num - for keyboard
    ZoomIn,
    /// Num + for keyboard
    ZoomOut,
    /// Esc for keyboard
    Pause,
    /// spawn skeleton near player
    DebugF1,
    /// regenerate dungeon
    DebugF2,
}
