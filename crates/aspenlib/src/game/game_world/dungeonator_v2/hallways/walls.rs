use std::collections::VecDeque;

use crate::game::{
    characters::components::CardinalDirection,
    game_world::dungeonator_v2::{
        hallways::hallway_builder::{spawn_tile, tile_is_corner, TexID},
        tile_graph::TileGraph,
    },
};

use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::{TilePos, TileStorage};

/// spawns straight sections for given hallway node
pub fn spawn_straight_section(
    tile_graph: &mut TileGraph,
    path_with_direction: &VecDeque<(usize, TilePos, CardinalDirection)>,
    c_dir: &CardinalDirection,
    c_coord: TilePos,
    hallway_storage: &mut Mut<'_, TileStorage>,
    parent: &mut ChildBuilder<'_>,
    hallway_container: Entity,
) {
    let (wall_a, wall_b, _double_tex) = match c_dir {
        CardinalDirection::North | CardinalDirection::South => (
            (
                TilePos {
                    x: c_coord.x + 1,
                    y: c_coord.y,
                },
                (TexID::WallEast),
            ),
            (
                TilePos {
                    x: c_coord.x - 1,
                    y: c_coord.y,
                },
                TexID::WallWest,
            ),
            TexID::WDoubleHori,
        ),
        CardinalDirection::East | CardinalDirection::West => (
            (
                TilePos {
                    x: c_coord.x,
                    y: c_coord.y + 1,
                },
                TexID::WallNorth,
            ),
            (
                TilePos {
                    x: c_coord.x,
                    y: c_coord.y - 1,
                },
                TexID::WallSouth,
            ),
            TexID::WDoubleVert,
        ),
    };

    let node_a = tile_graph.get_node_at_coord(wall_a.0.into()).expect("msg");
    let node_b = tile_graph.get_node_at_coord(wall_b.0.into()).expect("msg");

    if let Some(node_a) = tile_graph.node_weight(node_a) {
        if !node_a.data.is_hallway() {
            spawn_tile(
                path_with_direction,
                tile_graph,
                wall_a.0,
                wall_a.1,
                parent,
                hallway_container,
                hallway_storage,
            );
        }
    }
    if let Some(node_b) = tile_graph.node_weight(node_b) {
        if !node_b.data.is_hallway() {
            spawn_tile(
                path_with_direction,
                tile_graph,
                wall_b.0,
                wall_b.1,
                parent,
                hallway_container,
                hallway_storage,
            );
        }
    }
}

#[allow(clippy::too_many_lines, clippy::similar_names)]
/// spawns appropriate corner tile for given tile position
pub fn spawn_corner_section(
    tile_graph: &mut TileGraph,
    path_with_direction: &VecDeque<(usize, TilePos, CardinalDirection)>,
    c_idx: usize,
    c_dir: &CardinalDirection,
    c_coord: TilePos,
    parent: &mut ChildBuilder<'_>,
    hallway_container: Entity,
    hallway_storage: &mut Mut<'_, TileStorage>,
) {
    // // if next corner is start of another corner
    if tile_is_corner(tile_graph, path_with_direction, c_idx).is_some_and(|f| f) {
        let next_tile_direction = &path_with_direction
            .get(c_idx + 1)
            .map_or_else(|| c_dir.clone(), |f| f.2.clone());

        match (c_dir, next_tile_direction) {
            (CardinalDirection::South, CardinalDirection::East) => {
                let incorner = UVec2 {
                    x: c_coord.x.saturating_add(1),
                    y: c_coord.y,
                };
                let outwall = UVec2 {
                    x: c_coord.x.saturating_sub(1),
                    y: c_coord.y.saturating_sub(1),
                };
                let outcorner = UVec2 {
                    x: outwall.x,
                    y: outwall.y.saturating_sub(1),
                };

                let in_corner: TilePos = incorner.into();
                let out_wall: TilePos = outwall.into();
                let out_corner: TilePos = outcorner.into();

                spawn_tile(
                    path_with_direction,
                    tile_graph,
                    in_corner,
                    TexID::IcornerSw,
                    parent,
                    hallway_container,
                    hallway_storage,
                );
                spawn_tile(
                    path_with_direction,
                    tile_graph,
                    out_corner,
                    TexID::OcornerNe,
                    parent,
                    hallway_container,
                    hallway_storage,
                );

                // if previous is corner { spawn corner peice} else {spawn wall peice}
                if tile_is_corner(tile_graph, path_with_direction, c_idx.saturating_sub(1))
                    .is_some_and(|tile_is_corner| !tile_is_corner)
                    || c_idx == 0
                {
                    spawn_tile(
                        path_with_direction,
                        tile_graph,
                        out_wall,
                        TexID::WallWest,
                        parent,
                        hallway_container,
                        hallway_storage,
                    );
                } else if tile_is_corner(tile_graph, path_with_direction, c_idx.saturating_sub(1))
                    .is_some_and(|tile_is_corner| tile_is_corner)
                {
                    let new_corner = TilePos {
                        x: in_corner.x,
                        y: in_corner.y + 1,
                    };
                    spawn_tile(
                        path_with_direction,
                        tile_graph,
                        out_wall,
                        TexID::IcornerNe,
                        parent,
                        hallway_container,
                        hallway_storage,
                    );
                    spawn_tile(
                        path_with_direction,
                        tile_graph,
                        new_corner,
                        TexID::OcornerSw,
                        parent,
                        hallway_container,
                        hallway_storage,
                    );
                }
            }
            (CardinalDirection::East, CardinalDirection::South) => {
                let incorner = UVec2 {
                    x: c_coord.x,
                    y: c_coord.y.saturating_sub(1),
                };
                let outwall = UVec2 {
                    x: c_coord.x.saturating_add(1),
                    y: c_coord.y.saturating_add(1),
                };
                let outcorner = UVec2 {
                    x: outwall.x.saturating_add(1),
                    y: outwall.y,
                };
                let in_corner: TilePos = incorner.into();
                let out_corner: TilePos = outcorner.into();
                let out_wall: TilePos = outwall.into();

                spawn_tile(
                    path_with_direction,
                    tile_graph,
                    in_corner,
                    TexID::IcornerNe,
                    parent,
                    hallway_container,
                    hallway_storage,
                );
                spawn_tile(
                    path_with_direction,
                    tile_graph,
                    out_corner,
                    TexID::OcornerSw,
                    parent,
                    hallway_container,
                    hallway_storage,
                );
                // if previous is not corner { spawn corner peice} else {spawn wall peice}
                if tile_is_corner(tile_graph, path_with_direction, c_idx.saturating_sub(1))
                    .is_some_and(|tile_is_corner| !tile_is_corner)
                    || c_idx == 0
                {
                    spawn_tile(
                        path_with_direction,
                        tile_graph,
                        out_wall,
                        TexID::WallNorth,
                        parent,
                        hallway_container,
                        hallway_storage,
                    );
                } else if tile_is_corner(tile_graph, path_with_direction, c_idx.saturating_sub(1))
                    .is_some_and(|tile_is_corner| tile_is_corner)
                {
                    let new_corner = TilePos {
                        x: in_corner.x - 1,
                        y: in_corner.y,
                    };
                    spawn_tile(
                        path_with_direction,
                        tile_graph,
                        out_wall,
                        TexID::IcornerSw,
                        parent,
                        hallway_container,
                        hallway_storage,
                    );
                    spawn_tile(
                        path_with_direction,
                        tile_graph,
                        new_corner,
                        TexID::OcornerNe,
                        parent,
                        hallway_container,
                        hallway_storage,
                    );
                }
            }
            (CardinalDirection::North, CardinalDirection::East) => {
                let incorner = UVec2 {
                    x: c_coord.x.saturating_add(1),
                    y: c_coord.y,
                };
                let outwall = UVec2 {
                    x: c_coord.x.saturating_sub(1),
                    y: c_coord.y.saturating_add(1),
                };
                let outcorner = UVec2 {
                    x: outwall.x,
                    y: outwall.y.saturating_add(1),
                };
                let in_corner: TilePos = incorner.into();
                let out_corner: TilePos = outcorner.into();
                let out_wall: TilePos = outwall.into();

                spawn_tile(
                    path_with_direction,
                    tile_graph,
                    in_corner,
                    TexID::IcornerNw,
                    parent,
                    hallway_container,
                    hallway_storage,
                );
                spawn_tile(
                    path_with_direction,
                    tile_graph,
                    out_corner,
                    TexID::OcornerSe,
                    parent,
                    hallway_container,
                    hallway_storage,
                );
                // if previous is not corner { spawn corner peice} else {spawn wall peice}
                if tile_is_corner(tile_graph, path_with_direction, c_idx.saturating_sub(1))
                    .is_some_and(|tile_is_corner| !tile_is_corner)
                    || c_idx == 0
                {
                    spawn_tile(
                        path_with_direction,
                        tile_graph,
                        out_wall,
                        TexID::WallWest,
                        parent,
                        hallway_container,
                        hallway_storage,
                    );
                } else if tile_is_corner(tile_graph, path_with_direction, c_idx.saturating_sub(1))
                    .is_some_and(|tile_is_corner| tile_is_corner)
                {
                    let new_corner = TilePos {
                        x: in_corner.x,
                        y: in_corner.y - 1,
                    };
                    spawn_tile(
                        path_with_direction,
                        tile_graph,
                        out_wall,
                        TexID::IcornerSe,
                        parent,
                        hallway_container,
                        hallway_storage,
                    );
                    spawn_tile(
                        path_with_direction,
                        tile_graph,
                        new_corner,
                        TexID::OcornerNw,
                        parent,
                        hallway_container,
                        hallway_storage,
                    );
                }
            }
            (CardinalDirection::East, CardinalDirection::North) => {
                let incorner = UVec2 {
                    x: c_coord.x,
                    y: c_coord.y.saturating_add(1),
                };
                let outwall = UVec2 {
                    x: c_coord.x.saturating_add(1),
                    y: c_coord.y.saturating_sub(1),
                };
                let outcorner = UVec2 {
                    x: outwall.x.saturating_add(1),
                    y: outwall.y,
                };

                let in_corner: TilePos = incorner.into();
                let out_corner: TilePos = outcorner.into();
                let out_wall: TilePos = outwall.into();

                spawn_tile(
                    path_with_direction,
                    tile_graph,
                    in_corner,
                    TexID::IcornerSe,
                    parent,
                    hallway_container,
                    hallway_storage,
                );
                spawn_tile(
                    path_with_direction,
                    tile_graph,
                    out_corner,
                    TexID::OcornerNw,
                    parent,
                    hallway_container,
                    hallway_storage,
                );
                // if previous is not corner { spawn corner peice} else {spawn wall peice}
                if tile_is_corner(tile_graph, path_with_direction, c_idx.saturating_sub(1))
                    .is_some_and(|tile_is_corner| !tile_is_corner)
                    || c_idx == 0
                {
                    spawn_tile(
                        path_with_direction,
                        tile_graph,
                        out_wall,
                        TexID::WallSouth,
                        parent,
                        hallway_container,
                        hallway_storage,
                    );
                } else if tile_is_corner(tile_graph, path_with_direction, c_idx.saturating_sub(1))
                    .is_some_and(|tile_is_corner| tile_is_corner)
                {
                    let new_corner = TilePos {
                        x: in_corner.x - 1,
                        y: in_corner.y,
                    };
                    spawn_tile(
                        path_with_direction,
                        tile_graph,
                        out_wall,
                        TexID::IcornerNw,
                        parent,
                        hallway_container,
                        hallway_storage,
                    );
                    spawn_tile(
                        path_with_direction,
                        tile_graph,
                        new_corner,
                        TexID::OcornerSe,
                        parent,
                        hallway_container,
                        hallway_storage,
                    );
                }
            }
            (CardinalDirection::North, CardinalDirection::West) => {
                let incorner = UVec2 {
                    x: c_coord.x.saturating_sub(1),
                    y: c_coord.y,
                };
                let outwall = UVec2 {
                    x: c_coord.x.saturating_add(1),
                    y: c_coord.y.saturating_add(1),
                };
                let outcorner = UVec2 {
                    x: outwall.x,
                    y: outwall.y.saturating_add(1),
                };

                let in_corner: TilePos = incorner.into();
                let out_corner: TilePos = outcorner.into();
                let out_wall: TilePos = outwall.into();

                spawn_tile(
                    path_with_direction,
                    tile_graph,
                    in_corner,
                    TexID::IcornerNe,
                    parent,
                    hallway_container,
                    hallway_storage,
                );
                spawn_tile(
                    path_with_direction,
                    tile_graph,
                    out_corner,
                    TexID::OcornerSw,
                    parent,
                    hallway_container,
                    hallway_storage,
                );
                // if previous is not corner { spawn corner peice} else {spawn wall peice}
                if tile_is_corner(tile_graph, path_with_direction, c_idx.saturating_sub(1))
                    .is_some_and(|tile_is_corner| !tile_is_corner)
                    || c_idx == 0
                {
                    spawn_tile(
                        path_with_direction,
                        tile_graph,
                        out_wall,
                        TexID::WallEast,
                        parent,
                        hallway_container,
                        hallway_storage,
                    );
                } else if tile_is_corner(tile_graph, path_with_direction, c_idx.saturating_sub(1))
                    .is_some_and(|tile_is_corner| tile_is_corner)
                {
                    let new_corner = TilePos {
                        x: in_corner.x - 1,
                        y: in_corner.y,
                    };
                    spawn_tile(
                        path_with_direction,
                        tile_graph,
                        out_wall,
                        TexID::IcornerSw,
                        parent,
                        hallway_container,
                        hallway_storage,
                    );
                    spawn_tile(
                        path_with_direction,
                        tile_graph,
                        new_corner,
                        TexID::OcornerNe,
                        parent,
                        hallway_container,
                        hallway_storage,
                    );
                }
            }
            (CardinalDirection::West, CardinalDirection::North) => {
                let incorner = UVec2 {
                    x: c_coord.x,
                    y: c_coord.y.saturating_add(1),
                };
                let outwall = UVec2 {
                    x: c_coord.x.saturating_sub(1),
                    y: c_coord.y.saturating_sub(1),
                };
                let outcorner = UVec2 {
                    x: outwall.x.saturating_sub(1),
                    y: outwall.y,
                };

                let in_corner: TilePos = incorner.into();
                let out_corner: TilePos = outcorner.into();
                let out_wall: TilePos = outwall.into();

                spawn_tile(
                    path_with_direction,
                    tile_graph,
                    in_corner,
                    TexID::IcornerSw,
                    parent,
                    hallway_container,
                    hallway_storage,
                );
                spawn_tile(
                    path_with_direction,
                    tile_graph,
                    out_corner,
                    TexID::OcornerNe,
                    parent,
                    hallway_container,
                    hallway_storage,
                );
                // if previous is not corner { spawn corner peice} else {spawn wall peice}
                if tile_is_corner(tile_graph, path_with_direction, c_idx.saturating_sub(1))
                    .is_some_and(|tile_is_corner| !tile_is_corner)
                    || c_idx == 0
                {
                    spawn_tile(
                        path_with_direction,
                        tile_graph,
                        out_wall,
                        TexID::WallSouth,
                        parent,
                        hallway_container,
                        hallway_storage,
                    );
                } else if tile_is_corner(tile_graph, path_with_direction, c_idx.saturating_sub(1))
                    .is_some_and(|tile_is_corner| tile_is_corner)
                {
                    let new_corner = TilePos {
                        x: in_corner.x + 1,
                        y: in_corner.y,
                    };
                    spawn_tile(
                        path_with_direction,
                        tile_graph,
                        out_wall,
                        TexID::IcornerNe,
                        parent,
                        hallway_container,
                        hallway_storage,
                    );
                    spawn_tile(
                        path_with_direction,
                        tile_graph,
                        new_corner,
                        TexID::OcornerSw,
                        parent,
                        hallway_container,
                        hallway_storage,
                    );
                }
            }
            (CardinalDirection::West, CardinalDirection::South) => {
                let incorner = UVec2 {
                    x: c_coord.x,
                    y: c_coord.y.saturating_sub(1),
                };
                let outwall = UVec2 {
                    x: c_coord.x.saturating_sub(1),
                    y: c_coord.y.saturating_add(1),
                };
                let outcorner = UVec2 {
                    x: outwall.x.saturating_sub(1),
                    y: outwall.y,
                };

                let in_corner: TilePos = incorner.into();
                let out_corner: TilePos = outcorner.into();
                let out_wall: TilePos = outwall.into();

                spawn_tile(
                    path_with_direction,
                    tile_graph,
                    in_corner,
                    TexID::IcornerNw,
                    parent,
                    hallway_container,
                    hallway_storage,
                );
                spawn_tile(
                    path_with_direction,
                    tile_graph,
                    out_corner,
                    TexID::OcornerSe,
                    parent,
                    hallway_container,
                    hallway_storage,
                );
                // if previous is not corner { spawn corner peice} else {spawn wall peice}
                if tile_is_corner(tile_graph, path_with_direction, c_idx.saturating_sub(1))
                    .is_some_and(|tile_is_corner| !tile_is_corner)
                    || c_idx == 0
                {
                    spawn_tile(
                        path_with_direction,
                        tile_graph,
                        out_wall,
                        TexID::WallNorth,
                        parent,
                        hallway_container,
                        hallway_storage,
                    );
                } else if tile_is_corner(tile_graph, path_with_direction, c_idx.saturating_sub(1))
                    .is_some_and(|tile_is_corner| tile_is_corner)
                {
                    let new_corner = TilePos {
                        x: in_corner.x + 1,
                        y: in_corner.y,
                    };
                    spawn_tile(
                        path_with_direction,
                        tile_graph,
                        out_wall,
                        TexID::IcornerSe,
                        parent,
                        hallway_container,
                        hallway_storage,
                    );
                    spawn_tile(
                        path_with_direction,
                        tile_graph,
                        new_corner,
                        TexID::OcornerNw,
                        parent,
                        hallway_container,
                        hallway_storage,
                    );
                }
            }
            (CardinalDirection::South, CardinalDirection::West) => {
                let incorner = UVec2 {
                    x: c_coord.x.saturating_sub(1),
                    y: c_coord.y,
                };
                let outwall = UVec2 {
                    x: c_coord.x.saturating_add(1),
                    y: c_coord.y.saturating_sub(1),
                };
                let outcorner = UVec2 {
                    x: outwall.x,
                    y: outwall.y.saturating_sub(1),
                };

                let in_corner: TilePos = incorner.into();
                let out_corner: TilePos = outcorner.into();
                let out_wall: TilePos = outwall.into();

                spawn_tile(
                    path_with_direction,
                    tile_graph,
                    in_corner,
                    TexID::IcornerSe,
                    parent,
                    hallway_container,
                    hallway_storage,
                );
                spawn_tile(
                    path_with_direction,
                    tile_graph,
                    out_corner,
                    TexID::OcornerNw,
                    parent,
                    hallway_container,
                    hallway_storage,
                );
                // if previous is not corner { spawn corner peice} else {spawn wall peice}
                if tile_is_corner(tile_graph, path_with_direction, c_idx.saturating_sub(1))
                    .is_some_and(|tile_is_corner| !tile_is_corner)
                    || c_idx == 0
                {
                    spawn_tile(
                        path_with_direction,
                        tile_graph,
                        out_wall,
                        TexID::WallEast,
                        parent,
                        hallway_container,
                        hallway_storage,
                    );
                } else if tile_is_corner(tile_graph, path_with_direction, c_idx.saturating_sub(1))
                    .is_some_and(|tile_is_corner| tile_is_corner)
                {
                    let new_corner = TilePos {
                        x: in_corner.x,
                        y: in_corner.y + 1,
                    };
                    spawn_tile(
                        path_with_direction,
                        tile_graph,
                        out_wall,
                        TexID::IcornerNw,
                        parent,
                        hallway_container,
                        hallway_storage,
                    );
                    spawn_tile(
                        path_with_direction,
                        tile_graph,
                        new_corner,
                        TexID::OcornerSe,
                        parent,
                        hallway_container,
                        hallway_storage,
                    );
                }
            }
            _a => {
                // panic!("hit double directions even though it should be different: {a:?}")
            }
        }
    }
}
