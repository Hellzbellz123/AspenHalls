use bevy::{asset::HandleId, prelude::*, utils::HashSet};
use bevy_ecs_ldtk::{
    app::{LdtkEntityMap, LdtkIntCellMap},
    ldtk::{Level, LevelBackgroundPosition},
    level::spawn_level,
    prelude::TilesetDefinition,
    utils::{create_entity_definition_map, create_layer_definition_map},
    LdtkAsset, LdtkLevel, LdtkSettings, LevelEvent, Respawn, Worldly,
};
use bevy_prototype_lyon::{
    prelude::{Fill, FillOptions, GeometryBuilder, Path},
    shapes,
};

use rand::prelude::*;
use std::{cmp, collections::HashMap};

use super::{DungeonGeneratorSettings, GeneratorStage};
use crate::loading::assets::MapAssetHandles;

#[derive(Component, Default)]
pub struct DungeonContainerTag;

#[derive(Component, Default)]
pub struct DungeonRoomTag;

#[derive(Bundle, Default)]
pub struct DungeonContainerBundle {
    pub ldtk_handle: Handle<LdtkAsset>,
    pub tag: DungeonContainerTag,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}

#[derive(Bundle)]
pub struct DungeonRoomBundle {
    pub name: Name,
    pub ldtk_level: LdtkLevel,
    pub room_info: RoomInstance,
    pub tag: DungeonRoomTag,

    pub dv_shape: Path,
    pub dv_color: Fill,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct RoomID(i32);

#[derive(Component, Debug, Reflect, Clone)]
pub struct RoomInstance {
    room_id: i32,
    room_asset: LdtkLevel,
    width: i32,
    height: i32,
    position: Vec3,
}

impl Default for RoomInstance {
    fn default() -> Self {
        Self {
            room_id: Default::default(),
            room_asset: LdtkLevel {
                level: Level::default(),
                background_image: None,
            },
            width: Default::default(),
            height: Default::default(),
            position: Default::default(),
        }
    }
}

impl<'a> Default for DungeonRoomBundle {
    fn default() -> Self {
        let spawner_box_visual = shapes::Rectangle {
            extents: Vec2 { x: 5.0, y: 5.0 },
            origin: shapes::RectangleOrigin::Center,
        };
        let spawner_visual_bundle = GeometryBuilder::new().add(&spawner_box_visual).build();

        Self {
            name: Name::new("BlankDungeon"),
            ldtk_level: LdtkLevel {
                level: Level::default(),
                background_image: None,
            },
            //room info is normall filled out with information
            //from ldtk_level and spots passed in through generator
            room_info: RoomInstance::default(),
            tag: Default::default(),
            dv_shape: spawner_visual_bundle,
            dv_color: Fill {
                options: FillOptions::default(),
                color: Color::YELLOW,
            },
            transform: Default::default(),
            global_transform: Default::default(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
        }
    }
}

// TODO: pre place a "start room" for the dungeon at 0,0 and teleport the player too it
// it would be useful too have the startroom predefined in the dungeons asset
// spec for dungeon maps should be created,
// Dungeons should always have a SmallStartRoom, to make assumptions explainable?
pub fn setup_dungeon_environment(
    mut cmds: Commands,
    gen_settings: Res<DungeonGeneratorSettings>,
    dungeon_container: Query<Entity, &DungeonContainerTag>,
    dungeons: Res<MapAssetHandles>,
) {
    if dungeon_container.is_empty() {
        info!("No Dungeon Container, Creating Now.....");
        cmds.spawn((
            DungeonContainerBundle {
                tag: DungeonContainerTag,
                ldtk_handle: dungeons.homeworld.clone(),
                transform: Transform {
                    scale: Vec3 {
                        x: 3.0,
                        y: 3.0,
                        z: 1.0,
                    },
                    translation: gen_settings.dungeon_map_origin,
                    ..default()
                },
                ..default()
            },
            Name::new("DungeonContainer"),
        ));
        cmds.insert_resource(NextState(Some(GeneratorStage::PlaceRooms)));
        return;
    }
    // TODO: this branch of the if statement will probably be expanded for multiple levels,
    // after you complete the first dungeon layout, itll delete old dungeon and regen a new one
    info!("The Dungeon Container Already exists,");
    cmds.insert_resource(NextState(Some(GeneratorStage::PlaceRooms)));
}

pub fn create_dungeons_list(
    mut cmds: Commands,
    mut gen_settings: ResMut<DungeonGeneratorSettings>,
    level_assets: Res<Assets<LdtkLevel>>,
) {
    info!("Creating resource of dungeons too be used in dungeon generation");
    let mut done = false;
    let filtered_assets = filter_levels(&level_assets);
    let mut useable_rooms: HashMap<RoomID, LdtkLevel> = HashMap::new();
    let filtered_levels_len = filtered_assets.len() as i32;

    let mut id = 0;
    for level_asset in filtered_assets {
        if level_asset.level.identifier == "SmallStartRoom" {
            useable_rooms.insert(RoomID(0), level_asset.clone());
            info!(
                "Inserted {} into hashmap, {id}/{filtered_levels_len}",
                level_asset.level.identifier
            );
            id += 1;
        } else {
            useable_rooms.insert(RoomID(id), level_asset.clone());
            info!(
                "Inserted {} into hashmap, {id}/{filtered_levels_len}",
                level_asset.level.identifier
            );
            id += 1;
        }
        if id == filtered_levels_len {
            done = true;
        }
    }

    if done == true {
        for (id, ldtk_level) in &useable_rooms {
            info!("ID: {:?}, Level: {}", id, ldtk_level.level.identifier);
        }
        gen_settings.useable_rooms = Some(useable_rooms);
        cmds.insert_resource(NextState(Some(GeneratorStage::PlaceRooms)))
    }
}

pub fn layout_dungeon_and_place_skeleton(
    mut cmds: Commands,
    gen_settings: Res<DungeonGeneratorSettings>,
    dungeon_container: Query<(Entity, &DungeonContainerTag, &Handle<LdtkAsset>)>,
) {
    let dungeon_container = dungeon_container.single();
    let rng = thread_rng();
    let rooms = gen_settings.useable_rooms.clone();
    let useable_levels = rooms.as_ref().unwrap();

    let rooms = build_rooms(rng, useable_levels, &gen_settings);
    let rooms = layout_rooms(rooms, &gen_settings);

    for room in rooms {
        let name = format!("{}", room.room_asset.level.identifier);
        info!("Placing RoomID({}): {:?}", room.room_id, name);
        let spawner_box_visual = shapes::Rectangle {
            extents: Vec2 { x: 5.0, y: 5.0 },
            origin: shapes::RectangleOrigin::Center,
        };
        let spawner_visual_bundle = GeometryBuilder::new().add(&spawner_box_visual).build();

        cmds.entity(dungeon_container.0).with_children(|parent| {
            parent.spawn(DungeonRoomBundle {
                name: Name::new(name),
                ldtk_level: room.room_asset.clone(),
                transform: Transform::from_translation(room.position),
                room_info: room,
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
    cmds.insert_resource(NextState(Some(GeneratorStage::BuildDungeonRooms)));
    return;
}

/// Performs all the spawning of levels, layers, chunks, bundles, entities, tiles, etc.
/// when an LdtkLevelBundle is added.
///
///  stolen from bevy_ldtk, finds placed dungeon skeletons in the world and builds them
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn build_dungeons(
    //added explicity childless query for no exit, not sure if its actually faster/better than
    //the matches! childless fn below
    child_less_levels: Query<
        (Entity, &LdtkLevel, &Parent, Option<&Respawn>),
        (With<DungeonRoomTag>, Without<Children>),
    >,
    // changed level query to only grab my levels,
    // im pretty sure this is not necessary as build dungeons should just work if i use Handle<LdtkLevel>
    // but this way i feel like i can have more control later on (might change too using ldtks spawning fns)
    // wouldnt need too vendor anymore
    level_query: Query<
        (
            Entity,
            &LdtkLevel,
            &Parent,
            Option<&Respawn>,
            Option<&Children>,
        ),
        With<DungeonRoomTag>,
    >,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    ldtk_assets: Res<Assets<LdtkAsset>>,
    ldtk_entity_map: NonSend<LdtkEntityMap>,
    ldtk_int_cell_map: NonSend<LdtkIntCellMap>,
    ldtk_query: Query<&Handle<LdtkAsset>>,
    worldly_query: Query<&Worldly>,
    mut level_events: EventWriter<LevelEvent>,
    ldtk_settings: Res<LdtkSettings>,
) {
    if level_query.is_empty() {
        warn!("no levels too work with");
        return;
    }

    // we only need to run this loop if the levels arent built, layers
    // get added to levels as children so if the level has no children we know it needs to be built still
    if child_less_levels.is_empty() {
        commands.insert_resource(NextState(Some(GeneratorStage::Finished)));
        return;
    }

    warn!("Spawning tiles and Layers associated with spawned dungeons");
    for (ldtk_entity, level_handle, parent, respawn, children) in level_query.iter() {
        // Checking if the level has any children is an okay method of checking whether it has
        // already been processed.
        // Users will most likely not be adding children to the level entity betwen its creation
        // and its processing.
        //
        // Furthermore, there are no circumstances where an already-processed level entity needs to
        // be processed again.
        // In the case of respawning levels, the level entity will have its descendants *despawned*
        // first, by a separate system.
        // let already_processed = false;
        let already_processed = matches!(children, Some(children) if !children.is_empty());

        if !already_processed {
            if let Ok(ldtk_handle) = ldtk_query.get(parent.get()) {
                if let Some(ldtk_asset) = ldtk_assets.get(ldtk_handle) {
                    // Commence the spawning
                    let tileset_definition_map: HashMap<i32, &TilesetDefinition> = ldtk_asset
                        .project
                        .defs
                        .tilesets
                        .iter()
                        .map(|t| (t.uid, t))
                        .collect();

                    let entity_definition_map =
                        create_entity_definition_map(&ldtk_asset.project.defs.entities);

                    let layer_definition_map =
                        create_layer_definition_map(&ldtk_asset.project.defs.layers);

                    let worldly_set = worldly_query.iter().cloned().collect();

                    spawn_level(
                        level_handle,
                        &mut commands,
                        &asset_server,
                        &mut images,
                        &mut texture_atlases,
                        &ldtk_entity_map,
                        &ldtk_int_cell_map,
                        &entity_definition_map,
                        &layer_definition_map,
                        &ldtk_asset.tileset_map,
                        &tileset_definition_map,
                        &None,
                        worldly_set,
                        ldtk_entity,
                        &ldtk_settings,
                    );
                    level_events.send(LevelEvent::Spawned(level_handle.level.iid.clone()));

                    if respawn.is_some() {
                        commands.entity(ldtk_entity).remove::<Respawn>();
                    }
                }
            }
        }
    }
}

fn filter_levels(level_assets: &Res<Assets<LdtkLevel>>) -> Vec<LdtkLevel> {
    let excluded_levels = vec![
        // identifer for main level, this isnt placed for dungeons
        String::from("Sanctuary"),
    ];

    let mut vec_levels: Vec<LdtkLevel> = Vec::new();
    let mut filtered_levels: Vec<LdtkLevel> = Vec::new();
    let mut small_start_room: Option<LdtkLevel> = None;

    for (_, level) in level_assets.iter() {
        if excluded_levels.contains(&level.level.identifier) {
            filtered_levels.push(level.clone());
        } else if level.level.identifier == "SmallStartRoom" {
            small_start_room = Some(level.clone());
        } else {
            vec_levels.push(level.clone());
        }
    }

    if let Some(level) = small_start_room {
        vec_levels.insert(0, level);
    }

    let identifiers: Vec<String> = vec_levels
        .iter()
        .map(|level| level.level.identifier.clone())
        .collect();
    info!("levels useable as dungeon rooms \n {:#?}", identifiers);
    vec_levels
}

fn build_rooms(
    mut rng: ThreadRng,
    useable_levels: &HashMap<RoomID, LdtkLevel>,
    gen_settings: &Res<'_, DungeonGeneratorSettings>,
) -> Vec<RoomInstance> {
    let room_amount = gen_settings.dungeon_room_amount;
    let mut rooms = Vec::with_capacity(room_amount.try_into().unwrap());

    for room in 0..room_amount {
        let id_to_get: i32 = rng.gen_range(1..(useable_levels.len() as i32 - 1));

        if room == 0 {
            let start_room = useable_levels.get(&RoomID(0)).unwrap();
            info!(
                "inserting {} as RoomID({})",
                room, start_room.level.identifier
            );
            rooms.push(RoomInstance {
                room_id: room,
                width: start_room.level.px_wid.clone(),
                height: start_room.level.px_hei.clone(),
                room_asset: start_room.clone(),
                position: Vec3::ZERO,
            });
        } else {
            info!(id_to_get);
            let level = useable_levels.get(&RoomID(id_to_get)).expect("msg");
            info!("inserting {} as RoomID({})", room, level.level.identifier);
            rooms.push(RoomInstance {
                room_id: room,
                width: level.level.px_wid,
                height: level.level.px_hei,
                room_asset: level.clone(),
                position: Vec3::ZERO,
            });
        }
    }

    rooms
}

fn layout_rooms(
    mut rooms: Vec<RoomInstance>,
    settings: &Res<'_, DungeonGeneratorSettings>,
) -> Vec<RoomInstance> {
    let mut grid_spots: Vec<(usize, usize)> = Vec::new();
    let room_amount = (rooms.len() as f32 * 1.3) as usize;
    let grid_size = (room_amount as f32).sqrt().ceil() as usize;
    let mut placed_rooms: Vec<RoomInstance> = Vec::new();

    // Sort rooms by decreasing area
    rooms.sort_by(|a, b| {
        let area_a = a.width * a.height;
        let area_b = b.width * b.height;
        area_b.partial_cmp(&area_a).unwrap()
    });

    let largest_room = rooms.first().unwrap();
    let cell_width = (largest_room.width + settings.dungeons_space_between as i32) as f32;
    let cell_height = (largest_room.height + settings.dungeons_space_between as i32) as f32;

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

    // Iterate over the selected grid spots and place rooms in each spot
    for (row, col) in selected_spots {
        if let Some(mut room) = rooms.pop() {
            // Calculate the position of the room within the current grid cell
            let x = (col as f32 + 0.5) * cell_width;
            let y = (row as f32 + 0.5) * cell_height;

            // Adjust the position based on the room size and wall distance
            let adjusted_x =
                x - (room.width as f32 / 2.0) - (settings.dungeons_space_between as f32);
            let adjusted_y =
                y - (room.height as f32 / 2.0) - (settings.dungeons_space_between as f32);

            // Check if the grid spot is already occupied
            let is_occupied = occupied_spots.contains(&(row, col));

            // If the spot is occupied, find the nearest available spot
            let mut nearest_spot: Option<(usize, usize)> = None;
            if is_occupied {
                for &(row_offset, col_offset) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    let new_row = row as isize + row_offset;
                    let new_col = col as isize + col_offset;
                    if new_row >= 0
                        && new_row < grid_size as isize
                        && new_col >= 0
                        && new_col < grid_size as isize
                    {
                        let new_spot = (new_row as usize, new_col as usize);
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
                let new_adjusted_x =
                    new_x - (room.width as f32 / 2.0) - (settings.dungeons_space_between as f32);
                let new_adjusted_y =
                    new_y - (room.height as f32 / 2.0) - (settings.dungeons_space_between as f32);
                room.position = Vec3::new(new_adjusted_x, new_adjusted_y, 0.0);
                placed_rooms.push(room);
                occupied_spots.insert((new_row, new_col));
            } else {
                // Place the room at the adjusted position
                room.position = Vec3::new(adjusted_x, adjusted_y, 0.0);
                placed_rooms.push(room);
                occupied_spots.insert((row, col));
            }
        }
    }

    placed_rooms
}
