use bevy::prelude::*;

use crate::game::game_world::hideout::TPType;

/// just a marker for sensors, saying whether active
#[derive(Component, Clone, Debug, Default)]
pub struct Teleporter {
    /// is this sensor running
    pub active: bool,
    pub teleport_type: TPType,
    pub teleport_action: Option<String>,
    pub global_target: Option<Vec2>,
    pub local_target: Option<IVec2>
}

/// waits too teleport
#[derive(Resource, Clone, Debug, Default, Deref, DerefMut)]
pub struct TeleportTimer {
    /// wait timer for sensor
    pub timer: Timer,
}
