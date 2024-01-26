use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use seldom_map_nav::prelude::Pathfind;

use crate::{
    consts::TILE_SIZE,
    game::game_world::dungeonator_v2::{
        hallways::PlacedHallWay, path_map::PathMap, GeneratorState,
    },
};

// TODO: create a grid of occupied/unoccupied tiles encompassing the whole dungueon
// use pathfinding algorithm/crates too get 2 paths, path should return as Vec<tile positon>
// iterate over each position for 3 layers
// building
// collisons
// something else?
pub fn build_hallways(
    mut cmds: Commands,
    pathmap: Query<
        (
            Entity,
            &GlobalTransform,
            &TilemapSize,
            &TilemapGridSize,
            &TilemapType,
        ),
        With<PathMap>,
    >,
    hallways: Query<(Entity, &Pathfind, &PlacedHallWay), With<Name>>,
) {
    cmds.insert_resource(NextState(Some(GeneratorState::FinishedDungeonGen)))
}
