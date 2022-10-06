use bevy::prelude::*;
use bevy_ecs_ldtk::{LdtkWorldBundle, LevelSelection};

use crate::loading::assets::MapAssetHandles;

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

