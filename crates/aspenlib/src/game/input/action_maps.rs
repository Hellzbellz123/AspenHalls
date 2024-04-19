use bevy::prelude::*;
use leafwing_input_manager::{prelude::*, user_input::InputKind};

/// default keyboard/mouse input map
pub fn build_kbm_map(input_map: &mut InputMap<Gameplay>) {
    #[rustfmt::skip]
    input_map.insert_multiple(
        [
            (Gameplay::Move, UserInput::VirtualDPad(VirtualDPad::wasd())),
            (Gameplay::Sprint, UserInput::Single(InputKind::Keyboard(KeyCode::ShiftLeft))),
            (Gameplay::Attack, UserInput::Single(InputKind::Keyboard(KeyCode::Space))),
            (Gameplay::Interact, UserInput::Single(InputKind::Keyboard(KeyCode::E))),
            (Gameplay::CycleWeapon, UserInput::Single(InputKind::Keyboard(KeyCode::AltLeft))),
            (Gameplay::UseAction1, UserInput::Single(InputKind::Keyboard(KeyCode::Key1))),
            (Gameplay::UseAction2, UserInput::Single(InputKind::Keyboard(KeyCode::Key2))),
            (Gameplay::UseAction3, UserInput::Single(InputKind::Keyboard(KeyCode::Key3))),
            (Gameplay::UseAction4, UserInput::Single(InputKind::Keyboard(KeyCode::Key4))),
            (Gameplay::UseAction5, UserInput::Single(InputKind::Keyboard(KeyCode::Key5))),
            (Gameplay::ZoomIn, UserInput::Single(InputKind::Keyboard(KeyCode::NumpadAdd))),
            (Gameplay::ZoomOut,UserInput::Single(InputKind::Keyboard(KeyCode::NumpadSubtract))),
            (Gameplay::Pause, UserInput::Single(InputKind::Keyboard(KeyCode::Escape))),
            (Gameplay::DebugF1, UserInput::Single(InputKind::Keyboard(KeyCode::F1))),
            (Gameplay::DebugF2, UserInput::Single(InputKind::Keyboard(KeyCode::F2))),
            (Gameplay::Melee, UserInput::Single(InputKind::Keyboard(KeyCode::F))),
            (Gameplay::Heal, UserInput::Single(InputKind::Keyboard(KeyCode::C))),
            ],
    );
}

/// default gamepad input map
pub fn build_gamepad_map(input_map: &mut InputMap<Gameplay>) {
    #[rustfmt::skip]
    input_map.insert_multiple([
        (Gameplay::Move, UserInput::Single(InputKind::DualAxis(DualAxis::left_stick()))),
        (Gameplay::Look, UserInput::Single(InputKind::DualAxis(DualAxis::right_stick()))),
        (Gameplay::Sprint, UserInput::Single(InputKind::GamepadButton(GamepadButtonType::West))),
        (Gameplay::Attack, UserInput::Single(InputKind::GamepadButton(GamepadButtonType::South))),
        (Gameplay::Interact, UserInput::Single(InputKind::GamepadButton(GamepadButtonType::East))),
        (Gameplay::CycleWeapon, UserInput::Single(InputKind::GamepadButton(GamepadButtonType::North))),
        (Gameplay::ZoomIn, UserInput::Single(InputKind::GamepadButton(GamepadButtonType::DPadUp))),
        (Gameplay::ZoomOut, UserInput::Single(InputKind::GamepadButton(GamepadButtonType::DPadDown))),
        (Gameplay::Pause, UserInput::Single(InputKind::GamepadButton(GamepadButtonType::Start))),
        (Gameplay::Melee, UserInput::Single(InputKind::GamepadButton(GamepadButtonType::DPadLeft))),
        (Gameplay::Heal, UserInput::Single(InputKind::GamepadButton(GamepadButtonType::DPadRight))),
    ]);
}

impl Gameplay {
    /// default game input mappings
    pub fn default_input_map() -> InputMap<Self> {
        let mut input_map: InputMap<Self> = InputMap::default();
        build_kbm_map(&mut input_map);
        build_gamepad_map(&mut input_map);
        input_map
    }
}

/// non menu actions
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum Gameplay {
    // /// look target in window coordinates.
    // /// updated with `LookDelta``
    // CursorScreen,
    // /// look target in world space.
    // /// updated with `LookDelta`
    // CursorWorld,
    /// look target delta
    /// gathered from gamepad sticks and translated into a cursor position
    Look,
    /// Vec2: input from keyboard is collected via VirtualDPad, gamepad via DualAxis
    /// W/A/S/D for keyboard, LeftJoystick For mouse
    Move,
    /// Shift for keyboard,
    Sprint,
    /// Space for keyboard
    Attack,
    /// E for keyboard
    Interact,
    /// cycles equipped weapons
    CycleWeapon,

    // use actions
    /// action 1
    UseAction1,
    /// action 2
    UseAction2,
    /// action 3
    UseAction3,
    /// action 4
    UseAction4,
    /// action 5
    UseAction5,

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
    /// F for keyboard
    Melee,
    /// Q for keyboard
    Heal,
}
