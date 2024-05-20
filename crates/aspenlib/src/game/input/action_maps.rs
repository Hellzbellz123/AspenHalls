use bevy::prelude::*;
use leafwing_input_manager::{prelude::*, user_input::InputKind};

/// default keyboard/mouse input map
pub fn build_kbm_map(input_map: &mut InputMap<Gameplay>) {
    #[rustfmt::skip]
    input_map.insert_multiple(
        [
            (Gameplay::Move, UserInput::VirtualDPad(VirtualDPad::wasd())),
            (Gameplay::Sprint, UserInput::Single(InputKind::PhysicalKey(KeyCode::ShiftLeft))),
            (Gameplay::Attack, UserInput::Single(InputKind::PhysicalKey(KeyCode::Space))),
            (Gameplay::Interact, UserInput::Single(InputKind::PhysicalKey(KeyCode::KeyE))),
            (Gameplay::CycleWeapon, UserInput::Single(InputKind::PhysicalKey(KeyCode::AltLeft))),
            (Gameplay::UseAction1, UserInput::Single(InputKind::PhysicalKey(KeyCode::Digit1))),
            (Gameplay::UseAction2, UserInput::Single(InputKind::PhysicalKey(KeyCode::Digit2))),
            (Gameplay::UseAction3, UserInput::Single(InputKind::PhysicalKey(KeyCode::Digit3))),
            (Gameplay::UseAction4, UserInput::Single(InputKind::PhysicalKey(KeyCode::Digit4))),
            (Gameplay::UseAction5, UserInput::Single(InputKind::PhysicalKey(KeyCode::Digit5))),
            (Gameplay::ZoomAdd,UserInput::Single(InputKind::PhysicalKey(KeyCode::NumpadAdd))),
            (Gameplay::ZoomSubtract, UserInput::Single(InputKind::PhysicalKey(KeyCode::NumpadSubtract))),
            (Gameplay::Pause, UserInput::Single(InputKind::PhysicalKey(KeyCode::Escape))),
            (Gameplay::DebugF1, UserInput::Single(InputKind::PhysicalKey(KeyCode::F1))),
            (Gameplay::DebugF2, UserInput::Single(InputKind::PhysicalKey(KeyCode::F2))),
            (Gameplay::Melee, UserInput::Single(InputKind::PhysicalKey(KeyCode::KeyF))),
            (Gameplay::Heal, UserInput::Single(InputKind::PhysicalKey(KeyCode::KeyC))),
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
        (Gameplay::ZoomAdd, UserInput::Single(InputKind::GamepadButton(GamepadButtonType::DPadUp))),
        (Gameplay::ZoomSubtract, UserInput::Single(InputKind::GamepadButton(GamepadButtonType::DPadDown))),
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
    /// cursor position onscreen
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
    ZoomSubtract,
    /// Num + for keyboard
    ZoomAdd,
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
