use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::ldtk::ReferenceToAnEntityInstance;

use crate::loading::registry::RegistryIdentifier;

/// location of hero that player can choose at start of game
#[derive(Component, Default)]
pub struct HeroSpot {
    /// name of hero that should be spawned at this `HeroSpot`
    pub what_hero: String,
}

#[derive(Debug, Component, Default)]
pub struct BossArea {
    /// list of enemys that are considered "bosses"
    pub dungeon_boss: Vec<RegistryIdentifier>,
    /// true/false are bosses defeated
    pub boss_defeated: bool,
}

/// Marks player start location
#[derive(Component, Default)]
pub struct PlayerStartLocation {
    /// area of this start location
    /// any point can be chosen
    /// inset of 2 tiles is applied when spawning
    pub size: Vec2,
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


impl TpTriggerEffect {
    pub fn is_event(&self) -> bool {
        match self {
            TpTriggerEffect::Event(_) => true,
            TpTriggerEffect::Local(_) => false,
            TpTriggerEffect::Global(_) => false,
        }
    }
}