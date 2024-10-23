use bevy::prelude::*;

use crate::loading::registry::RegistryIdentifier;

/// general creep spawning plugin
pub struct CreepPlugin;

impl Plugin for CreepPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EventSpawnCreep>().add_systems(
            Update,
            utils::spawn_creep.run_if(on_event::<EventSpawnCreep>()),
        );
    }
}

/// request too create creep entity in world
#[derive(Debug, Event)]
pub struct EventSpawnCreep {
    /// registery id of requested creep
    pub actor_id: RegistryIdentifier,
    /// what entity requested this creep
    pub spawner: Entity,
    /// position in world too place this creep
    pub position: Vec2,
}

/// creep spawn function
pub mod utils {
    use bevy::prelude::*;
    use bevy_rapier2d::geometry::CollisionGroups;

    use crate::{
        bundles::ActorColliderBundle,
        consts::{actor_collider, AspenCollisionLayer, ACTOR_PHYSICS_Z_INDEX, ACTOR_Z_INDEX},
        game::{
            characters::creeps::EventSpawnCreep, components::ActorColliderType,
            game_world::components::CharacterSpawner,
        },
        loading::{custom_assets::actor_definitions::CharacterDefinition, registry::ActorRegistry},
        utilities::EntityCreator,
    };

    /// spawns creep character in world
    /// if requested by spawner, adds too spawner list
    pub fn spawn_creep(
        registry: Res<ActorRegistry>,
        char_assets: Res<Assets<CharacterDefinition>>,
        mut cmds: Commands,
        mut creep_spawns: EventReader<EventSpawnCreep>,
        mut spawners: Query<&mut CharacterSpawner>,
    ) {
        for spawn_event in creep_spawns.read() {
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

            let spawned_enemy = cmds
                .spawn(character.clone())
                .with_children(|child| {
                    let collider_name = format!("{}Collider", character.name.clone().as_str());
                    child.spawn(ActorColliderBundle {
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
                    });
                })
                .id();

            cmds.entity(spawned_enemy)
                .insert(EntityCreator(spawned_enemy));

            if let Ok(mut spawner_state) = spawners.get_mut(spawn_event.spawner) {
                spawner_state.spawned_characters.push(spawned_enemy);
            }
        }
    }
}
