use bevy::prelude::*;
use rand::{prelude::SliceRandom, thread_rng};

use crate::{
    game::{
        characters::{components::CharacterType, creeps, EventSpawnCharacter},
        game_world::components::{CharacterSpawner, SpawnerTimer},
    },
    loading::{custom_assets::actor_definitions::CharacterDefinition, registry::ActorRegistry},
};

// TODO: add waves too spawners, variable on spawner that is wave count, initialized at value and ticks down per wave
/// spawner timer system, send `SpawnEvent` based on spawner type and timer
pub fn creep_spawner_system(
    time: Res<Time>,
    mut event_writer: EventWriter<EventSpawnCharacter>,
    mut spawner_query: Query<(
        Entity,
        &GlobalTransform,
        &CharacterSpawner,
        &mut SpawnerTimer,
        &Parent,
    )>,
    all_characters: Query<(&Transform, &CharacterType)>,
    registry: Res<ActorRegistry>,
) {
    let mut rng = thread_rng();
    for (spawner_entity, spawner_transform, spawner_state, mut spawner_timer, _entity_level) in
        &mut spawner_query
    {
        spawner_timer.tick(time.delta());
        if !spawner_timer.finished() {
            continue;
        }
        let spawner_enemies = spawner_state.enemies_too_spawn.clone();
        let enemy_type = if spawner_enemies.is_empty() {
            // choose random creep from registry
            let Some(key) = registry.characters.random_creep() else {
                error!("no keys in creep registry");
                return;
            };
            key
        } else {
            // choose random creep from spawners list of creeps
            spawner_enemies.choose(&mut rng).unwrap()
        };

        let enemies_in_spawn_area = all_characters
            .iter()
            .filter(|(_, character_type)| {
                matches!(character_type, CharacterType::Boss | CharacterType::Creep)
            })
            .filter(|(enemy_transform, _)| {
                enemy_transform
                    .translation
                    .truncate()
                    .distance(spawner_transform.translation().truncate())
                    .abs()
                    .le(&spawner_state.spawn_radius)
            })
            .count();

        if enemies_in_spawn_area
            .max(spawner_state.spawned_characters.len())
            .ge(&(spawner_state.max_enemies as usize))
        {
            continue;
        }

        event_writer.send(EventSpawnCharacter {
            spawn_data: (enemy_type.clone(), 1),
            requester: spawner_entity,
        });
    }
}

/// takes enemy spawn events and actually spawns actors in world
pub fn spawn_character_on_event(
    mut commands: Commands,
    mut character_spawn_requests: EventReader<EventSpawnCharacter>,
    global_transforms: Query<&GlobalTransform>,
    registry: Res<ActorRegistry>,
    char_assets: Res<Assets<CharacterDefinition>>,
    mut enemy_spawners: Query<&mut CharacterSpawner>,
) {
    for event in character_spawn_requests.read() {
        let Ok(requester_transform) = global_transforms.get(event.requester) else {
            error!("entity requesting teleport does not have a transform");
            continue;
        };
        let spawn_pos = requester_transform.translation().truncate();

        let Some(character_type) = registry.characters.get_character_type(&event.spawn_data.0)
        else {
            error!(
                "requested item did not exist in weapon registry: {:?}",
                event.spawn_data.0
            );
            continue;
        };

        match character_type {
            CharacterType::Creep => {
                info!("got creep character type");
                creeps::utils::spawn_creep(
                    &registry,
                    &char_assets,
                    &event.spawn_data,
                    event.requester,
                    spawn_pos,
                    &mut commands,
                    &mut enemy_spawners,
                );
            }
            CharacterType::Hero => {
                info!("got hero character type");
            }
            CharacterType::Boss => {
                info!("got boss character type");
            }
            CharacterType::Critter | CharacterType::HeroPet | CharacterType::Shopkeep => {
                info!("character type unimplemented");
            }
        }
    }
}
