use bevy::prelude::*;
use bevy_ecs_ldtk::GridCoords;
use bevy_ecs_tilemap::prelude::TilemapSize;
use bevy_rapier2d::geometry::Collider;

use image::{ImageBuffer, Rgba};
use petgraph::{prelude::NodeIndex, visit::IntoNodeReferences, Graph, Undirected};
use std::fs;

use crate::{
    consts::TILE_SIZE,
    game::game_world::{
        components::RoomExitTile,
        dungeonator_v2::{
            components::{Dungeon, DungeonSettings},
            ensure_tile_pos,
            hallways::hallway_builder::distance_tiles,
            tile_graph::{TileGraph, TileGraphEdge, TileGraphNode, TileType},
        },
        RoomBoundryTile,
    },
};

#[allow(clippy::type_complexity)]
/// check each tile position for a tile in gameworld and add node too list
pub fn populate_tilegraph(
    dungeon: &mut Dungeon,
    tile_query: Query<
        (
            &GlobalTransform,
            Option<&Collider>,
            Option<&RoomExitTile>,
            Option<&RoomBoundryTile>,
        ),
        With<GridCoords>,
    >,
) {
    let tile_graph = &mut dungeon.tile_graph;
    let settings = &dungeon.settings;

    assert!(!tile_query.is_empty(), "no tiles too map");
    for x in 0..=settings.size.x {
        for y in 0..=settings.size.y {
            let coords = UVec2 { x, y };

            // Calculate the global translation of the tile based on dungeon position and coords
            let tile_translation = tile_graph.get_tiles_translation_world(settings, coords);
            let tile_on_position = tile_occupies_position(&tile_query, tile_translation);

            tile_graph.add_node(TileGraphNode {
                tile: coords,
                data: tile_on_position,
            });
        }
    }

    info!("connecting adjectent nodes in graph");
    connect_adjacent_nodes(dungeon);
    info!("finished connecting adjacent nodes");

    // TODO: for RoomNode in graph, for n.min.x..n.max.x
    // does not add borders too rooms for paths
    // let graph_copy = tile_graph.clone();
    // let room_graph = &dungeon.room_graph;
    // for x in 0..=settings.size.x {
    //     for y in 0..=settings.size.y {
    //         let coords = UVec2 { x, y };
    //         let tile_on_position = TileType::Unused;

    //         tile_graph.add_node(TileGraphNode {
    //             tile: coords,
    //             data: tile_on_position,
    //         });
    //     }
    // }
    // let room_areas: Vec<(UVec2, UVec2)> = room_graph.get_rooms().iter().map(|f| {
    //     (
    //         tile_graph.get_position_tile_coords(settings, f.position.as_vec2()),
    //         tile_graph.get_position_tile_coords(settings, (f.position + f.size).as_vec2()),
    //     )
    // }).collect();

    // for room in room_areas {
    //     for x in room.0.x..=room.1.x {
    //         for y in room.0.y..=room.1.y {
    //             let coords = UVec2 { x, y };
    //             // Calculate the actual translation of the tile based on dungeon position
    //             let tile_translation = tile_graph.get_tiles_translation_world(settings, coords);
    //             let tile_on_position = tile_occupies_position(&tile_query, tile_translation);
    //             let tile_id = tile_graph.get_node_at_coord(coords).expect("msg");

    //             if tile_on_position.is_wall() {
    //                 // remove all edges of this tile
    //                 let edges = graph_copy.edges(tile_id.id());
    //                 for edge in edges {
    //                     tile_graph.remove_edge(edge.id());
    //                 }
    //             }

    //             if !tile_on_position.is_unused() {
    //                 // get tile idx and mutably change value too proper tiletype
    //                 tile_graph.node_weight_mut(tile_id).expect("msg").data = tile_on_position
    //             }
    //         }
    //     }
    // }
}

// clippy really doesnt like this code
// TODO: find a better solution for this garbage
#[allow(clippy::type_complexity)]
#[allow(clippy::unnecessary_find_map)]
/// does position have a tile
pub fn tile_occupies_position(
    tiles: &Query<
        (
            &GlobalTransform,
            Option<&Collider>,
            Option<&RoomExitTile>,
            Option<&RoomBoundryTile>,
        ),
        With<GridCoords>,
    >,
    position: Vec2,
) -> TileType {
    tiles
        .iter()
        .filter(|(transform, ..)| transform.translation().truncate() == position)
        .find_map(
            |(_, exit, collider, roomboundry)| match (exit, collider, roomboundry) {
                (Some(_), None, None) => Some(TileType::Wall),
                (None, Some(_), None) => Some(TileType::RoomExit),
                (None, None, Some(_)) => Some(TileType::Unused),
                (None, None, None) => Some(TileType::Floor),
                _ => panic!("tile had exit collider and room boundry or some combination of these"),
            },
        )
        .unwrap_or(TileType::Unused)
}

//TODO: performance-: precalculate empty spaces using room dimensions, then check inside room dimensions for walkability
// it might be faster too do this 'destructively'.
// I.e. create edges as tilegraph is being made, for every tile, then remove all edges for tiles that are unwalkable

/// connects adjacent nodes in graph
/// 1.041611 seconds
/// 0.874678 | 0.677892 | 0.452816 with `last_node` sort
pub fn connect_adjacent_nodes(dungeon: &mut Dungeon) {
    // let references: Vec<(NodeIndex, &TileGraphNode)> = graph.node_references().collect();
    let mut edges: Vec<(NodeIndex, NodeIndex, TileGraphEdge)> = Vec::new();
    let tilegraph = &mut dungeon.tile_graph;

    info!("filtering adjacent nodes in tilegraph for walkability");
    let mut walkable_nodes: Vec<NodeIndex> = tilegraph
        .node_indices()
        .filter(|f| tilegraph[*f].data.can_be_hallway())
        .collect::<Vec<NodeIndex>>();

    let tile_center: UVec2 = <TilemapSize as Into<UVec2>>::into(dungeon.settings.size) / 2;

    let mut last_node = tile_center;
    // sort by distance from center, this makes checking for
    walkable_nodes.sort_by(|a, b| {
        let TileGraphNode { tile: tile_a, .. } = tilegraph[*a];
        let TileGraphNode { tile: tile_b, .. } = tilegraph[*b];
        let distance_a = distance_tiles(last_node, tile_a);
        let distance_b = distance_tiles(last_node, tile_b);
        last_node = if distance_a > distance_b {
            tile_a
        } else {
            tile_b
        };
        distance_a.cmp(&distance_b)
    });

    info!("pairing walkable nodes");
    for (i, current_idx) in walkable_nodes.iter().enumerate() {
        let current = &tilegraph[*current_idx];
        let mut connect_count = 0;

        if connect_count == 4 {
            continue;
        }

        for other_idx in walkable_nodes.iter().skip(i + 1) {
            let other = &tilegraph[*other_idx];

            if is_adjacent(current.tile, other.tile) {
                connect_count += 1;
                edges.push((
                    *current_idx,
                    *other_idx,
                    TileGraphEdge {
                        cost: calculate_weight(
                            &(*current_idx, current),
                            &(*other_idx, other),
                            tilegraph,
                        ),
                    },
                ));
            }
        }
    }

    info!("extending graph with edges");
    tilegraph.extend_with_edges(edges);
}

/// calculates edge weight for 2 graph nodes
pub fn calculate_weight(
    current: &(NodeIndex, &TileGraphNode),
    other: &(NodeIndex, &TileGraphNode),
    graph: &Graph<TileGraphNode, TileGraphEdge, Undirected, u32>,
) -> f32 {
    let (current_index, current_node) = current;
    let (other_index, other_node) = other;

    assert!(
        current_index != other_index,
        "calculating weights between same tiles not allowed"
    );

    // TODO: maybe change this assert too only check 1 tile distance
    assert!(
        distance_tiles(current_node.tile, other_node.tile) == 1
            || distance_tiles(current_node.tile, other_node.tile) == 2,
        "tiles are not adjacent: {}",
        distance_tiles(current_node.tile, other_node.tile)
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
pub const fn is_adjacent(pos1: UVec2, pos2: UVec2) -> bool {
    let row_diff = (pos1.x as i32 - pos2.x as i32).abs();
    let col_diff = (pos1.y as i32 - pos2.y as i32).abs();

    // Nodes are adjacent if their row or column difference is less than or equal to 1
    (row_diff == 1 && col_diff == 0) || (row_diff == 0 && col_diff == 1) //|| (row_diff == 1 && col_diff == 1)
}

#[allow(clippy::type_complexity)]
/// gets tilemap size in tiles
pub fn actual_map_tile_size(
    _settings: &DungeonSettings,
    tile_query: &Query<
        (
            &GlobalTransform,
            Option<&Collider>,
            Option<&RoomExitTile>,
            Option<&RoomBoundryTile>,
        ),
        With<GridCoords>,
    >,
) -> (TilemapSize, Vec2) {
    let (max_x, max_y, min_x, min_y) = calculate_tile_extents(tile_query);

    let max_tile_pos = Vec2 { x: max_x, y: max_y };
    let min_tile_pos = Vec2 { x: min_x, y: min_y };

    let expanded_rect = Rect::from_corners(max_tile_pos, min_tile_pos);

    let y_size = (expanded_rect.max.y - expanded_rect.min.y).abs();
    let x_size = (expanded_rect.max.x - expanded_rect.min.x).abs();

    (
        TilemapSize {
            x: (x_size / TILE_SIZE) as u32,
            y: (y_size / TILE_SIZE) as u32,
        },
        Vec2 {
            x: ensure_tile_pos(expanded_rect.center().x), // - 16.0,
            y: ensure_tile_pos(expanded_rect.center().y), // - 16.0,
        },
    )
}

/// finds furthest 4 corners of tile map
#[allow(clippy::type_complexity)]
pub fn calculate_tile_extents(
    tile_query: &Query<
        (
            &GlobalTransform,
            Option<&Collider>,
            Option<&RoomExitTile>,
            Option<&RoomBoundryTile>,
        ),
        With<GridCoords>,
    >,
) -> (f32, f32, f32, f32) {
    // Use map to extract the Vec2 translations from the iterator
    let translations: Vec<Vec2> = tile_query
        .iter()
        .filter(|f| f.3.is_some())
        // turn tile query into positions
        .map(|f| f.0.translation().truncate())
        // map data too translations
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

    // (ensure_tile_pos(max_x), ensure_tile_pos(max_y), ensure_tile_pos(min_x), ensure_tile_pos(min_y))
    (max_x, max_y, min_x, min_y)
}

impl TileGraph {
    /// turn tile coordinates into world translation
    pub fn get_tiles_translation_world(&self, settings: &DungeonSettings, coords: UVec2) -> Vec2 {
        let center = self.center_world;
        let corner_offset = Vec2 {
            x: (settings.size.x as f32 * TILE_SIZE),
            y: (settings.size.y as f32 * TILE_SIZE),
        };

        let p0 = center - (corner_offset / 2.0);
        let mut coords_offset = coords.as_vec2() * TILE_SIZE; // + 16.0;

        if settings.size.x % 2 == 0 {
            coords_offset.x += 16.0;
        }
        if settings.size.y % 2 == 0 {
            coords_offset.y += 16.0;
        }

        p0 + coords_offset
    }

    /// translates world position into tile position
    pub fn get_position_tile_coords(&self, settings: &DungeonSettings, position: Vec2) -> UVec2 {
        let center = self.center_world;
        let corner_offset = Vec2 {
            x: (settings.size.x as f32 * TILE_SIZE),
            y: (settings.size.y as f32 * TILE_SIZE),
        };
        let p0 = center - (corner_offset / 2.0);
        let relative_position = position - p0;

        // Convert relative position to tile coordinates
        let mut tile_coords = UVec2::new(
            (relative_position.x / TILE_SIZE).floor() as u32,
            (relative_position.y / TILE_SIZE).floor() as u32,
        );

        // Adjust for offsets if dungeon size is even
        if settings.size.x % 2 == 0 {
            tile_coords.x -= 1;
        }
        if settings.size.y % 2 == 0 {
            tile_coords.y -= 1;
        }

        tile_coords
    }

    /// turn tile coordinates into world translation
    pub fn get_tiles_offset(&self, settings: &DungeonSettings, coords: UVec2) -> Vec2 {
        let center = self.center_world;
        let corner_offset = Vec2 {
            x: (settings.size.x as f32 * TILE_SIZE),
            y: (settings.size.y as f32 * TILE_SIZE),
        };
        let p0 = center - (corner_offset / 2.0);
        let mut coords_offset = coords.as_vec2() * TILE_SIZE;

        if settings.size.x % 2 == 0 {
            coords_offset.x += 16.0;
        }
        if settings.size.y % 2 == 0 {
            coords_offset.y += 16.0;
        }

        p0 + coords_offset
    }

    /// finds `node_index` of tile at given world translation
    pub fn get_node_at_translation(
        &self,
        settings: &DungeonSettings,
        position: IVec2,
    ) -> Option<NodeIndex> {
        self.node_references()
            .find(|f| {
                let node_pos = self.get_tiles_translation_world(settings, f.1.tile);

                node_pos.as_ivec2() == position
            })
            .map(|f| f.0)
    }

    /// finds `node_index` for given tile coord
    pub fn get_node_at_coord(&self, coords: UVec2) -> Option<NodeIndex> {
        self.node_references()
            .find(|f| f.1.tile == coords)
            .map(|f| f.0)
    }
}

impl std::ops::Add for TileGraphEdge {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            cost: self.cost + rhs.cost,
        }
    }
}

impl std::ops::Add for &TileGraphEdge {
    type Output = TileGraphEdge;

    fn add(self, rhs: Self) -> Self::Output {
        TileGraphEdge {
            cost: self.cost + rhs.cost,
        }
    }
}

impl TileType {
    /// does this tile position have a floor tile on it
    pub const fn is_floor(self) -> bool {
        matches!(self, Self::Floor)
    }
    /// does this tile have a hallway tile on it
    pub const fn is_hallway(self) -> bool {
        matches!(self, Self::Hallway)
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
    size: TilemapSize,
) {
    let black = Rgba([0, 0, 0, 255]);
    let orange = Rgba([255, 100, 0, 255]);
    let red = Rgba([255, 0, 0, 255]);
    let blue = Rgba([17, 17, 116, 255]);
    let white = Rgba([255, 255, 255, 255]);
    let yellow = Rgba([227, 242, 0, 255]);
    let green = Rgba([91, 242, 0, 255]);

    let x_size = if size.x % 2 == 0 { size.x } else { size.x + 1 };
    let y_size = if size.y % 2 == 0 { size.y } else { size.y + 1 };
    let mut image: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(x_size + 10, y_size + 10);

    for pixel in image.pixels_mut() {
        *pixel = orange;
    }

    let nodes = graph.node_references();

    for (_idx, node) in nodes {
        let UVec2 { x, y } = node.tile;
        let x = x + 5;
        let y = (y_size + 5) - y;

        match node.data {
            TileType::Unused => image.get_pixel_mut(x, y).clone_from(&white),
            TileType::Hallway => image.get_pixel_mut(x, y).clone_from(&yellow),
            TileType::Floor => image.get_pixel_mut(x, y).clone_from(&blue),
            TileType::Wall => image.get_pixel_mut(x, y).clone_from(&green),
            TileType::RoomExit => image.get_pixel_mut(x, y).clone_from(&red),
        }
    }

    let corners = [(x_size, y_size), (x_size, 0), (0, y_size), (0, 0)];

    for corner in corners {
        image
            .get_pixel_mut(corner.0 + 5, corner.1 + 5)
            .clone_from(&black);
    }

    if let Err(e) = image.save("pathmap.png") {
        warn!("error saving image: {}", e);
    }
}
