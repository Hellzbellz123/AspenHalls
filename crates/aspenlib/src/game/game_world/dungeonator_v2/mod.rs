use std::time::Duration;

use bevy::{
    asset::Assets,
    ecs::schedule::States,
    log::info,
    math::Vec2,
    prelude::{
        apply_deferred, default, resource_changed, resource_exists, in_state,
        Commands, Condition, IntoSystemConfigs, OnEnter, OnExit, Plugin, Res, SpatialBundle,
        Transform, Update,
    },
    reflect::Reflect,
    time::common_conditions::on_timer,
};
use bevy_ecs_ldtk::{assets::LdtkExternalLevel, prelude::LdtkProject};

use crate::{
    game::game_world::{
        components::RoomExit,
        dungeonator_v2::{
            components::{
                Dungeon, DungeonContainerBundle, DungeonRoomDatabase, DungeonSettings,
                RoomBlueprint, RoomDistribution, RoomPreset,
            },
            hallways::HallWayBlueprint, tile_graph::TileGraph,
        }, random_point_inside,
    },
    loading::assets::AspenMapHandles,
    register_types,
};

use bevy::prelude::*;

use crate::game::{
    characters::{
        components::{CharacterMoveState, TeleportStatus},
        player::PlayerSelectedHero,
    },
    game_world::components::{ActorTeleportEvent, TpTriggerEffect},
};

/// Dungeon Generator components
pub mod components;
/// hallway creation system
pub mod hallways;
/// room selection and creation
pub mod rooms;
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
    SelectPresets,
    /// spawns rooms in world
    SpawnSelectedRooms,
    /// modify PlacedRoom position too be center of rooms
    FinalizeRooms,
    /// CreatingDungeon
    PlanHallways,
    /// place hallway blueprints
    PlaceHallwayRoots,
    /// build hallway tiles for each blueprint
    FinalizeHallways,
    /// finished making dunegon
    FinishedDungeonGen,
}

/// generates dungeons from ldtk level files
pub struct DungeonGeneratorPlugin;

impl Plugin for DungeonGeneratorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        register_types!(
            app,
            [
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

        app.add_systems(
            OnEnter(GeneratorState::SelectPresets),
            (spawn_dungeon_root, apply_deferred, rooms::select_presets).chain(),
        );
        app.add_systems(OnExit(GeneratorState::SelectPresets), (rooms::spawn_presets, teleport_player_too_start_location).chain());

        app.add_systems(
            Update,
            hallways::update_room_instances.run_if(
                in_state(GeneratorState::FinalizeRooms)
                    .and_then(on_timer(Duration::from_secs_f32(0.2))),
            ),
        );
        app.add_systems(
            OnExit(GeneratorState::FinalizeRooms),
            tile_graph::create_tile_graph,
        );
        app.add_systems(
            Update,
            hallways::plan_hallways.run_if(in_state(GeneratorState::PlanHallways)),
        );
        app.add_systems(
            OnEnter(GeneratorState::PlaceHallwayRoots),
            (
                apply_deferred,
                hallways::spawn_hallway_roots,
                apply_deferred,
            ),
        );
        app.add_systems(
            Update,
            (hallways::hallway_builder::build_hallways)
                .run_if(in_state(GeneratorState::FinalizeHallways)),
        );

        app.add_systems(
            Update,
            rooms::generate_room_database.run_if(
                resource_exists::<AspenMapHandles>.and_then(
                    resource_changed::<Assets<LdtkExternalLevel>>
                        .or_else(resource_changed::<Assets<LdtkProject>>),
                ),
            ),
        );
    }
}

/// spawns dungeon root
fn spawn_dungeon_root(mut cmds: Commands, ldtk_project_handles: Res<AspenMapHandles>) {
    info!("spawning dungeon container");
    let position = Transform::from_xyz(0.0, 0.0, 0.0);
    cmds.spawn((DungeonContainerBundle {
        name: "DungeonContainer".into(),
        dungeon: Dungeon {
            rooms: Vec::new(),
            hallways: Vec::new(),
            tile_graph: TileGraph::new(4000 / 32, position.translation.truncate()),
            settings: DungeonSettings {
                // room placing settings
                map_halfsize: 2000.0,
                tiles_between_rooms: 4,
                distribution: RoomDistribution {
                    small_short: 6,
                    small_long: 4,
                    medium_short: 2,
                    medium_long: 2,
                    large_short: 3,
                    large_long: 2,
                    huge_short: 1,
                    huge_long: 1,
                    special: 2,
                },
                // hallway placing settings/data
                hallway_loop_chance: 0.08,
            },
        },
        ldtk_project: ldtk_project_handles.default_levels.clone(),
        spatial: SpatialBundle {
            transform: position,
            ..default()
        },
    },));
}

/// teleports player too the average `Transform` of all entities with `PlayerStartLocation`
#[allow(clippy::type_complexity)]
fn teleport_player_too_start_location(
    mut player_query: Query<(Entity, &mut CharacterMoveState), With<PlayerSelectedHero>>,
    mut tp_events: EventWriter<ActorTeleportEvent>,
) {
    let start_size = Vec2 { x: 50.0, y: 50.0 };
    let start_pos = Vec2::ZERO;

    let start_loc_rect = Rect {
        min: Vec2 {
            x: start_pos.x - start_size.x,
            y: start_pos.y - start_size.y,
        },
        max: Vec2 {
            x: start_pos.x + start_size.x,
            y: start_pos.y + start_size.y,
        },
    };

    let pos = random_point_inside(&start_loc_rect, 3.0).unwrap_or(start_pos);

    warn!("teleporting player too start location: {}", pos);
    let (player_ent, mut player_tp_state) = player_query.single_mut();
    tp_events.send(ActorTeleportEvent {
        tp_type: TpTriggerEffect::Global(pos),
        target: Some(player_ent),
        sender: Some(player_ent),
    });
    player_tp_state.teleport_status = TeleportStatus::Requested;
}
