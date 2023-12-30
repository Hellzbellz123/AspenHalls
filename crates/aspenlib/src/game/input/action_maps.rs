use crate::prelude::engine::{
    Actionlike, Bundle, DeadZoneShape, DualAxis, GamepadAxisType, GamepadButtonType,
    InputManagerBundle, InputMap, KeyCode, QwertyScanCode, Reflect, VirtualDPad,
};

/// added too ActorBundle too make it a controllable player
#[derive(Bundle)]
pub struct PlayerBundle {
    /// actual bindings
    input: InputManagerBundle<Gameplay>,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        let mut input_map = InputMap::default();
        // ##### Keyboard Defaults
        // move
        input_map.insert(
            VirtualDPad {
                up: QwertyScanCode::W.into(),
                down: QwertyScanCode::S.into(),
                left: QwertyScanCode::A.into(),
                right: QwertyScanCode::D.into(),
            },
            Gameplay::Move,
        );
        // cycle weapon
        input_map.insert(KeyCode::AltLeft, Gameplay::CycleWeapon);
        // use ability/action/magic
        input_map.insert(KeyCode::Key1, Gameplay::UseAction1);
        input_map.insert(KeyCode::Key2, Gameplay::UseAction2);
        input_map.insert(KeyCode::Key3, Gameplay::UseAction3);
        input_map.insert(KeyCode::Key4, Gameplay::UseAction4);
        input_map.insert(KeyCode::Key4, Gameplay::UseAction5);
        // shoot weapon
        input_map.insert(KeyCode::Space, Gameplay::Shoot);
        // move faster
        input_map.insert(KeyCode::ShiftLeft, Gameplay::Sprint);
        // pause game
        input_map.insert(KeyCode::Escape, Gameplay::Pause);
        input_map.insert(KeyCode::E, Gameplay::Interact);
        //debug and misc
        input_map.insert(KeyCode::F1, Gameplay::DebugF1);
        input_map.insert(KeyCode::F2, Gameplay::DebugF2);
        input_map.insert(KeyCode::NumpadAdd, Gameplay::ZoomIn);
        input_map.insert(KeyCode::NumpadSubtract, Gameplay::ZoomOut);

        // ##### Gamepad Defaults
        // move
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
        // joystick too cursor value
        input_map.insert(DualAxis::right_stick(), Gameplay::JoystickDelta);
        // move faster
        input_map.insert(GamepadButtonType::West, Gameplay::Sprint);
        // pause game
        input_map.insert(GamepadButtonType::Start, Gameplay::Pause);

        // ##### return default
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
    Shoot,
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

    // /// F for keyboard
    // Melee,
    // /// Q for keyboard
    // Heal,
}
