use bevy::prelude::*;
use bevy_ecs_ldtk::{
    IntGridRendering, LdtkSettings, LdtkWorldBundle, LevelBackground, LevelSelection,
    LevelSpawnBehavior, SetClearColor,
};
use bevy_rapier2d::prelude::CollisionEvent;

use crate::{
    game::{
        actors::components::{Player, PlayerColliderTag},
        game_world::hideout::ActorTeleportEvent,
        // game_world::dungeonator::GeneratorStage,
    },
    loading::assets::MapAssetHandles,
};

use super::map_components::{Teleporter, TeleportTimer};

/// tag for map entity
#[derive(Debug, Component, Clone, Copy, Reflect, Default)]
#[reflect(Component)]
pub struct MapContainerTag;

/// spawns hideout and related resources
pub fn spawn_hideout(mut commands: Commands, maps: Res<MapAssetHandles>) {
    info!("spawning LdtkWorldBundle");

    commands.spawn((
        LdtkWorldBundle {
            ldtk_handle: maps.start_level.clone(),
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

    commands.insert_resource(TeleportTimer {
        timer: Timer::from_seconds(2.0, TimerMode::Once),
    });
    commands.insert_resource(LevelSelection::Identifier("TestingHall".to_string()));
    commands.insert_resource(LdtkSettings {
        level_spawn_behavior: LevelSpawnBehavior::UseZeroTranslation,
        set_clear_color: SetClearColor::No,
        int_grid_rendering: IntGridRendering::Invisible,
        level_background: LevelBackground::Nonexistent,
    });
}

/// system too check for player on teleport pad
pub fn teleporter_collisions(
    mut collision_events: EventReader<CollisionEvent>,
    mut teleport_events: EventWriter<ActorTeleportEvent>,
    world_sensors: Query<(Entity, &Teleporter)>,
    player_collider_query: Query<Entity, With<PlayerColliderTag>>,
    player_query: Query<Entity, With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }
    let player = player_query.single();
    let pc = player_collider_query.single();

    // TODO: check TeleportStatus if we are allowed too send this teleport
    // or on the EventReader side, get status and return with warning
    for event in &mut collision_events.read() {
        if let CollisionEvent::Started(a, b, _flags) = event {
            if *a == pc || *b == pc {
                if let Some((sensor, teleporter)) = world_sensors
                    .iter()
                    .find(|&(sensor, _)| sensor == *a || sensor == *b)
                {
                    info!("player and teleporter are colliding, sending teleport event");
                    teleport_events.send(ActorTeleportEvent {
                        tp_type: teleporter.teleport_type.clone(),
                        target: Some(player),
                        sender: Some(sensor),
                    });
                }
            }
        }
    }
    collision_events.clear();
}
