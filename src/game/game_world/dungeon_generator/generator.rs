use bevy::{asset::HandleId, prelude::*, reflect::TypeUuid};
use bevy_ecs_ldtk::{
    app::{LdtkEntityMap, LdtkIntCellMap},
    ldtk::{Level, LevelBackgroundPosition},
    prelude::TilesetDefinition,
    utils::{create_entity_definition_map, create_layer_definition_map},
    LdtkAsset, LdtkLevel, LdtkSettings, LevelEvent, Respawn, Worldly,
};
use bevy_prototype_lyon::{
    prelude::{Fill, FillOptions, GeometryBuilder, Path, ShapeBundle},
    shapes,
};
use bevy_rapier2d::rapier::geometry;
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

#[derive(Component, Debug, Reflect)]
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
pub fn create_dungeon_container(
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

fn filter_levels(level_assets: &Res<Assets<LdtkLevel>>) -> Vec<LdtkLevel> {
    let excluded_levels = vec![
        // identifer for main level, this isnt placed for dungeons
        String::from("Sanctuary"),
        // identifier for start room, we manually place at 0,0
        // String::from("SmallStartRoom"),
    ];

    let vector_of_assets: Vec<LdtkLevel> = level_assets
        .iter()
        .map(|(_, level)| (level))
        .cloned()
        .filter(|l| !excluded_levels.contains(&l.level.identifier))
        .collect();
    vector_of_assets
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct DungeonID(i32);

pub fn create_dungeon_hashmap(
    mut cmds: Commands,
    mut gen_settings: ResMut<DungeonGeneratorSettings>,
    level_assets: Res<Assets<LdtkLevel>>,
) {
    info!("Creating hashmap of dungeons to be used in dungeon generation");
    let mut done = false;
    let useablelevels = filter_levels(&level_assets);
    let mut useable_rooms: HashMap<DungeonID, LdtkLevel> = HashMap::new();
    let level_len = useablelevels.len() as i32;
    let mut id = 0;

    for ldtk in useablelevels {
        info!(
            "Inserting {} into hashmap, {id}/{level_len}",
            ldtk.level.identifier
        );
        if ldtk.level.identifier == "SmallStartRoom" {
            useable_rooms.insert(DungeonID(id), ldtk);
            id += 1
        } else {
            useable_rooms.insert(DungeonID(id), ldtk);
            id += 1
        }
        if id == level_len - 1 {
            done = true;
        }
    }

    if done == true {
        gen_settings.useable_rooms = Some(useable_rooms);
        cmds.insert_resource(NextState(Some(GeneratorStage::PlaceRooms)))
    }
}

pub fn layout_dungeon_and_place_skeleton(
    mut cmds: Commands,
    gen_settings: Res<DungeonGeneratorSettings>,
    dungeon_container: Query<(Entity, &DungeonContainerTag, &Handle<LdtkAsset>)>,
) {
    let mut rng = thread_rng();
    let useable_levels = gen_settings.useable_rooms.as_ref().unwrap();
    let mut rooms: Vec<RoomInstance> = Vec::new();

    let dungeon_origin = gen_settings.dungeon_map_origin;
    let map_half_extent = gen_settings.dungeon_map_halfextent;
    let space_between_dungeons = gen_settings.dungeons_space_between;
    let dungeon_count: i32 = gen_settings.dungeon_room_amount;

    let mut dungeon_spots = generate_points_within_distance(
        dungeon_origin,
        map_half_extent, // this is actually a half extent
        space_between_dungeons,
        dungeon_count.try_into().unwrap(),
    );

    // remove 0,0,0 from list of dungeons AFTER its been position checked against other spots
    let start_spot = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    remove_vec3(&mut dungeon_spots, &start_spot);

    for kvp in useable_levels {
        info!("id: {:?} name {}", kvp.0, kvp.1.level.identifier)
    }

    'createvec: for room in 0..=gen_settings.dungeon_room_amount {
        let id_from_hashmap: i32 = rng.gen_range(1..(useable_levels.len() as i32 - 1));
        info!(
            "requesting id: {id_from_hashmap} from {:?}",
            useable_levels.keys()
        );

        if room == gen_settings.dungeon_room_amount {
            break 'createvec;
        }
        if room == 0 {
            info!("inserting SmallStartRoom at 0 0");
            rooms.insert(
                room.try_into().unwrap(),
                RoomInstance {
                    room_id: room,
                    room_asset: useable_levels
                        .get_key_value(&DungeonID(0))
                        .unwrap()
                        .1
                        .clone(),
                    width: 0,
                    height: 0,
                    position: Vec3::ZERO,
                },
            );
        } else {
            let level = useable_levels
                .get_key_value(&DungeonID(id_from_hashmap))
                .unwrap()
                .1
                .clone();
            rooms.insert(
                room.try_into().unwrap(),
                RoomInstance {
                    room_id: room,
                    width: level.level.px_wid,
                    height: level.level.px_hei,
                    room_asset: level,
                    position: dungeon_spots.remove(0),
                },
            );
        }
    }

    let rooms = layout_rooms(rooms, gen_settings.clone());
    let dungeon_container = dungeon_container.single();

    for room in rooms {
        info!("{:?}", room);
        let spawner_box_visual = shapes::Rectangle {
            extents: Vec2 { x: 5.0, y: 5.0 },
            origin: shapes::RectangleOrigin::Center,
        };
        let spawner_visual_bundle = GeometryBuilder::new().add(&spawner_box_visual).build();

        cmds.entity(dungeon_container.0).with_children(|parent| {
            let name = format!("{}", room.room_asset.level.identifier);
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

fn layout_rooms(
    mut rooms: Vec<RoomInstance>,
    settings: DungeonGeneratorSettings,
) -> Vec<RoomInstance> {
    // Sort rooms by decreasing area
    rooms.sort_by(|a, b| {
        let area_a = a.width * a.height;
        let area_b = b.width * b.height;
        area_b.partial_cmp(&area_a).unwrap()
    });

    let mut placed_rooms = vec![rooms.remove(0)];
    let mut rng = rand::thread_rng();

    while let Some(mut room) = rooms.pop() {
        let mut min_distance = i32::MAX;
        let mut min_pos = Vec2::new(0.0, 0.0);

        // Try placing the room in all adjacent positions
        for placed_room in &placed_rooms {
            let x_min = cmp::max(0, placed_room.position.x as i32 - room.width);
            let x_max = placed_room.position.x as i32 + placed_room.width;
            let y_min = cmp::max(0, placed_room.position.y as i32 - room.height);
            let y_max = placed_room.position.y as i32 + placed_room.height;

            for x in x_min..=x_max {
                for y in y_min..=y_max {
                    let pos = Vec2::new(x as f32, y as f32);
                    let mut distance = i32::MAX;

                    // Calculate distance to nearest placed room
                    for placed_room in &placed_rooms {
                        let dx = cmp::max(
                            0,
                            cmp::max(
                                placed_room.position.x as i32 - x,
                                x - placed_room.position.x as i32 - placed_room.width,
                            ),
                        );
                        let dy = cmp::max(
                            0,
                            cmp::max(
                                placed_room.position.y as i32 - y,
                                y - placed_room.position.y as i32 - placed_room.height,
                            ),
                        );
                        distance = cmp::min(distance, dx + dy);
                    }

                    // If the position is valid and closer to other rooms than the current minimum, update the minimum
                    if distance >= room.width && distance >= room.height && distance < min_distance
                    {
                        min_distance = distance;
                        min_pos = pos;
                    }
                }
            }
        }

        // If a valid adjacent position was found, place the room there
        if min_distance < i32::MAX {
            room.position = Vec3::new(min_pos.x as f32, min_pos.y as f32, 0.0);
            placed_rooms.push(room);
        } else {
            // If no valid adjacent position was found, place the room at a random position
            let x = rng.gen_range(
                -settings.dungeon_map_halfextent
                    ..settings.dungeon_map_halfextent - room.width as f32,
            );
            let y = rng.gen_range(
                -settings.dungeon_map_halfextent
                    ..settings.dungeon_map_halfextent - room.height as f32,
            );
            room.position = Vec3::new(x as f32, y as f32, 0.0);
            placed_rooms.push(room);
        }
    }

    placed_rooms
}

// place ldtk levels at random places on the map
pub fn place_dungeon_skeleton(
    mut cmds: Commands,
    dungeon_container: Query<(Entity, &DungeonContainerTag, &Handle<LdtkAsset>)>,
    gen_settings: Res<DungeonGeneratorSettings>,
    mut ldtk_assets: ResMut<Assets<LdtkAsset>>,
    level_assets: Res<Assets<LdtkLevel>>,
) {
    let mut thread_rng = thread_rng();
    let dungeon_container = dungeon_container.single();
    let Some(dungeon_asset) = ldtk_assets.get_mut(dungeon_container.2) else {warn!("Couldnt get the dungeons asset from Assets<LdtkAsset>"); return;};

    let dungeon_origin = gen_settings.dungeon_map_origin;
    let map_half_extent = gen_settings.dungeon_map_halfextent;
    let space_between_dungeons = gen_settings.dungeons_space_between;
    let dungeon_count: i32 = gen_settings.dungeon_room_amount;

    let mut dungeon_spots = generate_points_within_distance(
        dungeon_origin,
        map_half_extent, // this is actually a half extent
        space_between_dungeons,
        dungeon_count.try_into().unwrap(),
    );

    info!(
        "Generated {} points within distance {} from origin {:?} and at least {} units apart\n {:?}",
        dungeon_count, map_half_extent, dungeon_origin, space_between_dungeons, dungeon_spots);

    let useable_levels: Vec<LdtkLevel> = filter_levels(&level_assets);

    // remove 0,0,0 from list of dungeons AFTER its been position checked against other spots
    let start_spot = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    remove_vec3(&mut dungeon_spots, &start_spot);

    let startroom: Option<(HandleId, &LdtkLevel)> = level_assets
        .iter()
        .find(|f| f.1.level.identifier == "SmallStartRoom");
    let room = startroom.unwrap().1;

    let spawner_box_visual = shapes::Rectangle {
        extents: Vec2 { x: 5.0, y: 5.0 },
        origin: shapes::RectangleOrigin::Center,
    };
    let spawner_visual_bundle = GeometryBuilder::new().add(&spawner_box_visual).build();

    // spawn startroom at 0,0,0
    cmds.entity(dungeon_container.0).with_children(|parent| {
        info!(
            "Creating Dungeon Room: SmallStartRoom 1/{} at world origin",
            dungeon_count
        );
        parent.spawn(DungeonRoomBundle {
            name: Name::new("StartRoom"),
            ldtk_level: room.clone(),
            room_info: RoomInstance {
                room_id: 0,
                width: room.level.px_wid,
                height: room.level.px_hei,
                position: start_spot,
                room_asset: room.clone(),
            },
            tag: DungeonRoomTag,
            dv_shape: spawner_visual_bundle,
            dv_color: Fill {
                color: (Color::RED),
                options: FillOptions::default(),
            },
            transform: Transform::from_translation(start_spot),
            ..default()
        });
    });

    for current_dungeon in 0..=dungeon_count {
        dungeon_spots.shuffle(&mut thread_rng);
        let Some(spot) = dungeon_spots.pop() else  {
            // if dungeon spots is empty, pop returns none,
            // dungeon spots should be empty when we have spawned all dungeons
            // we can insert the next state
            info!("Done spawning template");
            cmds.insert_resource(NextState(Some(GeneratorStage::BuildDungeonRooms)));
            return;
        }; //{warn!("Couldnt choose Vec3 from dungeon_spots array"); return;};
        let levelasset = useable_levels
            .choose(&mut thread_rng)
            .expect("error choosing from map")
            .to_owned();

        let name = format!("{}", levelasset.level.identifier);
        info!(
            "Creating Dungeon Room: {name} {}/{}",
            current_dungeon + 2,
            dungeon_count
        );

        let spawner_box_visual = shapes::Rectangle {
            extents: Vec2 { x: 1.0, y: 1.0 },
            origin: shapes::RectangleOrigin::Center,
        };

        let spawner_visual_bundle = GeometryBuilder::new().add(&spawner_box_visual).build();

        let dungeon_spot = Transform {
            translation: spot,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        };

        cmds.entity(dungeon_container.0).with_children(|container| {
            container.spawn((
                Name::new(name),
                LdtkLevel {
                    level: levelasset.level,
                    background_image: levelasset.background_image,
                },
                DungeonRoomTag,
                ShapeBundle {
                    path: spawner_visual_bundle,
                    transform: dungeon_spot,
                    ..default()
                },
                Fill {
                    options: FillOptions::default(),
                    color: Color::RED,
                },
            ));
        });
    }
}

fn remove_vec3(vec_list: &mut Vec<Vec3>, vec_to_remove: &Vec3) {
    vec_list.retain(|v| v != vec_to_remove);
}

/// CENTERED AROUND 0,0,0 (World Origin)
fn generate_points_within_distance(
    origin: Vec3,
    total_spread: f32,
    space_between: f32,
    num_points: usize,
) -> Vec<Vec3> {
    warn!("Generating postions too place dungeons at");

    let mut rng = rand::thread_rng();
    let mut points: Vec<Vec3> = Vec::with_capacity(num_points);

    points.insert(
        0,
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
    );

    while points.len() < num_points {
        let x = rng.gen_range(-total_spread..total_spread);
        let y = rng.gen_range(-total_spread..total_spread);
        let z = 0.0;

        let mut point = origin + Vec3::new(x, y, z);
        let mut is_far_enough = true;

        for p in &points {
            if p.distance(point) < space_between {
                is_far_enough = false;
                break;
            }
        }

        let tile_size = 32.0;

        if is_far_enough && origin.distance(point) <= total_spread {
            point.x = (point.x / tile_size).round() * tile_size;
            point.y = (point.y / tile_size).round() * tile_size;
            points.push(point);
        }
    }

    warn!("Done Generating points");
    points
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

                    bevy_ecs_ldtk::level::spawn_level(
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
