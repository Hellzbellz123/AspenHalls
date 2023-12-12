use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::ldtk::ReferenceToAnEntityInstance;

/// unfinished player marker
#[derive(Component, Default)]
pub struct UnBuiltPlayer;

/// Marks player start location
#[derive(Component, Default)]
pub struct PlayerStartLocation {
    pub size: Vec2
}

/// Marks Exits too dungeon rooms
#[derive(Component, Default)]
pub struct RoomExit {
    // /// is this exit used
    // map_used: bool,
    // /// direction of neighbor
    // neighbor_dir: Vec3,
}

/// just a marker for sensors, saying whether active
#[derive(Component, Clone, Debug, Default)]
pub struct Teleporter {
    /// is this teleporter allowed too trigger
    pub active: bool,
    /// what does this teleporter do when it triggers
    pub effect: TpTriggerEffect,
}

/// event for player teleportation
#[derive(Event, Debug)]
pub struct ActorTeleportEvent {
    /// enum deciding weather this teleport triggers an action or moves entity locally/globall directly
    /// unhandled tp_actions get warned about
    pub tp_type: TpTriggerEffect,
    /// affected entitiy for this teleport
    pub target: Option<Entity>,
    /// sensor entity that sent this event
    pub sender: Option<Entity>,
}

/// teleport data passed from teleporter too event
///
/// - `Local` must be given a tile uuid
/// - `Event` must be given a valid event string
/// - `Global` defaults too world 0 0 if invalid data is passed.
#[derive(Debug, Clone)]
pub enum TpTriggerEffect {
    //TODO: expand this for better type checking
    /// string type triggering other `Event`
    Event(String),
    /// local teleport. this is alays in tiles, per room
    Local(ReferenceToAnEntityInstance),
    /// teleport with a global pixel position
    Global(Vec2),
}

impl Default for TpTriggerEffect {
    fn default() -> Self {
        Self::Global(Vec2::ZERO)
    }
}
