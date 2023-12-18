use bevy::{
    asset::Assets,
    ecs::{entity::Entity, query::With, schedule::States},
    log::{info, warn},
    math::{Rect, Vec2},
    prelude::{
        default, resource_changed, resource_exists, state_exists_and_equals, BuildChildren,
        Commands, Condition, DespawnRecursiveExt, EventWriter, Handle, IntoSystemConfigs,
        NextState, OnExit, Plugin, Query, Res, SpatialBundle, Transform, Update, Without,
    },
    reflect::Reflect,
};
use bevy_ecs_ldtk::{assets::LdtkExternalLevel, prelude::LdtkProject};
use leafwing_input_manager::prelude::ActionState;
use rand::prelude::{Rng, ThreadRng};

use crate::{
    ahp::game::{action_maps, ActorType, Faction, Player, SpawnActorEvent},
    consts::TILE_SIZE,
    game::{actors::components::ActorMoveState, game_world::dungeonator_v2::components::*},
    loading::assets::MapAssetHandles,
    AppState,
};

/// Dungeon Generator components
mod components;

/// are we in dungeon yet?
#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default, Reflect)]
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

/// generates dungeons from ldtk level files
pub struct DungeonGeneratorPlugin;

impl Plugin for DungeonGeneratorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<RoomPreset>()
            .register_type::<RoomAmounts>()
            .register_type::<DungeonSettings>()
            .register_type::<DungeonRoomDatabase>();

        app.add_state::<DungeonGeneratorState>();
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
    // app_state:
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
            cmds.entity(f).despawn_recursive();
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
        },
        ldtk_project: ldtk_project_handles.default_levels.clone(),
        spatial: SpatialBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
    },));
}

/// maps `level_assets` too a `DungeonRoomDatabase`
/// dungeons are filtered into vecs based on level custom data
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
        hideouts: Vec::new(),
        start_rooms: Vec::new(),
        end_rooms: Vec::new(),
        special_rooms: Vec::new(),
        small_short_rooms: Vec::new(),
        small_long_rooms: Vec::new(),
        medium_short_rooms: Vec::new(),
        medium_long_rooms: Vec::new(),
        large_short_rooms: Vec::new(),
        large_long_rooms: Vec::new(),
        huge_short_rooms: Vec::new(),
        huge_long_rooms: Vec::new(),
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
                RoomType::Hideout => dungeon_database.hideouts.push(room),
                RoomType::DungeonStart => dungeon_database.start_rooms.push(room),
                RoomType::Boss => dungeon_database.end_rooms.push(room),
                RoomType::Normal | RoomType::Special => match room_shape {
                    RoomShape::NonStandard => dungeon_database.special_rooms.push(room),
                    RoomShape::SmallShort => dungeon_database.small_short_rooms.push(room),
                    RoomShape::SmallLong => dungeon_database.small_long_rooms.push(room),
                    RoomShape::MediumShort => dungeon_database.medium_short_rooms.push(room),
                    RoomShape::MediumLong => dungeon_database.medium_long_rooms.push(room),
                    RoomShape::LargeShort => dungeon_database.large_short_rooms.push(room),
                    RoomShape::LargeLong => dungeon_database.large_long_rooms.push(room),
                    RoomShape::HugeShort => dungeon_database.huge_short_rooms.push(room),
                    RoomShape::HugeLong => dungeon_database.huge_long_rooms.push(room),
                },
            }
        });
    cmds.insert_resource(dungeon_database);
}

/// creates vec of `RoomPreset` too be built into `DungeonRoomBundles` using  `DungeonSettings` and current progress in the dungeon
fn prepare_dungeon_rooms(
    mut cmds: Commands,
    room_database: Res<DungeonRoomDatabase>,
    mut dungeon_root: Query<&mut DungeonSettings, With<DungeonContainerTag>>,
) {
    let progress_level = RoomLevel::Level1;
    let mut room_positions: Vec<Rect> = Vec::new();
    let mut rooms_too_spawn: Vec<RoomPreset> = Vec::new();
    let mut settings = dungeon_root.single_mut();

    for _ in 0..settings.room_amount.small_short {
        if !room_database.small_short_rooms.is_empty() {
            rooms_too_spawn.push(get_random_preset(&room_database.small_short_rooms).unwrap());
        }
    }

    for _ in 0..settings.room_amount.small_long {
        if !room_database.small_long_rooms.is_empty() {
            rooms_too_spawn.push(get_random_preset(&room_database.small_long_rooms).unwrap());
        }
    }

    for _ in 0..settings.room_amount.medium_short {
        if !room_database.medium_short_rooms.is_empty() {
            rooms_too_spawn.push(get_random_preset(&room_database.medium_short_rooms).unwrap());
        }
    }

    for _ in 0..settings.room_amount.medium_long {
        if !room_database.medium_long_rooms.is_empty() {
            rooms_too_spawn.push(get_random_preset(&room_database.medium_long_rooms).unwrap());
        }
    }

    for _ in 0..settings.room_amount.large_short {
        if !room_database.large_short_rooms.is_empty() {
            rooms_too_spawn.push(get_random_preset(&room_database.large_short_rooms).unwrap());
        }
    }

    for _ in 0..settings.room_amount.large_long {
        if !room_database.large_long_rooms.is_empty() {
            rooms_too_spawn.push(get_random_preset(&room_database.large_long_rooms).unwrap());
        }
    }

    for _ in 0..settings.room_amount.huge_short {
        if !room_database.huge_short_rooms.is_empty() {
            rooms_too_spawn.push(get_random_preset(&room_database.huge_short_rooms).unwrap());
        }
    }

    for _ in 0..settings.room_amount.huge_long {
        if !room_database.huge_long_rooms.is_empty() {
            rooms_too_spawn.push(get_random_preset(&room_database.huge_long_rooms).unwrap());
        }
    }

    for _ in 0..settings.room_amount.special {
        if !room_database.special_rooms.is_empty() {
            rooms_too_spawn.push(get_random_preset(&room_database.special_rooms).unwrap());
        }
    }

    let mut start_room =
        get_leveled_preset(&room_database.start_rooms, progress_level.clone()).unwrap();
    let startroom_rect = Rect {
        min: Vec2::ZERO + -(start_room.size / 2.0),
        max: Vec2::ZERO + (start_room.size / 2.0),
    };
    start_room.position = Some(startroom_rect.min);
    room_positions.push(startroom_rect);

    rooms_too_spawn.push(get_leveled_preset(&room_database.end_rooms, progress_level).unwrap());

    let mut positioned_rooms = rooms_too_spawn
        .iter_mut()
        .map(|f| {
            let pos = random_room_positon(&room_positions, f.size, &settings);
            room_positions.push(pos);
            f.position = Some(pos.min);
            f.clone()
        })
        .collect::<Vec<RoomPreset>>();

    positioned_rooms.insert(0, start_room);

    warn!("rooms passed too room-placer: {:?}", positioned_rooms);
    settings.positioned_presets = positioned_rooms;
    cmds.insert_resource(NextState(Some(DungeonGeneratorState::SpawningDungeon)));
}

/// spawns possible dungeon rooms from `dungeon_root.dungeon_settings.useable_dungeons`
fn spawn_rooms(
    mut ew: EventWriter<SpawnActorEvent>,
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
        let name = format!("DungeonRoom-{}", ready_preset.name);
        to_spawn_dungeons.push(DungeonRoomBundle {
            tag: DungeonRoomTag,
            name: name.into(),
            id: preset.room_asset_id.clone().into(),
            spatial: SpatialBundle::from_transform(Transform::from_xyz(pos.x, pos.y, 0.0)),
            // preset: ready_preset.clone(),
        });
    }

    info!("room bundles too spawn: {:?}", to_spawn_dungeons);
    cmds.entity(dungeon_root).with_children(|dungeons| {
        if to_spawn_dungeons.is_empty() {
            warn!("no dungeons in to_spawn_dungeons");
            return;
        }
        for bundle in to_spawn_dungeons {
            dungeons.spawn(bundle);
        }
    });

    cmds.insert_resource(NextState(Some(DungeonGeneratorState::FinishedDungeonGen)));
    info!("finished spawning dungeons");
    let item_offset = Vec2 { x: 50.0, y: 0.0 };

    ew.send(SpawnActorEvent {
        actor_type: ActorType::Item,
        what_to_spawn: "smallsmg".to_string(),
        spawn_position: Vec2::ZERO + -item_offset,
        spawn_count: 1,
        spawner: None,
    });
    ew.send(SpawnActorEvent {
        actor_type: ActorType::Item,
        what_to_spawn: "smallpistol".to_string(),
        spawn_position: Vec2::ZERO + item_offset,
        spawn_count: 1,
        spawner: None,
    });
}

#[allow(clippy::redundant_else)]
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

        let x = ((rng.gen_range(-range..range)) / TILE_SIZE).round() * TILE_SIZE;
        let y = (rng.gen_range(-range..range) / TILE_SIZE).round() * TILE_SIZE;
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
                let t_rec2 = tested_rect.inset(settings.tiles_between_rooms as f32 * TILE_SIZE);
                rect.inset((settings.tiles_between_rooms as f32 * TILE_SIZE) * 2.0)
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
