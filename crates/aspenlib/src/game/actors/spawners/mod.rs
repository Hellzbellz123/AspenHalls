use std::str::FromStr;

//TODO: not sure how to deal with enemy's being spawned in colliders. can possibly scan in each direction and move to
//whichever direction has the least amount of colliders? maybe check spawning position for collider first, if no collider then spawn?
// after some more digging bevy_rapier has a raycast shape function, i think what i will do is raycast down on the position and check if it
// collides, if CollideShape doesn't collide then spawn, if does collide pick new position 40 or so pixels in any direction
use bevy::{math::vec2, prelude::*};
use rand::{
    prelude::{IteratorRandom, SliceRandom},
    thread_rng, Rng,
};

use self::{
    components::{EnemyContainerTag, SpawnActorEvent, Spawner, SpawnerTimer},
    spawn_functions_enemy::{spawn_skeleton, spawn_slime},
    spawn_functions_weapons::{spawn_small_pistol, spawn_small_smg},
};
use crate::{
    game::actors::spawners::components::{EnemyType, WeaponType},
    loading::assets::ActorTextureHandles,
    loading::config::DifficultyScales,
};

use super::ai::components::{ActorType, Enemy, Faction};

/// spawner components
pub mod components;
/// fn for enemy's
mod spawn_functions_enemy;
/// fn for weapons
mod spawn_functions_weapons;

/// spawner functionality
pub struct SpawnerPlugin;

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnActorEvent>().add_systems(
            Update,
            (
                spawn_enemy_container.run_if(|ect: Query<&EnemyContainerTag>| ect.is_empty()),
                receive_enemy_spawns,
                receive_weapon_spawns,
                spawner_timer_system,
            )
                .run_if(resource_exists::<ActorTextureHandles>()),
        );
    }
}

//TODO: move enemy definitions too assets
// load definitions on startup and put them in a database
// use actor spawn events too create new events
// create new weapon_spawn/actor_spawn etc systems that pull definitions from database and spawn in world

/// takes enemy spawn events and actually spawns actors in world
fn receive_enemy_spawns(
    entity_container: Query<Entity, With<EnemyContainerTag>>,
    mut events: EventReader<SpawnActorEvent>,
    mut commands: Commands,
    enemy_assets: Res<ActorTextureHandles>,
) {
    for event in events.read() {
        if event.actor_type != ActorType::Npc(Faction::Enemy) {
            return;
        }

        let mut rng = thread_rng();
        let spawn_count = if event.spawn_count > 100 {
            warn!(
                "too many {:?} spawns requested, will likely panic, aborting",
                event.actor_type
            );
            20
        } else {
            event.spawn_count
        };

        let pos = vec2(
            event.spawn_position.x + rng.gen_range(-100.0..=100.0),
            event.spawn_position.y + rng.gen_range(-100.0..=100.0),
        );

        let Ok(what_too_spawn) = EnemyType::from_str(event.what_to_spawn.as_str()) else {
            error!(
                "variant: {}, issue: spawners enemytype could not be converted too concrete type",
                event.what_to_spawn
            );
            return;
        };

        info!(
            "received enemy spawn. enemy: {}, amount: {}, pos: {}",
            what_too_spawn, spawn_count, event.spawn_position
        );
        for _spawn in 0..spawn_count {
            match what_too_spawn {
                EnemyType::Skeleton => spawn_skeleton(
                    entity_container.single(),
                    &mut commands,
                    enemy_assets.as_ref(),
                    &SpawnActorEvent {
                        spawner: event.spawner,
                        actor_type: event.actor_type,
                        what_to_spawn: event.what_to_spawn.clone(),
                        spawn_position: pos,
                        spawn_count: 1,
                    },
                ),
                EnemyType::Slime => spawn_slime(
                    entity_container.single(),
                    &mut commands,
                    enemy_assets.as_ref(),
                    &SpawnActorEvent {
                        spawner: event.spawner,
                        actor_type: event.actor_type,
                        what_to_spawn: event.what_to_spawn.clone(),
                        spawn_position: pos,
                        spawn_count: 1,
                    },
                ),
                #[allow(unreachable_patterns)]
                enemy_type => {
                    warn!("Enemy type not implemented yet: {}", enemy_type);
                }
            }
        }
    }
}

/// takes weapon spawn commands and spawns weapons in the world
fn receive_weapon_spawns(
    mut events: EventReader<SpawnActorEvent>,
    mut commands: Commands,
    enemy_assets: Res<ActorTextureHandles>,
) {
    for event in events.read() {
        info!("received event: {:#?}", event);
        if event.actor_type != ActorType::Item {
            return;
        }

        let spawn_count = if event.spawn_count > 100 {
            warn!(
                "too many {:?} spawns requested, will likely panic, aborting",
                event.actor_type
            );
            20
        } else {
            event.spawn_count
        };

        let Ok(what_too_spawn) = WeaponType::from_str(event.what_to_spawn.as_str()) else {
            error!(
                "variant: {}, issue: spawners weapontype could not be converted too concrete type",
                event.what_to_spawn
            );
            return;
        };

        info!(
            "received weapon spawn. weapon: {}, amount: {}, pos: {}",
            what_too_spawn, spawn_count, event.spawn_position
        );
        for _spawn in 0..spawn_count {
            match &what_too_spawn {
                WeaponType::SmallSMG => {
                    spawn_small_smg(enemy_assets.to_owned(), &mut commands, event);
                }
                WeaponType::SmallPistol => {
                    spawn_small_pistol(enemy_assets.to_owned(), &mut commands, event);
                }
                #[allow(unreachable_patterns)]
                weapon_type => {
                    warn!("WeaponType not implemented: {}", weapon_type);
                }
            }
        }
    }
}

/// creates enemy container entity, all enemy's are parented to this container
pub fn spawn_enemy_container(mut cmds: Commands) {
    info!("spawning enemy container");
    cmds.spawn((
        Name::new("EnemyContainer"),
        EnemyContainerTag,
        SpatialBundle {
            visibility: Visibility::Inherited,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
    ));
}

// TODO: add waves too spawners, variable on spawner that is wave count, initialized at value and ticks down per wave
/// spawner timer system, send `SpawnEvent` based on spawner type and timer
pub fn spawner_timer_system(
    time: Res<Time>,
    hard_settings: Res<DifficultyScales>,
    mut event_writer: EventWriter<SpawnActorEvent>,
    mut spawner_query: Query<(Entity, &GlobalTransform, &mut Spawner, &mut SpawnerTimer)>,
    all_enemies: Query<&Transform, With<Enemy>>,
) {
    if spawner_query.is_empty() {
        // warn!("No Spawners available to spawn from");
        return;
    }

    let total_enemy_count = i32::try_from(all_enemies.iter().len()).unwrap_or_else(|x| {
        warn!("{x:?}");
        1_000_000
    });

    if total_enemy_count.ge(&(hard_settings.max_enemies_per_room * 25)) {
        // warn!("Enemy Count is greater than or equal too total enemies allowed in game");
        return;
    }

    for (spawner_entity, spawner_transform, spawner_state, mut spawner_timer) in &mut spawner_query
    {
        spawner_timer.tick(time.delta());

        if !spawner_timer.finished() {
            continue;
        }

        let mut enemies_in_spawner_area = 0;
        let enemies = spawner_state.enemies_too_spawn.clone();
        let mut rng = thread_rng();
        let enemy_type: String = if enemies.is_empty() {
            warn!("No enemies for this spawner. Generating random enemy");
            let et: EnemyType = rng.gen();
            et.to_string()
        } else {
            warn!("Selecting random enemy too spawn from this spawners Enemy List");
            enemies.choose(&mut rng).unwrap().clone()
        };

        for enemy_transform in &all_enemies {
            let distance_too_spawner = spawner_transform
            .translation()
            .truncate()
            .distance(enemy_transform.translation.truncate())
            .abs()
            // add buffer for enemies that can maybe walk outside :/
                - 50.0;
            if distance_too_spawner.lt(&spawner_state.spawn_radius) {
                enemies_in_spawner_area += 1;
            }
        }

        if enemies_in_spawner_area.ge(&spawner_state.max_enemies) {
            warn!("enemies in spawn area is too high");
            return;
        }

        event_writer.send(SpawnActorEvent {
            spawner: Some(spawner_entity),
            actor_type: ActorType::Npc(Faction::Enemy),
            what_to_spawn: enemy_type.to_string(),
            spawn_position: (spawner_transform.translation().truncate()),
            spawn_count: 1,
        });
    }
}
