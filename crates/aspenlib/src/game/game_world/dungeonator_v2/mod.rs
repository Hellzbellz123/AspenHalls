use std::time::Duration;

use bevy::{
    asset::Assets,
    ecs::schedule::States,
    log::info,
    prelude::{
        default, resource_changed, resource_exists, run_once, state_exists_and_equals, Commands,
        Condition, IntoSystemConfigs, OnEnter, OnExit, Plugin, Res, SpatialBundle, Transform,
        Update,
    },
    reflect::Reflect,
    time::common_conditions::on_timer,
};
use bevy_ecs_ldtk::{assets::LdtkExternalLevel, prelude::LdtkProject};
use seldom_map_nav::prelude::Pathfind;

use crate::{
    game::game_world::{dungeonator_v2::{
        components::{
            DungeonContainerBundle, DungeonContainerTag, DungeonRoomDatabase, DungeonSettings,
            PlacedRoom, RoomAmounts, RoomPreset,
        },
        hallways::PlacedHallWay,
    }, components::RoomExit},
    loading::assets::AspenMapHandles,
    register_types, AppState,
};

/// Dungeon Generator components
pub mod components;
pub mod hallways;
pub mod path_map;
pub mod rooms;
pub mod utils;

/// are we in dungeon yet?
#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default, Reflect)]
pub enum GeneratorState {
    /// no dungeon is spawned
    #[default]
    NoDungeon,
    // select presets
    SelectPresets,
    // spawns rooms in world
    SpawnSelectedRooms,
    /// modify PlacedRoom position too be center of rooms
    FinalizeRooms,
    /// CreatingDungeon
    CreateHallwayTree,
    PlaceHallwayRoots,
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
                PlacedRoom,
                PlacedHallWay,
                RoomAmounts,
                DungeonSettings,
                DungeonRoomDatabase
            ]
        );

        app.add_state::<GeneratorState>();
        app.add_systems(OnExit(AppState::Loading), spawn_dungeon_root);

        app.add_systems(
            OnEnter(GeneratorState::SelectPresets),
            rooms::select_presets,
        );
        app.add_systems(OnExit(GeneratorState::SelectPresets), rooms::spawn_presets);
        app.add_systems(
            Update,
            hallways::update_room_instances.chain().run_if(
                state_exists_and_equals(GeneratorState::FinalizeRooms)
                    .and_then(on_timer(Duration::from_secs_f32(0.2))),
            ),
        );
        app.add_systems(
            OnExit(GeneratorState::FinalizeRooms),
            path_map::create_pathmap,
        );
        app.add_systems(
            OnEnter(GeneratorState::CreateHallwayTree),
            hallways::plan_hallways_two,
        );
        app.add_systems(
            OnEnter(GeneratorState::PlaceHallwayRoots),
            hallways::spawn_hallway_roots,
        );
        app.add_systems(
            Update,
            hallways::hallway_builder::build_hallways.run_if(
                on_timer(Duration::from_secs(2))
                    .and_then(state_exists_and_equals(GeneratorState::FinalizeHallways)),
            ),
        );

        app.add_systems(
            Update,
            rooms::generate_room_database.run_if(
                resource_exists::<AspenMapHandles>().and_then(
                    resource_changed::<Assets<LdtkExternalLevel>>()
                        .or_else(resource_changed::<Assets<LdtkProject>>()),
                ),
            ),
        );
    }
}

/// spawns dungeon root
fn spawn_dungeon_root(mut cmds: Commands, ldtk_project_handles: Res<AspenMapHandles>) {
    info!("spawning dungeon container");
    cmds.spawn((DungeonContainerBundle {
        tag: DungeonContainerTag,
        name: "DungeonContainer".into(),
        settings: DungeonSettings {
            // room placing settings
            map_halfsize: 2000.0,
            tiles_between_rooms: 4,
            room_amount: RoomAmounts {
                small_short: 3,
                small_long: 2,
                medium_short: 1,
                medium_long: 1,
                large_short: 1,
                large_long: 1,
                huge_short: 1,
                huge_long: 1,
                special: 2,
            },
            // hallway placing settings/data
            loops_percentage: 0.08,
            positioned_rooms: Vec::new(),
            positioned_hallways: Vec::new(),
        },
        ldtk_project: ldtk_project_handles.default_levels.clone(),
        spatial: SpatialBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
    },));
}
