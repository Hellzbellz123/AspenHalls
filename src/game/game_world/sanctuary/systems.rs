use bevy::prelude::*;
use bevy_ecs_ldtk::{
    IntGridRendering, LdtkSettings, LdtkWorldBundle, LevelBackground, LevelSelection,
    LevelSpawnBehavior, SetClearColor,
};
use bevy_rapier2d::prelude::CollisionEvent;

use crate::{
    components::actors::{bundles::PlayerColliderTag, general::Player},
    game::game_world::dungeon_generator::GeneratorStage,
    loading::assets::MapAssetHandles,
};

use super::map_components::{SanctuaryTeleportSensor, TeleportTimer};

#[derive(Debug, Component, Clone, Copy, Reflect, Default)]
#[reflect(Component)]
pub struct MapContainerTag;

pub fn spawn_mapbundle(mut commands: Commands, maps: Res<MapAssetHandles>) {
    info!("spawning ldtkworldbundle");

    commands.spawn((
        LdtkWorldBundle {
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
        },
        Name::new("MapContainer"),
        MapContainerTag,
    ));
}

pub fn spawn_homeworld(mut commands: Commands) {
    commands.insert_resource(TeleportTimer {
        timer: Timer::from_seconds(2.0, TimerMode::Once),
    });

    commands.insert_resource(LevelSelection::Index(0));
    commands.insert_resource(LdtkSettings {
        level_spawn_behavior: LevelSpawnBehavior::UseZeroTranslation {},
        set_clear_color: SetClearColor::No,
        int_grid_rendering: IntGridRendering::Invisible,
        level_background: LevelBackground::Nonexistent,
    });
}

pub fn homeworld_teleport(
    mut collision_events: EventReader<CollisionEvent>,
    world_sensors: Query<Entity, With<SanctuaryTeleportSensor>>,
    player_collider_query: Query<Entity, With<PlayerColliderTag>>,
    mut player_query: Query<&mut Player>,
) {
    if player_query.is_empty() {
        return;
    }

    for event in collision_events.iter() {
        if let CollisionEvent::Started(a, b, _flags) = event {
            if *a == player_collider_query.single() || *b == player_collider_query.single() {
                let mut colliding_sensor: Option<Entity>;
                for sensor in world_sensors.iter() {
                    if sensor == *a {
                        colliding_sensor = Some(*a);
                    } else if sensor == *b {
                        colliding_sensor = Some(*b);
                    } else {
                        colliding_sensor = None;
                    }

                    if colliding_sensor.is_some() {
                        info!("player and sensor are colliding, sending teleport event");
                        player_query
                            .get_single_mut()
                            .expect("always a player, especially here, see above")
                            .wants_to_teleport = true;
                    }
                }
            }
        }
        if let CollisionEvent::Stopped(a, b, _flags) = event {
            //| CollisionEvent::Stopped(a, b, _flags)
            if *a == player_collider_query.single() || *b == player_collider_query.single() {
                let mut colliding_sensor: Option<Entity>;
                for sensor in world_sensors.iter() {
                    if sensor == *a {
                        colliding_sensor = Some(*a);
                    } else if sensor == *b {
                        colliding_sensor = Some(*b);
                    } else {
                        colliding_sensor = None;
                    }

                    if colliding_sensor.is_some() {
                        info!("player and sensor are colliding, sending teleport event");
                        player_query
                            .get_single_mut()
                            .expect("always a player, especially here, see above")
                            .wants_to_teleport = false;
                    }
                }
            }
        }
    }
    collision_events.clear();
}

pub fn enter_the_dungeon(
    mut commands: Commands,
    time: Res<Time>,
    mut teleport_timer: ResMut<TeleportTimer>,
    mut player_query: Query<(&mut Transform, &mut Player)>,
    _homeworld_container: Query<Entity, With<MapContainerTag>>,
) {
    let (mut ptransform, mut player) = player_query
        .get_single_mut()
        .expect("should always be a player if we are getting the event");

    if !player.wants_to_teleport {
        teleport_timer.reset();
        player.just_teleported = false;
    }

    if !teleport_timer.finished() & player.wants_to_teleport {
        info!("timer not done, ticking timer");
        teleport_timer.tick(time.delta());
        if teleport_timer.finished() && !player.just_teleported {
            commands.insert_resource(NextState(Some(GeneratorStage::Initialization)));
            *ptransform = Transform::from_xyz(816.0, 464.0, 8.0);
            info!("player teleport/next playing sub-phase");
            player.just_teleported = true;
        }
    } else if player.just_teleported {
        player.wants_to_teleport = false;
        player.just_teleported = false;
        teleport_timer.reset();
    }
}
