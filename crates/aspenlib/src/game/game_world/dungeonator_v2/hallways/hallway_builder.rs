use std::collections::VecDeque;

use crate::game::game_world::dungeonator_v2::{
    components::Dungeon,
    hallways::HallWayBlueprint,
    tile_graph::{
        get_tile_translation, output_graph_dot, output_graph_image, TileGraph, TileGraphEdge,
        TileType,
    },
    GeneratorState,
};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_egui::egui::epaint::util::OrderedFloat;
use petgraph::{
    prelude::{NodeIndex, EdgeRef},
    graph::EdgeReference,
};

// TODO: create a grid of occupied/unoccupied tiles encompassing the whole dungueon
// use pathfinding algorithm/crates too get 2 paths, path should return as Vec<tile positon>
// iterate over each position for 3 layers
// building
// collisons
// something else?
///  builds hallway points for spawnnned hallway blueprints
pub fn build_hallways(
    mut cmds: Commands,
    mut hallways: Query<(Entity, &mut HallWayBlueprint, &GlobalTransform), With<Name>>,
    mut dungeon: Query<(Entity, &mut Dungeon)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let (dungeon_entity, mut dungeon_info) = dungeon.single_mut();
    let points_container = cmds
        .spawn((SpatialBundle::default(), Name::new("HallwayPoints")))
        .id();
    cmds.entity(dungeon_entity).add_child(points_container);

    let Dungeon { tile_graph, .. } = &mut *dungeon_info;
    output_graph_image(tile_graph);

    let graph_data = (tile_graph.center, tile_graph.size);

    for (_hallway_eid, mut hallway, _) in &mut hallways {
        if hallway.built {
            continue;
        }

        info!("generating path for hallway: {:?}", hallway.connected_rooms);
        let Some(hallway_path) = create_path_simple(tile_graph, &hallway) else {
            hallway.built = true;
            error!("could not generate path for {:?}", hallway);
            continue;
        };

        mark_path_as_hallway_tiles(&hallway_path, tile_graph);

        info!("converting path too positions");
        let mut path_positions: VecDeque<Vec2> = hallway_path
            .iter()
            .map(|a| {
                let a_weight = tile_graph
                    .node_weight(*a)
                    .expect("node weight a should exist in tilegraph");
                let gdata = &graph_data;
                get_tile_translation(gdata.0, tile_graph.size, a_weight.tile)
            })
            .collect();

        while let Some(pos_a) = path_positions.pop_front() {
            cmds.spawn((
                Name::new("HallwayPoint"),
                MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(Circle::new(5.0))).into(),
                    transform: Transform::from_xyz(pos_a.x, pos_a.y, 10.0),
                    material: materials.add(ColorMaterial::from(Color::RED)),
                    ..default()
                },
            ))
            .set_parent(points_container);
        }
        // hallway is built
        info!("finished spawning hallway");
        hallway.built = true;
    }

    if hallways.iter().all(|(_, hallway, _)| hallway.built) {
        info!("creating tile_graph debug files");
        output_graph_image(tile_graph);
        output_graph_dot(tile_graph);

        info!("all hallways finished");
        cmds.insert_resource(NextState(Some(GeneratorState::FinishedDungeonGen)));
    }
}

/// hallway start and end
pub struct HallwayPoints {
    /// hallway origin
    start: NodeIndex,
    /// hallway finish
    end: NodeIndex,
}

/// finds path start and end nodes and returns a `Vec<NodeIndex>` between the 2
fn create_path_simple(
    tile_graph: &TileGraph,
    hallway: &Mut<HallWayBlueprint>,
) -> Option<VecDeque<NodeIndex>> {
    info!("getting hallway side nodes");
    let start_pos = tile_graph.get_node_at_translation(hallway.start_pos);
    let end_pos = tile_graph.get_node_at_translation(hallway.end_pos);

    let Some(start) = start_pos else {
        error!("Could not get start translation from tile position");
        return None;
    };
    let Some(end) = end_pos else {
        error!("Could not get end translation from tile position");
        return None;
    };

    let first_side = HallwayPoints { start, end };

    info!("calculating first sides path");
    Some(dijkstra_path(tile_graph, first_side))
}

/// calculates dijkstra path between nodes
fn dijkstra_path(tile_graph: &TileGraph, hallway: HallwayPoints) -> VecDeque<NodeIndex> {
    info!("getting path list");

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
        info!("finding next node");
        path_node_indecies.push_front(current_node);
        if length == 0.0 {
            break; // Reached the start node
        }

        // Find the predecessor node with the shortest path
        let predecessor_node = tile_graph
            .neighbors(current_node)
            .filter_map(|neighbor| shortest_paths.get(&neighbor).map(|&len| (neighbor, len)))
            .min_by_key(|&(_, len)| OrderedFloat::from(len))
            .map(|(node, _)| node);

        if let Some(predecessor) = predecessor_node {
            current_node = predecessor;
        } else {
            // No predecessor found, the end node is unreachable from the start node
            error!("End node is unreachable from the start node");
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
        let path_start_debug = tile_graph.get_tiles_translation(path_start_pos);
        let path_end_debug = tile_graph.get_tiles_translation(path_end_pos);
        info!("path start debug: {:?}", path_start_debug);
        info!("path end debug: {:?}", path_end_debug);
        panic!("could not generate path");
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
