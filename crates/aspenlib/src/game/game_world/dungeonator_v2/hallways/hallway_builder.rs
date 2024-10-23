use std::collections::VecDeque;

use crate::game::{
    characters::components::CardinalDirection,
    game_world::dungeonator_v2::{
        components::{Dungeon, DungeonSettings},
        hallways::{
            walls::{spawn_corner_section, spawn_straight_section},
            HallWayBlueprint, HallwayLayer,
        },
        tile_graph::{
            r#impl::{output_graph_dot, output_graph_image},
            TileGraph, TileGraphEdge, TileType,
        },
        GeneratorState,
    },
};

use bevy::{math::FloatOrd, prelude::*};
use bevy_ecs_ldtk::prelude::TileEnumTags;
use bevy_ecs_tilemap::{
    map::TilemapId,
    prelude::{
        helpers::square_grid::neighbors::SquareDirection, TileTextureIndex, TilemapGridSize,
        TilemapSize, TilemapType,
    },
    tiles::{TileBundle, TilePos, TileStorage},
};
use petgraph::{
    graph::EdgeReference,
    prelude::{EdgeRef, NodeIndex},
};

// TODO: spawn TileEnumTags for hallway sections for collisions too be created
///  builds hallway points for spawned hallway blueprints
pub fn build_hallways(
    mut cmds: Commands,
    mut hallways: Query<(Entity, &mut HallWayBlueprint, &GlobalTransform), With<Name>>,
    mut dungeon: Query<(&mut Dungeon, &Transform)>,
    mut hallway_layer: Query<(Entity, &mut TileStorage), With<HallwayLayer>>,
) {
    let (mut dungeon_info, _) = dungeon.single_mut();
    let Dungeon {
        settings,
        tile_graph,
        ..
    } = &mut *dungeon_info;

    let (hallway_container, mut hallway_storage) = hallway_layer.single_mut();
    for (_, mut hallway, _) in &mut hallways {
        if hallway.built || hallway.node_path.len() == 2 {
            continue;
        }

        let path_with_direction = path_with_direction(tile_graph, &hallway.node_path);

        create_hallway_walls(
            &path_with_direction,
            tile_graph,
            &mut cmds,
            hallway_container,
            &mut hallway_storage,
        );

        handle_intersections(
            &path_with_direction,
            settings.size,
            tile_graph,
            &mut cmds,
            hallway_container,
            &mut hallway_storage,
        );

        create_floor_for_path(
            &path_with_direction,
            // tile_graph,
            &mut cmds,
            hallway_container,
            &mut hallway_storage,
        );

        // hallway is built
        info!("finished spawning hallway");
        hallway.built = true;
    }

    if hallways.iter().all(|(_, hallway, _)| hallway.built) {
        info!("creating tile_graph debug files");
        output_graph_image(tile_graph, settings.size);
        output_graph_dot(tile_graph);

        info!("all hallways finished");
        cmds.insert_resource(NextState::Pending(GeneratorState::FinishedDungeonGen));
    }
}

/// returns true if the next 2 tiles are on the same row or column
/// or if its the last tile in path, the tiles preceding it
pub fn tile_is_corner(
    _tile_graph: &TileGraph,
    path_with_direction: &VecDeque<(usize, TilePos, CardinalDirection)>,
    idx: usize,
    // TODO: return option bool symbolizing if there is a next tile or not
    // Option<bool>
) -> Option<bool> {
    let (c_index, c_coords, _c_dir) = path_with_direction
        .get(idx)
        .expect("this should always be here");
    assert!(*c_index == idx, "index passed into function and index retrieved from the path at that position should be the same");

    let previous = path_with_direction.get(idx.saturating_sub(2));
    let next1 = path_with_direction.get(idx + 1);
    let next2 = path_with_direction.get(idx + 2);

    // Check if next1 is None (probably last tile)
    if next1.is_none() {
        assert!(
            *c_index == path_with_direction.len() - 1,
            "not actually last tile, but couldnt get next tile"
        );

        // Check if next2 is None and next1 is Some
        if let (Some((_, prev_pos, _)), None) = (previous, next1) {
            // Check if previous, current, and next1 are inline
            if are_inline(*prev_pos, *c_coords) {
                return Some(false);
            }
            return Some(true);
        }
        return None;
    }

    // Check if previous is None (probably first tile)
    if previous.is_none() {
        assert!(
            *c_index == 0,
            "not actually first tile, but couldnt get previous tile"
        );
        if let (Some((_, _, _)), Some((_, next2, _))) = (next1, next2) {
            if are_inline(*c_coords, *next2) {
                return Some(false);
            }
            return Some(true);
        }
        return None;
    }

    if let (Some((_, _, _)), Some((_, next2, _))) = (next1, next2) {
        if are_inline(*c_coords, *next2) {
            Some(false)
        } else {
            Some(true)
        }
    } else {
        // Check if next2 is None and next1 is Some
        if let (Some((_, prev_coords, _)), Some((_, next1_coords, _))) = (previous, next1) {
            // Check if previous, current, and next1 are inline
            if are_inline(*prev_coords, *next1_coords) {
                return Some(false);
            }
            return Some(true);
        }
        None
    }
}

/// checks if tilepositions share a SINGLE common x/y value
/// or in other words
/// form a horizontal or vertical plane
pub fn are_inline(prev: TilePos, next: TilePos) -> bool {
    let pn_diff_y = next
        .y
        .saturating_sub(prev.y)
        .max(prev.y.saturating_sub(next.y));

    let pn_diff_x = next
        .x
        .saturating_sub(prev.x)
        .max(prev.x.saturating_sub(next.x));

    if prev.x == next.x && pn_diff_y == 2 {
        true
    } else {
        prev.y == next.y && pn_diff_x == 2
    }
}

#[allow(unused)]
/// hallway  texture id
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TexID {
    ArrowNorth = 0,
    ArrowSouth = 1,
    ArrowWest = 2,
    ArrowEast = 3,

    FloorBase = 4,
    FloorPoint = 6,
    FloorBad = 7,

    OcornerSe = 8,
    OcornerSw = 9,
    OcornerNe = 10,
    OcornerNw = 11,

    UWest = 12,
    UEast = 13,
    USouth = 14,
    UNorth = 15,

    IcornerNe = 16,
    IcornerNw = 17,
    IcornerSe = 18,
    IcornerSw = 19,

    WallNorth = 20,
    WallWest = 21,
    WallEast = 22,
    WallSouth = 23,
    WDoubleVert = 24,
    WDoubleHori = 25,
}

/// iterates over each node in the hallway and checks if it intersects
fn handle_intersections(
    path_with_direction: &VecDeque<(usize, TilePos, CardinalDirection)>,
    size: TilemapSize,
    tile_graph: &mut TileGraph,
    cmds: &mut Commands<'_, '_>,
    hallway_container: Entity,
    hallway_storage: &mut Mut<'_, TileStorage>,
) {
    for (_, coord_c, _) in path_with_direction {
        cmds.entity(hallway_container).with_children(|parent| {
            let up = coord_c
                .square_offset(&SquareDirection::North, &size)
                .expect("msg");
            let down = coord_c
                .square_offset(&SquareDirection::South, &size)
                .expect("msg");
            let left = coord_c
                .square_offset(&SquareDirection::West, &size)
                .expect("msg");
            let right = coord_c
                .square_offset(&SquareDirection::East, &size)
                .expect("msg");
            let up_left = (
                coord_c
                    .square_offset(&SquareDirection::NorthWest, &size)
                    .expect("msg"),
                SquareDirection::NorthWest,
            );
            let up_right = (
                coord_c
                    .square_offset(&SquareDirection::NorthEast, &size)
                    .expect("msg"),
                SquareDirection::NorthEast,
            );
            let down_left = (
                coord_c
                    .square_offset(&SquareDirection::SouthWest, &size)
                    .expect("msg"),
                SquareDirection::SouthWest,
            );
            let down_right = (
                coord_c
                    .square_offset(&SquareDirection::SouthEast, &size)
                    .expect("msg"),
                SquareDirection::SouthEast,
            );
            let cardinals = [up, down, left, right];
            let corners = [up_left, up_right, down_left, down_right];

            if cardinals.iter().all(|f| {
                let node = tile_graph.get_node_at_coord(f.into()).expect("msg");
                let tile = tile_graph[node];
                tile.data.is_hallway()
            }) {
                for (coord, corner) in corners {
                    let tex_id = match corner {
                        SquareDirection::NorthEast => TexID::IcornerSw,
                        SquareDirection::NorthWest => TexID::IcornerSe,
                        SquareDirection::SouthWest => TexID::IcornerNe,
                        SquareDirection::SouthEast => TexID::IcornerNw,
                        _ => panic!(),
                    };
                    spawn_tile(
                        path_with_direction,
                        tile_graph,
                        coord,
                        tex_id,
                        parent,
                        hallway_container,
                        hallway_storage,
                    );
                }
            };
        });
    }
}

// TODO: reddotgames tilemap information suggests using edges for hallways
// this might be a simpler solution for what im trying too accomplish
// idea:
// after all hallway floors are placed,
// create list of edges for all hallways,
// ignore edges that are shared between nodes
// edges are 32px long by 4-8px wide
/// creates walls around walkable hallway tiles
pub fn create_hallway_walls(
    path_with_direction: &VecDeque<(usize, TilePos, CardinalDirection)>,
    tile_graph: &mut TileGraph,
    cmds: &mut Commands<'_, '_>,
    hallway_container: Entity,
    hallway_storage: &mut Mut<'_, TileStorage>,
) {
    for (c_idx, c_coord, c_dir) in path_with_direction {
        cmds.entity(hallway_container).with_children(|parent| {
            // TODO: individually fix corner sections
            // if tile is occupied and TexId is corner section dont place new tile?
            spawn_corner_section(
                tile_graph,
                path_with_direction,
                *c_idx,
                c_dir,
                *c_coord,
                parent,
                hallway_container,
                hallway_storage,
            );
            spawn_straight_section(
                tile_graph,
                path_with_direction,
                c_dir,
                *c_coord,
                hallway_storage,
                parent,
                hallway_container,
            );
        });
    }
}

/// places tiles in world for a given hallway path
fn create_floor_for_path(
    path_with_direction: &VecDeque<(usize, TilePos, CardinalDirection)>,
    // tile_graph: &mut TileGraph,
    cmds: &mut Commands<'_, '_>,
    hallway_container: Entity,
    hallway_storage: &mut Mut<'_, TileStorage>,
) {
    for (_, tile_pos, _) in path_with_direction {
        cmds.entity(hallway_container).with_children(|parent| {
            let tx_idx = TexID::FloorBase;

            spawn_tile_unchecked(
                // path_with_direction,
                // tile_graph,
                *tile_pos,
                tx_idx,
                parent,
                hallway_container,
                hallway_storage,
            );
        });
    }
}

/// marker component for walkable hallway tiles
#[derive(Component)]
pub struct HallwayFloor;

/// spawns tile on position, aborting if position is already occupied
pub fn spawn_tile(
    path_with_direction: &VecDeque<(usize, TilePos, CardinalDirection)>,
    tile_graph: &mut TileGraph,
    coord: TilePos,
    tex_id: TexID,
    parent: &mut ChildBuilder<'_>,
    hallway_container: Entity,
    hallway_storage: &mut TileStorage,
) {
    let node_index = tile_graph
        .get_node_at_coord(coord.into())
        .expect("index must exist in graph");
    let local_position =
        coord.center_in_world(&TilemapGridSize::new(32.0, 32.0), &TilemapType::Square);
    let local_transfrorm = Transform::from_translation(local_position.extend(0.0));

    let node = tile_graph.node_weight_mut(node_index).expect("msg");

    // let position_type = node.data
    //     == match tex_id {
    //         TexID::FloorBase | TexID::FloorPoint | TexID::FloorBad => TileType::Hallway,
    //         TexID::OcornerSe
    //         | TexID::ArrowNorth
    //         | TexID::ArrowSouth
    //         | TexID::ArrowWest
    //         | TexID::ArrowEast
    //         | TexID::OcornerSw
    //         | TexID::OcornerNe
    //         | TexID::OcornerNw
    //         | TexID::UWest
    //         | TexID::UEast
    //         | TexID::USouth
    //         | TexID::UNorth
    //         | TexID::IcornerNe
    //         | TexID::IcornerNw
    //         | TexID::IcornerSe
    //         | TexID::IcornerSw
    //         | TexID::WallNorth
    //         | TexID::WallWest
    //         | TexID::WallEast
    //         | TexID::WallSouth => TileType::Wall,
    //     };

    if path_with_direction.iter().all(|(_, f, _)| *f != coord) && !node.data.is_hallway() {
        let tile_entity = parent
            .spawn((
                SpatialBundle::from_transform(local_transfrorm),
                populate_enum_tags(tex_id),
                TileBundle {
                    position: coord,
                    tilemap_id: TilemapId(hallway_container),
                    texture_index: TileTextureIndex(tex_id as u32),
                    ..Default::default()
                },
            ))
            .id();
        hallway_storage.set(&coord, tile_entity);
    }
}

/// spawns tile on positon without checking if position is occupied
pub fn spawn_tile_unchecked(
    coord: TilePos,
    tex_id: TexID,
    parent: &mut ChildBuilder<'_>,
    hallway_container: Entity,
    hallway_storage: &mut TileStorage,
) {
    let local_position =
        coord.center_in_world(&TilemapGridSize::new(32.0, 32.0), &TilemapType::Square);
    let local_transfrorm = Transform::from_translation(local_position.extend(0.0));

    let tile_entity = parent
        .spawn((
            SpatialBundle::from_transform(local_transfrorm),
            populate_enum_tags(tex_id),
            TileBundle {
                position: coord,
                tilemap_id: TilemapId(hallway_container),
                texture_index: TileTextureIndex(tex_id as u32),
                ..Default::default()
            },
        ))
        .id();
    hallway_storage.set(&coord, tile_entity);
}

/// adds tile enum tags for given `TexID`
fn populate_enum_tags(tex_id: TexID) -> TileEnumTags {
    let mut tile_enum = TileEnumTags {
        tags: Vec::new(),
        source_enum_uid: None,
    };
    match tex_id {
        TexID::OcornerSe => tile_enum.tags.push("CollideCornerUL".into()),
        TexID::OcornerSw => tile_enum.tags.push("CollideCornerUR".into()),
        TexID::OcornerNe => tile_enum.tags.push("CollideCornerLL".into()),
        TexID::OcornerNw => tile_enum.tags.push("CollideCornerLR".into()),
        TexID::IcornerNe => tile_enum.tags.push("CollideInnerUR".into()),
        TexID::IcornerNw => tile_enum.tags.push("CollideInnerUL".into()),
        TexID::IcornerSe => tile_enum.tags.push("CollideInnerLR".into()),
        TexID::IcornerSw => tile_enum.tags.push("CollideInnerLL".into()),
        TexID::WallNorth => tile_enum.tags.push("CollideUp".into()),
        TexID::WallWest => tile_enum.tags.push("CollideLeft".into()),
        TexID::WallEast => tile_enum.tags.push("CollideRight".into()),
        TexID::WallSouth => tile_enum.tags.push("CollideDown".into()),
        TexID::WDoubleVert => tile_enum.tags.push("DoubleWallVertical".into()),
        TexID::WDoubleHori => tile_enum.tags.push("DoubleWallHorizontal".into()),
        TexID::UWest
        | TexID::UEast
        | TexID::USouth
        | TexID::UNorth
        | TexID::FloorBase
        | TexID::FloorPoint
        | TexID::FloorBad
        | TexID::ArrowNorth
        | TexID::ArrowSouth
        | TexID::ArrowWest
        | TexID::ArrowEast => (),
    }
    tile_enum
}

/// transforms list of nodes into a directed path with '`TilePos`'
pub fn path_with_direction(
    tile_graph: &TileGraph,
    input_path: &VecDeque<NodeIndex>,
) -> VecDeque<(usize, TilePos, CardinalDirection)> {
    assert!(
        input_path.len() >= 2,
        "path should not be less than 2 elements"
    );

    let input_path = input_path
        .iter()
        .enumerate()
        .map(|(i, idx)| {
            let current_coord = tile_graph.node_weight(*idx).expect("msg").tile;

            // is end of path
            if i == input_path.len() - 1 {
                let &previous_node = input_path.get(i - 1).expect("preceeding node exists");
                let prev_coord = tile_graph
                    .node_weight(previous_node)
                    .expect("node is in graph")
                    .tile;

                (
                    i,
                    TilePos {
                        x: current_coord.x,
                        y: current_coord.y,
                    },
                    if prev_coord.x < current_coord.x && current_coord.y == prev_coord.y {
                        // moving west
                        CardinalDirection::West
                    } else if prev_coord.x > current_coord.x && current_coord.y == prev_coord.y {
                        // moving east
                        CardinalDirection::East
                    } else if prev_coord.x == current_coord.x && current_coord.y > prev_coord.y {
                        // moving north
                        CardinalDirection::North
                    } else if prev_coord.x == current_coord.x && current_coord.y < prev_coord.y {
                        // moving south
                        CardinalDirection::South
                    } else {
                        panic!("wtf bad no");
                    },
                )
            } else {
                let &next_node = input_path.get(i + 1).expect("next node exists");
                let next_coord = tile_graph
                    .node_weight(next_node)
                    .expect("node is in graph")
                    .tile;

                (
                    i,
                    TilePos {
                        x: current_coord.x,
                        y: current_coord.y,
                    },
                    if next_coord.x < current_coord.x && current_coord.y == next_coord.y {
                        // moving west
                        CardinalDirection::West
                    } else if next_coord.x > current_coord.x && current_coord.y == next_coord.y {
                        // moving east
                        CardinalDirection::East
                    } else if next_coord.x == current_coord.x && current_coord.y > next_coord.y {
                        // moving south
                        CardinalDirection::South
                    } else if next_coord.x == current_coord.x && current_coord.y < next_coord.y {
                        // moving north
                        CardinalDirection::North
                    } else {
                        panic!("wtf bad no");
                    },
                )
            }
        })
        .collect();

    input_path
}

/// hallway start and end nodes
#[derive(Debug)]
pub struct HallwayPoints {
    /// hallway origin
    start: NodeIndex,
    /// hallway finish
    end: NodeIndex,
}

/// finds path start and end nodes and returns a `Vec<NodeIndex>` between the 2
pub fn create_path_simple(
    settings: &DungeonSettings,
    tile_graph: &TileGraph,
    hallway: &Mut<HallWayBlueprint>,
    center: Vec2,
) -> Option<VecDeque<NodeIndex>> {
    trace!("getting hallway side nodes");
    let start_node_position = center.as_ivec2() + hallway.start_pos + IVec2::splat(16);
    let end_node_position = center.as_ivec2() + hallway.end_pos + IVec2::splat(16);

    let start_pos = tile_graph.get_node_at_translation(settings, start_node_position);
    let end_pos = tile_graph.get_node_at_translation(settings, end_node_position);

    let Some(start) = start_pos else {
        error!("could not get start node for {:?}", start_node_position);
        return None;
    };
    let Some(end) = end_pos else {
        error!("Could not get end node for {:?}", end_node_position);
        return None;
    };

    let first_side = HallwayPoints { start, end };

    trace!("calculating first sides path");
    Some(dijkstra_path(settings, tile_graph, first_side))
}

/// calculates dijkstra path between nodes
fn dijkstra_path(
    settings: &DungeonSettings,
    tile_graph: &TileGraph,
    hallway: HallwayPoints,
) -> VecDeque<NodeIndex> {
    trace!("calculating djikstra path for {:?}", hallway);

    let edge_cost = |edge: EdgeReference<TileGraphEdge>| {
        let mut cost = edge.weight().cost; // 1.0
        let (a, b) = tile_graph
            .edge_endpoints(edge.id())
            .expect("this edge should exist");
        let mut neighbor_count = 0;
        tile_graph.neighbors(a).for_each(|f| {
            neighbor_count += tile_graph.neighbors(f).count();
        }); // 4 neighbors if valid, each neighbor has 4 if valid, 16 total
        tile_graph.neighbors(b).for_each(|f| {
            neighbor_count += tile_graph.neighbors(f).count();
        }); // 4 neighbors if valid, each neighbor has 4 if valid, 16 total

        if neighbor_count == 32 {
            cost += 0.0;
        } else {
            cost += 5.0;
        }

        cost
    };

    let shortest_paths = petgraph::algo::dijkstra(
        &tile_graph.graph,
        hallway.start,
        Some(hallway.end),
        edge_cost,
    );

    // Reconstruct the shortest path to the end node
    let mut current_node = hallway.end;
    let mut path_node_indecies = VecDeque::new();

    while let Some(&length) = shortest_paths.get(&current_node) {
        trace!("finding next node for hallway path");
        path_node_indecies.push_front(current_node);
        if length == 0.0 {
            break; // Reached the start node
        }

        // Find the predecessor node with the shortest path
        let predecessor_node = tile_graph
            .neighbors(current_node)
            .filter_map(|neighbor| shortest_paths.get(&neighbor).map(|&len| (neighbor, len)))
            .min_by_key(|&(_, len)| FloatOrd(len))
            .map(|(node, _)| node);

        if let Some(predecessor) = predecessor_node {
            current_node = predecessor;
        } else {
            // No predecessor found, the end node is unreachable from the start node
            error!("No valid nodes too continue with, invalid path");
            break;
        }
    }

    if path_node_indecies.is_empty() {
        let path_start_pos = tile_graph
            .node_weight(hallway.start)
            .expect("node should exist")
            .tile;
        let path_end_pos = tile_graph
            .node_weight(hallway.end)
            .expect("node should exist")
            .tile;
        let path_start_debug = tile_graph.get_tiles_translation_world(settings, path_start_pos);
        let path_end_debug = tile_graph.get_tiles_translation_world(settings, path_end_pos);
        error!("path start debug: {:?}", path_start_debug);
        error!("path end debug: {:?}", path_end_debug);
    }

    path_node_indecies
}

/// marks list of node indecies as hallways in tile graph
pub fn mark_path_as_hallway_tiles(path: &VecDeque<NodeIndex>, tile_graph: &mut TileGraph) {
    for node_ids in path {
        let one = tile_graph
            .node_weight_mut(*node_ids)
            .expect("node should exist");
        if one.data.is_unused() {
            one.data = TileType::Hallway;
        } else if one.data.is_floor() || one.data.is_wall() {
            error!("building node included in path");
        }
    }
}

/// calculates Manhattan distance between 2 tiles on a grid
pub const fn distance_tiles(v1: UVec2, v2: UVec2) -> u32 {
    let dx = v2.x.saturating_sub(v1.x);
    let dy = v2.y.saturating_sub(v1.y);
    dx + dy
}
