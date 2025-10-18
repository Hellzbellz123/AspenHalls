use avian2d::prelude::{Collider, CollisionEnded, CollisionStarted, Sensor};
use bevy::prelude::*;
use bevy_ecs_ldtk::{
    prelude::{LevelSet, SpawnExclusions}, IntGridRendering, LdtkProjectHandle, LdtkSettings, LdtkWorldBundle, LevelBackground, LevelSelection, LevelSpawnBehavior, SetClearColor
};

use crate::{
    game::{
        characters::components::{CharacterMoveState, CharacterType, TeleportStatus},
        components::ActorColliderType,
        game_world::components::{ActorTeleportEvent, Teleporter},
    },
    loading::assets::AspenLevelsetHandles,
};

/// tag for map entity
#[derive(Debug, Component, Clone, Copy, Reflect, Default)]
#[reflect(Component)]
pub struct HideoutTag;

/// spawns hideout and related resources
pub fn spawn_hideout(mut commands: Commands, maps: Res<AspenLevelsetHandles>) {
    // TODO: use available levelset.hideout too spawn dungeon.
    // how do i get the handle for any given LdtkLevels' project?

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
            ldtk_handle: LdtkProjectHandle { handle: maps.default_levels.clone() },
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
    mut collision_start_events: EventReader<CollisionStarted>,
    // collision_end_events: EventReader<CollisionEnded>,
    mut teleport_events: EventWriter<ActorTeleportEvent>,
    mut characters: Query<(&mut CharacterMoveState, &CharacterType)>,
    actor_colliders: Query<(Entity, &ChildOf, &ActorColliderType), With<Collider>>,
    teleporter: Query<(Entity, &Teleporter), With<Sensor>>,
) {
    // NOTE: we are explicitly using returns instead of continue in an effort too prevent
    // multiple teleport events from triggering for the same entity at once
    // there is also a rudimentary teleport statemachine held on the `CharacterMoveState`
    for event in &mut collision_start_events.read() {
        info!("got collision");
        let CollisionStarted(collider_a, collider_b) = *event;

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
                    Some(parent.parent())
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
            if character_type != &CharacterType::Hero {
                warn!("teleporter should only be triggered by the player");
                return;
            }
            teleport_events.write(ActorTeleportEvent {
                tp_type: tp_data.effect.clone(),
                target: Some(character),
                sender: Some(teleporter),
            });
    }
}
