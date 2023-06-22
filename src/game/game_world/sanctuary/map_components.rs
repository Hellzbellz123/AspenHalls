use bevy::prelude::*;

/// just a marker for sensors, saying whether active
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct SanctuaryTeleportSensor {
    pub active: bool,
}

#[derive(Resource, Clone, Debug, Default, Deref, DerefMut)]
pub struct TeleportTimer {
    pub timer: Timer,
}
