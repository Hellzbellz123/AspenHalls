use bevy::prelude::*;
use bevy_ecs_ldtk::GridCoords;
use bevy_ecs_tilemap::{
    map::{TilemapId, TilemapSize, TilemapTexture, TilemapTileSize, TilemapType},
    prelude::TileTextureIndex,
    tiles::{TileBundle, TilePos},
    TilemapBundle,
};
use bevy_rapier2d::geometry::Collider;
use seldom_map_nav::mesh::{Navability, Navmeshes};

use crate::{consts::TILE_SIZE, game::game_world::dungeonator_v2::components::DungeonContainerTag};

// TODO:
// place hallways and walls around hallways with colliders
/// creates the navigation map from spawned tiles
pub fn create_pathmap(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    dungeon_container: Query<Entity, With<DungeonContainerTag>>,
    tile_query: Query<(Entity, &TilePos, &GridCoords, &Transform, &GlobalTransform)>,
    collider_q: Query<Entity, &Collider>,
) {
    warn!("creating pathmap");
    let tile_size = TilemapTileSize::from(Vec2::splat(TILE_SIZE));
    let (max_x, max_y, min_x, min_y) = get_map_axis_vals(&tile_query, 4.0);

    let map_size = TilemapSize {
        x: ((min_x.abs() + max_x) / TILE_SIZE).abs() as u32 + 2,
        y: ((min_y.abs() + max_y) / TILE_SIZE).abs() as u32 + 2,
    };

    info!("minimums x: {}, y: {}", min_x, min_y);
    info!("map size: {:?}", map_size);

    let global_tile_positions: Vec<(Entity, UVec2, bool)> = tile_query
        .iter()
        .map(|(ent, _tile_pos, _, _transform, global_transform)| {
            let has_collider = collider_q.get(ent).is_ok();
            let global_position = global_transform.translation().truncate();
            let global_tile_position = UVec2::new(
                ((global_position.x - min_x) / TILE_SIZE) as u32,
                ((global_position.y - min_y) / TILE_SIZE) as u32,
            );
            (ent, global_tile_position, has_collider)
        })
        .collect();

    let mut tilemap: Vec<Navability> =
        vec![Navability::Navable; (map_size.x * map_size.y) as usize];

    for (entity, global_tile_pos, has_collider) in &global_tile_positions {
        let index = (global_tile_pos.y * map_size.x + global_tile_pos.x) as usize;
        if index < tilemap.len() {
            tilemap[index] = Navability::Solid;
        }
    }

    let pathmap = commands
        .spawn((
            Name::new("NavGrid"),
            PathMap {
                tile_positions: Vec::new(),
                nav_map: Vec::new(),
            },
        ))
        .id();
    let map_type = TilemapType::Square;
    let grid_size = tile_size.into();
    let texture_handle: Handle<Image> =
        asset_server.load("packs/asha/levels/texture_atlas/homemade_tilemap32x32.png");
    let mut tile_storage = bevy_ecs_tilemap::tiles::TileStorage::empty(map_size);

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let mut collider_tiles: Vec<(Entity, UVec2)> = Vec::new();

            for (tile, pos, is_collider) in global_tile_positions.clone() {
                if is_collider && Vec2::from(tile_pos) == pos.as_vec2() {
                    collider_tiles.push((tile, pos));
                }
            }

            if !collider_tiles.is_empty() {
                for f in collider_tiles {
                    let mut tile_entity = None;
                    commands.entity(pathmap).with_children(|child_builder| {
                        tile_entity = Some(
                            child_builder
                                .spawn((
                                    Name::new("NavTile"),
                                    TileBundle {
                                        position: TilePos::from(f.1),
                                        tilemap_id: TilemapId(pathmap),
                                        texture_index: TileTextureIndex(23),
                                        ..Default::default()
                                    },
                                    NavTile,
                                ))
                                .id(),
                        );
                    });
                    if let Some(tile_entity) = tile_entity {
                        tile_storage.set(&tile_pos, tile_entity);
                    }
                }
            }
        }
    }

    let navability = |pos: UVec2| tilemap[(pos.y * map_size.x + pos.x) as usize];
    let navmeshes =
        Navmeshes::generate(map_size.into(), Vec2::splat(TILE_SIZE), navability, [0.1]).unwrap();

    commands
        .entity(dungeon_container.single())
        .push_children(&[pathmap]);
    commands.entity(pathmap).insert((
        TilemapBundle {
            grid_size,
            map_type,
            size: map_size,
            storage: tile_storage,
            texture: TilemapTexture::Single(texture_handle),
            tile_size,
            transform: Transform::from_translation(Vec2 { x: min_x, y: min_y }.extend(0.2)),
            ..Default::default()
        },
        navmeshes,
    ));
}
// TODO: rerun the pathmap algorithm for each dungeon room so paths can not overlap

fn get_map_axis_vals(
    tile_query: &Query<'_, '_, (Entity, &TilePos, &GridCoords, &Transform, &GlobalTransform)>,
    border: f32,
) -> (f32, f32, f32, f32) {
    let max_x = tile_query
        .iter()
        .max_by(|a, b| {
            let c1 = a.4.translation().truncate().x;
            let c2 = b.4.translation().truncate().x;
            c1.total_cmp(&c2)
        })
        .expect("tile should always exit")
        .4
        .translation()
        .x;

    let max_y = tile_query
        .iter()
        .max_by(|a, b| {
            let c1 = a.4.translation().truncate().y;
            let c2 = b.4.translation().truncate().y;
            c1.total_cmp(&c2)
        })
        .expect("msg")
        .4
        .translation()
        .y;

    let min_x = tile_query
        .iter()
        .min_by(|a, b| {
            let c1 = a.4.translation().truncate().x;
            let c2 = b.4.translation().truncate().x;
            c1.total_cmp(&c2)
        })
        .expect("tile should always exit")
        .4
        .translation()
        .x;

    let min_y = tile_query
        .iter()
        .min_by(|a, b| {
            let c1 = a.4.translation().truncate().y;
            let c2 = b.4.translation().truncate().y;
            c1.total_cmp(&c2)
        })
        .expect("msg")
        .4
        .translation()
        .y;
    (
        max_x + (TILE_SIZE * border),
        max_y + (TILE_SIZE * border),
        min_x - (TILE_SIZE * border),
        min_y - (TILE_SIZE * border),
    )
}

/// updates pathmap when new hallways are placed
fn update_pathmap(mut pathmap: Query<(&mut PathMap, &TilemapSize)>) {
    let (mut pathmap, size) = pathmap.single_mut();
    let tilemap = &pathmap.nav_map;
    let navability = |pos: UVec2| tilemap[(pos.y * size.x + pos.x) as usize];
    let navmeshes =
        Navmeshes::generate((*size).into(), Vec2::splat(TILE_SIZE), navability, [0.1]).unwrap();

    match std::fs::File::create("tiles.png") {
        Ok(_file) => {
            let img = draw_tiles((*size).into(), &pathmap.tile_positions);
            match img.save("tiles.png") {
                Ok(_) => info!("successfully wrote dungeon image"),
                Err(e) => warn!("error saving tiles.png {}", e),
            }
        }
        Err(e) => {
            warn!("failed too save tile image {}", e)
        }
    }
}

#[derive(Debug, Component)]
pub struct PathMap {
    pub tile_positions: Vec<(Entity, UVec2, bool)>,
    pub nav_map: Vec<Navability>,
}

#[derive(Debug, Component)]
pub struct NavTile;

use image::{ImageBuffer, Rgba};

/// draws `Vec<(Entity, UVec2)>` too an png
fn draw_tiles(
    tile_map_size: UVec2,
    global_tile_positions: &[(Entity, UVec2, bool)],
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let image_width = (tile_map_size.x) + 20;
    let image_height = (tile_map_size.y) + 20;

    let dark_gray = Rgba([64, 64, 64, 255]);
    let dark_red = Rgba([255, 64, 64, 255]);
    let light_blue = Rgba([20, 20, 87, 255]);

    let start_x = 10;
    let start_y = 10;

    let mut image = ImageBuffer::new(image_width, image_height);
    for pixel in image.pixels_mut() {
        *pixel = dark_gray;
    }

    let global_tile_positions: Vec<(&Entity, UVec2, bool)> = global_tile_positions
        .iter()
        .map(|(ent, pos, is_collider)| (ent, UVec2::new(pos.x, pos.y), *is_collider))
        .collect();

    for (_, pos, is_collider) in global_tile_positions {
        let color = if is_collider { dark_red } else { light_blue };

        let x = pos.x;
        // bevy is -y down, image is -y up
        // Flip the Y-coordinate to match
        // i dont understand how this works for u32 but it seems to work
        let y = tile_map_size.y - pos.y - 1;

        let pixel_x = start_x + x;
        let pixel_y = start_y + y;

        set_pixel(&mut image, pixel_x, pixel_y, color);
    }

    image
}

/// set Pixel on (image) at (x, y) too (color)
fn set_pixel(image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, x: u32, y: u32, color: Rgba<u8>) {
    if x < image.width() && y < image.height() {
        image.get_pixel_mut(x, y).clone_from(&color);
    }
}
