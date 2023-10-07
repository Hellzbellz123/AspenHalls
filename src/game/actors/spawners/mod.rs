//TODO: not sure how to deal with enemy's being spawned in colliders. can possibly scan in each direction and move to
//whichever direction has the least amount of colliders? maybe check spawning position for collider first, if no collider then spawn?
// after some more digging bevy_rapier has a raycast shape function, i think what i will do is raycast down on the position and check if it
// collides, if CollideShape doesn't collide then spawn, if does collide pick new position 40 or so pixels in any direction
use bevy::{math::vec2, prelude::*};
use rand::{thread_rng, Rng};

use self::{
    components::{EnemyContainerTag, SpawnEnemyEvent, SpawnWeaponEvent, Spawner, SpawnerTimer},
    z_enemy_spawners::{spawn_skeleton, spawn_slime},
    z_weapon_spawner::{spawn_small_pistol, spawn_small_smg},
};
use crate::{
    loading::config::DifficultyScale,
    game::actors::spawners::components::{EnemyType, WeaponType},
    loading::assets::ActorTextureHandles,
};

use super::ai::components::Enemy;

/// spawner components
pub mod components;
/// fn for enemy's
mod z_enemy_spawners;
/// fn for weapons
mod z_weapon_spawner;

/// spawner functionality
pub struct SpawnerPlugin;

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnWeaponEvent>()
            .add_event::<SpawnEnemyEvent>()
            .add_systems(
                Update,
                (
                    spawn_enemy_container.run_if(|ect: Query<&EnemyContainerTag>| ect.is_empty()),//run_once()),
                    receive_enemy_spawns,
                    receive_weapon_spawns,
                    spawner_timer_system,
                ).run_if(resource_exists::<ActorTextureHandles>())
            );
    }
}

///TODO: can cause panic if spawn count is larger than 100
fn receive_enemy_spawns(
    entity_container: Query<Entity, With<EnemyContainerTag>>,
    mut events: EventReader<SpawnEnemyEvent>,
    mut commands: Commands,
    enemy_assets: Res<ActorTextureHandles>,
) {
    events.iter().for_each(|event| {
        let mut rng = thread_rng();
        info!("received event: {:#?}", event);
        if event.spawn_count > 100 {
            warn!("too many spawns, will likely panic, aborting");
            return;
        }

        let pos = vec2(
            event.spawn_position.x + rng.gen_range(-100.0..=100.0),
            event.spawn_position.y + rng.gen_range(-100.0..=100.0),
        );

        for _event_num in 0..event.spawn_count {
            match event.enemy_to_spawn {
                EnemyType::Skeleton => spawn_skeleton(
                    entity_container.single(),
                    &mut commands,
                    enemy_assets.as_ref(),
                    &SpawnEnemyEvent {
                        enemy_to_spawn: event.enemy_to_spawn,
                        spawn_position: pos,
                        spawn_count: event.spawn_count,
                    },
                ),
                EnemyType::Slime => spawn_slime(
                    entity_container.single(),
                    &mut commands,
                    enemy_assets.as_ref(),
                    &SpawnEnemyEvent {
                        enemy_to_spawn: event.enemy_to_spawn,
                        spawn_position: pos,
                        spawn_count: event.spawn_count,
                    },
                ),
                #[allow(unreachable_patterns)]
                _ => {
                    warn!("not implemented yet");
                }
            }
        }
    });
    events.clear();
}

///TODO: can cause panic if spawn count is larger than 100 because spawning items on each other
fn receive_weapon_spawns(
    mut events: EventReader<SpawnWeaponEvent>,
    mut commands: Commands,
    enemy_assets: Res<ActorTextureHandles>,
) {
    events.iter().for_each(|event| {
        info!("received event: {:#?}", event);
        if event.spawn_count > 100 {
            warn!("too many spawns, will likely panic, aborting");
            return;
        }
        match event.weapon_to_spawn {
            WeaponType::SmallSMG => {
                for _spawn_count in 0..event.spawn_count {
                    spawn_small_smg(enemy_assets.to_owned(), &mut commands, event);
                }
            }
            WeaponType::SmallPistol => {
                for _spawn_count in 0..event.spawn_count {
                    spawn_small_pistol(enemy_assets.to_owned(), &mut commands, event);
                }
            }
            #[allow(unreachable_patterns)]
            _ => {
                warn!("not implemented yet");
            }
        }
    });
    events.clear();
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

    //TODO: make this an entity thats placed in ldtk.
    // info!("spawning enemy spawners");
    // cmds.spawn((
    //     Name::new("SpawnerOutside"),
    //     Spawner {
    //         enemy_type: EnemyType::Random,
    //         spawn_radius: 300.0,
    //         max_enemies: 7,
    //         random_enemy: true,
    //     },
    //     SpawnerTimer(Timer::from_seconds(5.0, TimerMode::Repeating)),
    //     Transform {
    //         translation: Vec3::new(-644.16, 2342.0, 8.0),
    //         ..default()
    //     },
    // ));
}

// TODO: add waves too spawners, variable on spawner that is wave count, initialized at value and ticks down per wave
/// spawner timer system, send `SpawnEvent` based on spawner type and timer
pub fn spawner_timer_system(
    time: Res<Time>,
    hard_settings: Res<DifficultyScale>,
    mut event_writer: EventWriter<SpawnEnemyEvent>,
    mut spawner_query: Query<(&GlobalTransform, &Spawner, &mut SpawnerTimer), With<Spawner>>,
    all_enemies: Query<&Transform, With<Enemy>>,
) {
    if spawner_query.is_empty() {
        // warn!("No Spawners available to spawn from");
        return;
    }

    let total_enemy_count = i32::try_from(all_enemies.iter().len()).unwrap_or_else(|x| {warn!("{x:?}"); 1_000_000});

    if total_enemy_count.ge(&hard_settings.max_enemies_per_room) {
        // warn!("Enemy Count is greater than or equal too total enemies allowed in game");
        return;
    }

    spawner_query.for_each_mut(|(spawner_transform, spawner_state, mut spawner_timer)| {
        if !spawner_timer.tick(time.delta()).finished() {
            return;
        }

        let mut enemies_in_spawner_area = 0;

        let enemy_type: EnemyType = rand::random();
        let enemy_to_spawn = if spawner_state.random_enemy { enemy_type } else { EnemyType::default() };

        all_enemies.for_each(|enemy_transform| {
            // add buffer for enemies that can maybe walk outside :/
            let distance_too_spawner = spawner_transform
                .translation()
                .truncate()
                .distance(enemy_transform.translation.truncate())
                .abs()
                - 50.0;
            if distance_too_spawner.lt(&spawner_state.spawn_radius) {
                enemies_in_spawner_area += 1;
            }
        });

        if enemies_in_spawner_area.ge(&spawner_state.max_enemies) {
            warn!("enemies in spawn area is too high");
            return;
        } //else

        event_writer.send(SpawnEnemyEvent {
            enemy_to_spawn,
            spawn_position: (spawner_transform.translation().truncate()),
            spawn_count: 1,
        });
    });
}
