use bevy::prelude::*;
use leafwing_input_manager::{prelude::*, user_input::InputKind};

impl Gameplay {
    fn variants() -> &'static [Gameplay] {
        use Gameplay::*;
        &[
            Attack,
            CursorScreen,
            CursorWorld,
            CycleWeapon,
            DebugF1,
            DebugF2,
            Heal,
            Interact,
            JoystickDelta,
            Melee,
            Move,
            Pause,
            Sprint,
            UseAction1,
            UseAction2,
            UseAction3,
            UseAction4,
            UseAction5,
            ZoomIn,
            ZoomOut,
        ]
    }
    pub fn default_input_map() -> InputMap<Gameplay> {
        let mut input_map: InputMap<Gameplay> = InputMap::default();
        // Loop through each action in `PlayerAction` and get the default `UserInput`,
        // then insert each default input into input_map
        for action in Gameplay::variants() {
            input_map.insert(*action, Gameplay::default_gamepad_mapping(action));
            input_map.insert(*action, Gameplay::default_keyboard_mouse_mapping(action));
        }
        input_map
    }

    fn default_keyboard_mouse_mapping(&self) -> UserInput {
        match self {
            Gameplay::CursorScreen => UserInput::Chord(Vec::new()),
            Gameplay::CursorWorld => UserInput::Chord(Vec::new()),
            Gameplay::JoystickDelta => UserInput::Chord(Vec::new()),
            Gameplay::Move => UserInput::VirtualDPad(VirtualDPad::wasd()),
            Gameplay::Sprint => UserInput::Single(InputKind::Keyboard(KeyCode::ShiftLeft)),
            Gameplay::Attack => UserInput::Single(InputKind::Keyboard(KeyCode::Space)),
            Gameplay::Interact => UserInput::Single(InputKind::Keyboard(KeyCode::E)),
            Gameplay::CycleWeapon => UserInput::Single(InputKind::Keyboard(KeyCode::AltLeft)),
            Gameplay::UseAction1 => UserInput::Single(InputKind::Keyboard(KeyCode::Key1)),
            Gameplay::UseAction2 => UserInput::Single(InputKind::Keyboard(KeyCode::Key2)),
            Gameplay::UseAction3 => UserInput::Single(InputKind::Keyboard(KeyCode::Key3)),
            Gameplay::UseAction4 => UserInput::Single(InputKind::Keyboard(KeyCode::Key4)),
            Gameplay::UseAction5 => UserInput::Single(InputKind::Keyboard(KeyCode::Key5)),
            Gameplay::ZoomIn => UserInput::Single(InputKind::Keyboard(KeyCode::Minus)),
            Gameplay::ZoomOut => UserInput::Single(InputKind::Keyboard(KeyCode::Plus)),
            Gameplay::Pause => UserInput::Single(InputKind::Keyboard(KeyCode::Escape)),
            Gameplay::DebugF1 => UserInput::Single(InputKind::Keyboard(KeyCode::F1)),
            Gameplay::DebugF2 => UserInput::Single(InputKind::Keyboard(KeyCode::F2)),
            Gameplay::Melee => UserInput::Single(InputKind::Keyboard(KeyCode::F)),
            Gameplay::Heal => UserInput::Single(InputKind::Keyboard(KeyCode::C)),
        }
    }
    fn default_gamepad_mapping(&self) -> UserInput {
        match self {
            Gameplay::CursorScreen => UserInput::Chord(Vec::new()),
            Gameplay::CursorWorld => UserInput::Chord(Vec::new()),
            Gameplay::JoystickDelta => UserInput::Chord(Vec::new()),
            Gameplay::Move => UserInput::Single(InputKind::DualAxis(DualAxis::left_stick())),
            Gameplay::Sprint => {
                UserInput::Single(InputKind::GamepadButton(GamepadButtonType::West))
            }
            Gameplay::Attack => {
                UserInput::Single(InputKind::GamepadButton(GamepadButtonType::South))
            }
            Gameplay::Interact => {
                UserInput::Single(InputKind::GamepadButton(GamepadButtonType::East))
            }
            Gameplay::CycleWeapon => {
                UserInput::Single(InputKind::GamepadButton(GamepadButtonType::North))
            }
            Gameplay::ZoomIn => {
                UserInput::Single(InputKind::GamepadButton(GamepadButtonType::DPadUp))
            }
            Gameplay::ZoomOut => {
                UserInput::Single(InputKind::GamepadButton(GamepadButtonType::DPadDown))
            }
            Gameplay::Pause => {
                UserInput::Single(InputKind::GamepadButton(GamepadButtonType::Start))
            }
            Gameplay::Melee => {
                UserInput::Single(InputKind::GamepadButton(GamepadButtonType::DPadLeft))
            }
            Gameplay::Heal => {
                UserInput::Single(InputKind::GamepadButton(GamepadButtonType::DPadRight))
            }
            Gameplay::UseAction1 => UserInput::Chord(Vec::new()),
            Gameplay::UseAction2 => UserInput::Chord(Vec::new()),
            Gameplay::UseAction3 => UserInput::Chord(Vec::new()),
            Gameplay::UseAction4 => UserInput::Chord(Vec::new()),
            Gameplay::UseAction5 => UserInput::Chord(Vec::new()),
            Gameplay::DebugF1 => UserInput::Chord(Vec::new()),
            Gameplay::DebugF2 => UserInput::Chord(Vec::new()),
        }
    }
}

/// non menu actions
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum Gameplay {
    /// look target in window coordinates.
    /// updated with `LookDelta``
    CursorScreen,
    /// look target in world space.
    /// updated with `LookDelta`
    CursorWorld,
    /// look target delta
    /// gathered from gamepad sticks and translated into a cursor position
    JoystickDelta,
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
