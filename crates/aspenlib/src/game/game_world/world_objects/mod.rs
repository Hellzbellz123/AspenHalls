/// functions for hydrating ldtk instances into entities
mod decode_instance;
/// systems too control hydrated instances
mod systems;

use bevy::prelude::{Bundle, Name};
use bevy_ecs_ldtk::prelude::LdtkEntity;
use bevy_rapier2d::prelude::{ActiveEvents, Collider, Sensor};

pub use self::systems::*;

use crate::game::game_world::{
    components::{
        CharacterSpawner, HeroLocation, PlayerStartLocation, SpawnerTimer, SpawnerWave, Teleporter,
        WeaponSpawner,
    },
    world_objects::decode_instance::*,
};

/// locations for placing playable heroes and hireable heroes
#[derive(Bundle, LdtkEntity, Default)]
pub struct LdtkHeroLocation {
    /// player selectable hero location
    #[with(hero_location_from_instance)]
    marker: HeroLocation,
}

/// locations for placing playable heroes and hireable heroes
#[derive(Bundle, LdtkEntity, Default)]
pub struct LdtkSpawnerWave {
    /// wave name
    #[with(name_from_instance)]
    name: Name,
    /// actual wave data
    #[with(spawner_wave_from_instance)]
    wave: SpawnerWave,
}

/// used to spawn player start location
#[derive(Bundle, LdtkEntity, Default)]
pub struct LdtkStartLocation {
    /// name of location in entity heirarchy
    #[with(name_from_instance)]
    name: Name,
    /// tag
    #[with(start_location_from_instance)]
    tag: PlayerStartLocation,
}

/// character spawner bundle too bind too `LdtkEntity` instance
#[derive(Bundle, LdtkEntity)]
pub struct LdtkCharacterSpawner {
    /// spawner name
    #[with(name_from_instance)]
    name: Name,
    /// spawner data
    #[with(character_spawner_from_instance)]
    state: CharacterSpawner,
    /// spawner timer
    #[with(spawn_timer_from_instance)]
    timer: SpawnerTimer,
}

/// bundle too bind too `LdtkEntity` instance
#[derive(Bundle, LdtkEntity)]
pub struct LdtkWeaponSpawner {
    /// spawner name
    #[with(name_from_instance)]
    name: Name,
    /// spawner data
    #[with(weapon_spawner_from_instance)]
    state: WeaponSpawner,
}

/// teleporter bundle that binds to `LdtkEntity` instances
#[derive(Bundle, LdtkEntity, Default)]
pub struct LdtkTeleporter {
    /// marks this collider as a sensor
    sensor_tag: Sensor,
    /// sensor name
    #[with(name_from_instance)]
    name: Name,
    /// teleporter data
    #[with(teleporter_from_instance)]
    teleporter: Teleporter,
    /// shape of sensor
    #[with(teleporter_collider_from_instance)]
    collision_shape: Collider,
    /// events from sensor
    #[with(events_from_instance)]
    events: ActiveEvents,
}
