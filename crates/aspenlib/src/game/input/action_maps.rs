use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

/// default keyboard/mouse input map
pub fn build_kbm_map(input_map: &mut InputMap<Gameplay>) {
    input_map.insert_multiple([
        (Gameplay::Sprint, KeyCode::ShiftLeft),
        (Gameplay::Attack, KeyCode::Space),
        (Gameplay::Interact, KeyCode::KeyE),
        (Gameplay::CycleWeapon, KeyCode::AltLeft),
        (Gameplay::UseAction1, KeyCode::Digit1),
        (Gameplay::UseAction2, KeyCode::Digit2),
        (Gameplay::UseAction3, KeyCode::Digit3),
        (Gameplay::UseAction4, KeyCode::Digit4),
        (Gameplay::UseAction5, KeyCode::Digit5),
        (Gameplay::ZoomAdd, KeyCode::NumpadAdd),
        (Gameplay::ZoomSubtract, KeyCode::NumpadSubtract),
        (Gameplay::Pause, KeyCode::Escape),
        (Gameplay::DebugF1, KeyCode::F1),
        (Gameplay::DebugF2, KeyCode::F2),
        (Gameplay::Melee, KeyCode::KeyF),
        (Gameplay::Heal, KeyCode::KeyC),
    ]);
    input_map.insert_dual_axis(Gameplay::Move, VirtualDPad::wasd());
}

/// default gamepad input map
pub fn build_gamepad_map(input_map: &mut InputMap<Gameplay>) {
    input_map.insert_dual_axis(Gameplay::Move, GamepadStick::LEFT);
    input_map.insert_dual_axis(Gameplay::Look, GamepadStick::RIGHT);
    input_map.insert_multiple([
        (Gameplay::Sprint, GamepadButtonType::West),
        (Gameplay::Attack, GamepadButtonType::South),
        (Gameplay::Interact, GamepadButtonType::East),
        (Gameplay::CycleWeapon, GamepadButtonType::North),
        (Gameplay::ZoomAdd, GamepadButtonType::DPadUp),
        (Gameplay::ZoomSubtract, GamepadButtonType::DPadDown),
        (Gameplay::Pause, GamepadButtonType::Start),
        (Gameplay::Melee, GamepadButtonType::DPadLeft),
        (Gameplay::Heal, GamepadButtonType::DPadRight),
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
    #[actionlike(DualAxis)]
    Look,
    /// Vec2: input from keyboard is collected via `VirtualDPad`, gamepad via `DualAxis`
    /// W/A/S/D for keyboard, `LeftJoystick` For mouse
    #[actionlike(DualAxis)]
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
