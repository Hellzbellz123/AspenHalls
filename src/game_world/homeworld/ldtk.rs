use bevy::prelude::*;
use bevy_ecs_ldtk::{LdtkWorldBundle, LevelSelection};
use heron::{CollisionLayers, CollisionShape, RigidBody};

use crate::{
    game_world::world_components::Collides, loading::assets::MapAssetHandles,
    utilities::game::PhysicsLayers,
};

pub fn spawn_mapbundle(mut commands: Commands, maps: Res<MapAssetHandles>) {
    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: maps.homeworld.clone(),
        transform: Transform {
            translation: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            scale: Vec3 {
                x: 3.0,
                y: 3.0,
                z: 1.0,
            },
            ..default()
        },
        ..default()
    });
}

pub fn spawn_level_0(mut commands: Commands) {
    commands.insert_resource(LevelSelection::Index(0));
}

pub fn name_colliders(
    mut commands: Commands,
    entity_query: Query<(Entity, &Parent, &Collides, &Transform), Added<Collides>>,
) {
    for entity in entity_query.iter() {
        info!("naming colliders: {}", entity.0.id());
        commands
            .entity(entity.0)
            .insert(Name::new("levelCollider"))
            .insert(RigidBody::Static)
            .insert(CollisionShape::Cuboid {
                half_extends: Vec3::new(16.0, 16.0, 0.0),
                border_radius: None,
            })
            .insert(
                CollisionLayers::none()
                    .with_group(PhysicsLayers::World)
                    .with_mask(PhysicsLayers::Player),
            );
    }
}
