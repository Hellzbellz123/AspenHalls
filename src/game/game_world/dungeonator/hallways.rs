#![allow(clippy::type_complexity)]

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::LdtkLevel;
use bevy_ecs_tilemap::tiles::TileStorage;
use bevy_prototype_lyon::prelude::{PathBuilder, ShapeBundle, Stroke};
use petgraph::{
    algo::min_spanning_tree,
    data::FromElements,
    dot::{Config, Dot},
    prelude::*,
    stable_graph::{EdgeIndices, StableGraph},
};
use rand::Rng;
use std::{cmp::Ordering, fs::File, io::Write, time::Duration};

use crate::game::game_world::components::RoomExit;

use super::{
    generator::{DungeonContainerTag, DungeonRoomTag, RoomID, RoomInstance},
    DungeonGeneratorSettings, GeneratorStage,
};

/// hallway representation
#[derive(Debug, Reflect, Clone, Component)]
pub struct HallWay {
    /// hallway start pos
    start_pos: IVec2,
    /// hallway end pos
    end_pos: IVec2,
    /// how long is pathway
    distance: f32,
    /// rooms connected too hallway
    connected_rooms: Vec<(RoomID, RoomID)>,
}

/// edges representation for graph
pub struct Edges<'a, E> {
    /// iter of edges and distance
    iter: EdgeIndices<'a, u32>,
    /// actual graph for edges
    graph: &'a StableGraph<RoomInstance, E, Undirected>,
}

impl<'a> Iterator for Edges<'a, HallWay> {
    type Item = (NodeIndex, NodeIndex, &'a HallWay, RoomID, RoomID);

    fn next(&mut self) -> Option<Self::Item> {
        let edge = self.iter.next()?;
        let edge_ref = self.graph.edge_weight(edge)?;
        let (source, target) = self.graph.edge_endpoints(edge)?;

        let source_room_id = self.graph[source].room_id;
        let target_room_id = self.graph[target].room_id;

        Some((source, target, edge_ref, source_room_id, target_room_id))
    }
}

impl Eq for HallWay {}

impl Ord for HallWay {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.total_cmp(&other.distance)
    }
}

impl PartialEq for HallWay {
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}


impl PartialOrd for HallWay {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


/// querys for empty levels
/// if no empty levels ldtk is finished procces
pub fn wait_for_ldtk_finish(
    mut timer: Local<Timer>,
    time: Res<Time>,
    mut cmds: Commands,
    //added explicity childless query for no exit, not sure if its actually faster/better than
    //the matches! childless fn below
    child_less_levels: Query<
        (Entity, &Handle<LdtkLevel>, &Parent),
        (With<DungeonRoomTag>, Without<Children>),
    >,
) {
    timer.set_duration(Duration::from_secs(2));
    // we only want too continue if the levels are built, layers get added to levels
    // as children so if the level has no children we know its built
    if child_less_levels.is_empty() && timer.tick(time.delta()).finished() {
        cmds.insert_resource(NextState(Some(GeneratorStage::GenerateConnections)));
    }
}

/// takes all room instances and creates nodes for an mst
pub fn create_mst_from_rooms(
    mut cmds: Commands,
    mut gen_settings: ResMut<DungeonGeneratorSettings>,
    children: Query<&Children>,
    mut room_query: Query<(Entity, &GlobalTransform, &mut RoomInstance), With<DungeonRoomTag>>,
    exit_query: Query<(Entity, &Transform), (With<RoomExit>,)>,
) {
    if gen_settings.hallways.is_some() {
        return;
    }

    let mut rng = rand::thread_rng();
    // Adjust this probability to control the likelihood of adding loops
    let loop_probability = gen_settings.looped_hallway_amount.clamp(0.0, 1.0) as f64;

    let mut points: Vec<RoomInstance> =
        Vec::with_capacity(gen_settings.dungeon_room_amount as usize);

    room_query.for_each_mut(|(r_ent, _r_gtrans, mut r_instance)| {
        let mut pathfind_room_node = RoomInstance {
            room_name: r_instance.room_name.clone(),
            room_id: r_instance.room_id,
            room_asset: r_instance.room_asset.clone(),
            width: r_instance.width,
            height: r_instance.height,
            position: (r_instance.position + (IVec2::new(r_instance.width, r_instance.height) / 2)),
            exits: Vec::new(),
        };

        let mut exits: Vec<Vec2> = Vec::new();

        for child in children.iter_descendants(r_ent) {
            if exit_query.get(child).is_ok() {
                let (_e_ent, e_trans) = exit_query.get(child).unwrap();
                exits.push(e_trans.translation.truncate())
            }
            info!("{:?}", exits);
        }

        (pathfind_room_node.exits = exits.clone());

        points.push(pathfind_room_node);
        r_instance.exits = exits;
    });

    if points.len() == gen_settings.dungeon_room_amount as usize {
        // Create a graph and add the rooms as vertices
        let mut room_graph = StableUnGraph::<RoomInstance, HallWay>::default();
        let indices: Vec<NodeIndex<_>> = points
            .iter()
            .map(|room| room_graph.add_node(room.clone()))
            .collect();

        // Calculate edge weights based on distance (Euclidean distance in this case)
        for i in 0..points.len() {
            for j in (i + 1)..points.len() {
                let room1 = &points[i];
                let room2 = &points[j];
                let distance = calculate_distance(
                    room1.position.x,
                    room1.position.y,
                    room2.position.x,
                    room2.position.y,
                );
                room_graph.add_edge(
                    indices[i],
                    indices[j],
                    HallWay {
                        start_pos: room1.position,
                        end_pos: room2.position,
                        distance,
                        connected_rooms: Vec::new(),
                    },
                );
            }
        }

        // Compute the minimum spanning tree
        let mst = min_spanning_tree(&room_graph);
        let mst_graph = StableUnGraph::<_, _>::from_elements(mst);
        let mut cyclic_graph: StableGraph<RoomInstance, HallWay, Undirected> = mst_graph;

        // Iterate over the edges in the room graph
        for edge in room_graph.edge_indices() {
            let source = room_graph.edge_endpoints(edge).unwrap().0;
            let target = room_graph.edge_endpoints(edge).unwrap().1;

            // Check if the edge is not already present in the MST and based on the random chance
            if !cyclic_graph.contains_edge(source, target) && rng.gen_bool(loop_probability) {
                cyclic_graph.add_edge(
                    source,
                    target,
                    room_graph.edge_weight(edge).unwrap().clone(),
                );
            }
        }

        let cyclic_copy = cyclic_graph.clone();

        // Iterate over the edges in the room graph
        for edge in cyclic_copy.edge_indices() {
            let immut_graph = room_graph.clone();
            let hallway = cyclic_graph.edge_weight_mut(edge).unwrap();
            let source_idx = immut_graph.edge_endpoints(edge).unwrap().0;
            let target_idx = immut_graph.edge_endpoints(edge).unwrap().1;
            let source = immut_graph.node_weight(source_idx).unwrap();
            let target = immut_graph.node_weight(target_idx).unwrap();

            let connected_rooms = vec![(source.room_id, target.room_id)];
            hallway.connected_rooms = connected_rooms;
        }

        let hallways: Vec<HallWay> = cyclic_graph
            .edge_indices()
            .map(|edge| cyclic_graph.edge_weight(edge).unwrap().clone())
            .collect();

        gen_settings.hallways = Some(hallways);

        // // Create a DOT representation of the MST
        let cyclic_dot = Dot::with_config(&cyclic_graph, &[Config::NodeIndexLabel]);
        save_dot(cyclic_dot, "cyclic.dot".into());
        cmds.insert_resource(NextState(Some(GeneratorStage::PathfindConnections)));

        // triangulate and scale points for display
        // let triangulation = delaunator::triangulate(&points).unwrap();
        // let points = center_and_scale(&points, &triangulation);

        // // Create a DOT representation of the MST
        // let dot_mst = Dot::with_config(&mst_graph, &[Config::NodeIndexLabel]);
        // save_dot(dot_mst, "mst-pre.dot".into());

        // // Create a DOT file of the Delauny Triangulation of rooms
        // let rooms_graph_dot = Dot::with_config(&room_graph, &[Config::NodeIndexLabel]);
        // save_dot(rooms_graph_dot, "delauny.dot".into());
    }
}

/// tag for edge representation
#[derive(Debug, Component)]
pub struct HallWayVisualTag;

/// spawns generated hallways from DungeonGeneratorSettings.hallways
pub fn spawn_hallway_roots(
    mut cmds: Commands,
    gen_settings: Res<DungeonGeneratorSettings>,
    dungeon_container: Query<Entity, &DungeonContainerTag>,
) {
    if gen_settings.hallways.is_none() {
        warn!("No HallWays too process");
        return;
    }

    let mut i = 0;
    cmds.entity(dungeon_container.single())
        .with_children(|container_child_builder| {
            for hallway in gen_settings.hallways.as_ref().unwrap() {
                let name = format!("Hallway {}", i);
                // let mut pathbuilder = PathBuilder::new();
                // pathbuilder.move_to(hallway.start_pos.as_vec2()); //hallway.start_pos.as_vec2());
                // pathbuilder.line_to(hallway.end_pos.as_vec2());
                // pathbuilder.close();
                // let path = pathbuilder.build();
                // container_child_builder.spawn((
                //     Name::new(name.clone()),
                //     HallWayVisualTag,
                //     ShapeBundle {
                //         path,
                //         transform: Transform::from_xyz(0., 0., -0.5),
                //         ..default()
                //     },
                //     Stroke::new(Color::RED, 10.0),
                //     // Fill::color(Color::RED),
                // ));
                container_child_builder.spawn((
                    Name::new(name),
                    hallway.clone(),
                    SpatialBundle {
                        transform: Transform::from_translation(
                            hallway.start_pos.extend(0).as_vec3(),
                        ),
                        ..default()
                    },
                ));
                i += 1;
            }
        });
}

/// iterated over all spawned hallways and actually builds them
pub fn pathfind_and_build_hallways(
    mut cmds: Commands,
    unbuilt_hallways: Query<(Entity, &HallWay), (With<Handle<LdtkLevel>>, Without<Children>)>,
) {
    // we only need to run this loop if the hallways arent built, layers
    // get added to halways as children so if the level has no children we know it needs to be built still
    if unbuilt_hallways.is_empty() {
        cmds.insert_resource(NextState(Some(GeneratorStage::Finished)));
        return;
    }

    unbuilt_hallways.for_each(|(h_ent, _h_data)| {
        let (mut ground, mut decoration, mut building) =
            (None::<Entity>, None::<Entity>, None::<Entity>);

        cmds.entity(h_ent).with_children(|hallway_layers| {
            ground = Some(
                hallway_layers
                    .spawn((Name::new("Ground_Layer"), TileStorage::default()))
                    .id(),
            );
            decoration = Some(
                hallway_layers
                    .spawn((Name::new("Decoration_Layer"), TileStorage::default()))
                    .id(),
            );
            building = Some(
                hallway_layers
                    .spawn((Name::new("Building_Layer"), TileStorage::default()))
                    .id(),
            );
        });

        // if ground.is_some() & decoration.is_some() & building.is_some() {}
    });
}

// TODO: finish these functions
/// spawns floor for hallway
/// takes a path: Vec<(Ivec2, Ivec2) or (TilePos, TilePos)> and places `floor tile` for each position
#[allow(dead_code)]
fn spawn_floor_layer() {}

/// spawns floor for hallway
/// takes a path: Vec<(Ivec2, Ivec2) or (TilePos, TilePos)> and places `floor tile` for each position
#[allow(dead_code)]
fn spawn_building_layer() {}

/// spawns floor for hallway
/// takes a path: Vec<(Ivec2, Ivec2) or (TilePos, TilePos)> and places `floor tile` for each position
#[allow(dead_code)]
fn spawn_decoration_layer() {}

/// saves dot graph as file
fn save_dot(dot: Dot<'_, &StableGraph<RoomInstance, HallWay, petgraph::Undirected>>, file: String) {
    // Save the DOT representation to a file
    let mut dot_file = File::create(file).expect("Failed to create DOT file");
    write!(dot_file, "{:?}", dot).expect("Failed to write DOT file");
}

/// calculates distance between too spots
fn calculate_distance(lat1: i32, lon1: i32, lat2: i32, lon2: i32) -> f32 {
    let (lat1, lat2, lon1, lon2) = (lat1 as f32, lat2 as f32, lon1 as f32, lon2 as f32);
    // Calculate Euclidean distance between two latitude-longitude coordinates
    let x = (lon2 - lon1) * f32::cos((lat1 + lat2) / 2.0);
    let y = lat2 - lat1;
    (x * x + y * y).sqrt()
}
