use bevy::prelude::*;
use bevy_ecs_ldtk::{
    IntGridRendering, LdtkSettings, LdtkWorldBundle, LevelBackground, LevelSelection,
    LevelSpawnBehavior, SetClearColor, prelude::LevelIid,
};
use bevy_rapier2d::prelude::{CollisionEvent, Sensor};

use crate::{
    game::{
        actors::components::{ActorColliderTag, ActorMoveState, TeleportStatus},
        game_world::components::{ActorTeleportEvent, Teleporter},
        // game_world::dungeonator::GeneratorStage,
    },
    loading::assets::MapAssetHandles,
};

/// tag for map entity
#[derive(Debug, Component, Clone, Copy, Reflect, Default)]
#[reflect(Component)]
pub struct MapContainerTag;

/// spawns hideout and related resources
pub fn spawn_hideout(mut commands: Commands, maps: Res<MapAssetHandles>) {
    info!("spawning LdtkWorldBundle");

    commands.spawn((
        LdtkWorldBundle {
            ldtk_handle: maps.default_levels.clone(),
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

    // TODO match on saved state/player progress
    commands.insert_resource(LevelSelection::Iid(LevelIid::new("e48949c0-8990-11ee-a550-f91ac169a863")));
    commands.insert_resource(LdtkSettings {
        level_spawn_behavior: LevelSpawnBehavior::UseZeroTranslation,
        set_clear_color: SetClearColor::No,
        int_grid_rendering: IntGridRendering::Invisible,
        level_background: LevelBackground::Nonexistent,
    });
}

/// system too check for actors on teleport pad
pub fn teleporter_collisions(
    mut collision_events: EventReader<CollisionEvent>,
    mut teleport_events: EventWriter<ActorTeleportEvent>,
    mut actors: Query<(Entity, &mut ActorMoveState)>,
    sensors: Query<(Entity, &Teleporter), With<Sensor>>,
    parents: Query<&Parent, With<ActorColliderTag>>,
) {
    // TODO: check TeleportStatus if we are allowed too send this teleport
    // or on the EventReader side, get status and return with warning
    for event in &mut collision_events.read() {
        let (mut actor, sensor, event_is_collision_start) = match event {
            //TODO: what happens if i shoot the teleporter?
            CollisionEvent::Started(a, b, _) => {
                if sensors.get(*a).is_ok() && parents.get(*b).is_ok() {
                    let ac = actors
                        .get_mut(
                            **parents
                                .get(*b)
                                .expect("actor collider should have a parent"),
                        )
                        .expect("colliders parent should have been an actor");
                    let tp = sensors.get(*a).expect("checking ok beforehand");
                    (ac, tp, true)
                } else if sensors.get(*b).is_ok() && parents.get(*a).is_ok() {
                    let ac = actors
                        .get_mut(
                            **parents
                                .get(*a)
                                .expect("actor collider should have a parent"),
                        )
                        .expect("msg");
                    let tp = sensors.get(*b).expect("checking ok beforehand");
                    (ac, tp, true)
                } else {
                    trace!("not handling collision event because neither entity is a teleporter");
                    return;
                }
            }
            CollisionEvent::Stopped(a, b, _) => {
                if sensors.get(*a).is_ok() && parents.get(*b).is_ok() {
                    let ac = actors
                        .get_mut(
                            **parents
                                .get(*b)
                                .expect("actor collider should have a parent"),
                        )
                        .expect("colliders parent should have been an actor");
                    let tp = sensors.get(*a).expect("checking ok beforehand");
                    (ac, tp, false)
                } else if sensors.get(*b).is_ok() && parents.get(*a).is_ok() {
                    let ac = actors
                        .get_mut(
                            **parents
                                .get(*a)
                                .expect("actor collider should have a parent"),
                        )
                        .expect("msg");
                    let tp = sensors.get(*b).expect("checking ok beforehand");
                    (ac, tp, false)
                } else {
                    trace!("not handling collision event because neither entity is a teleporter");
                    return;
                }
            }
        };

        if !event_is_collision_start {
            match actor.1.teleport_status {
                TeleportStatus::None => {
                    warn!("exited teleporter with 'none'");
                }
                TeleportStatus::Requested => {
                    warn!("exited teleporter with 'requested'");
                }
                TeleportStatus::Teleporting => {
                    warn!("exited teleporter with 'teleporting'");
                    actor.1.teleport_status = TeleportStatus::Done;
                }
                TeleportStatus::Done => {
                    warn!("exited teleporter with 'done'");
                    actor.1.teleport_status = TeleportStatus::None;
                }
            }
        } else if actor.1.teleport_status == TeleportStatus::None && event_is_collision_start {
            teleport_events.send(ActorTeleportEvent {
                tp_type: sensor.1.effect.clone(),
                target: Some(actor.0),
                sender: Some(sensor.0),
            });
            actor.1.teleport_status = TeleportStatus::Requested;
            warn!("requesting teleport");
            return;
        }

        // for (actor_ent, mut state) in &mut actor_query {
        //     let ac = children
        //         .iter_descendants(actor_ent)
        //         .find(|e| collider_query.get(*e).is_ok())
        //         .expect("Actors children did not have a collider.");
        //     let tp_status = state.clone();

        //     match &tp_status.teleport_status {
        //         TeleportStatus::None => {
        //             if let CollisionEvent::Started(a, b, _) = event {
        //                 if *a == ac || *b == ac {
        //                     if let Some((sensor, teleporter)) = teleporters
        //                         .iter()
        //                         .find(|&(sensor, _)| sensor == *a || sensor == *b)
        //                     {
        //                         teleport_events.send(ActorTeleportEvent {
        //                             tp_type: sensor.1.teleport_type.clone(),
        //                             target: Some(actor),
        //                             sender: Some(sensor),
        //                         });
        //                         actor.1.teleport_status = TeleportStatus::Requested;
        //                     }
        //                 }
        //             }
        //         }
        //         a => {
        //             if let CollisionEvent::Stopped(a, b, _flags) = event {
        //                 if *a == ac || *b == ac {
        //                     match state.teleport_status {
        //                         TeleportStatus::None => {
        //                             warn!("exited a teleporter while TeleportStatus::None")
        //                         }
        //                         TeleportStatus::Requested => {
        //                             warn!("exited while TeleportStatus::Requested, setting TeleportStatus::Teleporting");
        //                             state.teleport_status = TeleportStatus::Teleporting;
        //                         }
        //                         TeleportStatus::Teleporting => {
        //                             warn!("exited while TeleportStatus::Teleporting, setting TeleportStatus::Done");
        //                             state.teleport_status = TeleportStatus::Done;
        //                         }
        //                         TeleportStatus::Done => {
        //                             info!("exited while TeleportStatus::Done, setting TeleportStatus::None");
        //                             state.teleport_status = TeleportStatus::None;
        //                         }
        //                     }
        //                 }
        //             }

        //             match a {
        //                 TeleportStatus::Requested => {
        //                     warn!("already requested. doing nothing.")
        //                 }
        //                 TeleportStatus::Teleporting => {
        //                     state.teleport_status = TeleportStatus::Done;
        //                     warn!("already teleporting. doing nothing.")
        //                 }
        //                 TeleportStatus::Done => {
        //                     warn!("just finished. doing nothing.")
        //                 }
        //                 _ => {}
        //             }
        //             return;
        //         }
        //     }
        // }
    }
}
