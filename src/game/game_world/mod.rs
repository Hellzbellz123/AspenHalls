use bevy::prelude::*;
// use bevy_debug_grid::{Grid, GridAlignment, GridAxis, SubGrid, TrackedGrid};
use bevy_ecs_ldtk::{LdtkPlugin, TileEnumTags};
use bevy_rapier2d::prelude::{Collider, CollisionGroups, Group, RigidBody, Rot, Vect};

use crate::{consts::PLAYER_LAYER, game::GameStage};

use self::{components::CollisionBundle, dungeon_generator::GeneratorStage};

pub mod components;
mod dungeon_generator;
pub mod sanctuary;

pub struct GameWorldPlugin;

impl Plugin for GameWorldPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_state::<GeneratorStage>()
            .add_plugin(LdtkPlugin)
            .add_plugin(sanctuary::HideOutPlugin)
            .add_plugin(dungeon_generator::DungeonGeneratorPlugin) //;
            // `TilemapRenderSettings` be added before the `TilemapPlugin`.
            .insert_resource(TilemapRenderSettings {
                render_chunk_size: RENDER_CHUNK_SIZE,
                ..Default::default()
            })
            .add_system(process_tile_enum_tags.in_set(OnUpdate(GameStage::PlayingGame)));
        // .add_systems(
        //     (spawn_chunks_around_camera, despawn_outofrange_chunks)
        //         .in_set(OnUpdate(GameStage::PlayingGame)),
        // )
    }
}

// TODO: spawn a 16x16 grid so we can make sure all dungeons are aligned
// can probably use ecs tilemap but i dont really need _ALL_ the functionality?
// may make pathfinding easier tho so im not sure

use bevy_ecs_tilemap::prelude::*;

const DISTANCE_AROUND_CAMERA: i32 = 8;

// /// Press WASD to move the camera around, and watch as chunks spawn/despawn in response.
const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 32.0, y: 32.0 };
// For this example, don't choose too large a chunk size.
const CHUNK_SIZE: UVec2 = UVec2 { x: 4, y: 4 };
// Render chunk sizes are set to 4 render chunks per user specified chunk.
const RENDER_CHUNK_SIZE: UVec2 = UVec2 {
    x: CHUNK_SIZE.x * 2,
    y: CHUNK_SIZE.y * 2,
};

#[derive(Component)]
pub struct GridContainerTag;

fn process_tile_enum_tags(mut commands: Commands, new_tile_enums: Query<(Entity, &TileEnumTags)>) {
    if new_tile_enums.is_empty() {
        return;
    }
    // 90 degrees radian
    let ndgs = std::f32::consts::FRAC_PI_2;
    for (entity, tile_enum_tag) in new_tile_enums.iter() {
        for tag in tile_enum_tag.tags.iter() {
            // debug!("{}", tag);
            if "CollideUp" == tag {
                let shape: Vec<(Vect, Rot, Collider)> =
                    vec![(Vec2::new(0.0, -12.), 0.0, Collider::cuboid(16.0, 4.0))];
                commands.entity(entity).remove::<TileEnumTags>();
                commands.entity(entity).insert(CollisionBundle {
                    name: Name::new("CollideUp"),
                    rigidbody: RigidBody::Fixed,
                    collision_shape: Collider::compound(shape),
                    collision_group: CollisionGroups {
                        memberships: Group::all(),
                        filters: PLAYER_LAYER,
                    },
                });
            }
            if "CollideDown" == tag {
                let shape: Vec<(Vect, Rot, Collider)> =
                    vec![(Vec2::new(0.0, 12.0), 0.0, Collider::cuboid(16.0, 4.0))];
                commands.entity(entity).remove::<TileEnumTags>();
                commands.entity(entity).insert(CollisionBundle {
                    name: Name::new("CollideDown"),
                    rigidbody: RigidBody::Fixed,
                    collision_shape: Collider::compound(shape),
                    collision_group: CollisionGroups {
                        memberships: Group::all(),
                        filters: PLAYER_LAYER,
                    },
                });
            }
            if "CollideLeft" == tag {
                let shape: Vec<(Vect, Rot, Collider)> =
                    vec![(Vec2::new(12.0, 0.0), 0.0, Collider::cuboid(4.0, 16.0))];
                commands.entity(entity).remove::<TileEnumTags>();
                commands.entity(entity).insert(CollisionBundle {
                    name: Name::new("CollideLeft"),
                    rigidbody: RigidBody::Fixed,
                    collision_shape: Collider::compound(shape),
                    collision_group: CollisionGroups {
                        memberships: Group::all(),
                        filters: PLAYER_LAYER,
                    },
                });
            }
            if "CollideRight" == tag {
                let shape: Vec<(Vect, Rot, Collider)> =
                    vec![(Vec2::new(-12.0, 0.0), 0.0, Collider::cuboid(4.0, 16.0))];
                commands.entity(entity).remove::<TileEnumTags>();
                commands.entity(entity).insert(CollisionBundle {
                    name: Name::new("CollideRight"),
                    rigidbody: RigidBody::Fixed,
                    collision_shape: Collider::compound(shape),
                    collision_group: CollisionGroups {
                        memberships: Group::all(),
                        filters: PLAYER_LAYER,
                    },
                });
            }
            if "CollideWall" == tag {
                let shape: Vec<(Vect, Rot, Collider)> =
                    vec![(Vec2::new(0.0, 14.0), 0.0, Collider::cuboid(16.0, 4.0))];
                commands.entity(entity).remove::<TileEnumTags>();
                commands.entity(entity).insert(CollisionBundle {
                    name: Name::new("CollideWall"),
                    rigidbody: RigidBody::Fixed,
                    collision_shape: Collider::compound(shape),
                    collision_group: CollisionGroups {
                        memberships: Group::all(),
                        filters: PLAYER_LAYER,
                    },
                });
            }
            if "CollideCornerLR" == tag {
                let shape: Vec<(Vect, Rot, Collider)> =
                    vec![(Vec2::new(-12.0, 12.0), 0.0, Collider::cuboid(4.0, 4.0))];
                commands.entity(entity).remove::<TileEnumTags>();
                commands.entity(entity).insert(CollisionBundle {
                    name: Name::new("CollideCornerLR"), //upper left //FINISHED
                    rigidbody: RigidBody::Fixed,
                    collision_shape: Collider::compound(shape),
                    collision_group: CollisionGroups {
                        memberships: Group::all(),
                        filters: PLAYER_LAYER,
                    },
                });
            }
            if "CollideCornerUR" == tag {
                let shape: Vec<(Vect, Rot, Collider)> =
                    vec![(Vec2::new(-12.0, -12.0), 0.0, Collider::cuboid(4.0, 4.0))];
                commands.entity(entity).remove::<TileEnumTags>();
                commands.entity(entity).insert(CollisionBundle {
                    name: Name::new("CollideCornerUR"), //lower left
                    rigidbody: RigidBody::Fixed,
                    collision_shape: Collider::compound(shape),
                    collision_group: CollisionGroups {
                        memberships: Group::all(),
                        filters: PLAYER_LAYER,
                    },
                });
            }
            if "CollideCornerLL" == tag {
                let shape: Vec<(Vect, Rot, Collider)> =
                    vec![(Vec2::new(12.0, 12.0), 0.0, Collider::cuboid(4.0, 4.0))];
                commands.entity(entity).remove::<TileEnumTags>();
                commands.entity(entity).insert(CollisionBundle {
                    name: Name::new("CollideCornerLL"), //upper right   //done
                    rigidbody: RigidBody::Fixed,
                    collision_shape: Collider::compound(shape),
                    collision_group: CollisionGroups {
                        memberships: Group::all(),
                        filters: PLAYER_LAYER,
                    },
                });
            }
            if "CollideCornerUL" == tag {
                let shape: Vec<(Vect, Rot, Collider)> =
                    vec![(Vec2::new(12.0, -12.0), 0.0, Collider::cuboid(4.0, 4.0))];
                commands.entity(entity).remove::<TileEnumTags>();
                commands.entity(entity).insert(CollisionBundle {
                    name: Name::new("CollideCornerUL"), //lower right
                    rigidbody: RigidBody::Fixed,
                    collision_shape: Collider::compound(shape),
                    collision_group: CollisionGroups {
                        memberships: Group::all(),
                        filters: PLAYER_LAYER,
                    },
                });
            }
            if "CollideInnerUL" == tag {
                let shape: Vec<(Vect, Rot, Collider)> = vec![
                    (Vec2::new(-12.0, -4.0), ndgs, Collider::cuboid(12.0, 4.0)),
                    (Vec2::new(0.0, 12.0), 0.0, Collider::cuboid(16.0, 4.0)),
                ];
                commands.entity(entity).remove::<TileEnumTags>();
                commands.entity(entity).insert(CollisionBundle {
                    name: Name::new("CollideInnerUL"), //lower left inverted corner
                    rigidbody: RigidBody::Fixed,
                    collision_shape: Collider::compound(shape),
                    collision_group: CollisionGroups {
                        memberships: Group::all(),
                        filters: PLAYER_LAYER,
                    },
                });
            }
            if "CollideInnerLL" == tag {
                let shape: Vec<(Vect, Rot, Collider)> = vec![
                    (Vec2::new(-12.0, 4.0), ndgs, Collider::cuboid(12.0, 4.0)),
                    (Vec2::new(0.0, -12.0), 0.0, Collider::cuboid(16.0, 4.0)),
                ];
                commands.entity(entity).remove::<TileEnumTags>();
                commands.entity(entity).insert(CollisionBundle {
                    name: Name::new("CollideInnerLL"), //lower left inverted corner
                    rigidbody: RigidBody::Fixed,
                    collision_shape: Collider::compound(shape),
                    collision_group: CollisionGroups {
                        memberships: Group::all(),
                        filters: PLAYER_LAYER,
                    },
                });
            }
            if "CollideInnerUR" == tag {
                let shape: Vec<(Vect, Rot, Collider)> = vec![
                    (Vec2::new(12.0, -4.0), ndgs, Collider::cuboid(12.0, 4.0)),
                    (Vec2::new(0.0, 12.0), 0.0, Collider::cuboid(16.0, 4.0)),
                ];
                commands.entity(entity).remove::<TileEnumTags>();
                commands.entity(entity).insert(CollisionBundle {
                    name: Name::new("CollideInnerUR"), //upper right inverted corner
                    rigidbody: RigidBody::Fixed,
                    collision_shape: Collider::compound(shape),
                    collision_group: CollisionGroups {
                        memberships: Group::all(),
                        filters: PLAYER_LAYER,
                    },
                });
            }
            if "CollideInnerLR" == tag {
                let shape: Vec<(Vect, Rot, Collider)> = vec![
                    (Vec2::new(12.0, 4.0), ndgs, Collider::cuboid(12.0, 4.0)),
                    (Vec2::new(0.0, -12.0), 0.0, Collider::cuboid(16.0, 4.0)),
                ];
                commands.entity(entity).remove::<TileEnumTags>();
                commands.entity(entity).insert(CollisionBundle {
                    name: Name::new("CollideInnerLR"), //lower right inverted corner
                    rigidbody: RigidBody::Fixed,
                    collision_shape: Collider::compound(shape),
                    collision_group: CollisionGroups {
                        memberships: Group::all(),
                        filters: PLAYER_LAYER,
                    },
                });
            } else {
                // warn!("{:#?}", tile_enum_tag)
            }
        }
    }
}

// fn spawn_chunk(
//     commands: &mut Commands,
//     asset_server: &AssetServer,
//     chunk_pos: IVec2,
//     container: Entity,
// ) {
//     let tilemap_entity = commands.spawn().id();
//     let mut tile_storage = TileStorage::empty(CHUNK_SIZE.into());
//     // Spawn the elements of the tilemap.
//     for x in 0..CHUNK_SIZE.x {
//         for y in 0..CHUNK_SIZE.y {
//             let tile_pos = TilePos { x, y };
//             let tile_entity = commands
//                 .spawn(TileBundle {
//                     position: tile_pos,
//                     tilemap_id: TilemapId(tilemap_entity),
//                     ..Default::default()
//                 })
//                 .id();
//             commands.entity(tilemap_entity).add_child(tile_entity);
//             commands.entity(container).add_child(tilemap_entity);
//             tile_storage.set(&tile_pos, tile_entity);
//         }
//     }

//     let transform = Transform::from_translation(Vec3::new(
//         chunk_pos.x as f32 * CHUNK_SIZE.x as f32 * TILE_SIZE.x,
//         chunk_pos.y as f32 * CHUNK_SIZE.y as f32 * TILE_SIZE.y,
//         0.0,
//     ));

//     struct Point {
//         x: f32,
//         y: f32,
//     }

//     let point = Point { x: 123125.23423, y: 12313453.123123 };

//             let x = (point.x / TILE_SIZE.y).round() * TILE_SIZE.y;
//             let y = (point.y / TILE_SIZE.x).round() * TILE_SIZE.x;

//     let texture_handle: Handle<Image> = asset_server.load("tiles.png");
//     commands.entity(tilemap_entity).insert(TilemapBundle {
//         grid_size: TILE_SIZE.into(),
//         size: CHUNK_SIZE.into(),
//         storage: tile_storage,
//         texture: TilemapTexture::Single(texture_handle),
//         tile_size: TILE_SIZE,
//         transform,
//         ..Default::default()
//     });
// }

// fn camera_pos_to_chunk_pos(camera_pos: &Vec2) -> IVec2 {
//     let camera_pos = camera_pos.as_ivec2();
//     let chunk_size: IVec2 = IVec2::new(CHUNK_SIZE.x as i32, CHUNK_SIZE.y as i32);
//     let tile_size: IVec2 = IVec2::new(TILE_SIZE.x as i32, TILE_SIZE.y as i32);
//     camera_pos / (chunk_size * tile_size)
// }

// fn spawn_chunks_around_camera(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     camera_query: Query<&Transform, (With<Camera>, With<MainCameraTag>)>,
//     mut chunk_manager: ResMut<ChunkManager>,
//     chunk_container: Query<Entity, With<GridContainerTag>>,
// ) {
//     if chunk_container.is_empty() {
//         commands.spawn((
//             Name::new("GridContainer"),
//             GridContainerTag,
//             SpatialBundle {
//                 ..default()
//             },
//         ));
//         return;
//     }

//     for transform in camera_query.iter() {
//         let camera_chunk_pos = camera_pos_to_chunk_pos(&transform.translation.xy());
//         for y in (camera_chunk_pos.y - DISTANCE_AROUND_CAMERA)..(camera_chunk_pos.y + DISTANCE_AROUND_CAMERA) {
//             for x in (camera_chunk_pos.x - DISTANCE_AROUND_CAMERA)..(camera_chunk_pos.x + DISTANCE_AROUND_CAMERA) {
//                 if !chunk_manager.spawned_chunks.contains(&IVec2::new(x, y)) {
//                     chunk_manager.spawned_chunks.insert(IVec2::new(x, y));
//                     spawn_chunk(
//                         &mut commands,
//                         &asset_server,
//                         IVec2::new(x, y),
//                         chunk_container.single(),
//                     );
//                 }
//             }
//         }
//     }
// }

// fn despawn_outofrange_chunks(
//     mut commands: Commands,
//     camera_query: Query<&Transform, (With<Camera>, With<MainCameraTag>)>,
//     chunks_query: Query<(Entity, &Transform), With<ChunkTag>>,
//     mut chunk_manager: ResMut<ChunkManager>,
// ) {
//     for camera_transform in camera_query.iter() {
//         for (entity, chunk_transform) in chunks_query.iter() {
//             let chunk_pos = chunk_transform.translation.xy();
//             let distance = camera_transform.translation.xy().distance(chunk_pos);
//             if distance > (DISTANCE_AROUND_CAMERA - 2 * (32*4)) as f32 {
//                 let x = (chunk_pos.x / (CHUNK_SIZE.x as f32 * TILE_SIZE.x)).floor() as i32;
//                 let y = (chunk_pos.y / (CHUNK_SIZE.y as f32 * TILE_SIZE.y)).floor() as i32;
//                 chunk_manager.spawned_chunks.remove(&IVec2::new(x, y));
//                 commands.entity(entity).despawn_recursive();
//             }
//         }
//     }
// }

// fn main() {
//     App::new()
//         .add_plugins(
//             DefaultPlugins
//                 .set(WindowPlugin {
//                     primary_window: Some(Window {
//                         title: String::from("Basic Chunking Example"),
//                         ..Default::default()
//                     }),
//                     ..default()
//                 })
//                 .set(ImagePlugin::default_nearest()),
//         )
//         .add_startup_system(startup)
//         .add_system(helpers::camera::movement)
//         .run();
// }
