use bevy::{
    asset::HandleId,
    prelude::*,
    utils::{HashMap, HashSet},
};
use bevy_ecs_ldtk::prelude::{LdtkLevel, LdtkProject};

use bevy_prototype_lyon::{
    prelude::{Fill, FillOptions, GeometryBuilder, Path},
    shapes,
};

use rand::prelude::*;
use voronator::delaunator::{Coord, Vector};

use super::{DungeonGeneratorSettings, GeneratorStage};
use crate::loading::assets::MapAssetHandles;

/// is the whole dungeon
#[derive(Component, Default)]
pub struct DungeonContainerTag;

/// room for dungeon
#[derive(Component, Default)]
pub struct DungeonRoomTag;

/// bundle for easy Dungeon Container
#[derive(Bundle, Default)]
pub struct DungeonContainerBundle {
    /// dungeon asset
    pub ldtk_handle: Handle<LdtkProject>,
    /// tag
    pub tag: DungeonContainerTag,
    /// dungeon location
    pub transform: Transform,
    /// dungeon global location
    pub global_transform: GlobalTransform,
    /// dungeon visibility
    pub visibility: Visibility,
    /// computed visibility
    pub computed_visibility: ComputedVisibility,
}

/// bundle for dungeon rooms
#[derive(Bundle)]
pub struct DungeonRoomBundle {
    /// name of room
    pub name: Name,
    /// room definition
    pub ldtk_level: Handle<LdtkLevel>,
    /// room data for generation
    pub room_info: RoomInstance,
    /// room identifier
    pub tag: DungeonRoomTag,
    /// dungeon origin spot
    pub dv_shape: Path,
    /// dungeon origin fill
    pub dv_color: Fill,
    /// dungeon location relative too container
    pub transform: Transform,
    /// dungeon location relative too world
    pub global_transform: GlobalTransform,
    /// dungeon room visibility
    pub visibility: Visibility,
    /// dungeon computed visibility
    pub computed_visibility: ComputedVisibility,
}
/// ID for `LevelAssets`, used too track what dungeons are available too spawn
#[derive(PartialEq, Component, Eq, Hash, Debug, Clone, Copy, Reflect)]
pub struct RoomAssetID(pub i32);

/// ID for spawned `DungeonRooms`
#[derive(
    PartialEq, Component, Eq, Hash, Debug, Clone, Copy, Reflect, Default,
)]
pub struct RoomID(pub i32);

/// data used for graphs
#[derive(Component, Debug, Reflect, Clone, Default)]
pub struct RoomInstance {
    /// name of the room layout,
    pub room_name: String,
    /// ticks up from 0, room 0 is always a StartRoom<LdtkLevel>,
    pub room_id: RoomID,
    /// asset too fill this data from
    pub room_asset: Handle<LdtkLevel>,
    /// how wide is this room instance
    pub width: i32,
    /// how tall is this room instance
    pub height: i32,
    /// what is this rooms position
    pub position: IVec2,
    /// where are the exit tiles for this room
    pub exits: Vec<Vec2>,
}

impl Vector<Self> for RoomInstance {
    fn vector(p: &Self, q: &Self) -> Self {
        Self::from_xy(q.x() - p.x(), q.y() - p.y())
    }

    fn determinant(p: &Self, q: &Self) -> f64 {
        p.x().mul_add(q.y(), -p.y() * q.x())
    }

    fn dist2(p: &Self, q: &Self) -> f64 {
        let d = Self::vector(p, q);
        d.x().mul_add(d.x(), d.y() * d.y())
    }

    fn equals(p: &Self, q: &Self) -> bool {
        (p.x() - q.x()).abs() <= voronator::delaunator::EPSILON
            && (p.y() - q.y()).abs() <= voronator::delaunator::EPSILON
    }

    fn equals_with_span(p: &Self, q: &Self, span: f64) -> bool {
        let dist = Self::dist2(p, q) / span;
        dist < 1e-20 // dunno about this
    }
}

impl Coord for RoomInstance {
    fn from_xy(x: f64, y: f64) -> Self {
        Self {
            room_id: RoomID(0),
            room_asset: Handle::default(),
            width: 0,
            height: 0,
            position: IVec2 {
                x: x as i32,
                y: y as i32,
            },
            exits: Vec::new(),
            room_name: String::default(),
        }
    }

    fn x(&self) -> f64 {
        self.position.x.into()
    }

    fn y(&self) -> f64 {
        self.position.y.into()
    }
}

impl Default for DungeonRoomBundle {
    fn default() -> Self {
        let spawner_box_visual = shapes::Rectangle {
            extents: Vec2 { x: 5.0, y: 5.0 },
            origin: shapes::RectangleOrigin::Center,
        };
        let spawner_visual_bundle =
            GeometryBuilder::new().add(&spawner_box_visual).build();

        Self {
            name: Name::new("BlankDungeon"),
            ldtk_level: Handle::default(),
            //room info is normal filled out with information
            //from ldtk_level and spots passed in through generator
            room_info: RoomInstance::default(),
            tag: DungeonRoomTag,
            dv_shape: spawner_visual_bundle,
            dv_color: Fill {
                options: FillOptions::default(),
                color: Color::YELLOW,
            },
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            global_transform: GlobalTransform::default(),
            visibility: Visibility::default(),
            computed_visibility: ComputedVisibility::default(),
        }
    }
}

// TODO: it would be useful too have the start room predefined in the dungeons asset
// spec for dungeon maps should be created, Dungeons should always
// have a SmallStartRoom, to make assumptions explainable?
/// spawns initial dungeon container if it doesn't exist
pub fn setup_dungeon_environment(
    mut cmds: Commands,
    _gen_settings: Res<DungeonGeneratorSettings>,
    dungeon_container: Query<Entity, &DungeonContainerTag>,
    dungeons: Res<MapAssetHandles>,
) {
    if dungeon_container.is_empty() {
        info!("No Dungeon Container, Creating Now.....");
        cmds.spawn((
            DungeonContainerBundle {
                tag: DungeonContainerTag,
                ldtk_handle: dungeons.start_level.clone(),
                transform: Transform {
                    scale: Vec3 {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    },
                    translation: Vec3::ZERO,
                    ..default()
                },
                ..default()
            },
            Name::new("DungeonContainer"),
        ));
        return;
    }
    // TODO: this branch of the if statement will probably be expanded for multiple levels,
    // after you complete the first dungeon layout, it'll delete old dungeon and regenerate a new one
    info!("The Dungeon Container Already exists,");
}

/// iterates over Res<Assets<LdtkLevel>>> and adds them too list on dungeon gen settings
pub fn create_dungeons_list(
    mut done: Local<bool>,
    mut cmds: Commands,
    mut gen_settings: ResMut<DungeonGeneratorSettings>,
    level_assets: Res<Assets<LdtkLevel>>,
    asset_server: Res<AssetServer>,
) {
    info!(
        "Creating resource for dungeons too be used in dungeon generation"
    );

    let filtered_assets = filter_levels(&level_assets);
    let mut useable_room_assets: HashMap<RoomAssetID, Handle<LdtkLevel>> =
        HashMap::new();
    let filtered_levels_len = filtered_assets.len() as i32;

    let mut id = 0;
    for level_id in filtered_assets {
        let level_handle = asset_server.get_handle(level_id);
        let level_asset = level_assets.get(&level_handle).unwrap();

        if level_asset.data().identifier == "SmallStartRoom" {
            useable_room_assets.insert(RoomAssetID(0), level_handle);
            info!(
                "Inserted {} into hashmap, {id}/{filtered_levels_len}",
                level_asset.data().identifier
            );
        } else {
            useable_room_assets.insert(RoomAssetID(id), level_handle);
            info!(
                "Inserted {} into hashmap, {id}/{filtered_levels_len}",
                level_asset.data().identifier
            );
        }
        id += 1;
        if id == filtered_levels_len {
            *done = true;
        }
    }

    if *done {
        for (id, ldtk_level) in &useable_room_assets {
            let level_asset = level_assets.get(ldtk_level).unwrap();
            info!(
                "ID: {:?}, Level: {:?}",
                id,
                level_asset.data().identifier
            );
        }
        gen_settings.useable_rooms = Some(useable_room_assets);
        cmds.insert_resource(NextState(Some(
            GeneratorStage::GenerateRooms,
        )));
    }
}

/// takes useable rooms, does a layout using positions
/// and places each one down too be built by ldtk plugin
pub fn layout_dungeon_and_place_skeleton(
    mut cmds: Commands,
    gen_settings: Res<DungeonGeneratorSettings>,
    dungeon_container: Query<(
        Entity,
        &DungeonContainerTag,
        &Handle<LdtkProject>,
    )>,
    level_assets: Res<Assets<LdtkLevel>>,
) {
    let dungeon_container = dungeon_container.single();
    let rng = thread_rng();
    let rooms = &gen_settings.useable_rooms;
    let useable_levels = rooms.as_ref().unwrap();

    let pre_layout =
        build_rooms(rng, useable_levels, &gen_settings, &level_assets);
    let rooms = layout_rooms_on_grid(pre_layout, &gen_settings);

    for room in rooms {
        let name = format!("{} {:?}", &room.room_name, &room.room_asset);
        info!("Placing {:?} : {:?}", &room.room_id, &name);
        let spawner_box_visual = shapes::Rectangle {
            extents: Vec2 { x: 5.0, y: 5.0 },
            origin: shapes::RectangleOrigin::Center,
        };
        let spawner_visual_bundle =
            GeometryBuilder::new().add(&spawner_box_visual).build();

        cmds.entity(dungeon_container.0).with_children(|parent| {
            parent.spawn(DungeonRoomBundle {
                name: Name::new(name),
                ldtk_level: room.room_asset.clone(),
                transform: Transform::from_translation(
                    room.position.as_vec2().extend(0.0),
                ),
                room_info: room.clone(),
                tag: DungeonRoomTag,
                dv_shape: spawner_visual_bundle,
                dv_color: Fill {
                    color: (Color::RED),
                    options: FillOptions::default(),
                },
                ..default()
            });
        });
    }

    // if dungeon spots is empty, pop returns none,
    // dungeon spots should be empty when we have spawned all dungeons
    // we can insert the next state
    info!("Done spawning template");
    cmds.insert_resource(NextState(Some(GeneratorStage::PlaceRooms)));
}

//TODO: have this be set by a ron file next too the level asset.
/// filters level assets based on string
fn filter_levels(level_assets: &Res<Assets<LdtkLevel>>) -> Vec<HandleId> {
    let excluded_levels = vec![
        // identifier for main level, this isn't placed for dungeons
        String::from("Sanctuary"),
    ];

    let mut vec_levels: Vec<HandleId> = Vec::new();
    // let mut filtered_levels: Vec<HandleId> = Vec::new();
    let mut small_start_room: Option<HandleId> = None;

    for (id, level) in level_assets.iter() {
        if excluded_levels.contains(&level.data().identifier) {
            // filtered_levels.push(id);
        } else if level.data().identifier == "SmallStartRoom" {
            small_start_room = Some(id);
        } else {
            vec_levels.push(id);
        }
    }

    if let Some(room) = small_start_room {
        vec_levels.insert(0, room);
    }

    vec_levels
}

/// builds room instances based on room amount and room idx
/// first room is always start room
/// every other room is randomly chosen from `filter_levels` return value
fn build_rooms(
    mut rng: ThreadRng,
    useable_levels: &HashMap<RoomAssetID, Handle<LdtkLevel>>,
    gen_settings: &Res<'_, DungeonGeneratorSettings>,
    level_assets: &Res<Assets<LdtkLevel>>,
) -> Vec<RoomInstance> {
    let room_amount = gen_settings.dungeon_room_amount;
    let mut rooms = Vec::with_capacity(room_amount.try_into().unwrap());

    for room in 0..room_amount {
        if room == 0 {
            let room_handle =
                useable_levels.get(&RoomAssetID(0)).expect("msg");
            let room_asset: &LdtkLevel =
                level_assets.get(room_handle).unwrap();
            info!(
                "inserting room # {} as {}",
                room_asset.data().identifier,
                room
            );
            rooms.push(RoomInstance {
                room_name: room_asset.data().identifier.clone(),
                room_id: RoomID(room),
                width: room_asset.data().px_wid,
                height: room_asset.data().px_hei,
                room_asset: room_handle.clone(),
                position: IVec2::ZERO,
                exits: Vec::new(),
            });
        } else {
            let id_to_get: i32 =
                rng.gen_range(1..(useable_levels.len() as i32 - 1));
            let room_handle =
                useable_levels.get(&RoomAssetID(id_to_get)).expect("msg");
            let room_asset = level_assets.get(room_handle).unwrap();
            info!(
                "inserting room # {} as {}",
                room_asset.data().identifier,
                room
            );
            rooms.push(RoomInstance {
                room_name: room_asset.data().identifier.clone(),
                room_id: RoomID(room),
                width: room_asset.data().px_wid,
                height: room_asset.data().px_hei,
                room_asset: room_handle.clone(),
                position: IVec2::ZERO,
                exits: Vec::new(),
            });
        }
    }
    rooms
}

/// lays out `RoomInstance` onto grid derived from room amount and room size
fn layout_rooms_on_grid(
    mut rooms: Vec<RoomInstance>,
    settings: &Res<'_, DungeonGeneratorSettings>,
) -> Vec<RoomInstance> {
    let mut grid_spots: Vec<(usize, usize)> = Vec::new();
    let room_amount = rooms.len();
    let grid_size = (room_amount as f32 * settings.grid_too_room_ratio)
        .sqrt()
        .ceil() as usize;
    let mut placed_rooms: Vec<RoomInstance> = Vec::new();

    // Sort rooms by decreasing area
    rooms.sort_by(|a, b| {
        let area_a = a.width * a.height;
        let area_b = b.width * b.height;
        area_b.partial_cmp(&area_a).unwrap()
    });

    let largest_room = rooms.first().unwrap();
    let cell_width = (largest_room.width
        + settings.dungeons_space_between as i32)
        as f32;
    let cell_height = (largest_room.height
        + settings.dungeons_space_between as i32)
        as f32;

    // Generate grid spots
    for row in 0..grid_size {
        for col in 0..grid_size {
            grid_spots.push((row, col));
        }
    }

    // Randomly select the required number of grid spots
    let mut rng = rand::thread_rng();
    let num_rooms = room_amount.min(grid_spots.len());
    let selected_spots: Vec<(usize, usize)> =
        grid_spots.into_iter().choose_multiple(&mut rng, num_rooms);
    // .collect();

    // Mark occupied grid spots
    let mut occupied_spots: HashSet<(usize, usize)> = HashSet::new();

    // Calculate the center of the grid
    let grid_center_x = (grid_size as f32 / 2.0) - 0.5;
    let grid_center_y = (grid_size as f32 / 2.0) - 0.5;

    // Iterate over the selected grid spots and place rooms in each spot
    for (row, col) in selected_spots {
        if let Some(mut room) = rooms.pop() {
            // Calculate the position of the room within the current grid cell
            // let x = (col as f32 + 0.5) * cell_width;
            // let y = (row as f32 + 0.5) * cell_height;
            let x = (col as f32 - grid_center_x) * cell_width;
            let y = (row as f32 - grid_center_y) * cell_height;

            // Adjust the position based on the room size and wall distance
            let adjusted_x = x
                - (room.width as f32 / 2.0)
                - settings.dungeons_space_between;
            let adjusted_y = y
                - (room.height as f32 / 2.0)
                - settings.dungeons_space_between;

            // Check if the grid spot is already occupied
            let is_occupied = occupied_spots.contains(&(row, col));

            // If the spot is occupied, find the nearest available spot
            let mut nearest_spot: Option<(usize, usize)> = None;
            if is_occupied {
                for &(row_offset, col_offset) in
                    &[(0, 1), (0, -1), (1, 0), (-1, 0)]
                {
                    let new_row = row as isize + row_offset;
                    let new_col = col as isize + col_offset;
                    if new_row >= 0
                        && new_row < grid_size as isize
                        && new_col >= 0
                        && new_col < grid_size as isize
                    {
                        let new_spot =
                            (new_row as usize, new_col as usize);
                        if !occupied_spots.contains(&new_spot) {
                            nearest_spot = Some(new_spot);
                            break;
                        }
                    }
                }
            }

            // If a nearest spot is found, update the position
            if let Some((new_row, new_col)) = nearest_spot {
                let new_x = (new_col as f32 + 0.5) * cell_width;
                let new_y = (new_row as f32 + 0.5) * cell_height;
                let new_adjusted_x = new_x
                    - (room.width as f32 / 2.0)
                    - settings.dungeons_space_between;
                let new_adjusted_y = new_y
                    - (room.height as f32 / 2.0)
                    - settings.dungeons_space_between;
                room.position = IVec2::new(
                    new_adjusted_x as i32,
                    new_adjusted_y as i32,
                );
                placed_rooms.push(room);
                occupied_spots.insert((new_row, new_col));
            } else {
                // Place the room at the adjusted position
                room.position =
                    IVec2::new(adjusted_x as i32, adjusted_y as i32);
                placed_rooms.push(room);
                occupied_spots.insert((row, col));
            }
        }
    }

    placed_rooms
}
