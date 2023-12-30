use bevy::{
    prelude::{Component, Deref, DerefMut, Entity, Event, ReflectComponent, Vec2},
    reflect::Reflect,
    time::Timer,
};

use crate::{game::actors::ai::components::ActorType, loading::custom_assets::npc_definition::RegistryIdentifier};

/// Marker for enemy container
#[derive(Component)]
pub struct EnemyContainerTag;

/// timer for spawner
#[derive(Debug, Component, DerefMut, Deref, Default, Reflect, Clone)]
#[reflect(Component)]
pub struct SpawnerTimer(pub Timer);

/// spawner for enemies
#[derive(Component, Default, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct EnemySpawner {
    /// list of enemys too spawn
    pub enemies_too_spawn: Vec<RegistryIdentifier>,
    /// how far away can spawn
    pub spawn_radius: f32,
    /// max enemies in spawner radius
    pub max_enemies: i32,
    /// list of enemies spawned by spawner
    pub spawned_enemies: Vec<Entity>,
}

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

// TODO:
// split this into multiple events
// actor event should handle type and who spawned it,
// "sub events" should handle getting definiton and spawning
/// event for spawning actors (items/enemies/etc)
#[derive(Debug, Reflect, Clone, Event)]
pub struct SpawnActorEvent {
    /// type of actor
    pub actor_type: ActorType,
    /// set too spawner that requested this entity, none if its spawned by a player or some other reason
    pub who_spawned: Option<Entity>,
    //TODO: impl this
    // /// who this actor should be parented too
    // pub new_parent: Option<Entity>,
    /// string that is deserialized too variant of ActorType::Actor.value
    pub what_to_spawn: RegistryIdentifier,
    /// global pos too spawn actor
    pub spawn_position: Vec2,
    /// how many actors too spawn.
    //TODO impl spawn batch for spawning actors
    pub spawn_count: i32,
}
