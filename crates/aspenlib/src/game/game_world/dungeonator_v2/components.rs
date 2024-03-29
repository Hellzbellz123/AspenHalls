use bevy::{
    ecs::{
        bundle::Bundle,
        reflect::{ReflectComponent, ReflectResource},
        system::Resource,
    },
    log::warn,
    math::{IVec2, Vec2},
    prelude::{Component, Entity, Handle, Name, SpatialBundle},
    reflect::Reflect,
};
use bevy_ecs_ldtk::{
    prelude::{ldtk::FieldInstance, FieldValue, LdtkProject},
    LevelIid,
};

use crate::game::game_world::dungeonator_v2::hallways::PlacedHallWay;

/// bundle for easy spawning of dungeon
/// always 1 per dungeon, all dungeon rooms are children
#[derive(Bundle)]
pub struct DungeonContainerBundle {
    /// identifies dungeon root entity
    pub tag: DungeonContainerTag,
    /// identified dungeon root in hierarchy
    pub name: Name,
    /// configures spawning of child rooms and hallways
    pub settings: DungeonSettings,
    /// data used too spawn with
    pub ldtk_project: Handle<LdtkProject>,
    /// gives dungeons a position
    pub spatial: SpatialBundle,
}

/// placeable room preset
#[derive(Bundle, Debug)]
pub struct DungeonRoomBundle {
    /// identifies dungeon rooms
    pub tag: DungeonRoomTag,
    /// basically just `LevelIid`
    pub name: Name,
    /// id from `LdtkProject`
    pub id: LevelIid,
    /// spatial data
    pub spatial: SpatialBundle,
}

/// bundle for easy spawning of Dungeon Hallways
#[derive(Bundle)]
pub struct DungeonHallWayBundle {
    /// identifies dungeon hallways
    pub tag: DungeonHallwayTag,
    /// Hallway# from-to
    pub name: Name,
    /// spatial data
    pub spatial: SpatialBundle,
}

/// database generated from ldtk level assets on startup or when assets are changed,
/// splits levels based on level attributes
#[allow(unused)]
#[derive(Debug, Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct DungeonRoomDatabase {
    /// list of hideout room presets
    pub hideouts: Vec<RoomPreset>,
    /// list of dungeon start room presets
    pub start_rooms: Vec<RoomPreset>,
    /// list of dungeon end room presets
    pub end_rooms: Vec<RoomPreset>,
    /// list of special room presets
    pub special_rooms: Vec<RoomPreset>,
    /// list of 32 tile x 32 tile room presets
    pub small_short_rooms: Vec<RoomPreset>,
    /// list of 32 tile x 64 tile room presets
    pub small_long_rooms: Vec<RoomPreset>,
    /// list if 64 tile x 64 tile room presets
    pub medium_short_rooms: Vec<RoomPreset>,
    /// list of 64 tile x 128 tile room presets
    pub medium_long_rooms: Vec<RoomPreset>,
    /// list of 128 tile x 128 tile room presets
    pub large_short_rooms: Vec<RoomPreset>,
    /// list of 128 tile x 256 tile room presets
    pub large_long_rooms: Vec<RoomPreset>,
    /// list of 256 tile x 256 tile room presets
    pub huge_short_rooms: Vec<RoomPreset>,
    /// list of 256 tile x 512 tile room presets
    pub huge_long_rooms: Vec<RoomPreset>,
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
    /// postion checked rooms
    pub positioned_rooms: Vec<PlacedRoom>,
    /// position checked hallways
    pub positioned_hallways: Vec<PlacedHallWay>,
    /// amount of rooms inside dungeon
    pub room_amount: RoomAmounts,
    /// percentage of paths between
    /// rooms that are chosen to loop
    pub loops_percentage: f32,
}

/// tag too identify dungeons
#[derive(Component)]
pub struct DungeonContainerTag;

/// tag too identify dungeon hallways
#[derive(Component)]
pub struct DungeonHallwayTag;

/// tag too identify dungeon rooms
#[derive(Component, Debug, Clone)]
pub struct DungeonRoomTag;

/// room instances before being placed
#[derive(Debug, Clone, Reflect)]
pub struct RoomPreset {
    /// information describing the room
    pub descriptor: RoomDescriptor,
    /// asset id for level data
    pub room_asset_id: LevelIid,
    /// name of the room
    pub name: String,
    /// - Rect.max is position + size
    /// size of room
    pub size: Vec2,
}

impl PlacedRoom {
    pub fn from_preset(preset: &RoomPreset, position: Vec2, id: u32) -> PlacedRoom {
        PlacedRoom {
            descriptor: preset.descriptor.clone(),
            asset_id: preset.room_asset_id.clone(),
            position: position.as_ivec2(),
            name: preset.name.clone(),
            size: preset.size,
            id: RoomID(id),
            exits: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, Reflect, Component, Default, PartialEq, PartialOrd, Ord, Eq)]
pub struct RoomID(u32);

#[derive(Debug, Clone, Reflect, Component, Default, PartialEq)]
#[reflect(Component)]
pub struct PlacedRoom {
    pub descriptor: RoomDescriptor,
    pub asset_id: LevelIid,
    pub position: IVec2,
    pub exits: Vec<Entity>,
    pub name: String,
    pub size: Vec2,
    pub id: RoomID,
}

#[derive(Debug, Clone, Reflect, Default, PartialEq, Eq)]
pub struct HallwayNode {
    pub parent: RoomID,
    pub used: bool,
    pub position: IVec2,
}

#[derive(Debug, Clone, Reflect, Default, Eq, PartialOrd, Ord, PartialEq)]
pub struct RoomDescriptor {
    /// room shape as enum
    pub shape: RoomShape,
    /// rooms level
    pub level: RoomLevel,
    /// what function does this room serve for the dungeon
    pub rtype: RoomType,
}

/// amounts of each room that should be spawned
#[derive(Debug, Clone, Default, Reflect)]
pub struct RoomAmounts {
    /// max amount of this room too spawn
    pub small_short: i32,
    /// max amount of this room too spawn
    pub small_long: i32,
    /// max amount of this room too spawn
    pub medium_short: i32,
    /// max amount of this room too spawn
    pub medium_long: i32,
    /// max amount of this room too spawn
    pub large_short: i32,
    /// max amount of this room too spawn
    pub large_long: i32,
    /// max amount of this room too spawn
    pub huge_short: i32,
    /// max amount of this room too spawn
    pub huge_long: i32,
    /// max amount of this room too spawn
    pub special: i32,
}

/// what level is this room
#[derive(Debug, Clone, Reflect, PartialEq, Eq, PartialOrd, Default, Ord)]
pub enum RoomLevel {
    /// DEBUG LEVEL
    Level0,
    /// default level for rooms
    #[default]
    Level1,
    /// first upgrade too rooms
    Level2,
    /// second upgrade too rooms
    Level3,
}

/// what function does this room serve in the dungeon
#[derive(Debug, Clone, Reflect, PartialEq, Eq, PartialOrd, Default, Ord)]
pub enum RoomType {
    /// room player is moved too when dungeon generation finishes
    DungeonStart,
    // TODO: when killed make portal/something too trigger next zone
    /// final room in dungeon. 1 HARD enemy.
    DungeonEnd,
    /// room has special functions in dungeon
    Special,
    /// normal dungeon rooms, not leveled
    #[default]
    Normal,
    /// select hero and prepare for the coming dungeon run
    Hideout,
}

/// what size/shape is this room
#[derive(Debug, Clone, Reflect, PartialEq, Eq, PartialOrd, Default, Ord)]
pub enum RoomShape {
    /// shape doesnt fit below definitions.
    NonStandard,
    /// 32 tile x 32 tile
    #[default]
    SmallShort,
    /// 32 tile x 64 tile
    SmallLong,
    /// 64 tile x 64 tile
    MediumShort,
    /// 64 tile x 128 tile
    MediumLong,
    /// 128 tile x 128 tile
    LargeShort,
    /// 128 tile x 256 tile
    LargeLong,
    /// 256 tile x 256 tile
    HugeShort,
    /// 256 tile x 512 tile
    HugeLong,
}
///  returns `Some(RoomShape)` if field exists in `field_instances` else `None`
pub fn try_get_roomshape(field_instances: &[FieldInstance]) -> Option<RoomShape> {
    let room_ident = field_instances
        .iter()
        .find(|f| f.identifier == "IdentSize")?;
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
        "Special" => Some(RoomShape::NonStandard),
        _ => None,
    }
}

///  returns `Some(RoomLevel)` if field exists in `field_instances` else `None`
pub fn try_get_roomlevel(field_instances: &[FieldInstance]) -> Option<RoomLevel> {
    let room_ident = field_instances
        .iter()
        .find(|f| f.identifier == "IdentLevel")?;
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

/// returns `Some(RoomType)` if field exists in in `field_instances` else `None`
pub fn try_get_roomtype(field_instances: &[FieldInstance]) -> Option<RoomType> {
    let room_ident = field_instances
        .iter()
        .find(|f| f.identifier == "IdentType")?;
    let FieldValue::Enum(Some(enum_value)) = &room_ident.value else {
        return None;
    };

    match enum_value.as_str() {
        "DungeonStart" => Some(RoomType::DungeonStart),
        "Boss" => Some(RoomType::DungeonEnd),
        "Special" => Some(RoomType::Special),
        "Normal" => Some(RoomType::Normal),
        "Hideout" => Some(RoomType::Hideout),
        e => {
            warn!("unknown room type {:?}", e);
            None
        }
    }
}
