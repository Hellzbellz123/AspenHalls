use bevy_ecs_tilemap::prelude::TilemapSize;
use petgraph::{data::FromElements, prelude::EdgeRef, Graph};
use rand::prelude::Rng;
use std::collections::VecDeque;

use bevy::{prelude::*, reflect::Reflect};
use bevy_ecs_ldtk::{assets::LdtkExternalLevel, prelude::LdtkProject};

use crate::{
    consts::TILE_SIZE,
    game::{
        characters::player::PlayerSelectedHero,
        game_world::{
            components::{ActorTeleportEvent, RoomExit, TpTriggerEffect},
            dungeonator_v2::{
                components::{
                    Dungeon, DungeonContainerBundle, DungeonHallWayBundle, DungeonRoomBundle,
                    DungeonRoomDatabase, DungeonSettings, RoomBlueprint, RoomDistribution,
                    RoomPreset, RoomType,
                },
                hallways::{create_hallway_layer, HallWayBlueprint, HallwayLayer},
                room_graph::RoomGraph,
                tile_graph::TileGraph,
            },
            random_point_inside,
        },
    },
    loading::assets::AspenMapHandles,
    register_types,
};

/// Dungeon Generator components
pub mod components;
/// hallway creation system
pub mod hallways;
/// room selection and creation
pub mod room_database;
/// per dungeon graph of rooms and connections
pub mod room_graph;
/// global tile graph map thing
pub mod tile_graph;
/// dungeon generation utilitys
pub mod utils;

/// are we in dungeon yet?
#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default, Reflect)]
pub enum GeneratorState {
    /// no dungeon is spawned
    #[default]
    NoDungeon,
    /// select presets
    LayoutDungeon,
    /// spawns rooms in world
    CompleteHallways,
    /// build hallway tiles for each blueprint
    FinalizeHallways,
    /// finished making dunegon
    FinishedDungeonGen,
}

// TODO: implemenet as external reusable plugin
//fire events for dungeons instead of using states as we do.
// maybe wrap `GeneratorState` inside an event struct (<--, better imo, can more data) or make it an event
// this should make enabling multiple dungeons alot easier

/// generates dungeons from ldtk level files
pub struct DungeonGeneratorPlugin;

impl Plugin for DungeonGeneratorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        register_types!(
            app,
            [
                Dungeon,
                RoomExit,
                RoomPreset,
                RoomBlueprint,
                HallWayBlueprint,
                RoomDistribution,
                DungeonSettings,
                DungeonRoomDatabase
            ]
        );

        app.init_state::<GeneratorState>();

        // create a new room database anytime we get new room assets
        app.add_systems(
            Update,
            room_database::build_room_presets.run_if(
                resource_exists::<AspenMapHandles>.and_then(
                    resource_changed::<Assets<LdtkExternalLevel>>
                        .or_else(resource_changed::<Assets<LdtkProject>>),
                ),
            ),
        );

        // step 1 create graph
        app.add_systems(
            OnEnter(GeneratorState::LayoutDungeon),
            (
                spawn_new_dungeon,
                apply_deferred,
                layout_dungeon,
                apply_deferred,
            )
                .chain(),
        );

        app.add_systems(
            Update,
            (tile_graph::create_tile_graph, apply_deferred)
                .chain()
                .run_if(in_state(GeneratorState::CompleteHallways)),
        );

        app.add_systems(
            OnEnter(GeneratorState::FinalizeHallways),
            (create_hallway_layer, apply_deferred).chain(),
        );

        app.add_systems(
            Update,
            hallways::hallway_builder::build_hallways.run_if(
                in_state(GeneratorState::FinalizeHallways)
                    .and_then(any_with_component::<HallwayLayer>),
            ),
        );
    }
}

/// spawns dungeon root
fn spawn_new_dungeon(
    mut cmds: Commands,
    ldtk_project_handles: Res<AspenMapHandles>,
    dungeon_root: Query<(Entity, &Dungeon)>,
) {
    // TODO: proper dungeon end system with cleanup
    let level = if let Ok((ent, dungeon)) = dungeon_root.get_single() {
        cmds.entity(ent).despawn_recursive(); // this happens next frame so dungeon still exists
        dungeon.settings.level.clone().next_level()
    } else {
        components::RoomLevel::Level1
    };

    let span = 15000.0;
    let mut rng = rand::thread_rng();
    let origin = Transform::from_xyz(
        ensure_tile_pos(rng.gen_range(-span..span)),
        ensure_tile_pos(rng.gen_range(-span..span)),
        0.0,
    );
    info!("spawning dungeon at {origin:?}");

    cmds.spawn(DungeonContainerBundle {
        name: "The Aspen Halls".into(),
        dungeon: Dungeon {
            settings: DungeonSettings {
                level,
                // border is applied too each room asset so 0 here
                border: 4,
                // room placing settings
                size: TilemapSize { x: 64, y: 64 },
                // TODO: use this but make it working
                // tiles_between_rooms: 4,
                distribution: RoomDistribution {
                    small_short: 3,
                    small_long: 2,
                    medium_short: 1,
                    medium_long: 1,
                    large_short: 0,
                    large_long: 0,
                    huge_short: 0,
                    huge_long: 0,
                    special: 2,
                },
                // hallway placing settings/data
                hallway_loop_chance: 0.08,
            },
            tile_graph: TileGraph {
                graph: Graph::new_undirected(),
                center_world: origin.translation.truncate(),
            },
            room_graph: RoomGraph::default(),
        },
        ldtk_project: ldtk_project_handles.default_levels.clone(),
        spatial: SpatialBundle {
            transform: origin,
            ..default()
        },
    });
}

/// creates vec of `RoomPreset` too be built into `DungeonRoomBundles` using  `DungeonSettings` and current progress in the dungeon
pub fn layout_dungeon(
    mut cmds: Commands,
    room_database: Res<DungeonRoomDatabase>,
    mut dungeon_root: Query<(Entity, &mut Dungeon, &Transform)>,
    player_query: Query<Entity, With<PlayerSelectedHero>>,
    mut tp_events: EventWriter<ActorTeleportEvent>,
) {
    let (dungon_id, dungeon, dungeon_transform) = dungeon_root.single_mut();

    info!("creating dungeon room blueprints");
    let mut positioned_presets = create_dungeon_blueprint(dungeon, room_database);

    info!("creating room graph from blueprints");
    let mut room_graph = RoomGraph::new(positioned_presets.make_contiguous());

    info!("connecting graph");
    room_graph.connect_graph_randomly();

    info!("computing minimum spanning tree of graph");
    room_graph.graph = Graph::from_elements(petgraph::algo::min_spanning_tree(&room_graph.graph));

    info!("verifying graph connectivity");
    room_graph.verify_graph_connections();

    #[cfg(debug_assertions)]
    {
        info!("room graph finished, dumping too file");
        room_graph.dump_too_file();
    }

    info!("spawning rooms");
    room_graph.node_weights().for_each(|weight| {
        if let room_graph::RoomGraphNode::Room(bp) = weight {
            cmds.entity(dungon_id).with_children(|rooms| {
                rooms.spawn(DungeonRoomBundle {
                    name: bp.name.clone().into(),
                    id: bp.asset_id.clone(),
                    room: bp.clone(),
                    spatial: SpatialBundle::from_transform(Transform::from_translation(
                        bp.room_space.min.as_vec2().extend(0.0),
                    )),
                });
            });
        };
    });

    teleport_player_too_start_location(
        dungeon_transform.translation.truncate(),
        &player_query,
        &mut tp_events,
    );

    info!("spawning hallways");
    room_graph.edge_references().for_each(|edge| {
        let source = room_graph.graph.node_weight(edge.source()).expect("msg");
        let target = room_graph.graph.node_weight(edge.target()).expect("msg");

        if target.is_exit() && source.is_exit() && source != target {
            let hallway_name = format!(
                "Hallway{:?}->{:?}",
                source.get_node_id(),
                target.get_node_id()
            );

            let start_pos = source.get_nodes_offset();
            let end_pos = target.get_nodes_offset();

            cmds.spawn(DungeonHallWayBundle {
                name: Name::new(hallway_name),
                hallway: HallWayBlueprint {
                    start_pos,
                    end_pos,
                    distance: edge.weight().length,
                    node_path: VecDeque::new(),
                    connected_rooms: (*source.get_node_id(), *target.get_node_id()),
                    built: false,
                },
                spatial: SpatialBundle::from_transform(Transform::from_translation(
                    start_pos.as_vec2().extend(0.0),
                )),
            })
            .set_parent(dungon_id);
        } else {
            info!("bad graph edge");
        }
    });

    cmds.insert_resource(NextState::Pending(GeneratorState::CompleteHallways));
}

// TODO: use a quad-tree structure too improve performance?
// tbh this part of the module is actually pretty quick compared too building the tilegraph
/// returns random list of room blueprints for a dungeon
fn create_dungeon_blueprint(
    dungeon: Mut<Dungeon>,
    room_database: Res<DungeonRoomDatabase>,
) -> VecDeque<RoomBlueprint> {
    let mut room_positions = Vec::new();
    let progress_level = &dungeon.settings.level;

    // choose presets
    let mut presets = utils::choose_filler_presets(&dungeon.settings, &room_database);
    if presets.is_empty() {
        error!("presets could not be chosen from room database");
        error!("database {:?}", room_database);
    }

    // add start and end presets
    presets.push_back(utils::get_leveled_preset(&room_database.end_rooms, progress_level).unwrap());
    presets
        .push_front(utils::get_leveled_preset(&room_database.start_rooms, progress_level).unwrap());

    // turn room blueprint
    let mut positioned_blueprints: VecDeque<RoomBlueprint> = VecDeque::new();
    for (i, preset) in presets.iter().enumerate() {
        let rooms_space = if preset.descriptor.rtype == RoomType::DungeonStart {
            Rect::from_center_size(Vec2::ZERO, preset.size.as_vec2())
        } else {
            utils::random_room_positon(&room_positions, preset.size.as_vec2(), &dungeon.settings)
        };
        room_positions.push(rooms_space);
        positioned_blueprints.push_back(RoomBlueprint::from_preset(
            preset,
            IVec2 {
                x: ensure_tile_pos(rooms_space.min.x) as i32,
                y: ensure_tile_pos(rooms_space.min.y) as i32,
            },
            i as u32,
        ));
    }

    positioned_blueprints
}

/// rounds `element` too nearest multiple of tilesize
fn ensure_tile_pos(element: f32) -> f32 {
    (element / TILE_SIZE).round() * TILE_SIZE
}

/// teleports player too the average `Transform` of all entities with `PlayerStartLocation`
#[allow(clippy::type_complexity)]
fn teleport_player_too_start_location(
    dungeon_center: Vec2,
    player_query: &Query<Entity, With<PlayerSelectedHero>>,
    tp_events: &mut EventWriter<ActorTeleportEvent>,
) {
    let start_size = Vec2 { x: 50.0, y: 50.0 };

    let start_loc_rect = Rect {
        min: Vec2 {
            x: dungeon_center.x - start_size.x,
            y: dungeon_center.y - start_size.y,
        },
        max: Vec2 {
            x: dungeon_center.x + start_size.x,
            y: dungeon_center.y + start_size.y,
        },
    };

    let pos = random_point_inside(&start_loc_rect, 1.0).unwrap_or(dungeon_center);

    warn!("teleporting player too start location: {}", pos);
    let player_ent = player_query.single();
    tp_events.send(ActorTeleportEvent {
        tp_type: TpTriggerEffect::Global(pos),
        target: Some(player_ent),
        sender: Some(player_ent),
    });
}
