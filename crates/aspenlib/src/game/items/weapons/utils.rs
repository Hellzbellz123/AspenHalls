use bevy::prelude::*;
use bevy_rapier2d::geometry::{Collider, CollisionGroups};
use rand::{thread_rng, Rng};

use crate::{
    bundles::ActorColliderBundle,
    consts::{AspenCollisionLayer, ACTOR_PHYSICS_Z_INDEX, ACTOR_Z_INDEX},
    game::components::ActorColliderType,
    loading::{
        custom_assets::actor_definitions::ItemDefinition,
        registry::{ActorRegistry, RegistryIdentifier},
    },
};

/// spawns weapon item
pub fn spawn_weapon(
    registry: &Res<ActorRegistry>,
    item_assets: &Res<Assets<ItemDefinition>>,
    spawn_data: &(RegistryIdentifier, i32),
    spawn_position: Vec2,
    commands: &mut Commands,
) {
    let (_, item_def) = item_assets
        .iter()
        .find(|(_, asset)| asset.actor.identifier == spawn_data.0)
        .expect("Spawned characters asset definition did not exist");

    let Some(weapon_bundle) = registry.items.weapons.get(&spawn_data.0) else {
        error!(
            "could not get WeaponBundle from registry: {:?}",
            &spawn_data.0
        );
        return;
    };

    let mut rng = thread_rng();
    for _ in 0..spawn_data.1 {
        let position = Vec2 {
            x: spawn_position.x + rng.gen_range(-100.0..=100.0),
            y: spawn_position.y + rng.gen_range(-100.0..=100.0),
        };

        let mut modified_weapon_ref = weapon_bundle.clone();
        modified_weapon_ref.sprite.sprite_bundle.transform =
            Transform::from_translation(position.extend(ACTOR_Z_INDEX));
        info!("spawning weapon");
        commands
            .spawn(modified_weapon_ref.clone())
            .with_children(|child| {
                let collider_name = format!("{}Collider", modified_weapon_ref.name.as_str());
                let size = item_def.actor.pixel_size;
                child.spawn(ActorColliderBundle {
                    tag: ActorColliderType::Item,
                    name: Name::new(collider_name),
                    collider: Collider::capsule(
                        Vec2 {
                            x: size.x / 2.0,
                            y: 0.0,
                        },
                        Vec2 {
                            x: -(size.x / 2.0),
                            y: 0.0,
                        },
                        2.0,
                    ),
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
}
