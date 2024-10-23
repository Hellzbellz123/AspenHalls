use bevy::{
    asset::Assets,
    prelude::{Commands, IVec2, Res},
};
use bevy_ecs_ldtk::{assets::LdtkExternalLevel, prelude::LdtkProject};

use crate::{
    consts::TILE_SIZE,
    game::game_world::dungeonator_v2::components::{
        try_get_roomlevel, try_get_roomshape, try_get_roomtype, DungeonRoomDatabase,
        RoomDescriptor, RoomPreset, RoomShape, RoomType,
    },
    loading::assets::AspenMapHandles,
};

/// maps `level_assets` too a `DungeonRoomDatabase`
/// dungeons are filtered into vecs based on level custom data
pub fn build_room_presets(
    mut cmds: Commands,
    map_projects: Res<AspenMapHandles>,
    ldtk_projects: Res<Assets<LdtkProject>>,
    ldtk_levels: Res<Assets<LdtkExternalLevel>>,
) {
    #[cfg(feature = "bevy/file_watcher")]
    if ldtk_projects.is_changed() || ldtk_levels.is_changed() {
        info!("dungeon assets changed");
    }

    let dungeon_project = ldtk_projects
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
        .iter_external_levels(&ldtk_levels)
        .for_each(|level_def| {
            // TODO: if positioning is fixed this is uneeded?
            validate_room_size(level_def).expect("all rooms should be correct size");

            let field_instances = level_def.field_instances();
            let room_shape = try_get_roomshape(field_instances)
                .expect("No size ident on room definiton. Check Ldtk Editor for errors");
            let room_type = try_get_roomtype(field_instances)
                .expect("No type ident on room definition. Check Ldtk Editor for errors");
            let room_level = try_get_roomlevel(field_instances)
                .expect("No level ident on room definition. Check Ldtk Editor for errors");

            let tile_types = &dungeon_project
                .json_data()
                .defs
                .enums
                .iter()
                .find(|f| f.identifier == "CollisionType")
                .expect("CollisionType enum must exist");

            let _exit_definition = tile_types
                .values
                .iter()
                .find(|f| f.id == "RoomExit")
                .expect("RoomExit type must exist");

            let building_layer = &level_def
                .layer_instances()
                .iter()
                .find(|f| f.identifier == "Building_Layer")
                .expect("no entity layer on this level");

            let _layer_width = building_layer.c_wid * TILE_SIZE as i32;
            let layer_height = building_layer.c_hei * TILE_SIZE as i32;

            let building_tiles = &building_layer.grid_tiles;

            let exit_tile_positions: Vec<IVec2> = // Vec::new();
            building_tiles.iter().enumerate()
                .filter(|(_idx, tile)| tile.t == 2 )
                .map(|(_idx, tile)| {

                //TODO: this value need too be calculated with a bottom origin
                // ldtk gives offset with top origin
                let tile_px_y = layer_height - tile.px.y - 32;
                let tile_px_x = tile.px.x;

                IVec2 {
                        x: tile_px_x,
                        y: tile_px_y,
                    }
                })
                .collect();

            let room = RoomPreset {
                name: level_def.identifier().to_string(),
                room_asset_id: level_def.iid().clone().into(),
                size: IVec2::new(*level_def.px_wid(), *level_def.px_hei()),
                exits: exit_tile_positions,
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
                RoomType::Normal | RoomType::Special | RoomType::MiniBoss => match room_shape {
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

fn validate_room_size(
    level_def: bevy_ecs_ldtk::prelude::ldtk::loaded_level::LoadedLevel<'_>,
) -> Result<(), String> {
    // check if room assets are right size,
    // if they arent the correct size we get annoying panics elsewhere
    if ["DungeonStartL1", "TestingHalls"]
        .iter()
        .all(|f| f != level_def.identifier())
        && ((level_def.px_wid() / TILE_SIZE as i32) % 2 != 0
            || (level_def.px_hei() / TILE_SIZE as i32) % 2 != 0)
    {
        let msg = format!(
            "Dungeon filler room MUST be even number of tiles in size for x AND y: {}",
            level_def.identifier()
        );
        return Err(msg);
    }

    if ["DungeonStartL1"]
        .iter()
        .all(|f| f == level_def.identifier())
        && ((level_def.px_wid() / TILE_SIZE as i32) / 2 == 0
            || (level_def.px_hei() / TILE_SIZE as i32) / 2 == 0)
    {
        let msg = format!(
            "ONLY Dungeon Start room MUST be odd number of tiles in size for x AND y: {}",
            level_def.identifier()
        );
        return Err(msg);
    }
    Ok(())
}
