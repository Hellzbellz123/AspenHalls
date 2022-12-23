pub(crate) mod bindings;

use bevy::reflect::Reflect;
use bevy_inspector_egui::Inspectable;
use leafwing_input_manager::Actionlike;
use serde::Deserialize;

#[derive(
    Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Inspectable, Deserialize, Reflect,
)]

pub enum PlayerActions {
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
    // debug
    DebugF1,
    DebugF2,
}
