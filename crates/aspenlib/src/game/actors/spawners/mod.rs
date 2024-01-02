//TODO: not sure how to deal with enemy's being spawned in colliders. can possibly scan in each direction and move to
//whichever direction has the least amount of colliders? maybe check spawning position for collider first, if no collider then spawn?
// after some more digging bevy_rapier has a raycast shape function, i think what i will do is raycast down on the position and check if it
// collides, if CollideShape doesn't collide then spawn, if does collide pick new position 40 or so pixels in any direction
use bevy::prelude::*;
use bevy_rapier2d::geometry::{Collider, CollisionGroups};
use rand::{
    prelude::{IteratorRandom, SliceRandom},
    thread_rng, Rng,
};

use self::components::{EnemyContainerTag, EnemySpawner, SpawnActorEvent, SpawnerTimer};
use crate::{
    bundles::{CharacterBundle, ObjectColliderBundle},
    consts::{actor_collider, AspenCollisionLayer, ACTOR_PHYSICS_Z_INDEX, ACTOR_Z_INDEX},
    game::actors::{
        ai::components::{ActorType, NpcType},
        components::ActorColliderType,
    },
    loading::{custom_assets::actor_definitions::CharacterDefinition, registry::ActorRegistry},
    AppState,
};

/// spawner components
pub mod components;

/// spawner functionality
pub struct SpawnerPlugin;

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnActorEvent>().add_systems(
            Update,
            (
                spawn_enemy_container.run_if(|ect: Query<&EnemyContainerTag>| ect.is_empty()),
                (spawn_creeps_on_event, spawn_weapon_on_event),
                enemy_spawner_system.run_if(any_with_component::<EnemySpawner>()),
            )
                .run_if(state_exists_and_equals(AppState::PlayingGame)),
        );
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
pub fn enemy_spawner_system(
    time: Res<Time>,
    mut event_writer: EventWriter<SpawnActorEvent>,
    mut spawner_query: Query<(
        Entity,
        &GlobalTransform,
        &EnemySpawner,
        &mut SpawnerTimer,
        &Parent,
    )>,
    all_characters: Query<(&Transform, &ActorType)>,
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
            let Some(key) = registry
                .characters
                .creeps
                .clone()
                .into_keys()
                .choose(&mut rng)
            else {
                error!("no keys in creep registry");
                return;
            };
            key
            // choose random creep from spawners list of creeps
        } else {
            spawner_enemies.choose(&mut rng).unwrap().clone()
        };

        let enemies_in_spawn_area = all_characters
            .iter()
            .filter(|(_, character_type)| match character_type {
                ActorType::Hero | ActorType::Weapon | ActorType::Item => false,
                ActorType::Npc(char_type) => match char_type {
                    NpcType::Critter | NpcType::Friendly | NpcType::Minion => false,
                    NpcType::Boss | NpcType::Creep => true,
                },
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
            .max(spawner_state.spawned_enemies.len())
            .ge(&(spawner_state.max_enemies as usize))
        {
            continue;
        }
        event_writer.send(SpawnActorEvent {
            who_spawned: Some(spawner_entity),
            actor_type: ActorType::Npc(NpcType::Creep),
            what_to_spawn: enemy_type,
            spawn_position: (spawner_transform.translation().truncate()),
            spawn_count: 1,
        });
    }
}

/// takes enemy spawn events and actually spawns actors in world
fn spawn_creeps_on_event(
    enemy_container: Query<Entity, With<EnemyContainerTag>>,
    mut events: EventReader<SpawnActorEvent>,
    registry: Res<ActorRegistry>,
    mut commands: Commands,
    char_assets: Res<Assets<CharacterDefinition>>,
    mut enemy_spawners: Query<&mut EnemySpawner>,
) {
    for event in events.read() {
        if !event.actor_type.is_creep() {
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

        info!(
            "received enemy spawn. enemy: {:?}, amount: {}, pos: {}",
            event.what_to_spawn, spawn_count, event.spawn_position
        );

        let Some(registerd_actor_bundle) = registry.characters.creeps.get(&event.what_to_spawn)
        else {
            error!(
                "could not get ActorBundle from ActorRegistry: {:?}",
                event.what_to_spawn
            );
            return;
        };
        let mut bundle_copy: CharacterBundle = registerd_actor_bundle.clone();
        let (_, char_def) = char_assets
            .iter()
            .find(|(_, asset)| asset.actor.identifier == event.what_to_spawn)
            .expect("Spawned characters asset definition did not exist");

        commands
            .entity(enemy_container.single())
            .with_children(|f| {
                for _ in 0..event.spawn_count {
                    let pos = Vec2 {
                        x: event.spawn_position.x + rng.gen_range(-100.0..=100.0),
                        y: event.spawn_position.y + rng.gen_range(-100.0..=100.0),
                    };
                    bundle_copy.aseprite.sprite_bundle.transform =
                        Transform::from_xyz(pos.x, pos.y, ACTOR_Z_INDEX);
                    f.spawn(bundle_copy.clone()).with_children(|child| {
                        let collider_name =
                            format!("{}Collider", bundle_copy.name.clone().as_str());
                        let spawned_enemy = child
                            .spawn((ObjectColliderBundle {
                                tag: ActorColliderType::Character,
                                name: Name::new(collider_name),
                                transform_bundle: TransformBundle {
                                    local: (Transform {
                                        translation: (Vec3 {
                                            x: 0.0,
                                            y: 0.0,
                                            z: ACTOR_PHYSICS_Z_INDEX,
                                        }),
                                        ..default()
                                    }),
                                    ..default()
                                },
                                collider: actor_collider(char_def.actor.pixel_size),
                                collision_groups: CollisionGroups {
                                    memberships: AspenCollisionLayer::ACTOR,
                                    filters: AspenCollisionLayer::EVERYTHING,
                                },
                            },))
                            .id();

                        if let Some(ent) = event.who_spawned {
                            if let Ok(mut spawner_state) = enemy_spawners.get_mut(ent) {
                                spawner_state.spawned_enemies.push(spawned_enemy);
                            }
                        }
                    });
                }
            });
    }
}

/// takes weapon spawn commands and spawns weapons in the world
fn spawn_weapon_on_event(
    mut events: EventReader<SpawnActorEvent>,
    mut commands: Commands,
    registry: Res<ActorRegistry>,
) {
    for event in events.read() {
        if event.actor_type != ActorType::Weapon {
            return;
        }

        let spawn_count = if event.spawn_count > 100 {
            warn!(
                "too many {:?} spawns requested, using 20 instead",
                event.actor_type
            );
            20
        } else {
            event.spawn_count
        };

        info!(
            "received spawn. weapon: {:?}, amount: {}, pos: {}",
            event.what_to_spawn, spawn_count, event.spawn_position
        );

        let Some(weapon_bundle) = registry.objects.weapons.get(&event.what_to_spawn) else {
            panic!(
                "requested weapon did not exist in weapon registry: {:?}",
                event.what_to_spawn
            )
        };
        let mut modified_weapon_ref = weapon_bundle.clone();
        modified_weapon_ref.sprite.sprite_bundle.transform =
            Transform::from_translation(event.spawn_position.extend(ACTOR_Z_INDEX));

        for _spawn in 0..spawn_count {
            spawn_weapon_bundle(&mut commands, &modified_weapon_ref);
        }
    }
}

/// spawns weapon in ecs world with child collider
fn spawn_weapon_bundle(
    commands: &mut Commands<'_, '_>,
    bundle_copy: &crate::bundles::WeaponBundle,
) {
    commands.spawn(bundle_copy.clone()).with_children(|child| {
        let collider_name = format!("{}Collider", bundle_copy.name.as_str());
        child.spawn(ObjectColliderBundle {
            tag: ActorColliderType::Object,
            name: Name::new(collider_name),
            collider: Collider::capsule(Vec2 { x: 0.0, y: -10.0 }, Vec2 { x: 0.0, y: 10.0 }, 2.0),
            collision_groups: CollisionGroups::new(
                AspenCollisionLayer::ACTOR,
                AspenCollisionLayer::EVERYTHING,
            ),
            transform_bundle: TransformBundle {
                local: Transform {
                    translation: Vec3 {
                        x: -2.25,
                        y: -2.525,
                        z: ACTOR_PHYSICS_Z_INDEX,
                    },
                    rotation: Quat::IDENTITY,
                    scale: Vec3::ONE,
                },
                global: GlobalTransform::IDENTITY,
            },
        });
    });
}
