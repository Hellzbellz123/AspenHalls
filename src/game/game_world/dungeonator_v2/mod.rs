use bevy::{
    asset::Assets,
    ecs::{bundle::Bundle, entity::Entity, query::With, schedule::States, system::Resource},
    log::{info, warn},
    math::{Rect, Vec2},
    prelude::{
        default, state_exists_and_equals, BuildChildren, Commands, Component, DespawnRecursiveExt,
        Handle, IntoSystemConfigs, Name, NextState, OnExit, Plugin, Query, Res, SpatialBundle,
        Transform, Update,
    },
    reflect::Reflect,
};
use bevy_ecs_ldtk::{
    assets::LdtkExternalLevel,
    prelude::{ldtk::loaded_level::LoadedLevel, LdtkProject},
    LevelIid,
};
use leafwing_input_manager::prelude::ActionState;
use rand::prelude::{Rng, SliceRandom, ThreadRng};

use crate::{
    ahp::game::action_maps, consts::TILE_SIZE, loading::assets::MapAssetHandles, AppState,
};

/// generates dungeons from ldtk level files
pub struct DungeonGeneratorPlugin;

impl Plugin for DungeonGeneratorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<DungeonSettings>()
            .add_state::<DungeonGeneratorState>();
        app.add_systems(OnExit(AppState::Loading), spawn_dungeon_root);
        app.add_systems(
            Update,
            (
                rebuild_dungeon.run_if(state_exists_and_equals(
                    DungeonGeneratorState::FinishedDungeonGen,
                )),
                prepare_dungeon_root.run_if(state_exists_and_equals(
                    DungeonGeneratorState::PrepareDungeon,
                )),
                spawn_rooms.run_if(state_exists_and_equals(
                    DungeonGeneratorState::SpawningDungeon,
                )),
            ),
        );
    }
}

fn rebuild_dungeon(
    mut cmds: Commands,
    mut dungeon_root: Query<(Entity, &mut DungeonSettings), With<DungeonContainerTag>>,
    player_input: Query<&ActionState<action_maps::Gameplay>>,
) {
    let Ok(player_input) = player_input.get_single() else {
        return;
    };
    let Ok(mut dungeon_root) = dungeon_root.get_single_mut() else {
        return;
    };

    if player_input.just_pressed(action_maps::Gameplay::DebugF2) {
        cmds.entity(dungeon_root.0).despawn_descendants();
        dungeon_root.1.positioned_rooms = Vec::new();
        cmds.insert_resource(NextState(Some(DungeonGeneratorState::PrepareDungeon)));
    }
}

/// spawns dungeon root
fn spawn_dungeon_root(mut cmds: Commands, ldtk_project_handles: Res<MapAssetHandles>) {
    info!("spawning dungeon container");
    cmds.spawn((DungeonContainerBundle {
        tag: DungeonContainerTag,
        name: "DungeonContainer".into(),
        settings: DungeonSettings {
            map_halfsize: 5000.0,
            room_amount: 10,
            looped_hallway_percentage: 0.0,
            fill_percentage: 0.0,
            space_between: 6,
            positioned_rooms: Vec::with_capacity(10),
        },
        ldtk_project: ldtk_project_handles.dungeons.clone(),
        spatial: SpatialBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
    },));
}

fn prepare_dungeon_root(
    mut cmds: Commands,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    level_assets: Res<Assets<LdtkExternalLevel>>,
    mut dungeon_root: Query<
        (&mut DungeonSettings, &Handle<LdtkProject>),
        With<DungeonContainerTag>,
    >,
) {
    let (mut settings, project_handle) = dungeon_root.single_mut();
    let dungeon_project = ldtk_project_assets
        .get(project_handle)
        .expect("project assigned too dungeon root not found")
        .as_parent();
    let (mut start_room, other_levels): (Vec<LoadedLevel>, Vec<LoadedLevel>) = dungeon_project
        .iter_external_levels(&level_assets)
        .partition(|level| level.identifier() == "StartRoom");
    let mut occupied_positions: Vec<Rect> = Vec::new();

    info!("creating start room preset");
    generate_start_room(
        start_room.pop().expect("msg"),
        &mut occupied_positions,
        &mut settings,
    );

    info!("creating other room presets");
    generate_other_rooms(other_levels, &mut occupied_positions, &mut settings);

    if settings.positioned_rooms.len() == settings.room_amount as usize {
        info!(
            "finished generating rooms: amt checked {}",
            occupied_positions.len()
        );
        cmds.insert_resource(NextState(Some(DungeonGeneratorState::SpawningDungeon)))
    }
}

fn generate_start_room(
    start_room: LoadedLevel,
    occupied_positions: &mut Vec<Rect>,
    settings: &mut bevy::prelude::Mut<'_, DungeonSettings>,
) {
    let start_room_size = Vec2 {
        x: *start_room.px_wid() as f32,
        y: *start_room.px_hei() as f32,
    };
    let start_rect = Rect {
        min: Vec2::ZERO - start_room_size / 2.0,
        max: Vec2::ZERO + start_room_size / 2.0,
    };
    occupied_positions.insert(0, start_rect);

    settings.positioned_rooms.insert(
        0,
        RoomData {
            name: start_room.identifier().to_string(),
            iid: start_room.iid().clone().into(),
            rect: start_rect,
        },
    );
}

fn generate_other_rooms(
    presets: Vec<LoadedLevel>,
    occupied_positions: &mut Vec<Rect>,
    settings: &mut bevy::prelude::Mut<'_, DungeonSettings>,
) {
    for i in 1..settings.room_amount {
        let mut rng = rand::thread_rng();
        let level_data = presets.choose(&mut rng).unwrap();

        let size = Vec2 {
            x: *level_data.px_wid() as f32,
            y: *level_data.px_hei() as f32,
        };
        let pos = random_vec2(&occupied_positions, size, &settings);
        let new_rect = Rect {
            min: pos,
            max: pos + size,
        };

        info!("occupied positions: {:?}", occupied_positions);
        info!("room positon: {}", pos);
        info!("room size: {}", size);
        info!("creating room: [{i}]-{}", level_data.identifier());
        info!("testing intersections with previous rooms");
        if occupied_positions
            .iter()
            .any(|rect| !rect.intersect(new_rect).is_empty())
        {
            panic!("Overlap detected! Rooms should not overlap.");
        }
        occupied_positions.push(new_rect);
        settings.positioned_rooms.insert(
            1,
            RoomData {
                name: level_data.identifier().to_string(),
                iid: level_data.iid().clone().into(),
                rect: new_rect,
            },
        );
    }
}

/// spawns possible dungeon rooms from dungeon_root.dungeon_settings.useable_dungeons
fn spawn_rooms(
    mut cmds: Commands,
    dungeon_root: Query<
        (Entity, &DungeonSettings, &Handle<LdtkProject>),
        With<DungeonContainerTag>,
    >,
) {
    let (dungeon_root, dungeon_settings, _proj_handle) = dungeon_root.single();

    let mut to_spawn_dungeons: Vec<DungeonRoomBundle> =
        Vec::with_capacity(dungeon_settings.room_amount as usize);

    let mut positioned_rooms = dungeon_settings.positioned_rooms.clone();

    if positioned_rooms.is_empty() {
        warn!("no dungeon presets were prepared");
        return;
    }

    let start_room = positioned_rooms.remove(0);

    if start_room.name != "StartRoom".to_string() {
        panic!("small start room was the wrong RoomData")
    }

    let start_room_bundle = DungeonRoomBundle {
        tag: DungeonRoomTag,
        name: Name::new("StartRoom"),
        id: start_room.iid.clone(),
        spatial: SpatialBundle::from_transform(Transform::from_xyz(
            start_room.rect.min.x,
            start_room.rect.min.y,
            0.0,
        )),
    };

    to_spawn_dungeons.insert(0, start_room_bundle);

    for _i in 0..=positioned_rooms.len() {
        match positioned_rooms.pop() {
            Some(data) => {
                let name = format!("[DungeonRoom-{}", data.name);
                to_spawn_dungeons.push(DungeonRoomBundle {
                    tag: DungeonRoomTag,
                    name: name.into(),
                    id: data.iid,
                    spatial: SpatialBundle::from_transform(Transform::from_xyz(
                        data.rect.min.x,
                        data.rect.min.y,
                        0.0,
                    )),
                })
            }
            None => {
                info!("amount of dungeons too spawn: {}", to_spawn_dungeons.len());
                cmds.entity(dungeon_root).with_children(|dungeons| {
                    if to_spawn_dungeons.is_empty() {
                        warn!("no dungeons in to_spawn_dungeons");
                        return;
                    }
                    for _ in 0..to_spawn_dungeons.len() {
                        let a = to_spawn_dungeons.swap_remove(0);
                        dungeons.spawn(a);
                    }
                });
                info!("finished spawning dungeons");
                cmds.insert_resource(NextState(Some(DungeonGeneratorState::FinishedDungeonGen)));
            }
        }
    }
}

fn random_vec2(occupied_positions: &Vec<Rect>, size: Vec2, settings: &DungeonSettings) -> Vec2 {
    let range_abs = settings.map_halfsize;
    let cloned_positions = occupied_positions.clone();

    let mut rng = ThreadRng::default();
    let mut pos = Vec2::new(
        (rng.gen_range(-range_abs..range_abs) / TILE_SIZE.x).round() * TILE_SIZE.x,
        ((rng.gen_range(-range_abs..range_abs)) / TILE_SIZE.y).round() * TILE_SIZE.y,
    );

    let mut test_rect = Rect::new(pos.x, pos.y, pos.x + size.x, pos.y + size.y);

    // Ensure that the new position does not overlap with existing occupied positions
    while cloned_positions.iter().any(|rect| {
        !rect.intersect(test_rect).is_empty()
    }) {
        warn!("old position intersects: {}", pos);
        pos = Vec2::new(
            (rng.gen_range(-range_abs..range_abs) / TILE_SIZE.x).round() * TILE_SIZE.x,
            ((rng.gen_range(-range_abs..range_abs)) / TILE_SIZE.y).round() * TILE_SIZE.y,
        );
        test_rect = Rect::new(pos.x, pos.y, pos.x + size.x, pos.y + size.y);
        warn!("testing intersect on new position: {}", pos);
    }

    while cloned_positions.iter().all(|rect| {
        let a = rect.min.distance(test_rect.min);
        info!("distance: {}", a);
        !(a > settings.space_between as f32 * TILE_SIZE.x)
    }) {
        warn!("old position too far: {}", pos);
        pos = Vec2::new(
            (rng.gen_range(-range_abs..range_abs) / TILE_SIZE.x).round() * TILE_SIZE.x,
            ((rng.gen_range(-range_abs..range_abs)) / TILE_SIZE.y).round() * TILE_SIZE.y,
        );
        test_rect = Rect::new(pos.x, pos.y, pos.x + size.x, pos.y + size.y);
            warn!("testing distance on new position: {}", pos);
    }
    pos
}

/// are we in dungeon yet?
#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Resource, Default, Reflect)]
pub enum DungeonGeneratorState {
    /// no dungeon is spawned
    #[default]
    NoDungeon,
    /// prepare dungeon resources for gen
    PrepareDungeon,
    /// CreatingDungeon
    SpawningDungeon,
    /// finished making dunegon
    FinishedDungeonGen,
}

/// settings to configure the dungeon generator,
/// `useable_rooms` and hallways are filled by other systems
#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct DungeonSettings {
    /// dungeon max size / 2.0
    pub map_halfsize: f32,
    /// amount of rooms inside dungeon
    pub room_amount: i32,
    /// percentage of paths between
    /// rooms that are chosen to loop
    pub looped_hallway_percentage: f32,
    /// empty space too dungeon room percentage
    pub fill_percentage: f32,
    /// minimum space between dungeon rooms, in tiles
    pub space_between: i32,
    /// generated rooms list too be placed
    pub positioned_rooms: Vec<RoomData>,
}

/// tag too identify dungeons
#[derive(Component)]
pub struct DungeonContainerTag;

/// tag too identify dungeon hallways
#[derive(Component)]
pub struct DungeonHallwayTag;

/// tag too identify dungeon rooms
#[derive(Component, Debug)]
pub struct DungeonRoomTag;

/// bundle for easy spawning of dungeon
/// always 1 per dungeon, all dungeon rooms are children
#[derive(Bundle)]
pub struct DungeonContainerBundle {
    /// identifies dungeon root entity
    tag: DungeonContainerTag,
    /// identified dungeon root in hierarchy
    name: Name,
    /// configures spawning of child rooms and hallways
    settings: DungeonSettings,
    /// data used too spawn with
    ldtk_project: Handle<LdtkProject>,
    /// gives dungeons a position
    spatial: SpatialBundle,
}

/// bundle for easy spawning of Dungeon Rooms
#[derive(Bundle, Debug)]
pub struct DungeonRoomBundle {
    /// identifies dungeon rooms
    tag: DungeonRoomTag,
    /// basically just `LevelIid`
    name: Name,
    /// id from `LdtkProject`
    id: LevelIid,
    /// spatial data
    spatial: SpatialBundle,
}

/// bundle for easy spawning of Dungeon Hallways
#[derive(Bundle)]
pub struct DungeonHallWayBundle {
    /// identifies dungeon hallways
    tag: DungeonHallwayTag,
    /// Hallway# from-to
    name: Name,
    /// spatial data
    spatial: SpatialBundle,
}

#[derive(Debug, Component, Clone, Reflect)]
pub struct RoomData {
    name: String,
    iid: LevelIid,
    rect: Rect,
}
