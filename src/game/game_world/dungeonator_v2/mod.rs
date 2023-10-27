use bevy::{
    ecs::bundle::Bundle,
    prelude::{
        default, info, resource_exists, run_once, Commands, Component, Condition, Handle,
        IntoSystemConfigs, Name, OnEnter, Plugin, Res, SpatialBundle, Transform, Update,
    },
    reflect::Reflect,
};
use bevy_ecs_ldtk::{prelude::LdtkProject, LevelIid};

use crate::{game::AppStage, loading::assets::MapAssetHandles};

/// generates dungeons from ldtk level files
pub struct DungeonGeneratorPlugin;

impl Plugin for DungeonGeneratorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<DungeonSettings>();
        app.add_systems(
            Update,
            (
                spawn_dungeon_root
                    .run_if(resource_exists::<MapAssetHandles>().and_then(run_once())),
            ),
        );
    }
}

fn spawn_dungeon_root(mut cmds: Commands, ldtk_project_handles: Res<MapAssetHandles>) {
    info!("spawning dungeon container");
    cmds.spawn((DungeonContainerBundle {
        tag: DungeonContainerTag,
        name: "DungeonContainer".into(),
        settings: DungeonSettings::default(),
        ldtk_project: ldtk_project_handles.dungeons.clone(),
        spatial: SpatialBundle {
            transform: Transform::from_xyz(900.0, 2000.0, 0.0),
            ..default()
        },
    },));
}

/// settings to configure the dungeon generator,
/// `useable_rooms` and hallways are filled by other systems
#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct DungeonSettings {
    /// amount of rooms
    dungeon_room_amount: i32,
    /// looped hallway percentage
    looped_hallway_percentage: f32,
    /// grids too room percentage
    grid_too_room_percentage: f32,
    /// minimum space between dungeon rooms, in tiles
    min_space_between_rooms: f32,
}

#[derive(Component)]
pub struct DungeonContainerTag;

#[derive(Component)]
pub struct DungeonHallwayTag;

#[derive(Component)]
pub struct DungeonRoomTag;

#[derive(Bundle)]
pub struct DungeonContainerBundle {
    /// identifies dungeon root entity
    tag: DungeonContainerTag,
    /// identified dungeon root in hierarchy
    name: Name,
    /// configures spawning of child rooms and hallways
    settings: DungeonSettings,
    /// data used too spawn with
    ldtk_project: Handle<LdtkProject>,
    spatial: SpatialBundle,
}

#[derive(Bundle)]
pub struct DungeonRoomBundle {
    /// identifies dungeon rooms
    tag: DungeonRoomTag,
    /// basically just `LevelIid`
    name: Name,
    /// id from `LdtkProject`
    id: LevelIid,
}

#[derive(Bundle)]
pub struct DungeonHallWayBundle {
    /// identifies dungeon hallways
    tag: DungeonHallwayTag,
    /// Hallway# from-to
    name: Name,
}
