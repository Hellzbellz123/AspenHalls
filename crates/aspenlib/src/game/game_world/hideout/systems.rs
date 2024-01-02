use bevy::prelude::*;
use bevy_ecs_ldtk::{
    prelude::SpawnExclusions,
    IntGridRendering, LdtkSettings, LdtkWorldBundle, LevelBackground, LevelSelection,
    LevelSpawnBehavior, SetClearColor,
};
use bevy_rapier2d::prelude::{CollisionEvent, Sensor};

use crate::{
    game::{
        actors::components::{ActorColliderType, ActorMoveState, TeleportStatus},
        game_world::components::{ActorTeleportEvent, Teleporter},
        // game_world::dungeonator::GeneratorStage,
    },
    loading::assets::AspenMapHandles, prelude::game::ActorType,
};

/// tag for map entity
#[derive(Debug, Component, Clone, Copy, Reflect, Default)]
#[reflect(Component)]
pub struct MapContainerTag;

/// spawns hideout and related resources
pub fn spawn_hideout(mut commands: Commands, maps: Res<AspenMapHandles>) {
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

    #[cfg(not(feature = "develop"))]
    //TODO: use level progress for this?
    // probably not needed as this is first actual spawn of hideout.
    // unless loading a save then we need too account for progress
    let identifier = "HideoutL1".to_string();

    #[cfg(feature = "develop")]
    let identifier = "TestingHalls".to_string();

    // TODO match on saved state/player progress
    commands.insert_resource(LevelSelection::Identifier(identifier));
    commands.insert_resource(LdtkSettings {
        exclusions: SpawnExclusions::default(),
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
    mut actors: Query<(Entity, &mut ActorMoveState, &ActorType)>,
    sensors: Query<(Entity, &Teleporter), With<Sensor>>,
    parents: Query<&Parent, With<ActorColliderType>>,
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
            if sensor.1.effect.is_event() && !actor.2.is_hero() {
                warn!("events should only be triggered by the player");
                return;
            }

            teleport_events.send(ActorTeleportEvent {
                tp_type: sensor.1.effect.clone(),
                target: Some(actor.0),
                sender: Some(sensor.0),
            });
            actor.1.teleport_status = TeleportStatus::Requested;
            warn!("requesting teleport");
            return;
        }
    }
}
