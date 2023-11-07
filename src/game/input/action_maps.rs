use crate::ahp::engine::*;

/// bundle for player bindings
#[derive(Bundle)]
pub struct PlayerBindings {
    /// actual bindings
    input: InputManagerBundle<Gameplay>,
}

impl Default for PlayerBindings {
    fn default() -> Self {
        let mut input_map = InputMap::default();

        // ## movement ##
        // keyboard
        input_map.insert(
            VirtualDPad {
                up: QwertyScanCode::W.into(), //KeyCode::W.into(),
                down: QwertyScanCode::S.into(),
                left: QwertyScanCode::A.into(),
                right: QwertyScanCode::D.into(),
            },
            Gameplay::Move,
        );
        // gamepad
        input_map.insert(
            DualAxis::symmetric(
                GamepadAxisType::LeftStickX,
                GamepadAxisType::LeftStickY,
                DeadZoneShape::Ellipse {
                    radius_x: 0.001,
                    radius_y: 0.001,
                },
            ),
            Gameplay::Move,
        );

        // equip slot []
        input_map.insert(KeyCode::Key1, Gameplay::EquipSlot1);
        input_map.insert(KeyCode::Key2, Gameplay::EquipSlot2);
        input_map.insert(KeyCode::Key3, Gameplay::EquipSlot3);
        input_map.insert(KeyCode::Key4, Gameplay::EquipSlot4);

        input_map.insert(KeyCode::Space, Gameplay::Shoot);
        input_map.insert(MouseButton::Left, Gameplay::Shoot);

        input_map.insert(KeyCode::F, Gameplay::Melee);

        input_map.insert(KeyCode::ShiftLeft, Gameplay::Sprint);
        input_map.insert(GamepadButtonType::West, Gameplay::Sprint);

        input_map.insert(KeyCode::Escape, Gameplay::Pause);
        input_map.insert(GamepadButtonType::Start, Gameplay::Pause);

        input_map.insert(KeyCode::Q, Gameplay::Heal);
        input_map.insert(KeyCode::E, Gameplay::Interact);

        //debug and misc
        input_map.insert(KeyCode::F1, Gameplay::DebugF1);
        input_map.insert(KeyCode::F2, Gameplay::DebugF2);
        input_map.insert(KeyCode::NumpadAdd, Gameplay::ZoomIn);
        input_map.insert(KeyCode::NumpadSubtract, Gameplay::ZoomOut);

        Self {
            input: InputManagerBundle::<Gameplay> {
                input_map,
                ..Default::default()
            },
        }
    }
}

/// non menu actions
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum Gameplay {
    /// mouse position in  window coordinates.
    LookLocal,
    /// mouse position in world space
    LookWorld,
    /// Vec2: input from keyboard is collected via VirtualDPad, gamepad via DualAxis
    /// W/A/S/D for keyboard, LeftJoystick For mouse
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
