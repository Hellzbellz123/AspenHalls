use bevy::prelude::*;
use bevy_ecs_ldtk::{
    prelude::{LevelEvent, LevelIid},
    GridCoords,
};
use bevy_rapier2d::geometry::Collider;
use petgraph::{Graph, Undirected};

use crate::game::game_world::{
    components::RoomExitTile,
    dungeonator_v2::{
        components::{Dungeon, RoomBlueprint},
        hallways::{
            hallway_builder::{create_path_simple, mark_path_as_hallway_tiles},
            HallWayBlueprint,
        },
        tile_graph::r#impl::{actual_map_tile_size, populate_tilegraph},
        GeneratorState,
    },
    RoomBoundryTile,
};

/// implementations and utils for generations a tilegraph
pub mod r#impl;

// SLOW SLOW SLOW SLOW SLOW SLOW FUNCTION
// 2 hits 1.8 seconds each time under debugger
// pre sorting the data got this down too about half a second. feels pretty good now
/// creates tilegraph from currently spawned tiles
#[allow(clippy::type_complexity)]
pub fn create_tile_graph(
    mut cmds: Commands,
    mut dungeon_container: Query<(&mut Dungeon, &Transform)>,
    tile_query: Query<
        (
            &GlobalTransform,
            Option<&Collider>,
            Option<&RoomExitTile>,
            Option<&RoomBoundryTile>,
        ),
        With<GridCoords>,
    >,
    level_query: Query<(&LevelIid, &RoomBlueprint)>,
    mut spawn_events: EventReader<LevelEvent>,
    mut hallway_query: Query<&mut HallWayBlueprint>,
) {
    let mut transformed_count = 0;
    let level_amount = level_query.iter().count();

    for event in spawn_events.read() {
        match event {
            LevelEvent::Transformed(_) => transformed_count += 1,
            LevelEvent::SpawnTriggered(_) | LevelEvent::Spawned(_) | LevelEvent::Despawned(_) => {}
        }
    }
    if transformed_count < level_amount {
        info!("not all tiles are spawned yet");
        return;
    }

    info!("getting actual map dimensions");
    let (mut dungeon, dungeon_position) = dungeon_container.single_mut();
    (dungeon.settings.size, dungeon.tile_graph.center_world) =
        actual_map_tile_size(&dungeon.settings, &tile_query);

    info!("checking positions for tiles and creating nodes");
    populate_tilegraph(&mut dungeon, tile_query);
    info!("finished populating tile graph");

    // let (mut dungeon, dungeon_position) = dungeon_container.single_mut();
    let hallway_total = hallway_query.iter().len();
    for (i, mut hallway) in &mut hallway_query.iter_mut().enumerate() {
        info!(
            "generating path for hallway {}/{}",
            i + 1,
            hallway_total + 1
        );
        let Some(hallway_path) = create_path_simple(
            &dungeon.settings,
            &dungeon.tile_graph,
            &hallway,
            dungeon_position.translation.truncate(),
        ) else {
            hallway.built = true;
            continue;
        };

        mark_path_as_hallway_tiles(&hallway_path, &mut dungeon.tile_graph);
        hallway.node_path = hallway_path;
    }
    info!("created dungeon tile graph");
    cmds.insert_resource(NextState::Pending(GeneratorState::FinalizeHallways));
}

/// what dungeon structure does this node belong too
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TileType {
    /// this tile is walkable
    #[default]
    Floor,
    /// this tile is not walkable
    Wall,
    /// this position has no tile
    Unused,
    /// this tile is hallway meat
    Hallway,
    /// this tile is floor and hallway
    RoomExit,
}

/// map of notable tiles in each tile position for dungeon
#[derive(Debug, Deref, DerefMut, Clone, Default, Reflect)]
pub struct TileGraph {
    /// the actual graph structure
    #[reflect(ignore)]
    #[deref]
    pub graph: Graph<TileGraphNode, TileGraphEdge, Undirected>,
    /// tilegraphs center in worldspace coords
    pub center_world: Vec2,
}

/// dungeon tile node for graph containing position and `TileType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TileGraphNode {
    /// tile row/column
    pub tile: UVec2,
    /// traversability for node
    pub data: TileType,
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Default)]
/// cost too move between tiles in dungeon graph
pub struct TileGraphEdge {
    /// how much it costs too traverse this edge
    pub cost: f32,
}
