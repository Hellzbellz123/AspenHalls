use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::game::{
    actors::spawners::components::{EnemyType, Spawner, SpawnerTimer},
    game_world::hideout::TPType,
};

use super::hideout::map_components::Teleporter;

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

/// bundle too bind too `LdtkEntity` instance
#[derive(Bundle, LdtkEntity, Default)]
pub struct LdtkTeleporterBundle {
    /// marks this collider as a sensor
    sensor_tag: Sensor,
    /// sensor name
    #[with(name_from_json)]
    name: Name,
    /// teleporter data
    #[with(teleporter_from_json)]
    teleporter: Teleporter,
    /// shape of sensor
    #[with(teleporter_collider_from_json)]
    collision_shape: Collider,
    /// events from sensor
    #[with(events_from_json)]
    events: ActiveEvents,
}

fn teleporter_collider_from_json(instance: &EntityInstance) -> Collider {
    Collider::cuboid((instance.width / 4) as f32, (instance.height / 4) as f32)
}

fn events_from_json(_: &EntityInstance) -> ActiveEvents {
    ActiveEvents::COLLISION_EVENTS
}

/// sets up sensor bundle for binding
fn teleporter_from_json(instance: &EntityInstance) -> Teleporter {
    // let values = instance.field_instances.clone();
    // for val in values {
    //     info!("found field value for sensor: {:?}", val)
    // }
    let tp_type: TPType;
    let tp_action: Option<String>;
    let tp_local: Option<IVec2>;
    let tp_global: Option<Vec2>;

    match instance.get_enum_field("Teleport_Type") {
        Ok(val) => match val.as_str() {
            "Event" => {
                let action = match instance.get_string_field("Teleport_Action") {
                    Ok(val) => val.clone(),
                    Err(e) => {panic!("error getting Event Action: {}", e)},
                };
                tp_type = TPType::Event(action.clone());
                tp_action = Some(action.clone());
                tp_local = None;
                tp_global = None;
            }
            "Local" => {
                let local = match instance.get_point_field("Teleport_Local") {
                    Ok(val) => {val.clone()},
                    Err(e) => {panic!("error getting teleport local: {}", e)},
                };
                tp_type = TPType::Local(local.clone());
                tp_local = Some(local.clone());
                tp_action = None;
                tp_global = None;
            }
            "Global" => {
                let val = match instance.get_maybe_floats_field("Teleport_Global") {
                    Ok(val) => vec_from_array(val),
                    Err(e) => {panic!("error getting global vec: {}", e)},
                };
                tp_type = TPType::Global(val);
                tp_global = Some(val);
                tp_action = None;
                tp_local = None;
            }
            unknown => {
                panic!("encountered unknown enum value in json: {}", unknown)
            }
        },
        Err(e) => {
            warn!("couldnt get teleporter action {e}");
            let default = IVec2::ZERO;
            tp_type = TPType::Local(default);
            tp_local = Some(default);
            tp_action = None;
            tp_global = None;
        }
    };
    let teleporter = Teleporter {
        active: true,
        teleport_type: tp_type,
        teleport_action: tp_action,
        global_target: tp_global,
        local_target: tp_local,
    };
    info!("{:?}", teleporter);
    teleporter
}

    fn vec_from_array(option_array: &[Option<f32>]) -> Vec2 {
        // Extract values from the array using pattern matching
        let x = match option_array[0] {
            Some(value) => value,
            None => 0.0, // You can use a different default value if needed
        };

        let y = match option_array[1] {
            Some(value) => value,
            None => 0.0, // You can use a different default value if needed
        };

        Vec2 { x, y }
    }

fn name_from_json(instance: &EntityInstance) -> Name {
    Name::new(instance.identifier.clone())
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
        timer: SpawnerTimer(Timer::from_seconds(2.0, TimerMode::Repeating)),
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
