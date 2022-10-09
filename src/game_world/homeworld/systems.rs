use bevy::prelude::*;
use bevy_ecs_ldtk::{LdtkWorldBundle, LevelSelection};
use heron::CollisionEvent;

use crate::{
    actors::player::Player,
    game_world::{homeworld::PlayerTeleportEvent, world_components::WorldSensor},
    loading::assets::MapAssetHandles,
    utilities::game::is_sensor,
};

pub fn spawn_mapbundle(mut commands: Commands, maps: Res<MapAssetHandles>) {
    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: maps.homeworld.clone(),
        transform: Transform {
            translation: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            scale: Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            ..default()
        },
        ..default()
    });
}

pub fn spawn_level_0(mut commands: Commands) {
    commands.insert_resource(LevelSelection::Index(0));
}

pub fn homeworld_teleport(
    mut commands: Commands,
    mut ew: EventWriter<PlayerTeleportEvent>,
    mut collision_events: EventReader<CollisionEvent>,
    world_sensor: Query<&WorldSensor>,
    player_query: Query<Entity, With<Player>>,
) {
    let player = player_query
        .get_single()
        .expect("should always be a player");


    if !world_sensor.is_empty() {
        collision_events
            .iter()
            .filter(|e| e.is_started())
            .filter_map(|event| {
                let (entity_1, entity_2) = event.rigid_body_entities();
                let (layers_1, layers_2) = event.collision_layers();
                let with_sensor = is_sensor(layers_1) || is_sensor(layers_2);

                if with_sensor {
                    if let Ok(..) = world_sensor.get(entity_1) {
                        return Some(entity_2);
                    } else if let Ok(..) = world_sensor.get(entity_2) {
                        return Some(entity_1);
                    };
                }
                None
            })
            .for_each(|entity| {
                if entity == player {
                    ew.send(PlayerTeleportEvent);
                } else {
                    info!("it wasnt the player that collided, dont bother");
                    let i = entity.type_name();
                    info!("sensor collided with {:?}, despawning....", i);
                    commands.entity(entity).despawn_recursive();
                }
            });
    }
}

pub fn enter_the_dungeon(
    // mut commands: Commands,
    player_tp_events: EventReader<PlayerTeleportEvent>,
    mut player_transform: Query<&mut Transform, With<Player>>,
) {
    if !player_tp_events.is_empty() {
        let mut mutplayer_transform = player_transform.get_single_mut().expect("should always be a player if we are getting the event");

        *mutplayer_transform = Transform::from_xyz(0.0, 0.0, 8.0);
        //do some stuff then clear events
        info!("player teleport/next playing sub-phase");
        player_tp_events.clear();
    }
}
