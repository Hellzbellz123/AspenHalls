use bevy::prelude::*;
use bevy_ecs_ldtk::GridCoords;
use bevy_ecs_tilemap::tiles::TilePos;
use bevy_rapier2d::geometry::Collider;

use image::{ImageBuffer, Rgba};
use petgraph::{
    data::{Element, FromElements},
    prelude::Undirected,
    stable_graph::NodeIndex,
    visit::IntoNodeReferences,
    Graph,
};
use std::fs;

use crate::{
    consts::TILE_SIZE,
    game::game_world::{
        components::RoomExitTile,
        dungeonator_v2::{
            components::{Dungeon, RoomBlueprint}, hallways::hallway_builder::manhattan_distance_tiles, tile_graph,
        },
    },
};

// TODO
/// creates the pathmap from currently spawned tiles
#[allow(clippy::type_complexity)]
pub fn create_tile_graph(
    mut dungeon_container: Query<(&mut Dungeon, &GlobalTransform)>,
    tile_query: Query<(
        Entity,
        &TilePos,
        &GridCoords,
        &Transform,
        &GlobalTransform,
        Option<&Collider>,
        Option<&RoomExitTile>,
    )>,
    // room_query: Query<(&RoomBlueprint, &Transform)>
) {
    info!("creating tilegraph");
    let (mut dungeon, dungeon_position) = dungeon_container.single_mut();
    dungeon.tile_graph.size = actual_map_tile_size(&tile_query);
    dungeon.tile_graph.center = dungeon_position.translation().truncate();
    // TODO: fix tilegraph sub tile translations.
    // currently it will break if dungeon is not centered on 0?
    // room_query.iter().map(|f| f.1.translation.truncate() + f.0.size).sum::<Vec2>() / room_query.iter().len() as f32;

    let mut tile_nodes = Vec::new();

    info!("checking positions for tiles and creating nodes");
    filter_and_create_nodes(
        &dungeon.tile_graph,
        tile_query,
        &mut tile_nodes,
    );

    info!("creating graph from nodes");
    let mut filled_graph: Graph<TileGraphNode, TileGraphEdge, Undirected, u32> =
        Graph::from_elements(tile_nodes);

    info!("connecting adjectent nodes in graph");
    connect_adjacent_nodes(&mut filled_graph);
    info!("finished connecting adjacent nodes");

    info!("created dungeon tile graph");
    dungeon.tile_graph.graph = filled_graph;
}

// fn create_simple_tile_graph(
//     tile_graph: &TileGraph,
// ) -> Graph<>{}

#[allow(clippy::type_complexity)]
/// check each tile position for a tile in gameworld and add node too list
fn filter_and_create_nodes(
    tile_graph: &TileGraph,
    tile_query: Query<(
        Entity,
        &TilePos,
        &GridCoords,
        &Transform,
        &GlobalTransform,
        Option<&Collider>,
        Option<&RoomExitTile>,
    )>,
    tile_nodes: &mut Vec<Element<TileGraphNode, TileGraphEdge>>,
) {
    for x_pos in 0..=tile_graph.size {
        for y_pos in 0..=tile_graph.size {
            let coords = UVec2 { x: x_pos, y: y_pos };
            // Calculate the actual translation of the tile based on dungeon position
            let tile_translation = tile_graph.get_tiles_translation(coords);

            // Check if tile position is occupied
            if tile_occupies_position(&tile_query, tile_translation) {
                if position_has_collider(&tile_query, tile_translation) {
                    tile_nodes.push(Element::Node {
                        weight: TileGraphNode {
                            tile: coords,
                            data: TileType::Wall,
                        },
                    });
                } else if position_has_room_exit(&tile_query, tile_translation) {
                    tile_nodes.push(Element::Node {
                        weight: TileGraphNode {
                            tile: coords,
                            data: TileType::RoomExit,
                        },
                    });
                } else {
                    tile_nodes.push(Element::Node {
                        weight: TileGraphNode {
                            tile: coords,
                            data: TileType::Floor,
                        },
                    });
                }
            } else {
                tile_nodes.push(Element::Node {
                    weight: TileGraphNode {
                        tile: coords,
                        data: TileType::Unused,
                    },
                });
            }
        }
    }
}

/// connects adjacent nodes in graph
fn connect_adjacent_nodes(graph: &mut Graph<TileGraphNode, TileGraphEdge, Undirected, u32>) {
    let references: Vec<(NodeIndex, &TileGraphNode)> = graph.node_references().collect();
    let mut edges: Vec<(NodeIndex, NodeIndex, TileGraphEdge)> = Vec::new();

    for (i, current) in references.iter().enumerate() {
        for other in references.iter().skip(i + 1) {
            if is_adjacent(current.1.tile, other.1.tile)
                && (current.1.data.can_be_hallway() && other.1.data.can_be_hallway())
            {
                edges.push((
                    current.0,
                    other.0,
                    TileGraphEdge {
                        cost: calculate_weight(current, other, graph),
                    },
                ));
            }
        }
    }

    graph.extend_with_edges(edges);
}

/// calculates edge weight for 2 graph nodes
fn calculate_weight(
    current: &(NodeIndex, &TileGraphNode),
    other: &(NodeIndex, &TileGraphNode),
    graph: &Graph<TileGraphNode, TileGraphEdge, Undirected, u32>,
) -> f32 {
    let (current_index, current_node) = current;
    let (other_index, other_node) = other;

    // TODO: maybe change this assert too only check 1 tile distance
    assert!(
        manhattan_distance_tiles(current_node.tile, other_node.tile) == 1
            || manhattan_distance_tiles(current_node.tile, other_node.tile) == 2,
        "tiles are not adjacent: {}",
        manhattan_distance_tiles(current_node.tile, other_node.tile)
    );
    assert!(
        current_index != other_index,
        "calculating weights between same tiles not allowed"
    );

    let multiplier = if graph.neighbors(*current_index).count() < 4 {
        5.0
    } else {
        1.0
    };

    let inner: f32 = match current_node.data {
        TileType::Floor | TileType::Wall => 99.0,
        TileType::Hallway => 10.0,
        TileType::RoomExit => 1.0,
        TileType::Unused => 0.5,
    };

    let outer: f32 = match other_node.data {
        TileType::Floor | TileType::Wall => 99.0,
        TileType::Hallway => 10.0,
        TileType::RoomExit => 1.0,
        TileType::Unused => 0.5,
    };

    inner.mul_add(multiplier, outer)
}

/// Function to check if two positions are adjacent
const fn is_adjacent(pos1: UVec2, pos2: UVec2) -> bool {
    let row_diff = (pos1.x as i32 - pos2.x as i32).abs();
    let col_diff = (pos1.y as i32 - pos2.y as i32).abs();

    // Nodes are adjacent if their row or column difference is less than or equal to 1
    (row_diff == 1 && col_diff == 0) || (row_diff == 0 && col_diff == 1) //|| (row_diff == 1 && col_diff == 1)
}

#[allow(clippy::type_complexity)]
/// gets tilemap size in tiles
fn actual_map_tile_size(
    tile_query: &Query<
        '_,
        '_,
        (
            Entity,
            &TilePos,
            &GridCoords,
            &Transform,
            &GlobalTransform,
            Option<&Collider>,
            Option<&RoomExitTile>,
        ),
    >,
) -> u32 {
    let (max_x, max_y, min_x, min_y) = calculate_tile_extents(tile_query);
    let half_size = max_x
        .maximum(min_x.abs())
        .maximum(max_y.maximum(min_y.abs()));
    let px_size = half_size * 2.0;
    let map_tile_size = (px_size / TILE_SIZE) as u32;
    trace!(
        "tile values: x+: {} x-: {}, y+: {}, y-: {}",
        max_x,
        min_x,
        max_y,
        min_y
    );
    map_tile_size
}

#[allow(clippy::type_complexity)]
/// does position have a tile
fn tile_occupies_position(
    tiles: &Query<
        '_,
        '_,
        (
            Entity,
            &TilePos,
            &GridCoords,
            &Transform,
            &GlobalTransform,
            Option<&Collider>,
            Option<&RoomExitTile>,
        ),
    >,
    position: Vec2,
) -> bool {
    tiles
        .iter()
        .filter(|f| f.3.translation.z as i32 == 0)
        .find(|f| f.4.translation().truncate() == position)
        .map(|f| f.0)
        .is_some()
}

#[allow(clippy::type_complexity)]
/// does vec2 exist as tile with collider
fn position_has_collider(
    tiles: &Query<
        '_,
        '_,
        (
            Entity,
            &TilePos,
            &GridCoords,
            &Transform,
            &GlobalTransform,
            Option<&Collider>,
            Option<&RoomExitTile>,
        ),
    >,
    position: Vec2,
) -> bool {
    tiles
        .iter()
        .filter(|f| f.5.is_some())
        .find(|f| f.4.translation().truncate() == position)
        .map(|f| f.0)
        .is_some()
}

#[allow(clippy::type_complexity)]
/// does vec2 exist as tile with a `RoomExit`
fn position_has_room_exit(
    tiles: &Query<
        '_,
        '_,
        (
            Entity,
            &TilePos,
            &GridCoords,
            &Transform,
            &GlobalTransform,
            Option<&Collider>,
            Option<&RoomExitTile>,
        ),
    >,
    position: Vec2,
) -> bool {
    tiles
        .iter()
        .filter(|f| f.6.is_some())
        .find(|f| f.4.translation().truncate() == position)
        .map(|f| f.0)
        .is_some()
}

/// finds furthest 4 corners of tile map
#[allow(clippy::type_complexity)]
fn calculate_tile_extents(
    tile_query: &Query<
        '_,
        '_,
        (
            Entity,
            &TilePos,
            &GridCoords,
            &Transform,
            &GlobalTransform,
            Option<&Collider>,
            Option<&RoomExitTile>,
        ),
    >,
) -> (f32, f32, f32, f32) {
    // Use map to extract the Vec2 translations from the iterator
    let translations: Vec<Vec2> = tile_query
        .iter()
        // filter base layer cause its background filled
        .filter(|f| f.3.translation.z as i32 == 0)
        // map data too translations
        .map(|(_, _, _, _, global, _, _)| global.translation().truncate())
        .collect();

    // Find the maximum and minimum x and y values using iterators
    let (max_x, max_y) = translations
        .iter()
        .fold((f32::NEG_INFINITY, f32::NEG_INFINITY), |acc, t| {
            (acc.0.max(t.x), acc.1.max(t.y))
        });
    let (min_x, min_y) = translations
        .iter()
        .fold((f32::INFINITY, f32::INFINITY), |acc, t| {
            (acc.0.min(t.x), acc.1.min(t.y))
        });

    (max_x, max_y, min_x, min_y)
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
    /// this tille is floor and hallway
    RoomExit,
}

impl TileType {
    /// does this tile position have a floor tile on it
    pub const fn is_floor(self) -> bool {
        matches!(self, Self::Floor)
    }

    /// does this tile position have a collider spawned on it
    pub const fn is_wall(self) -> bool {
        matches!(self, Self::Wall)
    }

    /// does this tile position have a tile spawned
    pub const fn is_unused(self) -> bool {
        matches!(self, Self::Unused)
    }

    /// checks if tile is valid for hallway placement
    pub const fn can_be_hallway(self) -> bool {
        matches!(self, Self::Hallway | Self::RoomExit | Self::Unused)
    }
}

/// map of notable tiles in each tile position for dungeon
#[derive(Debug, Deref, DerefMut, Clone, Default)]
pub struct TileGraph {
    /// the actual graph structure
    #[deref]
    pub graph: Graph<TileGraphNode, TileGraphEdge, Undirected>,
    /// the x or y size of the dungeon
    pub size: u32,
    pub border: u32,
    /// the center translation of the dungeon
    pub center: Vec2,
}

impl TileGraph {
    /// creates a new tilegraph
    pub fn new(size: u32, border: u32, center: Vec2) -> Self {
        Self {
            graph: Graph::new_undirected(),
            size,
            border,
            center,
        }
    }

    /// get tiles translation relative too center of dungeon
    pub fn get_tiles_translation(&self, coords: UVec2) -> Vec2 {
        let cx = self.center.x;
        let cy = self.center.y;

        Vec2::new(
            (coords.x as f32).mul_add(TILE_SIZE, cx) - (((self.size - self.border) as f32 * TILE_SIZE) / 2.0),
            (coords.y as f32).mul_add(TILE_SIZE, cy) - (((self.size - self.border) as f32 * TILE_SIZE) / 2.0),
        )
    }

    /// finds `node_index` for given tile position
    pub fn get_node_at_translation(&self, position: IVec2) -> Option<NodeIndex> {
        self.node_references()
            .find(|f| {
                let node_pos = self.get_tiles_translation(f.1.tile);
                node_pos.as_ivec2() == position
            })
            .map(|f| f.0)
    }
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

impl std::ops::Add for TileGraphEdge {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            cost: self.cost + rhs.cost,
        }
    }
}

impl<'a> std::ops::Add for &'a TileGraphEdge {
    type Output = TileGraphEdge;

    fn add(self, rhs: Self) -> Self::Output {
        TileGraphEdge {
            cost: self.cost + rhs.cost,
        }
    }
}

/// outputs dungeon graph into dot file
pub fn output_graph_dot(
    graph: &Graph<TileGraphNode, TileGraphEdge, petgraph::prelude::Undirected, u32>,
) {
    let graph = petgraph::dot::Dot::with_config(graph, &[petgraph::dot::Config::EdgeNoLabel]);
    let graph = format!("{graph:?}");

    if let Err(e) = fs::write("pathmap.dot", graph) {
        warn!("error saving dot file: {}", e);
    }
}

/// turns dungeon graph into image
pub fn output_graph_image(
    graph: &Graph<TileGraphNode, TileGraphEdge, petgraph::prelude::Undirected, u32>,
) {
    let size = 768;

    let black = Rgba([0, 0, 0, 255]);
    let white = Rgba([255, 255, 255, 255]);
    let red = Rgba([255, 20, 20, 255]);
    let yellow = Rgba([255, 130, 20, 255]);
    let green = Rgba([20, 255, 20, 255]);
    let blue = Rgba([20, 20, 255, 255]);

    // Calculate the center offset
    let center_offset = size / 2;

    let mut image: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(size, size);

    for pixel in image.pixels_mut() {
        *pixel = black;
    }

    let nodes = graph.node_references();

    for (_idx, node) in nodes {
        let UVec2 { x, y } = node.tile;
        // Calculate the centered coordinates
        let centered_x = x + center_offset;
        let centered_y = y + center_offset;

        // draw in reverse becauase image/bevy have different y axis rulkes
        let y = size - centered_y;
        let x = centered_x;

        match node.data {
            TileType::Floor => image.get_pixel_mut(x, y).clone_from(&blue),
            TileType::Wall => image.get_pixel_mut(x, y).clone_from(&green),
            TileType::Unused => image.get_pixel_mut(x, y).clone_from(&white),
            TileType::Hallway => image.get_pixel_mut(x, y).clone_from(&red),
            TileType::RoomExit => image.get_pixel_mut(x, y).clone_from(&yellow),
        }
    }

    if let Err(e) = image.save("pathmap.png") {
        warn!("error saving image: {}", e);
    }
}
