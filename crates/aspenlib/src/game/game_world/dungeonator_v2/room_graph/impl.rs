use bevy::{prelude::*, utils::HashSet};
use petgraph::{prelude::NodeIndex, Graph};
use rand::seq::IteratorRandom;

use crate::game::game_world::dungeonator_v2::{
    components::{RoomBlueprint, RoomID},
    hallways::SiblingCount,
    room_graph::{RoomGraph, RoomGraphEdge, RoomGraphNode},
};

impl RoomGraph {
    /// outputs room graph into graphviz file
    pub fn dump_too_file(&self) {
        let graph =
            petgraph::dot::Dot::with_config(&self.graph, &[petgraph::dot::Config::EdgeNoLabel]);

        if let Err(e) = std::fs::write("dungeon.dot", format!("{graph:?}")) {
            warn!("error saving dot file: {}", e);
        }
    }

    /// creates new roomgraph from given '`RoomBlueprints`'
    pub fn new(presets: &[RoomBlueprint]) -> Self {
        let mut graph = Self {
            graph: Graph::new_undirected(),
        };

        for preset in presets {
            let room = graph.add_node(RoomGraphNode::Room(preset.clone()));

            for exit in &preset.exits {
                let exit = graph.add_node(RoomGraphNode::Exit {
                    exit: exit.clone(),
                    brothers: SiblingCount(preset.exits.len() as u32),
                });
                graph.add_edge(room, exit, RoomGraphEdge { length: 0.0 });
            }
        }

        graph
    }

    /// finds unconnected groups of rooms in room graph and connects them with a hallway edge
    pub fn verify_graph_connections(&mut self) {
        let max_loop_amount = 25;
        let mut current_amount = 0;
        let room_graph = &mut self.graph;
        let mut connected_components: Vec<Vec<NodeIndex>> =
            petgraph::algo::kosaraju_scc(&*room_graph);

        loop {
            if connected_components.len() > 1 {
                if current_amount >= max_loop_amount {
                    break;
                }
                warn!(
                    "graph is unconnected, fixing graph. groups : {}",
                    connected_components.len()
                );
                connected_components.sort_by_key(Vec::len);

                let (smallest_group, other_groups) = connected_components
                    .split_first()
                    .expect("graph should have len > 2");

                // TODO: collect node index into 2 lists of (exit, position),
                // get closest pair of exits in lists
                let Some(exit1_id) = other_groups
                    .first()
                    .expect("should not be empty")
                    .iter()
                    .find(|f| {
                        room_graph.node_weight(**f).expect("msg").is_exit()
                            && room_graph.edges(**f).count() == 1
                    })
                else {
                    current_amount += 1;
                    info!("no valid node in other groups");
                    continue;
                };

                let RoomGraphNode::Exit {
                    exit: exit1_data,
                    brothers: _exit1_brothers,
                } = room_graph
                    .node_weight(*exit1_id)
                    .expect("filtering based on if exit")
                else {
                    current_amount += 1;
                    info!("chosent node was not an exit");
                    continue;
                };

                let Some(exit2_id) = smallest_group
                    .iter()
                    .filter(|f| {
                        room_graph
                            .node_weight(**f)
                            .expect("node should exist")
                            .is_exit()
                            && room_graph.edges(**f).count() == 1
                    })
                    .min_by(|a, b| {
                        let a_pos = room_graph
                            .node_weight(**a)
                            .expect("node should exist")
                            .get_nodes_offset();
                        let b_pos = room_graph
                            .node_weight(**b)
                            .expect("node should exist")
                            .get_nodes_offset();
                        // not saving distance, dont need too square
                        let a_distance = ((a_pos - exit1_data.position).abs().length_squared()
                            as f32)
                            .sqrt() as i32;
                        let b_distance = ((b_pos - exit1_data.position).abs().length_squared()
                            as f32)
                            .sqrt() as i32;
                        a_distance.cmp(&b_distance)
                    })
                else {
                    current_amount += 1;
                    info!("could not get a close exit from smallest group");
                    continue;
                };

                let RoomGraphNode::Exit {
                    exit: exit2_data,
                    brothers: _exit2_brothers,
                } = room_graph
                    .node_weight(*exit2_id)
                    .expect("filtering based on if exit")
                else {
                    current_amount += 1;
                    info!("exit 2 id was not an exit");
                    continue;
                };

                // we are sqrting this distance so its not obsene
                let distance = (exit1_data.position.distance_squared(exit2_data.position) as f32)
                    .abs()
                    .sqrt();
                room_graph.add_edge(*exit1_id, *exit2_id, RoomGraphEdge { length: distance });
            } else {
                break;
            }
        }
    }

    /// randomly connects exits inside roomgraph,
    pub fn connect_graph_randomly(&mut self) {
        let mut rng = rand::thread_rng();
        let room_graph = &mut self.graph;
        let graph_copy = room_graph.clone();

        // Create a vector to store exit nodes
        let exit_nodes: Vec<(NodeIndex, &RoomGraphNode)> = room_graph
            .node_indices()
            .filter(|&node| matches!(room_graph[node], RoomGraphNode::Exit { .. }))
            .map(|f| (f, &graph_copy[f]))
            .collect();

        let mut exits_connected: HashSet<NodeIndex> = HashSet::new();

        loop {
            let Some(first_exit) = exit_nodes
                .iter()
                .filter(|f| !exits_connected.contains(&f.0))
                .choose(&mut rng)
            else {
                break;
            };

            let Some(second_exit) = exit_nodes
                .iter()
                .filter(|(id, node)| {
                    node.get_node_id() != first_exit.1.get_node_id()
                        && !exits_connected.contains(id)
                })
                // choose by distance
                .min_by(|a, b| {
                    let a_position = a.1.get_nodes_offset();
                    let b_position = b.1.get_nodes_offset();
                    // not saving distance so it can be squared
                    let a_distance = a_position.distance_squared(first_exit.1.get_nodes_offset());
                    let b_distance = b_position.distance_squared(first_exit.1.get_nodes_offset());
                    a_distance.cmp(&b_distance)
                })
            else {
                break;
            };

            let RoomGraphNode::Exit {
                exit: exit1_data,
                brothers: _exit1_brothers,
            } = first_exit.1
            else {
                panic!("node was actually a room?")
            };
            let RoomGraphNode::Exit {
                exit: exit2_data,
                brothers: _exit2_brothers,
            } = second_exit.1
            else {
                panic!("node was actually a room?")
            };

            // doing sqrt so it is acceptable too save
            let distance = (exit1_data.position - exit2_data.position).length_squared() as f32;

            if !exits_connected.contains(&first_exit.0) && !exits_connected.contains(&second_exit.0)
            {
                info!("adding edge too graph");
                exits_connected.insert(first_exit.0);
                exits_connected.insert(second_exit.0);
                room_graph.add_edge(
                    first_exit.0,
                    second_exit.0,
                    RoomGraphEdge {
                        length: distance.sqrt(),
                    },
                );
            }
        }
    }

    /// returns references to room blueprints inside roomgraph
    pub fn get_rooms(&self) -> Vec<&RoomBlueprint> {
        self.node_weights()
            .filter_map(|node: _| match node {
                RoomGraphNode::Room(a) => Some(a),
                RoomGraphNode::Exit {
                    exit: _,
                    brothers: _,
                } => None,
            })
            .collect()
    }
}

impl RoomGraphNode {
    /// is graph node an exit
    pub const fn is_exit(&self) -> bool {
        match self {
            Self::Room { .. } => false,
            Self::Exit { .. } => true,
        }
    }

    /// get graph node room id, parent id if exit
    pub const fn get_node_id(&self) -> &RoomID {
        match self {
            Self::Room(blueprint) => &blueprint.id,
            Self::Exit { exit, .. } => &exit.parent,
        }
    }

    /// get graph node position
    pub const fn get_nodes_offset(&self) -> IVec2 {
        match self {
            Self::Room(blueprint) => blueprint.room_space.min,
            Self::Exit { exit, .. } => exit.position,
        }
    }
}
