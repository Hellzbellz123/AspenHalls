#![allow(clippy::unnecessary_struct_initialization)]

use bevy::{
    log::{error, info, warn},
    math::Vec2,
    prelude::{Bundle, Name, Timer, TimerMode},
};
use bevy_ecs_ldtk::{
    ldtk::ReferenceToAnEntityInstance,
    prelude::{EntityInstance, GridCoords, LdtkEntity, LdtkFields, LdtkIntCell},
};
use bevy_rapier2d::prelude::{ActiveEvents, Collider, CollisionGroups, RigidBody, Sensor};

use crate::{
    game::{
        actors::spawners::components::{EnemySpawner, SpawnerTimer, WeaponSpawner},
        game_world::components::{
            BossArea, HeroSpot, PlayerStartLocation, RoomExit, Teleporter, TpTriggerEffect,
        },
    },
    loading::registry::RegistryIdentifier,
};

/// tiles that can collide get this
#[derive(Clone, Debug, Bundle, LdtkIntCell, Default)]
pub struct LdtkCollisionBundle {
    /// name of collider
    pub name: Name,
    /// entity has physics
    pub rigidbody: RigidBody,
    /// collision shape
    pub collision_shape: Collider,
    /// what too collide with
    pub collision_group: CollisionGroups,
}

/// used too spawn room exit in from LDTK levels
#[derive(Bundle, LdtkEntity, Default)]
pub struct LdtkRoomExitBundle {
    /// tag
    tag: RoomExit,
    /// position
    #[grid_coords]
    grid_coords: GridCoords,
}

/// teleporter bundle that binds to `LdtkEntity` instances
#[derive(Bundle, LdtkEntity, Default)]
pub struct LdtkTeleporterBundle {
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

/// bundle too bind too `LdtkEntity` instance
#[derive(Bundle, LdtkEntity)]
pub struct LdtkEnemySpawnerBundle {
    /// spawner name
    #[with(name_from_instance)]
    name: Name,
    #[with(enemy_spawner_from_instance)]
    /// spawner data
    state: EnemySpawner,
    #[with(spawn_timer_from_instance)]
    /// spawner timer
    timer: SpawnerTimer,
}

/// bundle too bind too `LdtkEntity` instance
#[derive(Bundle, LdtkEntity)]
pub struct LdtkWeaponSpawnerBundle {
    /// spawner name
    #[with(name_from_instance)]
    name: Name,
    #[with(weapon_spawner_from_instance)]
    /// spawner data
    state: WeaponSpawner,
}

/// bundle too bind too `LdtkEntity` instance
#[derive(Bundle, LdtkEntity, Default)]
pub struct LdtkBossManagerBundle {
    /// spawner name
    #[with(name_from_instance)]
    name: Name,
    #[with(boss_area_from_instance)]
    /// what bosses should be spawned
    manager: BossArea,
}

// TODO: use this or remove it
/// locations inside sanctuary too spawn heros that can be played inside dungeon
#[derive(Bundle, LdtkEntity, Default)]
pub struct LdtkHeroPlaceBundle {
    /// player not built yet
    tag: HeroSpot,
}

/// used to spawn player start location
#[derive(Bundle, LdtkEntity, Default)]
pub struct LdtkStartLocBundle {
    /// tag
    #[with(start_location_from_instance)]
    tag: PlayerStartLocation,
}

/// creates `PlayerStartLocation` from start location `EntityInstance`
const fn start_location_from_instance(instance: &EntityInstance) -> PlayerStartLocation {
    PlayerStartLocation {
        size: Vec2::new(instance.width as f32, instance.height as f32),
    }
}

/// creates a `Collider` from `EntityInstance` width/height
fn teleporter_collider_from_instance(instance: &EntityInstance) -> Collider {
    Collider::cuboid((instance.width / 4) as f32, (instance.height / 4) as f32)
}

/// creates `ActiveEvents` from `EntityInstance`
const fn events_from_instance(_: &EntityInstance) -> ActiveEvents {
    ActiveEvents::COLLISION_EVENTS
}

/// creates `Teleporter` from `EntityInstance`
fn teleporter_from_instance(instance: &EntityInstance) -> Teleporter {
    let tp_type = decipher_teleport_type(instance).unwrap_or_else(|| {
        warn!("couldnt get teleporter action");
        let default = ReferenceToAnEntityInstance::default();
        TpTriggerEffect::Local(default)
    });

    let teleporter = Teleporter {
        active: true,
        effect: tp_type,
    };
    info!("{:?}", teleporter);
    teleporter
}

/// creates a `BossManager` from boss manager `EntityInstance`
fn boss_area_from_instance(instance: &EntityInstance) -> BossArea {
    let identifiers_too_spawn = get_spawn_identifiers(instance);
    BossArea {
        dungeon_boss: identifiers_too_spawn,
        boss_defeated: false,
    }
}

/// gets list of `RegistryIdentifiers` from ldtk entity instance
fn get_spawn_identifiers(instance: &EntityInstance) -> Vec<RegistryIdentifier> {
    let strings = instance
        .get_maybe_strings_field("SpawnIdentifiers")
        .expect("Boss Area should ALWAYS have a DungeonBosses field")
        .iter()
        .filter_map(std::clone::Clone::clone)
        .map(RegistryIdentifier::from)
        .collect::<Vec<RegistryIdentifier>>();
    strings
}

/// creates `Name` from `EntityInstance.identifier`
fn name_from_instance(instance: &EntityInstance) -> Name {
    Name::new(instance.identifier.clone())
}

/// creates Spawner from spawner `EntityInstance`
fn enemy_spawner_from_instance(entity_instance: &EntityInstance) -> EnemySpawner {
    let identifiers_too_spawn = get_spawn_identifiers(entity_instance);
    let got_max_ents = entity_instance
        .get_maybe_int_field("MaxEnemies")
        .expect("Spawner should ALWAYS have MaxEnemies field")
        .unwrap_or(5);

    EnemySpawner {
        enemies_too_spawn: identifiers_too_spawn,
        spawn_radius: entity_instance.width as f32,
        max_enemies: got_max_ents,
        spawned_enemies: Vec::new(),
    }
}

/// creates Spawner from spawner `EntityInstance`
fn weapon_spawner_from_instance(entity_instance: &EntityInstance) -> WeaponSpawner {
    let identifiers_too_spawn = get_spawn_identifiers(entity_instance);
    let interacted_only = entity_instance
        .get_bool_field("Interacted")
        .expect("EnemySpawner should ALWAYS have Interacted field");

    let debug = entity_instance
        .get_bool_field("DebugWeaponSpawner")
        .expect("EnemySpawner should ALWAYS have Interacted field");

    WeaponSpawner {
        wanted_weapons: identifiers_too_spawn,
        interacted_only: *interacted_only,
        triggered: false,
        debug: *debug,
    }
}

/// creates a timer from spawner `EntityInstance`
fn spawn_timer_from_instance(entity_instance: &EntityInstance) -> SpawnerTimer {
    let ldtk_ent_duration = entity_instance
        .get_maybe_float_field("DurationSecs")
        .unwrap()
        .unwrap_or(5.0);
    SpawnerTimer(Timer::from_seconds(ldtk_ent_duration, TimerMode::Repeating))
}

/// creates `TpTriggerEffect` from data defined on teleporter `EntityInstance`
fn decipher_teleport_type(instance: &EntityInstance) -> Option<TpTriggerEffect> {
    let Ok(tp_type) = instance.get_enum_field("TeleporterType") else {
        return None;
    };

    match tp_type.as_str() {
        "Event" => instance
            .get_string_field("TeleporterAction")
            .map_or(None, |action| Some(TpTriggerEffect::Event(action.clone()))),
        "Local" => instance
            .get_entity_ref_field("TeleportLocalRef")
            .map_or(None, |local| Some(TpTriggerEffect::Local(local.clone()))),
        "Global" => {
            let Ok(val) = instance
                .get_maybe_floats_field("TeleportGlobalRef")
                .inspect_err(|e| error!("error getting TPType::Global from spawner instance: {e}"))
            else {
                return None;
            };
            let vec = vec2_from_array(val).expect("msg");
            Some(TpTriggerEffect::Global(vec))
        }
        unknown => {
            error!(
                "encountered unknown TPType for spawner instance: {}",
                unknown
            );
            None
        }
    }
}

/// creates Vec2 from array of f32s
fn vec2_from_array(option_array: &[Option<f32>]) -> Option<Vec2> {
    match option_array.len().cmp(&2) {
        std::cmp::Ordering::Equal => {}
        std::cmp::Ordering::Less => {
            warn!("array has 1 element, cant make Vec2");
            return None;
        }
        std::cmp::Ordering::Greater => {
            warn!("array has more than 2 elements, cant make Vec2");
            return None;
        }
    }

    // Extract values from the array using pattern matching
    let x = option_array[0].expect("unreachable expect?");
    let y = option_array[1].expect("unreachable expect?");

    Some(Vec2 { x, y })
}
