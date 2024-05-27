use bevy::prelude::*;

use crate::loading::registry::RegistryIdentifier;

pub struct BossPlugin;

impl Plugin for BossPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EventSpawnBoss>().add_systems(
            Update,
            utils::spawn_boss.run_if(on_event::<EventSpawnBoss>()),
        );
    }
}

#[derive(Debug, Event)]
pub struct EventSpawnBoss {
    pub actor_id: RegistryIdentifier,
    pub spawner: Entity,
    pub position: Vec2,
}

/// boss spawn function
pub mod utils {
    use bevy::prelude::*;
    use bevy_rapier2d::geometry::CollisionGroups;
    use rand::{thread_rng, Rng};

    use crate::{
        bundles::ActorColliderBundle,
        consts::{actor_collider, AspenCollisionLayer, ACTOR_PHYSICS_Z_INDEX, ACTOR_Z_INDEX},
        game::{
            characters::boss::EventSpawnBoss, components::ActorColliderType,
            game_world::components::CharacterSpawner,
        },
        loading::{custom_assets::actor_definitions::CharacterDefinition, registry::ActorRegistry},
    };

    /// spawns creep character in world
    /// if requested by spawner, adds too spawner list
    pub fn spawn_boss(
        registry: Res<ActorRegistry>,
        char_assets: Res<Assets<CharacterDefinition>>,
        mut commands: Commands,
        mut boss_spawns: EventReader<EventSpawnBoss>,
        mut spawners: Query<&mut CharacterSpawner>,
    ) {
        for spawn_event in boss_spawns.read() {
            let (_, char_def) = char_assets
                .iter()
                .find(|(_, asset)| asset.actor.identifier == spawn_event.actor_id)
                .expect("Spawned characters asset definition did not exist");

            let Some(mut character) = registry.characters.get_character(&spawn_event.actor_id)
            else {
                error!(
                    "could not get CharacterBundle from character registry: {:?}",
                    spawn_event.actor_id
                );
                return;
            };

            character.aseprite.sprite_bundle.transform =
                Transform::from_translation(spawn_event.position.extend(ACTOR_Z_INDEX));

            commands.spawn(character.clone()).with_children(|child| {
                let collider_name = format!("{}Collider", character.name.clone().as_str());
                let spawned_enemy = child
                    .spawn((ActorColliderBundle {
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

                if let Ok(mut spawner_state) = spawners.get_mut(spawn_event.spawner) {
                    spawner_state.spawned_characters.push(spawned_enemy);
                }
            });
        }
    }
}
