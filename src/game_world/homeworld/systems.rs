use bevy::prelude::*;
use bevy_ecs_ldtk::{
    IntGridRendering, LdtkSettings, LdtkWorldBundle, LevelBackground, LevelSelection,
    LevelSpawnBehavior, SetClearColor,
};
use heron::CollisionEvent;

use crate::{
    actors::components::Player, game_world::homeworld::PlayerTeleportEvent,
    loading::assets::MapAssetHandles, utilities::game::is_sensor,
};

use super::components::WorldSensor;

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
                x: 3.0,
                y: 3.0,
                z: 1.0,
            },
            ..default()
        },
        ..default()
    });
}

pub fn spawn_level_0(mut commands: Commands) {
    commands.insert_resource(LevelSelection::Index(1));
    commands.insert_resource(LdtkSettings {
        level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
            load_level_neighbors: true,
        },
        set_clear_color: SetClearColor::No,
        int_grid_rendering: IntGridRendering::Invisible,
        level_background: LevelBackground::Nonexistent,
    })
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
    mut player_query: Query<(&mut Transform, &mut Player)>,
) {
    if !player_tp_events.is_empty() {
        if !player_query.single().1.just_teleported {
            let (mut ptransform, mut player) = player_query
                .get_single_mut()
                .expect("should always be a player if we are getting the event");
            *ptransform = Transform::from_xyz(46.0, 2900.0, 8.0);
            player.just_teleported = true;
            //do some stuff then clear events
        }
        info!("player teleport/next playing sub-phase");
        player_tp_events.clear();
    } else {
        player_query.single_mut().1.just_teleported = false;
    }
}
