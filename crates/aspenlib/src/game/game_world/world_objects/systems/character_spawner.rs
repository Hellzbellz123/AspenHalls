use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::EntityIid;

use crate::{
    consts::CHARACTER_SPAWNERS_DISABLED,
    game::{
        characters::{components::CharacterType, EventSpawnCharacter},
        game_world::components::{CharacterSpawner, SpawnerTimer, SpawnerWave},
    },
    loading::registry::ActorRegistry,
};

// TODO: get waves from parent (spawner parent is entity layer),
// spawn each in wave and then remove wave, if no more waves then spawner is `empty` variable on spawner that is wave count, initialized at value and ticks down per wave
/// spawner timer system, send `SpawnEvent` based on spawner type and timer
pub fn character_spawners_system(
    mut cmds: Commands,
    time: Res<Time>,
    mut event_writer: EventWriter<EventSpawnCharacter>,
    mut spawner_query: Query<(
        Entity,
        &GlobalTransform,
        &mut CharacterSpawner,
        &mut SpawnerTimer,
        &Parent,
    )>,
    spawner_waves: Query<(Entity, &EntityIid, &SpawnerWave)>,
    all_characters: Query<(&Transform, &CharacterType)>,
    actor_registry: Res<ActorRegistry>,
) {
    if CHARACTER_SPAWNERS_DISABLED {
        error_once!("Character spawning disabled");
        return;
    }

    for (spawner_entity, spawner_transform, mut spawner_state, mut spawner_timer, _entity_level) in
        &mut spawner_query
    {
        spawner_timer.tick(time.delta());
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

        if !spawner_timer.finished() // if spawner hasnt ticked
            || (spawner_state.waves.is_empty() && !spawner_state.random_wave) // spawner has used all waves and isnt infinite
            || enemies_in_spawn_area // spawner has required amount of enemies near it
                .max(spawner_state.spawned_characters.len())
                .ge(&(spawner_state.max_spawned as usize))
        {
            // skip this spawner
            continue;
        }

        if spawner_state.random_wave {
            // get random entity from actor registry
            let wave = actor_registry.characters.random_creep_wave();
            for iid in wave {
                event_writer.send(EventSpawnCharacter {
                    identifier: iid.clone(),
                    requester: spawner_entity,
                });
            }
            continue;
        }

        let spawn_wave_id = spawner_state.waves.swap_remove(0).entity_iid;
        let (wave_ent, _, wave) = spawner_waves
            .iter()
            .find(|f| **f.1 == spawn_wave_id)
            .expect("wave did not exist in world");

        for iid in &wave.too_spawn {
            event_writer.send(EventSpawnCharacter {
                identifier: iid.clone(),
                requester: spawner_entity,
            });
        }
        cmds.entity(wave_ent).despawn_recursive();
    }
}
