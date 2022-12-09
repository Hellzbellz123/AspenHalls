use bevy::reflect::Reflect;
use bevy_inspector_egui::Inspectable;
use leafwing_input_manager::Actionlike;
use serde::Deserialize;

#[derive(
    Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Inspectable, Deserialize, Reflect,
)]
pub enum PlayerBindables {
    /// Vec2: input from keyboard is collected via VirtualDPad, gamepad via DualAxis
    Move,
    Climb,
    Sprint,
    Pause,
    Heal,
    Shoot,
    Melee,

    ZoomIn,
    ZoomOut,
    // debug
    DebugF1,
    DebugF2,
}
