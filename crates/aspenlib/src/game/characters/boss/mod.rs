use bevy::prelude::*;

use crate::loading::registry::RegistryIdentifier;

/// boss spawning system/utils
pub struct BossPlugin;

impl Plugin for BossPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EventSpawnBoss>().add_systems(
            Update,
            utils::spawn_boss
                .after(TransformSystem::TransformPropagate)
                .run_if(on_event::<EventSpawnBoss>),
        );
    }
}

/// request too spawn boss entity in world
#[derive(Debug, Event)]
pub struct EventSpawnBoss {
    /// registry id of boss
    pub actor_id: RegistryIdentifier,
    /// entity that requested boss spawn
    pub spawner: Entity,
    /// position too spawn boss in world
    pub position: Vec2,
}

/// boss spawn function
pub mod utils {

    use bevy::prelude::*;

    use crate::{
        bundles::{Aspen2dPhysicsBundle, AspenColliderBundle, NeedsCollider},
        consts::{AspenCollisionLayer, ACTOR_PHYSICS_Z_INDEX, ACTOR_Z_INDEX},
        game::{
            characters::boss::EventSpawnBoss, components::ActorColliderType,
            game_world::components::CharacterSpawner,
        },
        loading::{custom_assets::actor_definitions::CharacterDefinition, registry::ActorRegistry},
        utilities::EntityCreator,
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

            let Some(character) = registry.characters.get_character(&spawn_event.actor_id) else {
                error!(
                    "could not get CharacterBundle from character registry: {:?}",
                    spawn_event.actor_id
                );
                return;
            };

            commands
                .spawn((
                    character.clone(),
                    Aspen2dPhysicsBundle::default_character(),
                    Transform::from_translation(spawn_event.position.extend(ACTOR_Z_INDEX)),
                ))
                .with_children(|child| {
                    let collider_name = format!("{}Collider", character.name.clone().as_str());
                    let spawned_enemy = child
                        .spawn((
                            EntityCreator(child.target_entity()),
                            AspenColliderBundle {
                                tag: ActorColliderType::Character,
                                name: Name::new(collider_name),
                                transform: Transform {
                                    translation: (Vec3 {
                                        x: 0.0,
                                        y: 0.0,
                                        z: ACTOR_PHYSICS_Z_INDEX,
                                    }),
                                    ..default()
                                },
                                collider: NeedsCollider::Aabb,
                                collision_groups: AspenCollisionLayer::dynamic_actor(),
                            },
                        ))
                        .id();

                    if let Ok(mut spawner_state) = spawners.get_mut(spawn_event.spawner) {
                        spawner_state.spawned_characters.push(spawned_enemy);
                    }
                });
        }
    }
}
