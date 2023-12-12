use bevy::{
    asset::Assets,
    ecs::{bundle::Bundle, entity::Entity, query::With, schedule::States, system::Resource},
    log::{info, warn},
    math::{Rect, Vec2},
    prelude::{
        default, resource_changed, resource_exists, state_exists_and_equals, BuildChildren,
        Commands, Component, Condition, DespawnRecursiveExt, Handle, IntoSystemConfigs, Name,
        NextState, OnExit, Plugin, Query, Res, SpatialBundle, Transform, Update, Without,
    },
    reflect::Reflect,
};
use bevy_ecs_ldtk::{
    assets::LdtkExternalLevel,
    prelude::{
        ldtk::{FieldInstance},
        FieldValue, LdtkProject,
    },
    LevelIid,
};
use leafwing_input_manager::prelude::ActionState;
use rand::prelude::{IteratorRandom, Rng, SliceRandom, ThreadRng};

use crate::{
    ahp::game::{action_maps, Player},
    consts::TILE_SIZE,
    game::actors::components::ActorMoveState,
    loading::assets::MapAssetHandles,
    AppState,
};

/// generates dungeons from ldtk level files
pub struct DungeonGeneratorPlugin;

impl Plugin for DungeonGeneratorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<RoomAmounts>()
            .register_type::<DungeonSettings>()
            .add_state::<DungeonGeneratorState>();
        app.add_systems(OnExit(AppState::Loading), spawn_dungeon_root);
        app.add_systems(
            Update,
            (
                generate_room_database.run_if(
                    resource_exists::<MapAssetHandles>().and_then(
                        resource_changed::<Assets<LdtkExternalLevel>>()
                            .or_else(resource_changed::<Assets<LdtkProject>>()),
                    ),
                ),
                (prepare_dungeon_rooms).run_if(state_exists_and_equals(
                    DungeonGeneratorState::PrepareDungeon,
                )),
                (spawn_rooms).run_if(state_exists_and_equals(
                    DungeonGeneratorState::SpawningDungeon,
                )),
                (listen_rebuild_dungeon_request,).run_if(state_exists_and_equals(
                    DungeonGeneratorState::FinishedDungeonGen,
                )),
            ),
        );
    }
}

/// listens for dungeon rebuild request if dungeon is finished spawning
fn listen_rebuild_dungeon_request(
    mut cmds: Commands,
    mut dungeon_root: Query<(Entity, &mut DungeonSettings), With<DungeonContainerTag>>,
    enemies: Query<Entity, (With<ActorMoveState>, Without<Player>)>,
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
        enemies.for_each(|f| {
            cmds.entity(f).despawn_descendants();
        });
        dungeon_root.1.positioned_presets = Vec::new();
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
            tiles_between_rooms: 6,
            positioned_presets: Vec::new(),
            room_amount: RoomAmounts {
                small_short: 9,
                small_long: 8,
                medium_short: 7,
                medium_long: 6,
                large_short: 5,
                large_long: 4,
                huge_short: 4,
                huge_long: 3,
                special: 2,
            },
            // looped_hallway_percentage: 0.0,
            // fill_percentage: 0.0,
            // positioned_rooms: Vec::new(),
        },
        ldtk_project: ldtk_project_handles.default_levels.clone(),
        spatial: SpatialBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
    },));
}

fn generate_room_database(
    mut cmds: Commands,
    map_projects: Res<MapAssetHandles>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    level_assets: Res<Assets<LdtkExternalLevel>>,
) {
    let dungeon_project = ldtk_project_assets
        .get(map_projects.default_levels.clone())
        .expect("project not found")
        .as_parent();

    let mut dungeon_database = DungeonRoomDatabase {
        sactuary: Vec::new(),
        starts: Vec::new(),
        ends: Vec::new(),
        specials: Vec::new(),
        small_shorts: Vec::new(),
        small_longs: Vec::new(),
        medium_shorts: Vec::new(),
        medium_longs: Vec::new(),
        large_shorts: Vec::new(),
        large_longs: Vec::new(),
        huge_shorts: Vec::new(),
        huge_longs: Vec::new(),
    };

    dungeon_project
        .iter_external_levels(&level_assets)
        .for_each(|level_def| {
            let field_instances = level_def.field_instances();
            let room_shape = try_get_roomshape(field_instances)
                .expect("No size ident on room definiton. Check Ldtk Editor for errors");
            let room_type = try_get_roomtype(field_instances)
                .expect("No type ident on room definition. Check Ldtk Editor for errors");
            let room_level = try_get_roomlevel(field_instances)
                .expect("No level ident on room definition. Check Ldtk Editor for errors");

            let room = RoomPreset {
                name: level_def.identifier().to_string(),
                room_asset_id: level_def.iid().to_string(),
                size: Vec2::new(*level_def.px_wid() as f32, *level_def.px_hei() as f32),
                position: None,
                shape: room_shape.clone(),
                level: room_level,
                rtype: room_type.clone(),
            };

            match &room_type {
                RoomType::Sanctuary => dungeon_database.sactuary.push(room),
                RoomType::DungeonStart => dungeon_database.starts.push(room),
                RoomType::Boss => dungeon_database.ends.push(room),
                RoomType::Special => dungeon_database.specials.push(room),
                RoomType::Normal => match room_shape {
                    RoomShape::Special => dungeon_database.specials.push(room),
                    RoomShape::SmallShort => dungeon_database.small_shorts.push(room),
                    RoomShape::SmallLong => dungeon_database.small_longs.push(room),
                    RoomShape::MediumShort => dungeon_database.medium_shorts.push(room),
                    RoomShape::MediumLong => dungeon_database.medium_longs.push(room),
                    RoomShape::LargeShort => dungeon_database.large_shorts.push(room),
                    RoomShape::LargeLong => dungeon_database.large_longs.push(room),
                    RoomShape::HugeShort => dungeon_database.huge_shorts.push(room),
                    RoomShape::HugeLong => dungeon_database.huge_longs.push(room),
                },
            }
        });
    cmds.insert_resource(dungeon_database);
}

fn prepare_dungeon_rooms(
    // difficulty: Res<GeneralSettings>,
    mut cmds: Commands,
    room_database: Res<DungeonRoomDatabase>,
    mut dungeon_root: Query<&mut DungeonSettings, With<DungeonContainerTag>>,
) {
    let progress_level = RoomLevel::Level1;
    let mut filled_positions: Vec<Rect> = Vec::new();
    let mut chosen_rooms: Vec<RoomPreset> = Vec::new();
    let mut settings = dungeon_root.single_mut();

    let mut start_room =
        get_leveled_preset(&room_database.starts, progress_level.clone()).unwrap();
    let startroom_rect = Rect {
        min: Vec2::ZERO + -(start_room.size / 2.0),
        max: Vec2::ZERO + (start_room.size / 2.0),
    };
    start_room.position = Some(startroom_rect.min);
    filled_positions.push(startroom_rect);

    chosen_rooms.push(get_leveled_preset(&room_database.ends, progress_level).unwrap());

    for _ in 0..settings.room_amount.small_short {
        if !room_database.small_shorts.is_empty() {
            chosen_rooms.push(get_random_preset(&room_database.small_shorts).unwrap());
        }
    }

    for _ in 0..settings.room_amount.small_long {
        if !room_database.small_longs.is_empty() {
            chosen_rooms.push(get_random_preset(&room_database.small_longs).unwrap());
        }
    }

    for _ in 0..settings.room_amount.medium_short {
        if !room_database.medium_shorts.is_empty() {
            chosen_rooms.push(get_random_preset(&room_database.medium_shorts).unwrap());
        }
    }

    for _ in 0..settings.room_amount.medium_long {
        if !room_database.medium_longs.is_empty() {
            chosen_rooms.push(get_random_preset(&room_database.medium_longs).unwrap());
        }
    }

    for _ in 0..settings.room_amount.large_short {
        if !room_database.large_shorts.is_empty() {
            chosen_rooms.push(get_random_preset(&room_database.large_shorts).unwrap());
        }
    }

    for _ in 0..settings.room_amount.large_long {
        if !room_database.large_longs.is_empty() {
            chosen_rooms.push(get_random_preset(&room_database.large_longs).unwrap());
        }
    }

    for _ in 0..settings.room_amount.huge_short {
        if !room_database.huge_shorts.is_empty() {
            chosen_rooms.push(get_random_preset(&room_database.huge_shorts).unwrap());
        }
    }

    for _ in 0..settings.room_amount.huge_long {
        if !room_database.huge_longs.is_empty() {
            chosen_rooms.push(get_random_preset(&room_database.huge_longs).unwrap());
        }
    }

    for _ in 0..settings.room_amount.special {
        if !room_database.specials.is_empty() {
            chosen_rooms.push(get_random_preset(&room_database.specials).unwrap());
        }
    }

    let mut positioned_rooms = chosen_rooms
        .iter_mut()
        .map(|f| {
            let pos = random_room_positon(&filled_positions, f.size, &settings);
            filled_positions.push(pos);
            f.position = Some(pos.min);
            f.clone()
        })
        .collect::<Vec<RoomPreset>>();

    positioned_rooms.insert(0, start_room);

    settings.positioned_presets = positioned_rooms;
    cmds.insert_resource(NextState(Some(DungeonGeneratorState::SpawningDungeon)));
}

/// spawns possible dungeon rooms from `dungeon_root.dungeon_settings.useable_dungeons`
fn spawn_rooms(
    mut cmds: Commands,
    dungeon_root: Query<
        (Entity, &DungeonSettings, &Handle<LdtkProject>),
        With<DungeonContainerTag>,
    >,
) {
    let (dungeon_root, dungeon_settings, _proj_handle) = dungeon_root.single();
    if dungeon_settings.positioned_presets.is_empty() {
        warn!("no dungeon presets were prepared");
        return;
    }
    let mut to_spawn_dungeons: Vec<DungeonRoomBundle> =
        Vec::with_capacity(dungeon_settings.positioned_presets.len());

    for ready_preset in &dungeon_settings.positioned_presets {
        let preset = ready_preset;
        let pos = preset
            .position
            .expect("all ready presets should have a Vec2 position");
        let name = format!("[DungeonRoom-{}", ready_preset.name);
        to_spawn_dungeons.push(DungeonRoomBundle {
            tag: DungeonRoomTag,
            name: name.into(),
            id: preset.room_asset_id.clone().into(),
            spatial: SpatialBundle::from_transform(Transform::from_xyz(pos.x, pos.y, 0.0)),
        });
    }

    info!("amount of dungeons too spawn: {}", to_spawn_dungeons.len());
    cmds.entity(dungeon_root).with_children(|dungeons| {
        if to_spawn_dungeons.is_empty() {
            warn!("no dungeons in to_spawn_dungeons");
            return;
        }
        for bundle in to_spawn_dungeons {
            dungeons.spawn(bundle);
        }
    });

    info!("finished spawning dungeons");
    cmds.insert_resource(NextState(Some(DungeonGeneratorState::FinishedDungeonGen)));
}

/// Creates randomly positioned `Rect` that doesnt overlap any `Rect` in `occupied_positions`
///
/// configured with `DungeonSettings`
fn random_room_positon(filled_positions: &[Rect], size: Vec2, settings: &DungeonSettings) -> Rect {
    let start_range = settings.map_halfsize;
    let mut range_abs = settings.map_halfsize;
    let cloned_positions = filled_positions.to_owned();
    let mut rng = ThreadRng::default();
    let mut attempt_count = 0;
    let max_attempts = 100;

    loop {
        let range = if attempt_count < max_attempts {
            start_range
        } else {
            range_abs *= 2.0;
            range_abs
        };

        let x = ((rng.gen_range(-range..range)) / TILE_SIZE.y).round() * TILE_SIZE.y;
        let y = (rng.gen_range(-range..range) / TILE_SIZE.x).round() * TILE_SIZE.x;
        let (width, height) = (size.x, size.y);
        let tested_rect = Rect::new(x, y, x + width, y + height);

        info!("occupied positions: {:?}", filled_positions);
        info!("room position: X: {}, Y: {}", x, y);
        info!("room size: {}", size);

        // test if test_rect has no intersections with currently spawned recs
        if cloned_positions
            .iter()
            .all(|f| f.intersect(tested_rect).is_empty())
        {
            // test if test rect is far enough from other rects
            if cloned_positions.iter().all(|rect| {
                let t_rec2 = tested_rect.inset(settings.tiles_between_rooms as f32 * TILE_SIZE.x);
                rect.inset((settings.tiles_between_rooms as f32 * TILE_SIZE.x) * 2.0)
                    .intersect(t_rec2)
                    .is_empty()
            }) {
                warn!("returning valid value");
                return tested_rect;
            } else {
                attempt_count += 1;
                warn!("room intersect: false, too close: true");
                warn!("restarting with new alue");
                continue;
            }
        } else {
            attempt_count += 1;
            warn!("room intersect: true, too close: true");
            warn!("restarting with new value");
            continue;
        };
    }
}

fn get_random_preset(section: &[RoomPreset]) -> Option<RoomPreset> {
    let mut rng = ThreadRng::default();

    section.iter().choose(&mut rng).cloned()
}

fn get_leveled_preset(section: &[RoomPreset], level: RoomLevel) -> Option<RoomPreset> {
    let mut rng = ThreadRng::default();

    section
        .iter()
        .filter(|f| f.level == level)
        .choose(&mut rng)
        .cloned()
}

fn try_get_roomshape(field_instances: &[FieldInstance]) -> Option<RoomShape> {
    let Some(room_ident) = field_instances.iter().find(|f| f.identifier == "IdentSize") else {
        return None;
    };
    let FieldValue::Enum(Some(enum_value)) = &room_ident.value else {
        return None;
    };

    match enum_value.as_str() {
        "SmallShort" => Some(RoomShape::SmallShort),
        "SmallLong" => Some(RoomShape::SmallLong),
        "MediumShort" => Some(RoomShape::MediumShort),
        "MediumLong" => Some(RoomShape::MediumLong),
        "LargeShort" => Some(RoomShape::LargeShort),
        "LargeLong" => Some(RoomShape::LargeLong),
        "HugeShort" => Some(RoomShape::HugeShort),
        "HugeLong" => Some(RoomShape::HugeLong),
        "Special" => Some(RoomShape::Special),
        _ => None,
    }
}

fn try_get_roomlevel(field_instances: &[FieldInstance]) -> Option<RoomLevel> {
    let Some(room_ident) = field_instances
        .iter()
        .find(|f| f.identifier == "IdentLevel")
    else {
        return None;
    };

    let FieldValue::Enum(Some(enum_value)) = &room_ident.value else {
        return None;
    };

    match enum_value.as_str() {
        "Level0" => Some(RoomLevel::Level0),
        "Level1" => Some(RoomLevel::Level1),
        "Level2" => Some(RoomLevel::Level2),
        "Level3" => Some(RoomLevel::Level3),
        _ => None,
    }
}

fn try_get_roomtype(field_instances: &[FieldInstance]) -> Option<RoomType> {
    let Some(room_ident) = field_instances.iter().find(|f| f.identifier == "IdentType") else {
        return None;
    };

    let FieldValue::Enum(Some(enum_value)) = &room_ident.value else {
        return None;
    };

    match enum_value.as_str() {
        "DungeonStart" => Some(RoomType::DungeonStart),
        "Boss" => Some(RoomType::Boss),
        "Special" => Some(RoomType::Special),
        "Normal" => Some(RoomType::Normal),
        "Sanctuary" => Some(RoomType::Sanctuary),
        _ => None,
    }
}

#[derive(Debug, Clone, Default, Reflect)]
pub struct RoomAmounts {
    small_short: i32,
    small_long: i32,
    medium_short: i32,
    medium_long: i32,
    large_short: i32,
    large_long: i32,
    huge_short: i32,
    huge_long: i32,
    special: i32,
}

#[derive(Debug, Clone, Reflect, PartialEq, Eq, PartialOrd)]
pub enum RoomLevel {
    Level0,
    Level1,
    Level2,
    Level3,
}

#[derive(Debug, Clone, Reflect, PartialEq, Eq, PartialOrd)]
pub enum RoomType {
    DungeonStart,
    Boss,
    Special,
    Normal,
    Sanctuary,
}

#[derive(Debug, Clone, Reflect, PartialEq, Eq, PartialOrd)]
pub enum RoomShape {
    Special,
    SmallShort,
    SmallLong,
    MediumShort,
    MediumLong,
    LargeShort,
    LargeLong,
    HugeShort,
    HugeLong,
}

#[allow(unused)]
#[derive(Debug, Resource)]
pub struct DungeonRoomDatabase {
    sactuary: Vec<RoomPreset>,
    starts: Vec<RoomPreset>,
    ends: Vec<RoomPreset>,
    specials: Vec<RoomPreset>,
    small_shorts: Vec<RoomPreset>,
    small_longs: Vec<RoomPreset>,
    medium_shorts: Vec<RoomPreset>,
    medium_longs: Vec<RoomPreset>,
    large_shorts: Vec<RoomPreset>,
    large_longs: Vec<RoomPreset>,
    huge_shorts: Vec<RoomPreset>,
    huge_longs: Vec<RoomPreset>,
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

/// room instances before being placed
#[derive(Debug, Component, Clone, Reflect)]
pub struct RoomPreset {
    /// name of the room
    name: String,
    /// asset id for level data
    room_asset_id: String,
    /// size of room
    /// - Rect.max is position + size
    size: Vec2,
    /// position of room in dungeon
    /// - Rect.min is position
    position: Option<Vec2>,
    shape: RoomShape,
    level: RoomLevel,
    rtype: RoomType,
}

// TODO: add dungeon level too settings
/// settings to configure the dungeon generator,
/// `useable_rooms` and hallways are filled by other systems
#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct DungeonSettings {
    /// dungeon max size / 2.0
    pub map_halfsize: f32,
    /// minimum space between dungeon rooms, in tiles
    pub tiles_between_rooms: i32,
    /// new pos rooms
    pub positioned_presets: Vec<RoomPreset>,
    /// amount of rooms inside dungeon
    pub room_amount: RoomAmounts,
    // /// percentage of paths between
    // /// rooms that are chosen to loop
    // pub looped_hallway_percentage: f32,
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

/// placeable room preset
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
