use bevy::{
    asset::Assets,
    core::Name,
    ecs::entity::Entity,
    log::{info, warn},
    math::{Rect, Vec2},
    prelude::{BuildChildren, Commands, Handle, NextState, Query, Res, SpatialBundle, Transform},
};
use bevy_ecs_ldtk::{assets::LdtkExternalLevel, prelude::LdtkProject};

use crate::{
    consts::TILE_SIZE,
    game::game_world::dungeonator_v2::{
        components::{
            try_get_roomlevel, try_get_roomshape, try_get_roomtype, Dungeon, DungeonRoomBundle,
            DungeonRoomDatabase, RoomBlueprint, RoomDescriptor, RoomLevel, RoomPreset, RoomShape,
            RoomType,
        },
        utils::{choose_filler_presets, get_leveled_preset, random_room_positon},
        GeneratorState,
    },
    loading::assets::AspenMapHandles,
};

/// maps `level_assets` too a `DungeonRoomDatabase`
/// dungeons are filtered into vecs based on level custom data
pub fn generate_room_database(
    mut cmds: Commands,
    map_projects: Res<AspenMapHandles>,
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
            // check if room assets are right size,
            // if they arent the correct size we get annoying panics elsewhere
            if ["DungeonStartL1", "TestingHalls"].iter().all(|f| f != level_def.identifier())
                && ((level_def.px_wid() / TILE_SIZE as i32) % 2 != 0
                    || (level_def.px_hei() / TILE_SIZE as i32) % 2 != 0)
            {
                panic!("Dungeon filler room MUST be even number of tiles in size for x AND y: {}", level_def.identifier());
            }
            if ["DungeonStartL1"].iter().all(|f| f == level_def.identifier())
                && ((level_def.px_wid() / TILE_SIZE as i32) / 2 == 0
                    || (level_def.px_hei() / TILE_SIZE as i32) / 2 == 0)
            {
                panic!("ONLY Dungeon Start room MUST be odd number of tiles in size for x AND y: {}", level_def.identifier());
            }

            let field_instances = level_def.field_instances();
            let room_shape = try_get_roomshape(field_instances)
                .expect("No size ident on room definiton. Check Ldtk Editor for errors");
            let room_type = try_get_roomtype(field_instances)
                .expect("No type ident on room definition. Check Ldtk Editor for errors");
            let room_level = try_get_roomlevel(field_instances)
                .expect("No level ident on room definition. Check Ldtk Editor for errors");

            let room = RoomPreset {
                name: level_def.identifier().to_string(),
                room_asset_id: level_def.iid().clone().into(),
                size: Vec2::new(*level_def.px_wid() as f32, *level_def.px_hei() as f32),
                descriptor: RoomDescriptor {
                    shape: room_shape.clone(),
                    level: room_level,
                    rtype: room_type.clone(),
                },
            };

            match &room_type {
                RoomType::Hideout => dungeon_database.hideouts.push(room),
                RoomType::DungeonStart => dungeon_database.start_rooms.push(room),
                RoomType::DungeonEnd => dungeon_database.end_rooms.push(room),
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
pub fn select_presets(
    mut cmds: Commands,
    room_database: Res<DungeonRoomDatabase>,
    mut dungeon_root: Query<&mut Dungeon>,
) {
    let mut dungeon = dungeon_root.single_mut();

    let progress_level = RoomLevel::Level1;
    let mut presets = choose_filler_presets(&dungeon.settings, &room_database);
    let mut room_positions: Vec<Rect> = Vec::new();

    // add start and end room
    presets.push(get_leveled_preset(&room_database.end_rooms, &progress_level).unwrap());

    // start room position is manually calculated and added too existing rooms
    // so its not re placed when mapping presets too positioned rooms
    let start = get_leveled_preset(&room_database.start_rooms, &progress_level).unwrap();
    let start_rect = Rect::from_center_size(Vec2::ZERO, start.size);
    room_positions.push(start_rect);

    // create PlacedRoom's from room presets
    let mut placed_rooms: Vec<RoomBlueprint> = Vec::new();
    for (i, f) in presets.iter().enumerate() {
        let pos = random_room_positon(&room_positions, f.size, &dungeon.settings);
        room_positions.push(pos);
        placed_rooms.push(RoomBlueprint::from_preset(f, pos.min, i as u32 + 1));
    }
    placed_rooms.push(RoomBlueprint::from_preset(&start, start_rect.min, 0));

    // warn!("rooms passed too room-placer: {:?}", placed_rooms);
    dungeon.rooms = placed_rooms;

    info!("finished picking and positioning room presets");
    cmds.insert_resource(NextState(Some(GeneratorState::FinalizeRooms)));
}

/// spawns possible dungeon rooms from `dungeon_root.dungeon_settings.useable_dungeons`
pub fn spawn_presets(
    mut cmds: Commands,
    dungeon_root: Query<(Entity, &Dungeon, &Handle<LdtkProject>)>,
) {
    let (dungeon_root, dungeon_settings, _proj_handle) = dungeon_root.single();
    if dungeon_settings.rooms.is_empty() {
        warn!("no dungeon presets were prepared");
        return;
    }

    cmds.entity(dungeon_root).with_children(|rooms| {
        for room in &dungeon_settings.rooms {
            let name = format!("DungeonRoom-{}", room.name);
            rooms.spawn((DungeonRoomBundle {
                room: room.clone(),
                name: Name::new(name),
                id: room.asset_id.clone(),
                spatial: SpatialBundle::from_transform(Transform::from_xyz(
                    room.position.x as f32,
                    room.position.y as f32,
                    0.0,
                )),
            },));
        }
    });
}
