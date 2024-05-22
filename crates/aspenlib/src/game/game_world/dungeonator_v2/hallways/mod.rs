use std::collections::VecDeque;

use bevy::{prelude::*, utils::petgraph::prelude::NodeIndex};
use bevy_ecs_ldtk::prelude::LdtkProject;
use bevy_ecs_tilemap::{
    map::{TilemapTexture, TilemapTileSize, TilemapType},
    tiles::TileStorage,
    TilemapBundle,
};

use crate::{
    consts::TILE_SIZE,
    game::game_world::dungeonator_v2::components::{Dungeon, RoomID},
    loading::assets::AspenMapHandles,
};

/// hallway creation functions
pub mod hallway_builder;
pub mod walls;

/// amount of this type that shares parents
#[derive(Debug, PartialEq, Eq, PartialOrd, Clone)]
pub struct SiblingCount(pub u32);

/// hallway representation
#[derive(Debug, Reflect, Clone, Component)]
pub struct HallWayBlueprint {
    /// hallway start pos
    pub start_pos: IVec2,
    /// hallway end pos
    pub end_pos: IVec2,
    /// how long is pathway
    pub distance: f32,
    /// rooms connected too hallway
    pub connected_rooms: (RoomID, RoomID),
    #[reflect(ignore)]
    pub node_path: VecDeque<NodeIndex>,
    /// hallway finished building
    pub built: bool,
}

#[derive(Debug, Component)]
pub struct HallwayLayer;

pub fn create_hallway_layer(
    mut cmds: Commands,
    dungeon: Query<(Entity, &Dungeon, &GlobalTransform)>,
    project_assets: Res<Assets<LdtkProject>>,
    level_assets: Res<AspenMapHandles>,
) {
    let (dungeon_entity, dungeon_info, dungeon_global_transform) = dungeon.single();
    let Dungeon {
        settings,
        tile_graph,
        ..
    } = dungeon_info;

    let hallway_container = cmds
        .spawn((Name::new("HallwayTiles"), HallwayLayer))
        .set_parent(dungeon_entity)
        .id();

    let tile_storage = TileStorage::empty(settings.size);
    let project_handle = &level_assets.default_levels;
    let project = project_assets
        .get(project_handle.id())
        .expect("asset should exist");
    let hallway_tileset = project
        .tileset_map()
        .get(&89) // TODO: magic numbers are bad
        .expect("tileset uid should not change");
    let tile_size = TilemapTileSize {
        x: TILE_SIZE,
        y: TILE_SIZE,
    };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    let min_tile_pos = tile_graph.get_tiles_translation_world(settings, UVec2::ZERO);
    let dungeon_global_transform = dungeon_global_transform.translation().truncate();
    let hallway_tiles_origin = min_tile_pos - dungeon_global_transform;

    cmds.entity(hallway_container).insert(TilemapBundle {
        grid_size,
        map_type,
        size: tile_storage.size,
        storage: tile_storage,
        texture: TilemapTexture::Single(hallway_tileset.clone()),
        tile_size,
        transform: Transform::from_translation(hallway_tiles_origin.extend(1.0)),
        ..Default::default()
    });
}
