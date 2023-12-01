use bevy::prelude::*;
use bevy_ecs_ldtk::{
    IntGridRendering, LdtkSettings, LdtkWorldBundle, LevelBackground, LevelSelection,
    LevelSpawnBehavior, SetClearColor,
};
use bevy_rapier2d::prelude::CollisionEvent;

use crate::{
    game::{
        actors::components::{Player, PlayerColliderTag},
        game_world::hideout::PlayerTeleportEvent,
        // game_world::dungeonator::GeneratorStage,
    },
    loading::assets::MapAssetHandles,
};

use super::map_components::{SanctuaryTeleportSensor, TeleportTimer};

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
    commands.insert_resource(LevelSelection::Identifier("Sanctuary".to_string()));
    commands.insert_resource(LdtkSettings {
        level_spawn_behavior: LevelSpawnBehavior::UseZeroTranslation,
        set_clear_color: SetClearColor::No,
        int_grid_rendering: IntGridRendering::Invisible,
        level_background: LevelBackground::Nonexistent,
    });
}

/// system too check for player on teleport pad
pub fn home_world_teleporter_collisions(
    mut collision_events: EventReader<CollisionEvent>,
    mut teleport_events: EventWriter<PlayerTeleportEvent>,
    world_sensors: Query<Entity, With<SanctuaryTeleportSensor>>,
    player_collider_query: Query<Entity, With<PlayerColliderTag>>,
    player_query: Query<&Player>,
) {
    if player_query.is_empty() {
        return;
    }
    let pc = player_collider_query.single();

    for event in &mut collision_events.read() {
        if let CollisionEvent::Started(a, b, _flags) = event {
            if *a == pc || *b == pc {
                if let Some(_sensor) = world_sensors
                    .iter()
                    .find(|&sensor| sensor == *a || sensor == *b)
                {
                    teleport_events.send(PlayerTeleportEvent {
                        tp_action: "StartDungeonGen".into(),
                    });
                    info!("player and sensor are colliding, sending teleport event");
                }
            }
        }
        if let CollisionEvent::Stopped(a, b, _flags) = event {
            if pc == *a || pc == *b {
                if let Some(_sensor) = world_sensors
                    .iter()
                    .find(|&sensor| sensor == *a || sensor == *b)
                {
                    info!("player and sensor stopped colliding, sending teleport event");
                }
            }
        }
    }
    collision_events.clear();
}

// acts on player standing on pad for required time
// pub fn enter_the_dungeon(
//     // mut commands: Commands,
//     time: Res<Time>,
//     mut teleport_timer: ResMut<TeleportTimer>,
//     mut player_query: Query<(&Transform, &mut Player)>,
//     _sanctuary_container: Query<Entity, With<MapContainerTag>>,
// ) {
//     let (_player_transform, mut player) = player_query
//         .get_single_mut()
//         .expect("should always be a player if we are getting the event");

//     if !player.wants_to_teleport {
//         teleport_timer.reset();
//         player.enter_dungeon_requested = false;
//     }

//     if !teleport_timer.finished() & player.wants_to_teleport {
//         info!("timer not done, ticking timer");
//         teleport_timer.tick(time.delta());
//         if teleport_timer.finished() && !player.enter_dungeon_requested {
//             // commands.insert_resource(NextState(Some(GeneratorStage::Initialization)));
//             info!("Starting Dungeon Generation");
//             player.enter_dungeon_requested = true;
//         }
//     } else if player.enter_dungeon_requested {
//         player.wants_to_teleport = false;
//         player.enter_dungeon_requested = false;
//         teleport_timer.reset();
//     }
// }
