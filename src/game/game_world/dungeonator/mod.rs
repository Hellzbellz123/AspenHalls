use bevy::{prelude::*, utils::HashMap};
use bevy_ecs_ldtk::{prelude::LdtkLevel, GridCoords};
use bevy_ecs_tilemap::{
    prelude::{
        get_tilemap_center_transform, TilemapId, TilemapSize, TilemapTexture, TilemapTileSize,
        TilemapType,
    },
    tiles::{TileBundle, TilePos},
    TilemapBundle,
};
use bevy_rapier2d::prelude::Collider;
use image::{ImageBuffer, Rgba};
use leafwing_input_manager::prelude::ActionState;
use seldom_map_nav::prelude::{NavPathMode, NavQuery, NavVec3, Navability, Navmeshes};

use crate::{
    consts::TILE_SIZE,
    game::{
        actors::{
            ai::components::Enemy,
            combat::components::WeaponTag,
            spawners::components::{SpawnWeaponEvent, WeaponType},
        },
        input::actions,
    },
};

use self::{
    generator::{DungeonContainerTag, DungeonRoomTag, RoomAssetID, RoomInstance},
    hallways::HallWay,
};

use super::{components::PlayerStartLocation, teleport_player_too_startloc};

/// systems and functions related too generating rooms
mod generator;
/// systems and functions for spawning hallways
mod hallways;
/// empty state file, may change generator
mod state;

/// generates dungeons from ldtk level files
pub struct DungeonGeneratorPlugin;

impl Plugin for DungeonGeneratorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(DungeonGeneratorSettings {
            grid_too_room_ratio: 2.6,
            dungeon_room_amount: 10,
            dungeons_space_between: (32.0 * 6.0), // width of tiles in pixels * tiles amount
            looped_hallway_amount: 0.125,
            useable_rooms: None,
            hallways: None,
        })
        .register_type::<HallWay>()
        .register_type::<RoomInstance>()
        .register_type::<DungeonGeneratorSettings>()
        .register_type::<NavMeshTest>()
        .add_systems(
            Update,
            (
                // TODO: fix the scheduling for these systems
                (
                    generator::setup_dungeon_environment,
                    generator::create_dungeons_list,
                )
                    .run_if(state_exists_and_equals(GeneratorStage::Initialization)),
                (generator::layout_dungeon_and_place_skeleton)
                    .run_if(state_exists_and_equals(GeneratorStage::GenerateRooms)),
                (regeneration_system).run_if(state_exists_and_equals(GeneratorStage::Finished)),
                (hallways::wait_for_ldtk_finish)
                    .run_if(state_exists_and_equals(GeneratorStage::PlaceRooms)),
            ),
        )
        .add_systems(
            OnEnter(GeneratorStage::GenerateConnections),
            (hallways::create_mst_from_rooms),
        )
        .add_systems(
            OnExit(GeneratorStage::GenerateConnections),
            (hallways::spawn_hallway_roots),
        )
        .add_systems(
            OnEnter(GeneratorStage::PathfindConnections),
            (hallways::pathfind_and_build_hallways),
        )
        .add_systems(
            OnEnter(GeneratorStage::Finished),
            (
                teleport_player_too_startloc,
                spawn_weapons_startloc,
                spawn_navigation_grid,
            ),
        );
    }
}

// generator::layout_dungeon_and_place_skeleton,
//     // .in_set(OnUpdate(GeneratorStage::GenerateRooms)),
// hallways::wait_for_ldtk_finish,
//     // .in_set(OnUpdate(GeneratorStage::PlaceRooms)),
// hallways::create_mst_from_rooms,
//     // .in_schedule(OnEnter(GeneratorStage::GenerateConnections)),
// hallways::spawn_hallway_roots,
//     // .in_schedule(OnExit(GeneratorStage::GenerateConnections)),
// hallways::pathfind_and_build_hallways,
//     // .in_schedule(OnEnter(GeneratorStage::PathfindConnections)),
// spawn_weapons_startloc,
//     // .in_schedule(OnEnter(GeneratorStage::Finished)),
// teleport_player_too_startloc,
//     // .in_schedule(OnEnter(GeneratorStage::Finished)),
// regeneration_system,
//     // .in_set(OnUpdate(GeneratorStage::Finished)),
// spawn_navigation_grid,
//     // .in_schedule(OnEnter(GeneratorStage::Finished)),
// test_navmesh
//     // .in_set(OnUpdate(GeneratorStage::Finished)),

/// different states of the DungeonGenerator
#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Resource, Default, Reflect)]
pub enum GeneratorStage {
    /// No Dungeon stuff has been spawned or Computed, \
    /// probably in the menu
    #[default]
    NoDungeon,
    /// try to get Dungeon entity, if none create, \
    /// fill list of useable dungeons
    Initialization,
    /// create rooms from list of useable dungeons
    GenerateRooms,
    /// spawn rooms into world, \
    /// generate MST from rooms, \
    /// add cycles too MST,
    PlaceRooms,
    /// Create Hallways from cycle_mst.edges
    GenerateConnections,
    /// pathfind hallways from hallway.start_pos too hallway.end_pos, \
    /// place tiles as we pathfind, \
    /// maybe create a Vec of tile positions and tile type, then spawn the tiles using a function
    PathfindConnections,
    /// Take all Tile_grids that are placed and create navigation tilemap from them,
    GenerateNavigationGrid,
    /// Dungeon Generation is finished, settings can be changed
    Finished,
}

/// settings to configure the dungeon generator,
/// useable_rooms and hallways are filled by other systems
#[derive(Debug, Clone, Resource, Default, Reflect)]
pub struct DungeonGeneratorSettings {
    /// amount of rooms
    dungeon_room_amount: i32,
    /// looped hallway percentage
    looped_hallway_amount: f32,
    /// grids too room percentage
    grid_too_room_ratio: f32,
    /// minimum space between dungeons, in tiles
    dungeons_space_between: f32,
    /// rooms that the generator may use
    #[reflect(ignore)]
    useable_rooms: Option<HashMap<RoomAssetID, Handle<LdtkLevel>>>,
    /// hallways too be generated
    hallways: Option<Vec<HallWay>>,
}

/// spawn weapons at start location
fn spawn_weapons_startloc(
    mut ew: EventWriter<SpawnWeaponEvent>,
    start_location: Query<&GlobalTransform, With<PlayerStartLocation>>,
) {
    if start_location.is_empty() {
        return;
    }

    let mut sum = Vec2::ZERO;
    let mut count = 0;

    for gtrans in start_location.iter() {
        sum += gtrans.translation().truncate();
        count += 1;
    }

    if count >= start_location.iter().len() {
        let spawn_pos = sum / count as f32;
        let tile_width: Vec2 = Vec2 { x: 36.0, y: 0.0 };

        ew.send(SpawnWeaponEvent {
            weapon_to_spawn: WeaponType::SmallSMG,
            spawn_position: spawn_pos + (tile_width * 2.0),
            spawn_count: 1,
        });

        ew.send(SpawnWeaponEvent {
            weapon_to_spawn: WeaponType::SmallPistol,
            spawn_position: spawn_pos - (tile_width * 2.0),
            spawn_count: 1,
        });
    }
}

/// Despawn and regenerate the map and supporting stuff
fn regeneration_system(
    mut cmds: Commands,
    mut dungeon_settings: ResMut<DungeonGeneratorSettings>,
    query_action_state: Query<&ActionState<actions::Combat>>,
    dungeon_container: Query<Entity, With<DungeonContainerTag>>,
    nav_mesh: Query<Entity, With<Navmeshes>>,
    weapons: Query<Entity, (With<WeaponTag>, Without<Parent>)>,
    enemys: Query<Entity, With<Enemy>>,
) {
    let input = query_action_state.single();
    let dungeon = dungeon_container.single();

    if !input.just_pressed(actions::Combat::DebugF2) {
        return;
    }
    info!("regenerate dungeon pressed");

    dungeon_settings.hallways = None;
    dungeon_settings.useable_rooms = None;

    cmds.entity(dungeon).despawn_descendants();

    nav_mesh.for_each(|navmesh| {
        cmds.entity(navmesh).despawn_recursive();
    });

    weapons.for_each(|weapon| {
        cmds.entity(weapon).despawn_recursive();
    });

    enemys.for_each(|enemy| {
        cmds.entity(enemy).despawn_recursive();
    });

    cmds.insert_resource(NextState(Some(GeneratorStage::Initialization)));
}

/// NavMesh Entity tag
#[derive(Debug, Component)]
pub struct NavMeshTag;

/// holds start/end point too generate
#[derive(Debug, Component, Reflect)]
pub struct NavMeshTest {
    /// start of path
    start: Vec3,
    /// end of path
    end: Vec3,
    /// should calculate yet?
    calculate: bool,
}

/// marker component for tiles in navgrid
#[derive(Debug, Component)]
pub struct NavTile;

// TODO: add Components too the room exit for querying those tiles,
// create mst with all roomexits and create hallways with a* pathfinding
// prims algorithm too generate cycles and other fun stuff in dungeon
// place hallways and walls around hallways with colliders

/// creates the navmesh from spawned tiles
fn spawn_navigation_grid(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    dungeon_container: Query<Entity, With<DungeonContainerTag>>,
    room_query: Query<(&GlobalTransform, &RoomInstance), With<DungeonRoomTag>>,
    tile_query: Query<
        (Entity, &TilePos, &GridCoords, &Transform, &GlobalTransform),
        With<Collider>,
    >,
    collider_q: Query<Entity, &Collider>,
) {
    // calculate width of tilemap
    let mut furthest_distance = 0.0;

    for (i, (transform1, room1)) in room_query.iter().enumerate() {
        for (transform2, room2) in room_query.iter().skip(i + 1) {
            let furthest_distance1 = calculate_furthest_point_distance(transform1, room1);
            let furthest_distance2 = calculate_furthest_point_distance(transform2, room2);
            let distance = furthest_distance1 + furthest_distance2;

            // Check if the distance is greater than the current furthest distance
            if distance > furthest_distance {
                furthest_distance = distance;
            }
        }
    }

    let tile_size = TilemapTileSize::from(TILE_SIZE);
    let dungeon_width_in_tiles: u32 =
        (((furthest_distance / tile_size.x).ceil() * tile_size.x) as i32 / TILE_SIZE.x as i32)
            .try_into()
            .unwrap();

    let map_size = TilemapSize {
        x: dungeon_width_in_tiles,
        y: dungeon_width_in_tiles,
    };

    let min_x = tile_query
        .iter()
        .fold(f32::INFINITY, |min, (_, _, _, _, global_transform)| {
            f32::min(min, global_transform.translation().x)
        });

    let min_y = tile_query
        .iter()
        .fold(f32::INFINITY, |min, (_, _, _, _, global_transform)| {
            f32::min(min, global_transform.translation().y)
        });

    info!("min_x: {}, min_y: {}", min_x, min_y);

    let global_tile_positions: Vec<(Entity, UVec2)> = tile_query
        .iter()
        .map(|(ent, _tile_pos, _, _transform, global_transform)| {
            let global_position = global_transform.translation().truncate();
            let global_tile_position = UVec2::new(
                ((global_position.x - min_x) / TILE_SIZE.x) as u32,
                ((global_position.y - min_y) / TILE_SIZE.y) as u32,
            );
            (ent, global_tile_position)
        })
        .collect();

    let mut tilemap: Vec<Navability> =
        vec![Navability::Navable; (map_size.x * map_size.y) as usize];

    for (entity, global_tile_pos) in &global_tile_positions {
        let does_tile_have_collider = collider_q.get(*entity).is_ok();
        let index = (global_tile_pos.y * map_size.x + global_tile_pos.x) as usize;
        if index < tilemap.len() {
            tilemap[index] = if does_tile_have_collider {
                Navability::Solid
            } else {
                Navability::Navable
            };
        }
    }

    let tilemap_entity = commands.spawn((Name::new("NavGrid"), NavMeshTag)).id();
    let map_type = TilemapType::Square;
    let grid_size = tile_size.into();
    let texture_handle: Handle<Image> = asset_server.load("tiles.png");
    let mut tile_storage = bevy_ecs_tilemap::tiles::TileStorage::empty(map_size);

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let mut collider_tiles: Vec<(Entity, UVec2)> = Vec::new();

            for (x, f) in global_tile_positions.clone().into_iter() {
                if collider_q.get(x).is_ok() && Vec2::from(tile_pos) == f.as_vec2() {
                    collider_tiles.push((x, f));
                }
            }

            if !collider_tiles.is_empty() {
                collider_tiles.into_iter().for_each(|f| {
                    let mut tile_entity = None;
                    commands
                        .entity(tilemap_entity)
                        .with_children(|child_builder| {
                            tile_entity = Some(
                                child_builder
                                    .spawn((
                                        Name::new("NavTile"),
                                        TileBundle {
                                            position: TilePos::from(f.1),
                                            tilemap_id: TilemapId(tilemap_entity),
                                            ..Default::default()
                                        },
                                        NavTile,
                                    ))
                                    .id(),
                            );
                        });
                    if let Some(entity) = tile_entity {
                        tile_storage.set(&tile_pos, entity);
                    }
                })
            }
        }
    }

    commands
        .entity(dungeon_container.single())
        .push_children(&[tilemap_entity]);
    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });

    // let image = draw_tilemap(&tilemap, map_size, tile_size.x);
    // image.save("tilemap.png").unwrap();

    let img = draw_tiles(map_size.into(), &global_tile_positions);
    img.save("tiles.png").unwrap();

    // let navability = |pos: UVec2| tilemap[(pos.y * map_size.x + pos.x) as usize];
    // let navmeshes = Navmeshes::generate(map_size.into(), TILE_SIZE, navability, [0.1]).unwrap();

    // // Spawn the tilemap with a `Navmeshes` component
    // commands.spawn((
    //     navmeshes,
    //     Name::new("NavMesh"),
    //     NavMeshTest {
    //         start: Vec3 {
    //             x: -12412.0,
    //             y: -12551.0,
    //             z: 0.0,
    //         },
    //         end: Vec3 {
    //             x: 12412.0,
    //             y: 12551.0,
    //             z: 0.0,
    //         },
    //         calculate: false,
    //     },
    // ));
}

/// function too test generated navmesh
/// prints path too console
fn test_navmesh(nav_mesh: Query<(&Navmeshes, &NavMeshTest)>) {
    nav_mesh.for_each(|(nav_mesh, data)| {
        if data.calculate {
            let start_pos = NavVec3 {
                x: data.start.x,
                y: data.start.y,
                z: 0.0,
            };
            let end_pos = NavVec3 {
                x: data.end.x,
                y: data.end.y,
                z: 0.0,
            };

            let path = nav_mesh.mesh(0.1).unwrap().find_path(
                start_pos,
                end_pos,
                NavQuery::Closest,
                NavPathMode::MidPoints,
            );
            info!("{:?}", path);
        }
    });
}

/// draws Vec of tiles too image
#[allow(dead_code)]
fn draw_tilemap(
    tilemap: &[Navability],
    map_size: TilemapSize,
    tile_size: f32,
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let _tile_size = tile_size as u32;
    let width = map_size.x * 2;
    let height = map_size.y * 2;

    let mut image = ImageBuffer::new(width, height);

    // Calculate the starting point for drawing the tiles from the center
    let start_x = (width / 2) as i32 - (map_size.x as i32);
    let start_y = (height / 2) as i32 - (map_size.y as i32);

    for y in 0..height {
        for x in 0..width {
            let index = ((y / 2) * map_size.x + (x / 2)) as usize;
            let navability = tilemap.get(index).unwrap_or(&Navability::Navable);

            let color = match navability {
                Navability::Solid => Rgba([255, 0, 0, 255]),   // Red
                Navability::Navable => Rgba([0, 0, 255, 255]), // Blue
            };

            // Normalize the x and y coordinates
            let pixel_x = start_x + x as i32;
            let pixel_y = start_y + (height as i32 - y as i32) - 1;

            if pixel_x >= 0 && pixel_y >= 0 {
                image.put_pixel(pixel_x as u32, pixel_y as u32, color);
            }
        }
    }
    image
}

// 1800925-5438

/// draws vec(Entity, Uvec2) too an png
fn draw_tiles(
    tile_map_size: UVec2,
    global_tile_positions: &[(Entity, UVec2)],
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    // Calculate the size of the image based on the tile map size
    let image_width = tile_map_size.x * 2;
    let image_height = tile_map_size.y * 2;

    // Calculate the starting point for drawing the tiles from the center
    let start_x = (image_width / 2) - (tile_map_size.x);
    let start_y = (image_height / 2) - (tile_map_size.y);

    // Create a new image buffer with a white background
    let mut image = ImageBuffer::new(image_width, image_height);
    for pixel in image.pixels_mut() {
        *pixel = Rgba([255, 255, 255, 255]);
    }

    let global_tile_positions: Vec<(&Entity, UVec2)> = global_tile_positions
        .iter()
        .map(|(ent, pos)| (ent, UVec2::new(pos.x, pos.y)))
        .collect();

    // Draw the individual tiles with a contrasting color
    for (_, pos) in global_tile_positions {
        let x = pos.x;
        let y = pos.y;

        let pixel_x = start_x + x;
        let pixel_y = start_y + y;

        // Set the color of the tile to a contrasting color (e.g., red)
        set_pixel(&mut image, pixel_x * 2, pixel_y * 2, Rgba([255, 0, 0, 255]));
        set_pixel(
            &mut image,
            pixel_x * 2 + 1,
            pixel_y * 2,
            Rgba([255, 0, 0, 255]),
        );
        set_pixel(
            &mut image,
            pixel_x * 2,
            pixel_y * 2 + 1,
            Rgba([255, 0, 0, 255]),
        );
        set_pixel(
            &mut image,
            pixel_x * 2 + 1,
            pixel_y * 2 + 1,
            Rgba([255, 0, 0, 255]),
        );
    }

    image
}

/// set Pixel on (image) at (x, y) too (color)
fn set_pixel(image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, x: u32, y: u32, color: Rgba<u8>) {
    if x < image.width() && y < image.height() {
        image.get_pixel_mut(x, y).clone_from(&color);
    }
}

/// calculates which corner in a roominstance is furthest from Vec3::Zero
fn calculate_furthest_point_distance(transform: &GlobalTransform, room: &RoomInstance) -> f32 {
    let half_width = room.width as f32 / 2.0;
    let half_height = room.height as f32 / 2.0;

    // Calculate the corner points of the room in local coordinates
    let top_left = Vec3::new(-half_width, -half_height, 0.0);
    let top_right = Vec3::new(half_width, -half_height, 0.0);
    let bottom_left = Vec3::new(-half_width, half_height, 0.0);
    let bottom_right = Vec3::new(half_width, half_height, 0.0);

    // Transform the corner points to global coordinates
    let transformed_top_left = transform.compute_matrix() * top_left.extend(1.0);
    let transformed_top_right = transform.compute_matrix() * top_right.extend(1.0);
    let transformed_bottom_left = transform.compute_matrix() * bottom_left.extend(1.0);
    let transformed_bottom_right = transform.compute_matrix() * bottom_right.extend(1.0);

    // Calculate the distances between the corner points and the origin (0, 0, 0)
    let distance1 = transformed_top_left.distance(Vec3::ZERO.extend(1.0));
    let distance2 = transformed_top_right.distance(Vec3::ZERO.extend(1.0));
    let distance3 = transformed_bottom_left.distance(Vec3::ZERO.extend(1.0));
    let distance4 = transformed_bottom_right.distance(Vec3::ZERO.extend(1.0));

    // Return the furthest distance among the corner points
    distance1.max(distance2).max(distance3).max(distance4)
}
