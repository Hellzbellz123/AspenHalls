use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::ldtk::ReferenceToAnEntityInstance;

use crate::{
    game::game_world::dungeonator_v2::components::RoomID, loading::registry::RegistryIdentifier,
};

/// location of hero that player can choose at start of game
#[derive(Component, Default)]
pub struct HeroSpot {
    /// name of hero that should be spawned at this `HeroSpot`
    pub what_hero: String,
}

/// location for Dungeon final bosses
#[derive(Debug, Component, Default)]
pub struct BossArea {
    /// list of enemys that are considered "bosses"
    pub dungeon_boss: Vec<RegistryIdentifier>,
    /// true/false are bosses defeated
    pub boss_defeated: bool,
}

// TODO: spawner waves
// if empty wave then randomly fill wave based on room level
/// spawner for enemies
#[derive(Component, Default, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct CharacterSpawner {
    /// list of enemys too spawn
    pub enemies_too_spawn: Vec<RegistryIdentifier>,
    /// how far away can spawn
    pub spawn_radius: f32,
    /// max enemies in spawner radius
    pub max_enemies: i32,
    /// list of enemies spawned by spawner
    pub spawned_characters: Vec<Entity>,
}

/// tile is a room exit
#[derive(Component, Default, Debug, Clone)]
pub struct RoomExitTile;

/// spawner for enemies
#[derive(Component, Default, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct WeaponSpawner {
    /// list of weapons too spawn
    pub wanted_weapons: Vec<RegistryIdentifier>,
    /// is this spawner interacted?
    pub interacted_only: bool,
    /// has this spawner been activated yet?
    pub triggered: bool,
    /// is this a debug spawner?
    pub debug: bool,
}

/// just a marker for sensors, saying whether active
#[derive(Component, Clone, Debug, Default)]
pub struct Teleporter {
    /// is this teleporter allowed too trigger
    pub active: bool,
    /// what does this teleporter do when it triggers
    pub effect: TpTriggerEffect,
}

/// Marks player start location
#[derive(Component, Default)]
pub struct PlayerStartLocation {
    /// area of this start location. any point inside can be chosen.
    /// - inset of 2 tiles is applied when spawning
    pub size: Vec2,
}

/// Marks Exits too dungeon rooms
#[derive(Debug, Component, Clone, Default, PartialEq, Eq, Reflect)]
#[reflect(Component)]
pub struct RoomExit {
    /// room this exit is part of
    pub parent: RoomID,
    /// has a hallway been connected too this exit
    pub hallway_connected: bool,
    /// position relative too lower left most tile
    pub position: IVec2,
}

/// timer for spawner
#[derive(Debug, Component, DerefMut, Deref, Default, Reflect, Clone)]
#[reflect(Component)]
pub struct SpawnerTimer(pub Timer);

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

// ########### impls ########### //
impl Default for TpTriggerEffect {
    fn default() -> Self {
        Self::Global(Vec2::ZERO)
    }
}

impl TpTriggerEffect {
    // /// checks if this `TpTriggerEffect` is of the event type
    // pub const fn is_event(&self) -> bool {
    //     match self {
    //         Self::Event(_) => true,
    //         Self::Local(_) | Self::Global(_) => false,
    //     }
    // }
}
