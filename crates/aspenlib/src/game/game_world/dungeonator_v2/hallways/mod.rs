pub mod hallway_builder;

use bevy::prelude::*;
use bevy_prototype_lyon::{
    draw::{Fill, Stroke},
    entity::ShapeBundle,
    path::PathBuilder,
};
use rand::prelude::{IteratorRandom, Rng, ThreadRng};
use seldom_map_nav::prelude::Navmeshes;

use crate::game::game_world::{
    components::RoomExit,
    dungeonator_v2::{
        components::{DungeonContainerTag, DungeonRoomTag, DungeonSettings, PlacedRoom, RoomID},
        path_map::PathMap,
        GeneratorState,
    },
};

pub fn update_room_instances(
    children: Query<&Children>,
    mut cmds: Commands,
    mut exit_query: Query<(&mut RoomExit, &GlobalTransform)>,
    mut room_query: Query<(Entity, &mut PlacedRoom), With<DungeonRoomTag>>,
) {
    for (room, mut room_instance) in &mut room_query {
        info!("updating room with new pos and room exit list");

        // fill room exits vec
        let mut exits: Vec<Entity> = Vec::new();
        for child in children.iter_descendants(room) {
            if let Ok((mut room_exit, position)) = exit_query.get_mut(child) {
                room_exit.parent = room_instance.id;
                room_exit.position = position.translation().truncate().as_ivec2();
                exits.push(child);
                info!("position of exit: {}", room_exit.position);
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
    cmds.insert_resource(NextState(Some(GeneratorState::CreateHallwayTree)));
}

// TODO: rethink this code too only spawn 1 hallway per room
pub fn plan_hallways_one(
    mut cmds: Commands,
    mut dungeon_root: Query<&mut DungeonSettings, With<DungeonContainerTag>>,
    room_query: Query<(&GlobalTransform, &mut PlacedRoom), With<DungeonRoomTag>>,
    mut exit_query: Query<(&GlobalTransform, &mut RoomExit)>,
) {
    let mut settings = dungeon_root.single_mut();
    let mut hallways: Vec<PlacedHallWay> = Vec::new();

    let unused_exits = exit_query
        .iter()
        .map(|f| f.1)
        .map(|f| f.clone())
        .collect::<Vec<RoomExit>>();

    let mut exit_pairs = Vec::new();
    let mut distances = Vec::new();

    // Calculate distances between all exits
    for i in 0..unused_exits.len() {
        for j in 0..unused_exits.len() {
            let exit1 = &unused_exits[i];
            let exit2 = &unused_exits[j];
            let room1 = room_query
                .iter()
                .find(|f| f.1.id == exit1.parent)
                .expect("msg");
            let room2 = room_query
                .iter()
                .find(|f| f.1.id == exit2.parent)
                .expect("msg");

            if room2.1.exits.len().le(&2) && room1.1.exits.len().le(&2) {
                continue;
            }

            if exit1.parent.eq(&exit2.parent) {
                info!("functionally same exits, skipping");
                continue;
            }

            let distance: i32 = exit1.position.distance_squared(exit2.position);
            distances.push((i, j, distance));
        }
    }

    // Sort distances
    distances.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    // Pair by distance
    let mut paired_set = Vec::new();
    for pair in distances {
        let (idx1, idx2, _) = pair;
        let exit1 = unused_exits[idx1].clone();
        let exit2 = unused_exits[idx2].clone();

        if exit_pairs.iter().any(|(a, b): &(RoomExit, RoomExit)| {
            a.parent == exit1.parent || b.parent == exit1.parent //|| a.parent == exit2.parent || b.parent == exit2.parent
        }) {
            continue;
        } 

        if !paired_set.contains(&idx1) && !paired_set.contains(&idx2) {
            exit_pairs.push((unused_exits[idx1].clone(), unused_exits[idx2].clone()));
            paired_set.push(idx1);
            paired_set.push(idx2);
        }
    }

    for (wanted_start, wanted_end) in exit_pairs {
        hallways.push(PlacedHallWay {
            start_pos: wanted_start.position,
            end_pos: wanted_end.position,
            distance: wanted_start.position.distance_squared(wanted_end.position) as f32,
            connected_rooms: (wanted_start.parent, wanted_end.parent),
        });
        if let Some((_, mut start)) = exit_query.iter_mut().find(|(_, exit)| **exit == wanted_start) {
            start.hallway_connected = true
        }
        if let Some((_, mut end)) = exit_query.iter_mut().find(|(_, exit)| **exit == wanted_end) {
            end.hallway_connected = true
        }
    }

    settings.positioned_hallways = hallways;
    cmds.insert_resource(NextState(Some(GeneratorState::PlaceHallwayRoots)));
}

// TODO: rethink this code too only spawn 1 hallway per room
// maybe try this apporach again now that the room ids are
pub fn plan_hallways_two(
    mut cmds: Commands,
    mut dungeon_root: Query<&mut DungeonSettings, With<DungeonContainerTag>>,
    room_query: Query<(&GlobalTransform, &mut PlacedRoom), With<DungeonRoomTag>>,
    mut exit_query: Query<(&GlobalTransform, &mut RoomExit)>,
) {
    let mut settings = dungeon_root.single_mut();
    let mut hallways: Vec<PlacedHallWay> = Vec::new();
    let mut rng = ThreadRng::default();

    for (_, working_room) in &room_query {
        info!("working on room {}", working_room.name);
        let Some(wanted_start) = working_room
            .exits
            .iter()
            .filter(|f| {
                !exit_query
                    .get(**f)
                    .expect("room exit missing component")
                    .1
                    .hallway_connected
            })
            .map(|f| (*f, exit_query.get(*f).expect("exit missing components")))
            .choose(&mut rng)
        else {
            warn!("all room exits were full");
            continue;
        };

        let Some(end_room) = &room_query
            .iter()
            .filter(|f| {
                // end room should not be start room
                f.1.id != working_room.id &&
                // end room should not be room already having a hallway
                hallways.iter().all(|h| f.1.id != h.connected_rooms.0 || f.1.id != h.connected_rooms.1) &&
                // end room should have more than 1 exit
                f.1.exits.len() > 1 &&
                // end exit should not one that already has a hallway attached
                f.1.exits
                        .iter()
                        .any(|f| {exit_query.get(*f).expect("msg").1.hallway_connected == false})
            })
            .min_by(|a, b| {
                let distance1 = a.1
                    .position
                    .distance_squared(wanted_start.1 .0.translation().truncate().as_ivec2());
                let distance2 = b.1
                    .position
                    .distance_squared(wanted_start.1 .0.translation().truncate().as_ivec2());
                distance1.cmp(&distance2)
            }) else {continue};

        let Some(wanted_end) = end_room
            .1
            .exits
            .iter()
            .filter(|f| {
                exit_query
                    .get(**f)
                    .expect("room exit missing component")
                    .1
                    .hallway_connected
                    == false
            })
            .map(|f| (*f, exit_query.get(*f).expect("msg")))
            .min_by(|a, b| {
                let distance1 =
                    a.1 .0
                        .translation()
                        .distance_squared(wanted_start.1 .0.translation());
                let distance2 =
                    b.1 .0
                        .translation()
                        .distance_squared(wanted_start.1 .0.translation());
                distance1.total_cmp(&distance2)
            })
        else {
            continue;
        };

        let lenght = wanted_start
            .1
             .0
            .translation()
            .distance_squared(wanted_end.1 .0.translation()) as f32;

        hallways.push(PlacedHallWay {
            start_pos: wanted_start.1 .0.translation().truncate().as_ivec2(),
            end_pos: wanted_end.1 .0.translation().truncate().as_ivec2(),
            distance: lenght,
            connected_rooms: (wanted_start.1 .1.parent, wanted_end.1 .1.parent),
        });

        if let Ok(changed_exits) = exit_query.get_many_mut([wanted_start.0, wanted_end.0]) {
            for mut exit in changed_exits {
                exit.1.hallway_connected = true
            }
        }
    }

    settings.positioned_hallways = hallways;
    cmds.insert_resource(NextState(Some(GeneratorState::PlaceHallwayRoots)));
}

/// spawns generated hallways from DungeonGeneratorSettings.hallways
pub fn spawn_hallway_roots(
    mut cmds: Commands,
    pathmap: Query<(Entity, &Navmeshes), With<PathMap>>,
    dungeon_container: Query<(Entity, &DungeonSettings), With<DungeonContainerTag>>,
) {
    let (dungeon_root, settings) = dungeon_container.single();
    let (pathmap, navmeshes) = pathmap.single();
    if settings.positioned_hallways.is_empty() {
        warn!("No HallWays too process");
        return;
    }

    let mut i = 0;
    cmds.entity(dungeon_root)
        .with_children(|container_child_builder| {
            for hallway in &settings.positioned_hallways {
                let start = hallway.start_pos.extend(0).as_vec3();
                let end = hallway.end_pos.extend(0).as_vec3();
                let name = format!("Hallway-{}", i);
                container_child_builder.spawn((
                    Name::new(name),
                    hallway.clone(),
                    SpatialBundle {
                        transform: Transform::from_translation(start),
                        ..default()
                    },
                ));
                //
                let path = create_hallway_path(hallway);
                let hallway_name = format!("HallwayVisual-{}", i);
                container_child_builder.spawn((
                    Name::new(hallway_name.clone()),
                    HallWayVisualTag,
                    ShapeBundle {
                        path,
                        spatial: SpatialBundle::from_transform(Transform::from_xyz(0., 0., 10.0)),
                        ..default()
                    },
                    Stroke::new(Color::LIME_GREEN, 10.0),
                    Fill::color(Color::RED),
                ));
                i += 1;
            }
        });
    cmds.insert_resource(NextState(Some(GeneratorState::FinalizeHallways)))
}

/// creates path from hallway start and end position
fn create_hallway_path(hallway: &PlacedHallWay) -> bevy_prototype_lyon::prelude::Path {
    let mut path_builder = PathBuilder::new();
    path_builder.move_to(hallway.start_pos.as_vec2());
    path_builder.line_to(hallway.end_pos.as_vec2());
    path_builder.close();
    let path = path_builder.build();
    path
}

/// tag for edge representation
#[derive(Debug, Component)]
pub struct HallWayVisualTag;

/// hallway representation
#[derive(Debug, Reflect, Clone, Component)]
pub struct PlacedHallWay {
    /// hallway start pos
    start_pos: IVec2,
    /// hallway end pos
    end_pos: IVec2,
    /// how long is pathway
    distance: f32,
    /// rooms connected too hallway
    connected_rooms: (RoomID, RoomID),
}

// collect all exits
// pair exits by distance
// place 0..room_query.len() -1? hallways and flip hallway nodes coo

// for i in 0..clipped_exits.len() {
//     if !rng.gen_bool(settings.loops_percentage.into()) {
//         continue;
//     }

//     if clipped_exits.is_empty() {
//         break;
//     }

//     let start = clipped_exits.swap_remove(0);
//     let Some(end) = clipped_exits
//         .iter()
//         .filter(|f| f.used == false && f.parent != start.parent)
//         .max_by(|a, b| {
//             let distance1 = a.position.distance_squared(start.position);
//             let distance2 = b.position.distance_squared(start.position);
//             distance1.cmp(&distance2)
//         })
//     else {
//         warn!("no more connectable hallways");
//         break;
//     };

//     let end_idx = clipped_exits
//         .iter()
//         .position(|f| f == end)
//         .expect("idx did not exist in positions?");
//     let end = clipped_exits.remove(end_idx);

//     let lenght = start.position.distance_squared(end.position) as f32;
//     hallways.push(PlacedHallWay {
//         start_pos: start.position,
//         end_pos: end.position,
//         distance: lenght,
//         connected_rooms: Some((start.parent, end.parent)),
//     })
// }

//
// let mut hallways: Vec<PlacedHallWay> = Vec::new();

// // TODO: rethink this code too only spawn 1 hallway per room
// pub fn plan_hallways_old(
//     mut cmds: Commands,
//     mut dungeon_root: Query<&mut DungeonSettings, With<DungeonContainerTag>>,
//     room_query: Query<(&GlobalTransform, &PlacedRoom), With<DungeonRoomTag>>,
//     mut room_exits: Query<&mut RoomExit>,
// ) {
//     let mut settings = dungeon_root.single_mut();

//     info!("collecting room exits");
//     let exits = room_query
//         .iter()
//         .map(|f| &f.1.exits)
//         .flatten()
//         .cloned()
//         .map(|f| (f, room_exits.get(f).expect("should always exist")))
//         .collect::<Vec<(Entity, &RoomExit)>>();

//     let mut rng = ThreadRng::default();
//     let mut hallways: Vec<PlacedHallWay> = Vec::new();
//     let mut clipped_exits = exits.clone();

//     for (_, room) in &room_query {
//         info!("working on room {}", room.name);
//         let local_copy = clipped_exits.clone();
//         let wanted_start = local_copy
//             .iter()
//             .filter(|f| !f.1.hallway_connected && f.1.parent == room.id)
//             .min_by(|a, b| {
//                 let distance1 = a.1.position.distance_squared(room.position);
//                 let distance2 = b.1.position.distance_squared(room.position);
//                 distance1.cmp(&distance2)
//             })
//             .unwrap_or_else(|| {
//                 local_copy
//                     .iter()
//                     .filter(|f| !f.1.hallway_connected)
//                     .choose(&mut rng)
//                     .expect("msg")
//             });

//         let wanted_end = local_copy
//             .iter()
//             .filter(|f| !f.1.hallway_connected && f.1.parent != wanted_start.1.parent)
//             .min_by(|a, b| {
//                 let distance1 = a.1.position.distance_squared(wanted_start.1.position);
//                 let distance2 = b.1.position.distance_squared(wanted_start.1.position);
//                 distance1.cmp(&distance2)
//             })
//             .unwrap_or_else(|| {
//                 local_copy
//                     .iter()
//                     .filter(|f| !f.1.hallway_connected)
//                     .choose(&mut rng)
//                     .expect("msg")
//             });

//         let start_idx = clipped_exits
//             .iter()
//             .position(|f| f == wanted_start)
//             .expect("msg");
//         let start = clipped_exits.remove(start_idx);
//         let end_idx = clipped_exits
//             .iter()
//             .position(|f| f == wanted_end)
//             .expect("idx did not exist in positions?");
//         let end = clipped_exits.remove(end_idx);

//         let lenght = start.1.position.distance_squared(end.1.position) as f32;
//         hallways.push(PlacedHallWay {
//             start_pos: start.1.position,
//             end_pos: end.1.position,
//             distance: lenght,
//             connected_rooms: (start.1.parent, end.1.parent),
//         });
//     }

//     for i in 0..clipped_exits.len() {
//         if !rng.gen_bool(settings.loops_percentage.into()) {
//             continue;
//         }

//         if clipped_exits.is_empty() {
//             break;
//         }

//         let start = clipped_exits.swap_remove(0);
//         let Some(end) = clipped_exits
//             .iter()
//             .filter(|f| !f.1.hallway_connected && f.1.parent != start.1.parent)
//             .max_by(|a, b| {
//                 let distance1 = a.1.position.distance_squared(start.1.position);
//                 let distance2 = b.1.position.distance_squared(start.1.position);
//                 distance1.cmp(&distance2)
//             })
//         else {
//             warn!("no more connectable hallways");
//             break;
//         };

//         let end_idx = clipped_exits
//             .iter()
//             .position(|f| f == end)
//             .expect("idx did not exist in positions?");
//         let end = clipped_exits.remove(end_idx);

//         let lenght = start.1.position.distance_squared(end.1.position) as f32;
//         hallways.push(PlacedHallWay {
//             start_pos: start.1.position,
//             end_pos: end.1.position,
//             distance: lenght,
//             connected_rooms: (start.1.parent.clone(), end.1.parent.clone()),
//         })
//     }

//     // TODO:
//     // for each room, get 1 endpoint on the room and 1 random endpoint from hallway positions
//     // remove both positions from hallwaypositions

//     // loop 0..hallway_posiitons.len()
//     // if change bool == true add hall way between rooms
//     // get and remove 2 idx from hallway positions
//     settings.positioned_hallways = hallways;
//     cmds.insert_resource(NextState(Some(GeneratorState::PlaceHallwayRoots)));
// }
