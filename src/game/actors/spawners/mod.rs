//TODO: not sure how to deal with enemys being spawned in colliders. can possibly scan in each direction and move to
//whichever direction has the least amount of colliders? maybe check spawning positon for collider first, if no collider then spawn?
// after some more digging bevy_rapier has a raycast shape function, i think what i will do is raycast down on the position and check if it
// collides, if collideshape doesnt collide then spawn, if does collide pick new positon 40 or so pixels in any direction
use bevy::{math::vec3, prelude::*};
use rand::{thread_rng, Rng};

use self::{
    zenemy_spawners::{spawn_skeleton, spawn_slime},
    zweapon_spawner::{spawn_smallpistol, spawn_smallsmg},
};
use crate::{
    app_config::DifficultySettings,
    components::actors::{
        ai::AIEnemy,
        spawners::{
            EnemyContainerTag, EnemyType, SpawnEnemyEvent, SpawnWeaponEvent, Spawner, SpawnerTimer,
            WeaponType,
        },
    },
    consts::ACTOR_Z_INDEX,
    game::GameStage,
    loading::assets::ActorTextureHandles,
};

mod zenemy_spawners;
mod zweapon_spawner;

pub struct SpawnerPlugin;

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnWeaponEvent>()
            .add_event::<SpawnEnemyEvent>()
            .add_system(
                spawn_enemy_container
                    .run_if(|container_q: Query<&EnemyContainerTag>| container_q.is_empty())
                    .in_schedule(OnEnter(GameStage::PlayingGame)),
            )
            .add_systems(
                (
                    recieve_enemy_spawns,
                    recieve_weapon_spawns,
                    spawner_timer_system,
                )
                    .in_set(OnUpdate(GameStage::PlayingGame)),
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
    for event in events.iter() {
        let mut rng = thread_rng();
        let copied_event = event;
        info!("recieved event: {:#?}", event);
        if event.spawn_count > 100 {
            warn!("too many spawns, will likely panick, aborting");
            return;
        }
        let pos = vec3(
            copied_event.spawn_position.x + rng.gen_range(-300.0..=300.0),
            copied_event.spawn_position.y + rng.gen_range(-300.0..=300.0),
            ACTOR_Z_INDEX,
        );
        let new_event = SpawnEnemyEvent {
            enemy_to_spawn: event.enemy_to_spawn,
            spawn_position: pos,
            spawn_count: event.spawn_count,
        };

        match event.enemy_to_spawn {
            EnemyType::Skeleton => {
                for _eventnum in 0..event.spawn_count {
                    spawn_skeleton(
                        entity_container.single(),
                        &mut commands,
                        enemyassets.as_ref(),
                        &new_event,
                    )
                }
            }
            EnemyType::Slime => {
                for _eventnum in 0..event.spawn_count {
                    spawn_slime(
                        entity_container.single(),
                        &mut commands,
                        enemyassets.as_ref(),
                        &new_event,
                    )
                }
            }
            #[allow(unreachable_patterns)]
            _ => {
                warn!("not implemented yet")
            }
        }
        // events.clear()
    }
    events.clear();
}

///TODO: can cause panick if spawncount is larger than 100 because spawning items on eachother
fn recieve_weapon_spawns(
    mut events: EventReader<SpawnWeaponEvent>,
    mut commands: Commands,
    enemyassets: Res<ActorTextureHandles>,
) {
    for event in events.iter() {
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
    }
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

pub fn spawner_timer_system(
    time: Res<Time>,
    hard_settings: Res<DifficultySettings>,
    mut _ew: EventWriter<SpawnEnemyEvent>,
    mut spawner_query: Query<(&GlobalTransform, &Spawner, &mut SpawnerTimer), With<Spawner>>,
    all_enemys: Query<&Transform, With<AIEnemy>>,
) {
    if spawner_query.is_empty() {
        // warn!("No Spawners available to spawn from");
        return;
    }

    let totalenemycount = all_enemys.iter().len() as i32;
    if totalenemycount.ge(&hard_settings.max_enemies) {
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
                .distance(enemy_transform.translation)
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

        _ew.send(SpawnEnemyEvent {
            enemy_to_spawn,
            spawn_position: (spawner_transform.translation()),
            spawn_count: 1,
        });
    });
}
