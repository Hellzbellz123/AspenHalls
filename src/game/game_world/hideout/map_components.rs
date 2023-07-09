use bevy::prelude::*;

/// just a marker for sensors, saying whether active
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct SanctuaryTeleportSensor {
    /// is this sensor running
    pub active: bool,
}

/// waits too teleport
#[derive(Resource, Clone, Debug, Default, Deref, DerefMut)]
pub struct TeleportTimer {
    /// wait timer for sensor
    pub timer: Timer,
}
