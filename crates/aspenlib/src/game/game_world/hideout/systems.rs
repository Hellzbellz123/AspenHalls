use bevy::prelude::*;
use bevy_ecs_ldtk::{
    prelude::{LevelSet, SpawnExclusions},
    IntGridRendering, LdtkSettings, LdtkWorldBundle, LevelBackground, LevelSelection,
    LevelSpawnBehavior, SetClearColor,
};
use bevy_rapier2d::{
    geometry::Collider,
    prelude::{CollisionEvent, Sensor},
    rapier::geometry::CollisionEventFlags,
};

use crate::{
    game::{
        characters::components::{CharacterMoveState, CharacterType, TeleportStatus},
        components::ActorColliderType,
        game_world::components::{ActorTeleportEvent, Teleporter},
    },
    loading::assets::AspenMapHandles,
    utilities::collision_to_data,
};

/// tag for map entity
#[derive(Debug, Component, Clone, Copy, Reflect, Default)]
#[reflect(Component)]
pub struct HideoutTag;

/// spawns hideout and related resources
pub fn spawn_world_container(mut commands: Commands, maps: Res<AspenMapHandles>) {
    info!("spawning LdtkWorldBundle");
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

    commands.spawn((
        LdtkWorldBundle {
            ldtk_handle: maps.default_levels.clone(),
            level_set: LevelSet::default(),
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
        Name::new("HideOut"),
        HideoutTag,
    ));
}

/// system too check for actors on teleport pad
pub fn teleporter_collisions(
    mut collision_events: EventReader<CollisionEvent>,
    mut teleport_events: EventWriter<ActorTeleportEvent>,
    mut characters: Query<(&mut CharacterMoveState, &CharacterType)>,
    actor_colliders: Query<(Entity, &Parent, &ActorColliderType), With<Collider>>,
    teleporter: Query<(Entity, &Teleporter), With<Sensor>>,
) {
    for event in &mut collision_events.read() {
        let (collider_a, collider_b, flags, is_start_event) = collision_to_data(event);
        if !flags.contains(CollisionEventFlags::SENSOR) {
            continue;
        }

        let Some((teleporter, tp_data)) = teleporter
            .iter()
            .find(|(f, _)| *f == collider_a || *f == collider_b)
        else {
            return;
        };

        let Some(character) = actor_colliders
            .iter()
            .filter(|(_, _, at)| at == &&ActorColliderType::Character)
            .find_map(|(character_collider, parent, _)| {
                if character_collider == collider_a || character_collider == collider_b {
                    Some(parent.get())
                } else {
                    None
                }
            })
        else {
            return;
        };
        let Ok((mut character_movestate, character_type)) = characters.get_mut(character) else {
            return;
        };

        info!("got teleporter collision");
        if !is_start_event {
            match character_movestate.teleport_status {
                TeleportStatus::None => {
                    warn!("exited teleporter with 'none'");
                }
                TeleportStatus::Requested => {
                    warn!("exited teleporter with 'requested'");
                }
                TeleportStatus::Teleporting => {
                    warn!("exited teleporter with 'teleporting'");
                    character_movestate.teleport_status = TeleportStatus::Done;
                }
                TeleportStatus::Done => {
                    warn!("exited teleporter with 'done'");
                    character_movestate.teleport_status = TeleportStatus::None;
                }
            }
        } else if character_movestate.teleport_status == TeleportStatus::None && is_start_event {
            if character_type != &CharacterType::Hero {
                warn!("teleporter should only be triggered by the player");
                return;
            }

            teleport_events.send(ActorTeleportEvent {
                tp_type: tp_data.effect.clone(),
                target: Some(character),
                sender: Some(teleporter),
            });
            character_movestate.teleport_status = TeleportStatus::Requested;
            warn!("requesting teleport");
            return;
        }
    }
}
