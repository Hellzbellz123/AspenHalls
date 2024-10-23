use bevy::prelude::*;
use petgraph::{Graph, Undirected};

use crate::game::game_world::{
    components::RoomExit,
    dungeonator_v2::{components::RoomBlueprint, hallways::SiblingCount},
};

/// implementation for roomgraph
pub mod r#impl;

#[derive(Debug, Deref, DerefMut, Clone, Default, Reflect)]
/// rooms and hallways for a given dungeon
pub struct RoomGraph {
    /// underlying room and hallway information
    #[deref]
    #[reflect(ignore)] // TODO: fix reflect for graphs
    pub graph: Graph<RoomGraphNode, RoomGraphEdge, Undirected>,
}

/// node for dungeon graph structure
#[derive(Debug, PartialEq, Eq, Clone, Reflect)]
pub enum RoomGraphNode {
    /// node is a dungeon room
    Room(RoomBlueprint),
    /// node is a dungeon room exit
    Exit {
        /// 2 tiles that makeup exit
        exit: RoomExit,
        /// how many other exits are related too this exit
        brothers: SiblingCount,
    },
}

/// aka hallway
#[derive(Debug, PartialEq, PartialOrd, Clone, Reflect)]
pub struct RoomGraphEdge {
    /// how long is this connection between rooms
    pub length: f32,
}
