use bevy::prelude::*;
use bevy_ecs_ldtk::{
    IntGridRendering, LdtkSettings, LdtkWorldBundle, LevelBackground, LevelSelection,
    LevelSpawnBehavior, SetClearColor,
};
use bevy_rapier2d::prelude::CollisionEvent;

use crate::{
    actors::{components::Player, player::utilities::PlayerColliderTag},
    loading::assets::MapAssetHandles,
};

use super::{
    components::{HomeWorldTeleportSensor, TeleportTimer},
    PlayerTeleportEvent,
};

pub fn spawn_mapbundle(mut commands: Commands, maps: Res<MapAssetHandles>) {
    commands.spawn(LdtkWorldBundle {
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
    commands
        .spawn(TeleportTimer {
            timer: Timer::from_seconds(5.0, TimerMode::Once),
        })
        .insert(Name::new("TeleportSensor"));
    commands.insert_resource(LevelSelection::Index(1));
    commands.insert_resource(LdtkSettings {
        level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
            load_level_neighbors: true,
        },
        set_clear_color: SetClearColor::No,
        int_grid_rendering: IntGridRendering::Invisible,
        level_background: LevelBackground::Nonexistent,
    });
}

pub fn homeworld_teleport(
    mut collision_events: EventReader<CollisionEvent>,
    world_sensors: Query<Entity, With<HomeWorldTeleportSensor>>,
    player_collider_query: Query<Entity, With<PlayerColliderTag>>,
    mut player_query: Query<&mut Player>,
    mut ew: EventWriter<PlayerTeleportEvent>,
    // rapier_context: Res<RapierContext>,
) {
    let _player = player_collider_query
        .get_single()
        .expect("should always be a player");

    for event in collision_events.iter() {
        match event {
            CollisionEvent::Started(a, b, _flags) | CollisionEvent::Stopped(a, b, _flags) => {
                let mut colliding_sensor: Option<Entity> = None;
                if *a == player_collider_query.single() || *b == player_collider_query.single() {
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
                            player_query.get_single_mut().expect("alwayas a player, especially here, see above").wants_to_teleport = true;
                            ew.send(PlayerTeleportEvent);
                        }
                    }
                }
            }
            _ => (),
        }
    }
    collision_events.clear();
}

pub fn enter_the_dungeon(
    _commands: Commands,
    time: Res<Time>,
    mut t_timer_query: Query<&mut TeleportTimer>,
    player_tp_events: EventReader<PlayerTeleportEvent>,
    mut player_query: Query<(&mut Transform, &mut Player)>,
) {
    let (mut ptransform, mut player) = player_query
        .get_single_mut()
        .expect("should always be a player if we are getting the event");
    let timer = &mut t_timer_query.get_single_mut().unwrap().timer;

    if !timer.finished() & player.wants_to_teleport {
        info!("timer not done, ticking timer");
        timer.tick(time.delta());
    }

    if !player_tp_events.is_empty() {
        info!("teleport event is queued");
        if timer.finished() & !player.just_teleported {
        *ptransform = Transform::from_xyz(46.0, 2900.0, 8.0);
            info!("player teleport/next playing sub-phase");

            player.just_teleported = false;
            player.wants_to_teleport = false;
        }
        player_tp_events.clear();
    }
}
