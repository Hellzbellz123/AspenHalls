use std::fs;

use bevy::{prelude::*, utils::HashSet};

use bevy_prototype_lyon::{
    draw::Stroke,
    prelude::{Fill, PathBuilder, ShapeBundle},
};
use petgraph::{
    algo::min_spanning_tree,
    data::FromElements,
    prelude::{Graph, NodeIndex, StableUnGraph},
    stable_graph::StableGraph,
    Undirected,
};
use rand::prelude::IteratorRandom;

use crate::game::{
    game_world::{
        components::RoomExit,
        dungeonator_v2::{
            components::{Dungeon, RoomBlueprint, RoomID},
            GeneratorState,
        },
    },
    interface::random_color,
};

/// hallway creation functions
pub mod hallway_builder;

/// modifys placed room blueprints with correct positions
pub fn update_room_instances(
    children: Query<&Children>,
    mut cmds: Commands,
    mut exit_query: Query<(&mut RoomExit, &GlobalTransform)>,
    mut room_query: Query<(Entity, &mut RoomBlueprint)>,
    // TODO: have this run for each room when the room is `Transformed`
    // mut level_events: EventReader<LevelEvent>,
) {
    for (room, mut room_instance) in &mut room_query {
        info!("updating room with new pos and room exit list");

        // fill room exits vec
        let mut exits: Vec<RoomExit> = Vec::new();

        for child in children.iter_descendants(room) {
            if let Ok((mut room_exit, transform)) = exit_query.get_mut(child) {
                room_exit.parent = room_instance.id;
                let position = transform.translation().truncate();
                room_exit.position = position.as_ivec2();
                // let position
                info!("position of exit: {}", room_exit.position);
                exits.push(room_exit.clone());
            }
        }

        // update exits with new merged exits
        room_instance.exits = exits;

        // update position too be at center of room
        room_instance.position = room_instance.position
            + (IVec2 {
                x: room_instance.size.x as i32,
                y: room_instance.size.y as i32,
            } / 2);
    }

    // create mst next
    info!("finished updating room positions");
    cmds.insert_resource(NextState(Some(GeneratorState::PlanHallways)));
}

// TODO: rethink this code too only spawn 1 hallway per room
// maybe try this apporach again now that the room ids are
/// create hallway blueprints from spawned rooms
pub fn plan_hallways(
    mut cmds: Commands,
    mut dungeon_root: Query<&mut Dungeon>,
    room_query: Query<(&GlobalTransform, &mut RoomBlueprint)>,
    exit_query: Query<(&GlobalTransform, &mut RoomExit)>,
) {
    // undirected graph because hallways are both ways
    // use petgraph, nodes in graph have 2 states, room/exit, hallway is edge.
    // add all rooms with exits and edges from room -> exit.
    // impl custom function that takes graph and adds all possible exit/exit edges.
    // compute MST from outputted graph
    // for edge in MST add hallway too hallways vec

    let room_graph = build_room_graph(&room_query, exit_query);
    let mut room_graph: Graph<DungeonGraphNode, DungeonGraphEdge, Undirected> =
        Graph::from_elements(min_spanning_tree(&room_graph));

    output_graph_dot(&room_graph);
    fix_graph(&mut room_graph);

    let mut settings = dungeon_root.single_mut();
    let hallways = create_hallway_plans(room_graph);

    info!(
        "room amount: {}, hallway amount: {}",
        &room_query.iter().len(),
        hallways.len()
    );

    settings.hallways = hallways;
    cmds.insert_resource(NextState(Some(GeneratorState::PlaceHallwayRoots)));
}

/// outputs room graph into graphviz file
fn output_graph_dot(graph: &Graph<DungeonGraphNode, DungeonGraphEdge, Undirected>) {
    let graph = petgraph::dot::Dot::with_config(graph, &[petgraph::dot::Config::EdgeNoLabel]);

    if let Err(e) = fs::write("dungeon.dot", format!("{graph:?}")) {
        warn!("error saving dot file: {}", e);
    }
}

/// finds unconnected groups of rooms in room graph and connects them with a hallway edge
fn fix_graph(room_graph: &mut Graph<DungeonGraphNode, DungeonGraphEdge, Undirected>) {
    loop {
        let mut connected_components: Vec<Vec<NodeIndex>> =
            petgraph::algo::kosaraju_scc(&*room_graph);

        if connected_components.len() > 1 {
            warn!("graph is bad, fixing graph. groups : {}", connected_components.len());
            warn!("connected nodes: {:?}", connected_components);
            connected_components.sort_by_key(Vec::len);

            let Some((smallest_group, other_groups)) = connected_components.split_first() else {
                return;
            };

            // TODO: collect node index into 2 lists of (exit, position),
            // get closest pair of exits in lists

            let Some(exit1_id) = other_groups
                .last()
                .expect("should not be empty")
                .iter()
                .find(|f| {
                    room_graph.node_weight(**f).expect("msg").is_exit()
                        && room_graph.edges(**f).count() == 1
                })
            else {
                continue;
            };

            let DungeonGraphNode::Exit {
                exit: exit1_data,
                brothers: _exit1_brothers,
            } = room_graph
                .node_weight(*exit1_id)
                .expect("filtering based on if exit")
            else {
                return;
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
                        .get_position();
                    let b_pos = room_graph
                        .node_weight(**b)
                        .expect("node should exist")
                        .get_position();
                    let a_distance = a_pos.distance_squared(exit1_data.position);
                    let b_distance = b_pos.distance_squared(exit1_data.position);
                    a_distance.cmp(&b_distance)
                })
            else {
                continue;
            };

            let DungeonGraphNode::Exit {
                exit: exit2_data,
                brothers: _exit2_brothers,
            } = room_graph
                .node_weight(*exit2_id)
                .expect("filtering based on if exit")
            else {
                return;
            };

            let distance = exit1_data.position.distance_squared(exit2_data.position) as f32;
            room_graph.add_edge(
                *exit1_id,
                *exit2_id,
                DungeonGraphEdge::Hallway {
                    start: (exit1_data.clone()),
                    end: (exit2_data.clone()),
                    length: distance,
                },
            );
        } else {
            break;
        }
    }
}

/// turns room graph edges into hallway blueprints
fn create_hallway_plans(
    room_graph: Graph<DungeonGraphNode, DungeonGraphEdge, Undirected>,
) -> Vec<HallWayBlueprint> {
    let mut hallways: Vec<HallWayBlueprint> = Vec::new();
    for edge_idx in room_graph.edge_indices() {
        let edge_data = room_graph
            .edge_weight(edge_idx)
            .expect("just grabbed from room graph");

        match edge_data {
            DungeonGraphEdge::Hallway { length, start, end } => hallways.push(HallWayBlueprint {
                start_pos: start.position,
                end_pos: end.position,
                distance: *length,
                connected_rooms: (start.parent, end.parent),
                built: false,
            }),
            DungeonGraphEdge::ExitLink { .. } => {
                continue;
            }
        }
    }

    assert!(
        !hallways.is_empty(),
        "hallways where not properly filled from room graph"
    );
    hallways
}

/// turns room and exit query into a `stable_graph`
fn build_room_graph(
    room_query: &Query<(&GlobalTransform, &mut RoomBlueprint)>,
    exit_query: Query<(&GlobalTransform, &mut RoomExit)>,
) -> StableGraph<DungeonGraphNode, DungeonGraphEdge, Undirected> {
    let room_amt = room_query.iter().len();
    let mut room_graph: StableUnGraph<DungeonGraphNode, DungeonGraphEdge> =
        petgraph::stable_graph::StableUnGraph::with_capacity(room_amt * 4, room_amt * room_amt);

    if exit_query.is_empty() {
        warn!("no exits exist");
    }

    if exit_query
        .iter()
        .all(|f| f.0.translation().truncate() == Vec2::ZERO)
    {
        warn!("some exit has translation set too 0,0");
    }

    for (_, dungeon_room) in room_query {
        if dungeon_room.exits.is_empty() {
            error!("no exits added too room blueprints");
        }
        let exit_amount = ExitAmount(dungeon_room.exits.len() as u32);
        let room = room_graph.add_node(DungeonGraphNode::Room {
            position: dungeon_room.position,
            id: dungeon_room.id,
        });

        for exit_id in &dungeon_room.exits {
            let exit = room_graph.add_node(DungeonGraphNode::Exit {
                exit: exit_id.clone(),
                brothers: exit_amount.clone(),
            });
            room_graph.add_edge(room, exit, DungeonGraphEdge::ExitLink { length: 0.0 });
        }
    }

    // Create a vector to store exit node indices
    let exit_nodes_list: Vec<NodeIndex> = room_graph
        .node_indices()
        .filter(|&node| matches!(room_graph[node], DungeonGraphNode::Exit { .. }))
        .collect();

    let mut rng = rand::thread_rng();
    let graph_copy = room_graph.clone();

    let exit_nodes: Vec<(&NodeIndex, &DungeonGraphNode)> = exit_nodes_list
        .iter()
        .map(|f| (f, graph_copy.node_weight(*f).expect("msg")))
        .collect();

    let mut exits_connected: HashSet<NodeIndex> = HashSet::new();

    loop {
        let Some(first_exit) = exit_nodes
            .iter()
            .filter(|f| match f.1 {
                DungeonGraphNode::Room { .. } => true,
                DungeonGraphNode::Exit { brothers, .. } => brothers > &ExitAmount(1),
            } && !exits_connected.contains(f.0))
            .choose(&mut rng)
        else {
            break;
        };
        let Some(second_exit) = exit_nodes
            .iter()
            .filter(|(id, node)| {
                node.is_exit()
                    && node.get_parent_id() != first_exit.1.get_parent_id()
                    && !exits_connected.contains(*id)
            })
            // choose by distance
            .min_by(|a, b| {
                let a_position = a.1.get_position();
                let b_position = b.1.get_position();
                let a_distance = a_position.distance_squared(first_exit.1.get_position());
                let b_distance = b_position.distance_squared(first_exit.1.get_position());
                a_distance.cmp(&b_distance)
            })
        // choose randomly
        //  .choose(&mut rng)
        else {
            break;
        };

        let DungeonGraphNode::Exit {
            exit: exit1_data,
            brothers: _exit1_brothers,
        } = first_exit.1
        else {
            break;
        };
        let DungeonGraphNode::Exit {
            exit: exit2_data,
            brothers: _exit2_brothers,
        } = second_exit.1
        else {
            break;
        };
        let distance = exit1_data.position.distance_squared(exit2_data.position) as f32;

        if !exits_connected.contains(first_exit.0) && !exits_connected.contains(second_exit.0) {
            info!("adding edge too graph");
            exits_connected.insert(*first_exit.0);
            exits_connected.insert(*second_exit.0);
            room_graph.add_edge(
                *first_exit.0,
                *second_exit.0,
                DungeonGraphEdge::Hallway {
                    start: exit1_data.clone(),
                    end: exit2_data.clone(),
                    length: distance,
                },
            );
        }
    }

    room_graph
}

/// node for dungeon graph structure
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DungeonGraphNode {
    /// node is a dungeon room
    Room {
        /// room center position
        position: IVec2,
        /// room id
        id: RoomID,
    },
    /// node is a dungeon room exit
    Exit {
        /// 2 tiles that makeup exit
        exit: RoomExit,
        /// how many other exits are related too this exit
        brothers: ExitAmount,
    },
}

impl DungeonGraphNode {
    // pub fn is_room(self: &Self) -> bool {
    //     match self {
    //         DungeonGraphNode::Room { .. } => true,
    //         DungeonGraphNode::Exit { .. } => false,
    //     }
    // }

    /// is graph node an exit
    pub const fn is_exit(&self) -> bool {
        match self {
            Self::Room { .. } => false,
            Self::Exit { .. } => true,
        }
    }

    /// get graph node room id, parent id if exit
    pub const fn get_parent_id(&self) -> &RoomID {
        match self {
            Self::Room { id, .. } => id,
            Self::Exit { exit, .. } => &exit.parent,
        }
    }

    /// get graph node position
    pub const fn get_position(&self) -> IVec2 {
        match self {
            Self::Room { position, .. } => *position,
            Self::Exit { exit, .. } => exit.position,
        }
    }
}

/// aka hallway
#[derive(Debug, PartialEq, Clone)]
pub enum DungeonGraphEdge {
    /// this edge links 2 exits from seperate rooms
    Hallway {
        /// exit tiles on start room
        start: RoomExit,
        /// exit tiles on end room
        end: RoomExit,
        /// euclid distance between start and end
        length: f32,
    },
    /// this edge links an exit too its parent room
    ExitLink {
        /// always 0.0
        length: f32,
    },
}

impl PartialOrd for DungeonGraphEdge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            // Compare Hallway edges based on their lengths
            (
                Self::Hallway {
                    length: length1, ..
                },
                Self::Hallway {
                    length: length2, ..
                },
            ) => length1.partial_cmp(length2),
            // Internal edges are considered less than Hallway edges
            (Self::ExitLink { .. }, Self::Hallway { .. }) => Some(std::cmp::Ordering::Less),
            // Hallway edges are considered greater than Internal edges
            (Self::Hallway { .. }, Self::ExitLink { .. }) => Some(std::cmp::Ordering::Greater),
            // Internal edges are equal to other Internal edges
            (Self::ExitLink { .. }, Self::ExitLink { .. }) => Some(std::cmp::Ordering::Equal), // // Hallway edges are considered greater than Internal edges
                                                                                               // (DungeonGraphEdge::Hallway { .. }, DungeonGraphEdge::Hallway { .. }) => None, // You might want to handle this case depending on your logic
        }
    }
}

/// amount of exits this exit shares parents with
#[derive(Debug, PartialEq, Eq, PartialOrd, Clone)]
pub struct ExitAmount(u32);

/// spawns generated hallways from DungeonGeneratorSettings.hallways
pub fn spawn_hallway_roots(mut cmds: Commands, dungeon_container: Query<(Entity, &Dungeon)>) {
    let (dungeon_root, settings) = dungeon_container
        .get_single()
        .expect("should only ever be one at a time");

    if settings.hallways.is_empty() {
        warn!("No HallWays too process");
        return;
    }

    cmds.entity(dungeon_root)
        .with_children(|container_child_builder| {
            for (i, hallway) in settings.hallways.iter().enumerate() {
                #[cfg(debug_assertions)]
                {
                    let vname = Name::new(format!("HallwayVisual-{i}"));
                    let color = random_color(None);
                    let mut path = PathBuilder::new();
                    path.move_to(hallway.start_pos.as_vec2());
                    path.line_to(hallway.end_pos.as_vec2());
                    path.close();
                    let path = path.build();

                    container_child_builder.spawn((
                        vname,
                        Fill::color(color),
                        Stroke::new(Color::BLACK, 5.0),
                        ShapeBundle { path, ..default() },
                    ));
                }

                let start = hallway.start_pos.extend(0).as_vec3();
                let name = Name::new(format!("Hallway-{i}"));
                container_child_builder.spawn((
                    name,
                    hallway.clone(),
                    SpatialBundle {
                        transform: Transform::from_translation(start),
                        ..default()
                    },
                ));
            }
        });
    cmds.insert_resource(NextState(Some(GeneratorState::FinalizeHallways)));
}

/// tag for edge representation
#[derive(Debug, Component)]
pub struct HallWayVisualTag;

/// hallway representation
#[derive(Debug, Reflect, Clone, Component)]
pub struct HallWayBlueprint {
    /// hallway start pos
    start_pos: IVec2,
    /// hallway end pos
    end_pos: IVec2,
    /// how long is pathway
    distance: f32,
    /// rooms connected too hallway
    connected_rooms: (RoomID, RoomID),
    /// hallway finished building
    built: bool,
}
