//TODO: not sure how to deal with enemys being spawned in colliders. can possibly scan in each direction and move to
//whichever direction has the least amount of colliders? maybe check spawning positon for collider first, if no collider then spawn?
// after some more digging bevy_rapier has a raycast shape function, i think what i will do is raycast down on the position and check if it
// collides, if collideshape doesnt collide then spawn, if does collide pick new positon 40 or so pixels in any direction
use bevy::{math::vec2, prelude::*};
use rand::{thread_rng, Rng};

use self::{
    components::{EnemyContainerTag, SpawnEnemyEvent, SpawnWeaponEvent, Spawner, SpawnerTimer},
    zenemy_spawners::{spawn_skeleton, spawn_slime},
    zweapon_spawner::{spawn_smallpistol, spawn_smallsmg},
};
use crate::{
    launch_config::DifficultyScale,
    game::{
        actors::spawners::components::{EnemyType, WeaponType},
        AppStage,
    },
    loading::assets::ActorTextureHandles,
};

use super::ai::components::Enemy;

/// spawner components
pub mod components;
/// fn for enemys
mod zenemy_spawners;
/// fn for weapons
mod zweapon_spawner;

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
                    recieve_enemy_spawns,
                    recieve_weapon_spawns,
                    spawner_timer_system,
                ).run_if(resource_exists::<ActorTextureHandles>())
            );
    }
}

///TODO: can cause panick if spawncount is larger than 100
fn recieve_enemy_spawns(
    entity_container: Query<Entity, With<EnemyContainerTag>>,
    mut events: EventReader<SpawnEnemyEvent>,
    mut commands: Commands,
    enemyassets: Res<ActorTextureHandles>,
) {
    events.iter().for_each(|event| {
        let mut rng = thread_rng();
        info!("recieved event: {:#?}", event);
        if event.spawn_count > 100 {
            warn!("too many spawns, will likely panick, aborting");
            return;
        }

        let pos = vec2(
            event.spawn_position.x + rng.gen_range(-100.0..=100.0),
            event.spawn_position.y + rng.gen_range(-100.0..=100.0),
        );

        for _eventnum in 0..event.spawn_count {
            match event.enemy_to_spawn {
                EnemyType::Skeleton => spawn_skeleton(
                    entity_container.single(),
                    &mut commands,
                    enemyassets.as_ref(),
                    &SpawnEnemyEvent {
                        enemy_to_spawn: event.enemy_to_spawn,
                        spawn_position: pos,
                        spawn_count: event.spawn_count,
                    },
                ),
                EnemyType::Slime => spawn_slime(
                    entity_container.single(),
                    &mut commands,
                    enemyassets.as_ref(),
                    &SpawnEnemyEvent {
                        enemy_to_spawn: event.enemy_to_spawn,
                        spawn_position: pos,
                        spawn_count: event.spawn_count,
                    },
                ),
                #[allow(unreachable_patterns)]
                _ => {
                    warn!("not implemented yet")
                }
            }
        }
    });
    events.clear();
}

///TODO: can cause panick if spawncount is larger than 100 because spawning items on eachother
fn recieve_weapon_spawns(
    mut events: EventReader<SpawnWeaponEvent>,
    mut commands: Commands,
    enemyassets: Res<ActorTextureHandles>,
) {
    events.iter().for_each(|event| {
        info!("recieved event: {:#?}", event);
        if event.spawn_count > 100 {
            warn!("too many spawns, will likely panick, aborting");
            return;
        }
        match event.weapon_to_spawn {
            WeaponType::SmallSMG => {
                for _spawncount in 0..event.spawn_count {
                    spawn_smallsmg(enemyassets.to_owned(), &mut commands, event)
                }
            }
            WeaponType::SmallPistol => {
                for _spawncount in 0..event.spawn_count {
                    spawn_smallpistol(enemyassets.to_owned(), &mut commands, event)
                }
            }
            #[allow(unreachable_patterns)]
            _ => {
                warn!("not implemented yet")
            }
        }
    });
    events.clear();
}

/// creates enemy container entity, all enemys are parented to this container
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
    //         enemytype: EnemyType::Random,
    //         spawn_radius: 300.0,
    //         max_enemies: 7,
    //         randomenemy: true,
    //     },
    //     SpawnerTimer(Timer::from_seconds(5.0, TimerMode::Repeating)),
    //     Transform {
    //         translation: Vec3::new(-644.16, 2342.0, 8.0),
    //         ..default()
    //     },
    // ));
}

// TODO: add waves too spawners, variable on spawner that is wave count, initialized at value and ticks down per wave
/// spawner timer system, send spawnevents based on spawner type and timer
pub fn spawner_timer_system(
    time: Res<Time>,
    hard_settings: Res<DifficultyScale>,
    mut eventwriter: EventWriter<SpawnEnemyEvent>,
    mut spawner_query: Query<(&GlobalTransform, &Spawner, &mut SpawnerTimer), With<Spawner>>,
    all_enemys: Query<&Transform, With<Enemy>>,
) {
    if spawner_query.is_empty() {
        // warn!("No Spawners available to spawn from");
        return;
    }

    let totalenemycount = all_enemys.iter().len() as i32;
    if totalenemycount.ge(&hard_settings.max_enemies_per_room) {
        // warn!("Enemy Count is greater than or equal too total enemies allowed in game");
        return;
    }

    spawner_query.for_each_mut(|(spawner_transform, spawner_state, mut spawner_timer)| {
        if !spawner_timer.tick(time.delta()).finished() {
            return;
        }

        let mut enemys_in_spawner_area = 0;
        let mut enemy_to_spawn = EnemyType::default();

        if spawner_state.randomenemy {
            let etype: EnemyType = rand::random();

            enemy_to_spawn = etype;
        }

        all_enemys.for_each(|enemy_transform| {
            // add buffer for enemies that can maybe walk outside :/
            let distance_too_spawner = spawner_transform
                .translation()
                .truncate()
                .distance(enemy_transform.translation.truncate())
                .abs()
                - 50.0;
            if distance_too_spawner.lt(&spawner_state.spawn_radius) {
                enemys_in_spawner_area += 1;
            }
        });

        if enemys_in_spawner_area.ge(&spawner_state.max_enemies) {
            warn!("enemies in spawn area is too high");
            return;
        } //else

        eventwriter.send(SpawnEnemyEvent {
            enemy_to_spawn,
            spawn_position: (spawner_transform.translation().truncate()),
            spawn_count: 1,
        });
    });
}
