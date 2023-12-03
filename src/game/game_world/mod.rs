use std::{cmp::max, time::Duration};

use bevy::{
    prelude::*, time::common_conditions::on_timer, transform::systems::propagate_transforms,
};
use bevy_ecs_ldtk::{
    prelude::{GridCoords, LayerMetadata, LdtkEntityAppExt},
    utils::{grid_coords_to_translation_relative_to_tile_layer, grid_coords_to_translation},
    TileEnumTags,
};
use bevy_ecs_tilemap::prelude::*;
use bevy_rapier2d::prelude::{Collider, CollisionGroups, Group, RigidBody, Rot, Vect};

use crate::{
    ahp::game::ActorType,
    consts::{AspenCollisionLayer, ACTOR_Z_INDEX, TILE_SIZE},
    game::game_world::{
        dungeonator_v2::DungeonGeneratorState,
        hideout::{ActorTeleportEvent, TPType},
    },
    utilities::on_component_added,
    AppState,
};

use self::{
    components::{
        CollisionBundle, LdtkRoomExitBundle, LdtkSpawnerBundle, LdtkStartLocBundle,
        LdtkTeleporterBundle, PlayerStartLocation,
    },
    // dungeonator_v1::GeneratorStage,
};

use super::actors::components::Player;

/// shared components for dungeon and home
pub mod components;
// pub mod dungeonator_v1;
/// holds dungeon generator plugin
pub mod dungeonator_v2;
/// hideout plugin, spawns home area for before and after dungeons
pub mod hideout;

/// chunk size
const CHUNK_SIZE: UVec2 = UVec2 { x: 16, y: 16 };
/// Render chunk sizes are set to 4 render chunks per user specified chunk.
const RENDER_CHUNK_SIZE: UVec2 = UVec2 {
    x: CHUNK_SIZE.x * 2,
    y: CHUNK_SIZE.y * 2,
};

/// game world plugin handles home area and dungeon generator functions
pub struct GameWorldPlugin;

// TODO:
// Take main ldtk asset, create 3 tile layers from the many tile layers on the ldtk rooms
// spawn these 3 tile layers on large grid, rooms will have their own entity for holding data about room
// tiles will be set too the main dungeon grid.
// this should all be done using bevy_ecs_tilemap data structures,
// Implement PathFinding Algorithms for the tilemap
// a function that creates a path from point a on the tilemap too point b with references too each tile position,
// probably a Vec((TilePos, TileType))

impl Plugin for GameWorldPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app
            //.add_state::<GeneratorStage>()
            .insert_resource(TilemapRenderSettings {
                render_chunk_size: RENDER_CHUNK_SIZE,
                ..Default::default()
            })
            .add_plugins((
                hideout::HideOutPlugin,
                dungeonator_v2::DungeonGeneratorPlugin,
            ))
            .register_ldtk_entity::<LdtkTeleporterBundle>("TeleportSensor")
            .register_ldtk_entity::<LdtkSpawnerBundle>("EnemySpawner")
            .register_ldtk_entity::<LdtkStartLocBundle>("PlayerStartLoc")
            .register_ldtk_entity::<LdtkRoomExitBundle>("RoomExit")
            .add_systems(
                Update,
                (
                    process_tile_enum_tags.run_if(any_with_component::<TileEnumTags>()),
                    handle_teleport_events.run_if(on_event::<ActorTeleportEvent>()),
                    teleport_player_too_start_location.run_if(
                        state_exists_and_equals(AppState::StartMenu)
                            .and_then(on_timer(Duration::from_secs_f32(0.2)).and_then(run_once())),
                    ),
                ),
            )
            .add_systems(
                OnEnter(DungeonGeneratorState::FinishedDungeonGen),
                teleport_player_too_start_location.run_if(state_exists_and_equals(
                    DungeonGeneratorState::FinishedDungeonGen,
                )),
            );
    }
}

/// Holds `NavGrid`, for easier query
#[derive(Component)]
pub struct GridContainerTag;

/// handles passed player teleport events
/// warns if event is unknown
fn handle_teleport_events(
    mut cmds: Commands,
    mut tp_events: EventReader<ActorTeleportEvent>,
    mut actor_transforms: Query<(&mut Transform, Option<&Parent>), With<ActorType>>,
    other_transform: Query<(&Transform, Option<&Parent>), Without<ActorType>>,
    layer_data: Query<(&LayerMetadata, &TileStorage, &TilemapSize)>,
    names: Query<&Name>,
    children_query: Query<&Children>,
    parents: Query<&Parent>,
    grid_coords: Query<&GridCoords>,
) {
    for event in tp_events.read() {
        info!("recieved Tp Event: {:?}", event);
        match &event.tp_type {
            hideout::TPType::Local(target_tile) => {
                if let Some(target) = event.target {
                    let sensor = event.sender.unwrap();
                    let entity_layer = parents.get(sensor).unwrap().get();
                    let level = parents.get(entity_layer).unwrap().get();
                    let ground_layer = children_query
                        .iter_descendants(level)
                        .find(|f| {
                            let name = names.get(*f).expect("should have name");
                            name.as_str() == "Ground_Layer"
                        })
                        .expect("the level should always have `Ground_Layer`");

                    let (metadata, tilestorage, tilemapsize) = layer_data
                        .get(ground_layer)
                        .expect("got the wrong `Ground_Layer`");

                        // TODO: this doesnt work

                    let tpos = IVec2::new(target_tile.x, tilemapsize.y as i32 - target_tile.y);
                    let coords = GridCoords::new(tpos.x, tpos.y);
                    let target_pos =
                        grid_coords_to_translation(coords, IVec2::splat(32));
                    let (mut target_transform, _) = actor_transforms
                        .get_mut(target)
                        .expect("ActorTeleportEvent targeting entity without transform");
                    info!("moving player this many: {}", target_pos);
                    target_transform.translation += target_pos.extend(0.0);
                } else {
                    warn!("TPType::Local requires a valid entity");
                }
            }
            hideout::TPType::Event(event) => match event.as_str() {
                "StartDungeonGen" => {
                    cmds.insert_resource(NextState(Some(DungeonGeneratorState::PrepareDungeon)));
                }
                event => {
                    warn!("unhandled Teleport Event Action: {}", event);
                }
            },
            //TODO: target_tile is a tileid. get this tile ids positon from the sensors parent
            hideout::TPType::Global(pos) => {
                if let Some(ent) = event.target {
                    let (mut target_transform, parent) = actor_transforms
                        .get_mut(ent)
                        .expect("ActorTeleportEvent targeting entity without transform");

                    target_transform.translation = pos.extend(ACTOR_Z_INDEX);
                } else {
                    warn!("TPType::Global requires a valid entity");
                }
            }
        }
    }
}

/// teleports player too the average `Transform` of all entities with `PlayerStartLocation`
// TODO: find all uses of cmds.spawn(()) and add cleanup component
// cleanup component should be a system that querys for a specific DespawnComponent and despawns all entitys in the query
#[allow(clippy::type_complexity)]
fn teleport_player_too_start_location(
    player_query: Query<Entity, With<Player>>,
    start_location: Query<&GlobalTransform, With<PlayerStartLocation>>,
    mut tp_events: EventWriter<ActorTeleportEvent>,
) {
    if start_location.is_empty() {
        warn!("no start locations");
        return;
    }

    let mut sum = Vec2::ZERO;
    let mut current_count: i32 = 0;
    let length = start_location.iter().len() as i32;

    for global_transform in start_location.iter() {
        let global = global_transform.translation().truncate();
        let pos = global;
        sum += pos;
        current_count += 1;
        info!("found transform: {}", pos);
        info!("new count: {}", current_count);
        info!("new sum: {}", sum);
    }

    if current_count == length {
        let avg = sum / (current_count as f32);
        tp_events.send(ActorTeleportEvent {
            tp_type: TPType::Global(avg),
            target: Some(player_query.single()),
            sender: None,
        });
        info!("got start pos: {:?}, total sampled: {}", avg, length);
    }
}

/// Takes `TileEnumTags` that is added from ldtk editor
fn process_tile_enum_tags(
    mut commands: Commands,
    mut tiles_with_enums: Query<(Entity, &mut TileEnumTags)>,
) {
    if tiles_with_enums.is_empty() {
        return;
    }
    // 90 degrees radian
    let ninety_degrees = std::f32::consts::FRAC_PI_2;
    for (entity, mut tile_enum_tag) in &mut tiles_with_enums {
        let tags = tile_enum_tag.tags.clone();
        if tags.is_empty() {
            // info!("Tile has no more tags");
            commands.entity(entity).remove::<TileEnumTags>();
        }
        for tag in tags {
            check_tag_colliders(
                &tag,
                &mut commands,
                entity,
                &mut tile_enum_tag,
                ninety_degrees,
            );
        }
    }
}

/// checks tile enum tag for collider tag, creates shape for collider, passes too `insert_collider`, tag is then removed from `tile_enum_tags`
fn check_tag_colliders(
    tag: &str,
    commands: &mut Commands<'_, '_>,
    entity: Entity,
    tile_enum_tag: &mut Mut<'_, TileEnumTags>,
    degrees: f32,
) {
    if "CollideUp" == tag {
        let shape: Vec<(Vect, Rot, Collider)> =
            vec![(Vec2::new(0.0, -12.), 0.0, Collider::cuboid(16.0, 4.0))];
        insert_collider(commands, entity, shape, tag, tile_enum_tag);
    }
    if "CollideDown" == tag {
        let shape: Vec<(Vect, Rot, Collider)> =
            vec![(Vec2::new(0.0, 12.0), 0.0, Collider::cuboid(16.0, 4.0))];
        insert_collider(commands, entity, shape, tag, tile_enum_tag);
    }
    if "CollideLeft" == tag {
        let shape: Vec<(Vect, Rot, Collider)> =
            vec![(Vec2::new(12.0, 0.0), 0.0, Collider::cuboid(4.0, 16.0))];
        insert_collider(commands, entity, shape, tag, tile_enum_tag);
    }
    if "CollideRight" == tag {
        let shape: Vec<(Vect, Rot, Collider)> =
            vec![(Vec2::new(-12.0, 0.0), 0.0, Collider::cuboid(4.0, 16.0))];
        insert_collider(commands, entity, shape, tag, tile_enum_tag);
    }
    if "CollideWall" == tag {
        let shape: Vec<(Vect, Rot, Collider)> =
            vec![(Vec2::new(0.0, 14.0), 0.0, Collider::cuboid(16.0, 4.0))];
        insert_collider(commands, entity, shape, tag, tile_enum_tag);
    }
    if "CollideCornerLR" == tag {
        let shape: Vec<(Vect, Rot, Collider)> =
            vec![(Vec2::new(-12.0, 12.0), 0.0, Collider::cuboid(4.0, 4.0))];
        insert_collider(commands, entity, shape, tag, tile_enum_tag);
    }
    if "CollideCornerUR" == tag {
        let shape: Vec<(Vect, Rot, Collider)> =
            vec![(Vec2::new(-12.0, -12.0), 0.0, Collider::cuboid(4.0, 4.0))];
        insert_collider(commands, entity, shape, tag, tile_enum_tag);
    }
    if "CollideCornerLL" == tag {
        let shape: Vec<(Vect, Rot, Collider)> =
            vec![(Vec2::new(12.0, 12.0), 0.0, Collider::cuboid(4.0, 4.0))];
        insert_collider(commands, entity, shape, tag, tile_enum_tag);
    }
    if "CollideCornerUL" == tag {
        let shape: Vec<(Vect, Rot, Collider)> =
            vec![(Vec2::new(12.0, -12.0), 0.0, Collider::cuboid(4.0, 4.0))];
        insert_collider(commands, entity, shape, tag, tile_enum_tag);
    }
    if "CollideInnerUL" == tag {
        let shape: Vec<(Vect, Rot, Collider)> = vec![
            (Vec2::new(-12.0, -4.0), degrees, Collider::cuboid(12.0, 4.0)),
            (Vec2::new(0.0, 12.0), 0.0, Collider::cuboid(16.0, 4.0)),
        ];
        insert_collider(commands, entity, shape, tag, tile_enum_tag);
    }
    if "CollideInnerLL" == tag {
        let shape: Vec<(Vect, Rot, Collider)> = vec![
            (Vec2::new(-12.0, 4.0), degrees, Collider::cuboid(12.0, 4.0)),
            (Vec2::new(0.0, -12.0), 0.0, Collider::cuboid(16.0, 4.0)),
        ];
        insert_collider(commands, entity, shape, tag, tile_enum_tag);
    }
    if "CollideInnerUR" == tag {
        let shape: Vec<(Vect, Rot, Collider)> = vec![
            (Vec2::new(12.0, -4.0), degrees, Collider::cuboid(12.0, 4.0)),
            (Vec2::new(0.0, 12.0), 0.0, Collider::cuboid(16.0, 4.0)),
        ];
        insert_collider(commands, entity, shape, tag, tile_enum_tag);
    }
    if "CollideInnerLR" == tag {
        let shape: Vec<(Vect, Rot, Collider)> = vec![
            (Vec2::new(12.0, 4.0), degrees, Collider::cuboid(12.0, 4.0)),
            (Vec2::new(0.0, -12.0), 0.0, Collider::cuboid(16.0, 4.0)),
        ];
        insert_collider(commands, entity, shape, tag, tile_enum_tag);
    }
}

/// inserts collider onto passed entity, collides with everything
fn insert_collider(
    commands: &mut Commands<'_, '_>,
    entity: Entity,
    shape: Vec<(Vec2, f32, Collider)>,
    tag: &str,
    tags: &mut Mut<'_, TileEnumTags>,
) {
    commands.entity(entity).insert(CollisionBundle {
        name: Name::new(tag.to_owned()),
        rigidbody: RigidBody::Fixed,
        collision_shape: Collider::compound(shape),
        collision_group: CollisionGroups {
            memberships: AspenCollisionLayer::WORLD,
            filters: Group::ALL,
        },
    });
    remove_value(&mut tags.tags, tag);
}

/// takes reference too string and a value, removes from the Vec<String>
fn remove_value(vec: &mut Vec<String>, value: &str) {
    vec.retain(|elem| elem != value);
}
