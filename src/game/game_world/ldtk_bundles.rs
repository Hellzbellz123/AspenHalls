use bevy::{
    log::{error, info, warn},
    math::Vec2,
    prelude::{Bundle, Name, Timer, TimerMode},
};
use bevy_ecs_ldtk::{
    ldtk::ReferenceToAnEntityInstance,
    prelude::{EntityInstance, GridCoords, LdtkEntity, LdtkFields, LdtkIntCell, Worldly},
};
use bevy_rapier2d::prelude::{ActiveEvents, Collider, CollisionGroups, RigidBody, Sensor};

use crate::game::{
    actors::spawners::components::{Spawner, SpawnerTimer},
    game_world::components::{
        PlayerStartLocation, RoomExit, Teleporter, TpTriggerEffect, UnBuiltPlayer,
    },
};

/// tiles that can collide get this
#[derive(Clone, Debug, Bundle, LdtkIntCell, Default)]
pub struct LdtkCollisionBundle {
    /// name of collider
    pub name: Name,
    /// physics object
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
pub struct LdtkSpawnerBundle {
    /// spawner name
    #[with(name_from_instance)]
    name: Name,
    #[with(spawner_from_instance)]
    /// spawner data
    state: Spawner,
    #[with(spawn_timer_from_instance)]
    /// spawner timer
    timer: SpawnerTimer,
}

/// bundle to bind too `LdtkEntity` instance
#[derive(Bundle, LdtkEntity, Default)]
#[worldly]
pub struct LdtkPlayerBundle {
    /// player not built yet
    tag: UnBuiltPlayer,
    /// don't despawn
    world: Worldly,
}

/// used to spawn player start location
#[derive(Bundle, LdtkEntity, Default)]
pub struct LdtkStartLocBundle {
    /// tag
    #[with(start_location_from_instance)]
    tag: PlayerStartLocation,
}

fn start_location_from_instance(instance: &EntityInstance) -> PlayerStartLocation {
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
        TpTriggerEffect::Local(default.clone())
    });

    let teleporter = Teleporter {
        active: true,
        effect: tp_type,
    };
    info!("{:?}", teleporter);
    teleporter
}

/// creates `Name` from `EntityInstance.identifier`
fn name_from_instance(instance: &EntityInstance) -> Name {
    Name::new(instance.identifier.clone())
}

fn spawner_from_instance(entity_instance: &EntityInstance) -> Spawner {
    let got_ents: Vec<String> = entity_instance
        .get_maybe_strings_field("EnemyTypes")
        .expect("Spawner instances should ALWAYS have an EnemyTypes field")
        .iter()
        .filter(|f| f.is_some())
        .map(|f| f.clone().unwrap())
        .collect();
    let got_max_ents = entity_instance
        .get_maybe_int_field("MaxEnemies")
        .expect("Spawner should ALWAYS have MaxEnemies field")
        .unwrap_or(5);

    Spawner {
        enemies_too_spawn: got_ents,
        spawn_radius: entity_instance.width as f32,
        max_enemies: got_max_ents,
        spawned_enemies: Vec::new(),
    }
}

fn spawn_timer_from_instance(entity_instance: &EntityInstance) -> SpawnerTimer {
    let ldtk_ent_duration = entity_instance
        .get_maybe_float_field("DurationSecs")
        .unwrap()
        .unwrap_or(5.0);
    SpawnerTimer(Timer::from_seconds(ldtk_ent_duration, TimerMode::Repeating))
}

fn decipher_teleport_type(instance: &EntityInstance) -> Option<TpTriggerEffect> {
    let Ok(tp_type) = instance.get_enum_field("Teleport_Type") else {
        return None;
    };

    match tp_type.as_str() {
        "Event" => {
            if let Ok(action) = instance.get_string_field("Teleport_Action") {
                return Some(TpTriggerEffect::Event(action.clone()));
            } else {
                return None;
            }
        }
        "Local" => {
            if let Ok(local) = instance.get_entity_ref_field("Teleport_Local") {
                return Some(TpTriggerEffect::Local(local.clone()));
            } else {
                return None;
            }
        }
        "Global" => {
            let Ok(val) = instance
                .get_maybe_floats_field("Teleport_Global")
                .inspect_err(|e| error!("error getting TPType::Global from spawner instance: {e}"))
            else {
                return None;
            };
            let vec = vec2_from_array(val).expect("msg");
            return Some(TpTriggerEffect::Global(vec));
        }
        unknown => {
            error!(
                "encountered unknown TPType for spawner instance: {}",
                unknown
            );
            return None;
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
