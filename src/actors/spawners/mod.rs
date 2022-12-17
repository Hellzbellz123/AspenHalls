//TODO: not sure how to deal with enemys being spawned in colliders. can possibly scan in each direction and move to
//whichever direction has the least amount of colliders? maybe check spawning positon for collider first, if no collider then spawn?

// after some more digging bevy_rapier has a raycast shape function, i think what i will do is raycast down on the position and check if it
// collides, if collideshape doesnt collide then spawn, if does collide pick new positon 40 or so pixels in any direction
use bevy::{prelude::*, time::Timer};

use crate::{
    actors::spawners::{
        zenemy_spawners::{spawn_skeleton, spawn_slime},
        zweapon_spawner::{spawn_smallpistol, spawn_smallsmg},
    },
    components::actors::{
        ai::AIEnemy,
        spawners::{
            EnemyContainerTag, EnemyType, SpawnEnemyEvent, SpawnWeaponEvent, Spawner, SpawnerTimer,
            WeaponType,
        },
    },
    game::GameStage,
    loading::assets::ActorTextureHandles,
    utilities::game::{SystemLabels, MAX_ENEMIES},
};

mod zenemy_spawners;
mod zweapon_spawner;

pub struct SpawnerPlugin;

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnWeaponEvent>()
            .add_event::<SpawnEnemyEvent>()
            .add_system_set(
                SystemSet::on_enter(GameStage::Playing)
                    .with_system(on_enter)
                    .label(SystemLabels::Spawn),
            )
            .add_system_set(
                SystemSet::on_update(GameStage::Playing)
                    .with_system(recieve_enemy_spawns)
                    .with_system(recieve_weapon_spawns)
                    .with_system(spawn_timer_system),
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
        info!("recieved event: {:#?}", event);
        if event.spawn_count > 100 {
            warn!("too many spawns, will likely panick, aborting");
            return;
        }
        match event.enemy_to_spawn {
            EnemyType::Skeleton => {
                for _eventnum in 0..event.spawn_count {
                    spawn_skeleton(
                        entity_container.single(),
                        &mut commands,
                        enemyassets.as_ref(),
                        event,
                    )
                }
            }
            EnemyType::Slime => {
                for _eventnum in 0..event.spawn_count {
                    spawn_slime(
                        entity_container.single(),
                        &mut commands,
                        enemyassets.as_ref(),
                        event,
                    )
                }
            }
            #[allow(unreachable_patterns)]
            _ => {
                warn!("not implemented yet")
            }
        }
    }
}

///TODO: can cause panick if spawncount is larger than 100
fn recieve_weapon_spawns(
    mut events: EventReader<SpawnWeaponEvent>,
    mut commands: Commands,
    enemyassets: Res<ActorTextureHandles>,
) {
    for event in events.iter() {
        info!("recieved event: {:#?}", event);
        match event.weapon_to_spawn {
            WeaponType::SmallSMG => {
                for _spawncount in 0..event.spawn_count {
                    if event.spawn_count > 100 {
                        warn!("too many spawns, will likely panick, aborting");
                        return;
                    }
                    spawn_smallsmg(enemyassets.to_owned(), &mut commands, event)
                }
            }
            WeaponType::SmallPistol => {
                for _spawncount in 0..event.spawn_count {
                    if event.spawn_count > 100 {
                        warn!("too many spawns, will likely panick, aborting");
                        return;
                    }
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
pub fn on_enter(mut cmds: Commands) {
    info!("spawning enemy container");
    cmds.spawn((
        Name::new("EnemyContainer"),
        EnemyContainerTag,
        SpatialBundle {
            visibility: Visibility::VISIBLE,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
    ));

    //TODO: make this an entity thats placed in ldtk.
    info!("spawning enemy spawners");
    cmds.spawn((
        Name::new("SpawnerOutside"),
        Spawner {
            enemytype: EnemyType::Skeleton,
            spawn_radius: 300.0,
            max_enemies: 7,
        },
        SpawnerTimer(Timer::from_seconds(5.0, TimerMode::Repeating)),
        Transform {
            translation: Vec3::new(-644.16, 2342.0, 8.0),
            ..default()
        },
    ));
}

pub fn spawn_timer_system(
    mut ew: EventWriter<SpawnEnemyEvent>,
    spawner_query: Query<(&Transform, &Spawner), With<Spawner>>,
    enemy_count: Query<(Entity,), With<AIEnemy>>,
) {
    if enemy_count.iter().len() >= MAX_ENEMIES {
        return;
    }
    for (transform, spawner) in spawner_query.iter() {
        for _enemy_to_spawn in 0..spawner.max_enemies {
            ew.send(SpawnEnemyEvent {
                enemy_to_spawn: spawner.enemytype,
                spawn_position: (transform.translation),
                spawn_count: 1,
            });
        }
    }
}
