/// creep spawn function
pub mod utils {
    use bevy::prelude::*;
    use bevy_rapier2d::geometry::CollisionGroups;
    use rand::{thread_rng, Rng};

    use crate::{
        bundles::ItemColliderBundle,
        consts::{actor_collider, AspenCollisionLayer, ACTOR_PHYSICS_Z_INDEX, ACTOR_Z_INDEX},
        game::{components::ActorColliderType, game_world::components::CharacterSpawner},
        loading::{
            custom_assets::actor_definitions::CharacterDefinition,
            registry::{ActorRegistry, RegistryIdentifier},
        },
    };

    /// spawns creep character in world
    /// if requested by spawner, adds too spawner list
    pub fn spawn_creep(
        registry: &Res<'_, ActorRegistry>,
        char_assets: &Res<'_, Assets<CharacterDefinition>>,
        spawn_data: &(RegistryIdentifier, i32),
        requester: Entity,
        spawn_position: Vec2,
        commands: &mut Commands<'_, '_>,
        enemy_spawners: &mut Query<'_, '_, &mut CharacterSpawner>,
    ) {
        let (_, char_def) = char_assets
            .iter()
            .find(|(_, asset)| asset.actor.identifier == spawn_data.0)
            .expect("Spawned characters asset definition did not exist");

        let Some(mut character) = registry.characters.get_character(&spawn_data.0) else {
            error!(
                "could not get CharacterBundle from character registry: {:?}",
                spawn_data.0
            );
            return;
        };

        let mut rng = thread_rng();
        for _ in 0..spawn_data.1 {
            let pos = Vec2 {
                x: spawn_position.x + rng.gen_range(-100.0..=100.0),
                y: spawn_position.y + rng.gen_range(-100.0..=100.0),
            };
            character.aseprite.sprite_bundle.transform =
                Transform::from_xyz(pos.x, pos.y, ACTOR_Z_INDEX);
            commands.spawn(character.clone()).with_children(|child| {
                let collider_name = format!("{}Collider", character.name.clone().as_str());
                let spawned_enemy = child
                    .spawn((ItemColliderBundle {
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

                if let Ok(mut spawner_state) = enemy_spawners.get_mut(requester) {
                    spawner_state.spawned_characters.push(spawned_enemy);
                }
            });
        }
    }
}
