use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::game::actors::spawners::components::{
    EnemyType, Spawner, SpawnerTimer,
};

use super::hideout::map_components::SanctuaryTeleportSensor;

/// tiles that can collide get this
#[derive(Clone, Debug, Bundle, LdtkIntCell, Default)]
pub struct CollisionBundle {
    /// name of collider
    pub name: Name,
    /// physics object
    pub rigidbody: RigidBody,
    /// collision shape
    pub collision_shape: Collider,
    /// what too collide with
    pub collision_group: CollisionGroups,
}

/// sensor components
#[derive(Clone, Debug, Bundle, LdtkEntity, Default)]
pub struct SensorBundle {
    /// sensor component
    name: Name,
    /// marks this collider as a sensor
    sensor: Sensor,
    /// marker
    tag: SanctuaryTeleportSensor,
    /// shape of sensor
    collision_shape: Collider,
    /// events from sensor
    events: ActiveEvents,
}

/// bundle too bind too `LdtkEntity` instance
#[derive(Bundle, LdtkEntity)]
pub struct LdtkSensorBundle {
    /// sensor
    #[with(sensor_bundle)]
    sensor_bundle: SensorBundle,
}

/// sets up sensor bundle for binding
fn sensor_bundle(_ent_instance: &EntityInstance) -> SensorBundle {
    SensorBundle {
        name: Name::new("SensorBundle"),
        collision_shape: Collider::cuboid(8., 8.),
        sensor: Sensor,
        events: ActiveEvents::COLLISION_EVENTS,
        tag: SanctuaryTeleportSensor { active: true },
    }
}

/// spawner components
#[derive(Clone, Debug, Bundle, LdtkEntity, Default)]
pub struct SpawnerBundle {
    /// spawner name
    name: Name,
    /// spawner data
    state: Spawner,
    /// spawner timer
    timer: SpawnerTimer,
}

/// bundle too bind too `LdtkEntity` instance
#[derive(Bundle, LdtkEntity)]
pub struct LdtkSpawnerBundle {
    /// what too add too entity
    #[with(spawner_bundle)]
    spawner_bundle: SpawnerBundle,
}

/// sets up spawner bundle for binding
fn spawner_bundle(_ent_instance: &EntityInstance) -> SpawnerBundle {
    SpawnerBundle {
        name: Name::new("Spawner"),
        state: Spawner {
            enemy_type: EnemyType::Skeleton,
            spawn_radius: 100.0,
            max_enemies: 7,
            random_enemy: true,
            spawned_enemies: Vec::with_capacity(7),
        },
        timer: SpawnerTimer(Timer::from_seconds(
            2.0,
            TimerMode::Repeating,
        )),
    }
}

/// unfinished player marker
#[derive(Component, Default)]
pub struct UnBuiltPlayer;

/// bundle to bind too `LdtkEntity` instance
#[derive(Bundle, LdtkEntity, Default)]
#[worldly]
pub struct LdtkPlayerBundle {
    /// player not built yet
    tag: UnBuiltPlayer,
    /// don't despawn
    world: Worldly,
}

/// Marks player start location
#[derive(Component, Default)]
pub struct PlayerStartLocation;

/// used to spawn player start location
#[derive(Bundle, LdtkEntity, Default)]
pub struct LdtkStartLocBundle {
    /// tag
    tag: PlayerStartLocation,
}

/// Marks Exits too dungeon rooms
#[derive(Component, Default)]
pub struct RoomExit {
    // /// is this exit used
    // map_used: bool,
    // /// direction of neighbor
    // neighbor_dir: Vec3,
}

// impl Default for RoomExit {
//     fn default() -> Self {
//         Self {
//             map_used: false,
//             neighbor_dir: Vec3::ZERO,
//         }
//     }
// }

/// used too spawn room exit in from LDTK levels
#[derive(Bundle, LdtkEntity, Default)]
pub struct LdtkRoomExitBundle {
    /// tag
    tag: RoomExit,
    /// position
    #[grid_coords]
    grid_coords: GridCoords,
}
