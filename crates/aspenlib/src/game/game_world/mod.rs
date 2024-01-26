use bevy::prelude::*;
use bevy_ecs_ldtk::{
    prelude::{EntityIid, LdtkEntityAppExt},
    TileEnumTags,
};
use bevy_ecs_tilemap::prelude::*;
use bevy_rapier2d::prelude::{Collider, CollisionGroups, Group, RigidBody, Rot, Vect};
use leafwing_input_manager::action_state::ActionState;
use rand::prelude::{Rng, ThreadRng};

use crate::{
    consts::{AspenCollisionLayer, ACTOR_Z_INDEX, TILE_SIZE},
    game::{
        characters::{
            components::{CharacterMoveState, CharacterType, TeleportStatus},
            player::PlayerSelectedHero,
        },
        game_world::{
            components::{ActorTeleportEvent, PlayerStartLocation, TpTriggerEffect},
            dungeonator_v2::{
                components::{DungeonContainerTag, DungeonSettings},
                GeneratorState,
            },
            ldtk_bundles::{
                LdtkCollisionBundle, LdtkEnemySpawnerBundle, LdtkHeroPlaceBundle,
                LdtkRoomExitBundle, LdtkStartLocBundle, LdtkTeleporterBundle,
                LdtkWeaponSpawnerBundle,
            },
        },
        input::action_maps,
        items::EventSpawnItem,
    },
    loading::registry::RegistryIdentifier,
};

/// shared components for dungeon and home
pub mod components;
// pub mod dungeonator_v1;
/// holds dungeon generator plugin
pub mod dungeonator_v2;
/// hideout plugin, spawns home area for before and after dungeons
pub mod hideout;
/// bundles for entities that are defined inside ldtk
mod ldtk_bundles;

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
        app.add_event::<ActorTeleportEvent>()
            //.add_state::<GeneratorStage>()
            .insert_resource(TilemapRenderSettings {
                render_chunk_size: RENDER_CHUNK_SIZE,
                ..Default::default()
            })
            .add_plugins((
                hideout::HideOutPlugin,
                dungeonator_v2::DungeonGeneratorPlugin,
            ))
            .register_ldtk_entity::<LdtkTeleporterBundle>("Teleporter")
            .register_ldtk_entity::<LdtkEnemySpawnerBundle>("EnemySpawner")
            .register_ldtk_entity::<LdtkWeaponSpawnerBundle>("WeaponSpawner")
            .register_ldtk_entity::<LdtkStartLocBundle>("PlayerStartLoc")
            .register_ldtk_entity::<LdtkRoomExitBundle>("RoomExit")
            .register_ldtk_entity::<LdtkHeroPlaceBundle>("HeroLocation")
            .add_systems(
                Update,
                (
                    process_tile_enum_tags.run_if(any_with_component::<TileEnumTags>()),
                    handle_teleport_events.run_if(on_event::<ActorTeleportEvent>()),
                    listen_rebuild_dungeon_request
                        .run_if(state_exists_and_equals(GeneratorState::FinishedDungeonGen)),
                ),
            )
            .add_systems(
                OnEnter(GeneratorState::PlaceHallwayRoots),
                (teleport_player_too_start_location, populate_start_room),
            );
    }
}

/// listens for dungeon rebuild request if dungeon is finished spawning
fn listen_rebuild_dungeon_request(
    mut cmds: Commands,
    mut dungeon_root: Query<(Entity, &mut DungeonSettings), With<DungeonContainerTag>>,
    enemies: Query<Entity, (With<CharacterMoveState>, Without<PlayerSelectedHero>)>,
    actions: Res<ActionState<action_maps::Gameplay>>,
) {
    if actions.just_pressed(&action_maps::Gameplay::DebugF2) {
        cmds.entity(dungeon_root.single().0).despawn_descendants();
        enemies.for_each(|f| {
            cmds.entity(f).despawn_recursive();
        });
        dungeon_root.single_mut().1.positioned_rooms = Vec::new();
        cmds.insert_resource(NextState(Some(GeneratorState::SelectPresets)));
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
    mut characters: Query<(&mut Transform, &mut CharacterMoveState), With<CharacterType>>,
    global_transforms: Query<&GlobalTransform>,
    parents: Query<&Parent>,
    children: Query<&Children>,
    iids: Query<&EntityIid>,
) {
    for event in tp_events.read() {
        info!("recieved Tp Event: {:?}", event);
        let (mut target_transform, mut move_state) =
            match characters.get_mut(event.target.expect("target should not be empty")) {
                Ok((a, b)) => (a, b),
                Err(e) => {
                    warn!("teleport event for entity without `Transform`: {e}");
                    return;
                }
            };

        if move_state.teleport_status.teleport_not_requested() && event.sender.is_some() {
            warn!(
                "got a teleport event not requested by teleporter. actor move state: {:?}",
                move_state
            );
        }

        move_state.teleport_status = TeleportStatus::Teleporting;
        match &event.tp_type {
            //TODO: target_tile is a tileid. get this tile ids positon from the sensors parent
            TpTriggerEffect::Local(target_tile_reference) => {
                let entity_layer = parents.get(event.sender.unwrap()).unwrap().get();
                let ent_ids = children
                    .iter_descendants(entity_layer)
                    .filter(|f| {
                        iids.get(*f)
                            .expect("all entities on entity_layer should have EntityIid")
                            == &EntityIid::new(target_tile_reference.entity_iid.clone())
                    })
                    .collect::<Vec<Entity>>();

                let Some(target_tile_ent) = ent_ids.last() else {
                    warn!("target tile entity did not exist as a child of this level");
                    return;
                };

                let target_tile_transform = global_transforms
                    .get(*target_tile_ent)
                    .expect("any entity should have a transform");

                info!(
                    "moving player this many: {}",
                    target_tile_transform.translation()
                );
                target_transform.translation = target_tile_transform.translation();
                move_state.teleport_status = TeleportStatus::Teleporting;
            }
            TpTriggerEffect::Global(pos) => {
                target_transform.translation = pos.extend(ACTOR_Z_INDEX);
                move_state.teleport_status = TeleportStatus::None;
            }
            // expand this for better type checking
            TpTriggerEffect::Event(event) => {
                match event.as_str() {
                    "StartDungeonGen" => {
                        cmds.insert_resource(NextState(Some(GeneratorState::SelectPresets)));
                    }
                    event => {
                        warn!("unhandled Teleport Event Action: {}", event);
                    }
                }
                move_state.teleport_status = TeleportStatus::Teleporting;
            }
        }
    }
}

/// spawns items in the dungeon start room for the player too use
fn populate_start_room(
    mut ew: EventWriter<EventSpawnItem>,
    dungeon_root: Query<Entity, With<DungeonContainerTag>>,
) {
    let Ok(dungeon) = dungeon_root.get_single() else {
        error!("no dungeon too spawn starting weaoins at");
        return;
    };

    info!("sending item spawns for dungeon start");
    ew.send(EventSpawnItem {
        spawn_data: (RegistryIdentifier("smallsmg".to_string()), 1),
        requester: dungeon,
    });
    ew.send(EventSpawnItem {
        spawn_data: (RegistryIdentifier("smallpistol".to_string()), 1),
        requester: dungeon,
    });
}

/// teleports player too the average `Transform` of all entities with `PlayerStartLocation`
// TODO: find all uses of cmds.spawn(()) and add cleanup component
// cleanup component should be a system that querys for a specific DespawnComponent and despawns all entitys in the query
#[allow(clippy::type_complexity)]
fn teleport_player_too_start_location(
    mut player_query: Query<(Entity, &mut CharacterMoveState), With<PlayerSelectedHero>>,
    start_location: Query<(&PlayerStartLocation, &GlobalTransform)>,
    mut tp_events: EventWriter<ActorTeleportEvent>,
) {
    if start_location.is_empty() {
        warn!("error teleporting player too start location: no locations");
        return;
    }

    if start_location.get_single().is_err() {
        warn!("issue teleporting player too start location: multiple locations");
    }

    let (start_size, start_pos) = {
        let a = start_location
            .iter()
            .next()
            .expect("PlayerStartLocation should always exist");
        (a.0.size, a.1.translation().truncate())
    };

    let start_loc_rect = Rect {
        min: Vec2 {
            x: start_pos.x - start_size.x,
            y: start_pos.y - start_size.y,
        },
        max: Vec2 {
            x: start_pos.x + start_size.x,
            y: start_pos.y + start_size.y,
        },
    };

    let pos = random_point_inside(&start_loc_rect, 3.0).unwrap_or(start_pos);

    warn!("teleporting player too start location: {}", pos);
    let (player_ent, mut player_tp_state) = player_query.single_mut();
    tp_events.send(ActorTeleportEvent {
        tp_type: TpTriggerEffect::Global(pos),
        target: Some(player_ent),
        sender: None,
    });
    player_tp_state.teleport_status = TeleportStatus::Requested;
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
            if let Some(mut cmds) = commands.get_entity(entity) {
                cmds.remove::<TileEnumTags>();
            } else {
                warn!("tag entity was despawned")
            }
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

// TODO:
// maybe make this a system the registers a bundle?
/// checks tile enum tag for collider tag, creates shape for collider, passes too `insert_collider`, tag is then removed from `tile_enum_tags`
fn check_tag_colliders(
    tag: &str,
    commands: &mut Commands<'_, '_>,
    entity: Entity,
    tile_enum_tag: &mut Mut<'_, TileEnumTags>,
    degrees: f32,
) {
    match tag {
        "CollideUp" => {
            let shape: Vec<(Vect, Rot, Collider)> =
                vec![(Vec2::new(0.0, -12.), 0.0, Collider::cuboid(16.0, 4.0))];
            insert_collider(commands, entity, shape, tag, tile_enum_tag);
        }
        "CollideDown" => {
            let shape: Vec<(Vect, Rot, Collider)> =
                vec![(Vec2::new(0.0, 12.0), 0.0, Collider::cuboid(16.0, 4.0))];
            insert_collider(commands, entity, shape, tag, tile_enum_tag);
        }
        "CollideLeft" => {
            let shape: Vec<(Vect, Rot, Collider)> =
                vec![(Vec2::new(12.0, 0.0), 0.0, Collider::cuboid(4.0, 16.0))];
            insert_collider(commands, entity, shape, tag, tile_enum_tag);
        }
        "CollideRight" => {
            let shape: Vec<(Vect, Rot, Collider)> =
                vec![(Vec2::new(-12.0, 0.0), 0.0, Collider::cuboid(4.0, 16.0))];
            insert_collider(commands, entity, shape, tag, tile_enum_tag);
        }
        "CollideCornerLR" => {
            let shape: Vec<(Vect, Rot, Collider)> =
                vec![(Vec2::new(-12.0, 12.0), 0.0, Collider::cuboid(4.0, 4.0))];
            insert_collider(commands, entity, shape, tag, tile_enum_tag);
        }
        "CollideCornerUR" => {
            let shape: Vec<(Vect, Rot, Collider)> =
                vec![(Vec2::new(-12.0, -12.0), 0.0, Collider::cuboid(4.0, 4.0))];
            insert_collider(commands, entity, shape, tag, tile_enum_tag);
        }
        "CollideCornerLL" => {
            let shape: Vec<(Vect, Rot, Collider)> =
                vec![(Vec2::new(12.0, 12.0), 0.0, Collider::cuboid(4.0, 4.0))];
            insert_collider(commands, entity, shape, tag, tile_enum_tag);
        }
        "CollideCornerUL" => {
            let shape: Vec<(Vect, Rot, Collider)> =
                vec![(Vec2::new(12.0, -12.0), 0.0, Collider::cuboid(4.0, 4.0))];
            insert_collider(commands, entity, shape, tag, tile_enum_tag);
        }
        "CollideInnerUL" => {
            let shape: Vec<(Vect, Rot, Collider)> = vec![
                (Vec2::new(-12.0, -4.0), degrees, Collider::cuboid(12.0, 4.0)),
                (Vec2::new(0.0, 12.0), 0.0, Collider::cuboid(16.0, 4.0)),
            ];
            insert_collider(commands, entity, shape, tag, tile_enum_tag);
        }
        "CollideInnerLL" => {
            let shape: Vec<(Vect, Rot, Collider)> = vec![
                (Vec2::new(-12.0, 4.0), degrees, Collider::cuboid(12.0, 4.0)),
                (Vec2::new(0.0, -12.0), 0.0, Collider::cuboid(16.0, 4.0)),
            ];
            insert_collider(commands, entity, shape, tag, tile_enum_tag);
        }
        "CollideInnerUR" => {
            let shape: Vec<(Vect, Rot, Collider)> = vec![
                (Vec2::new(12.0, -4.0), degrees, Collider::cuboid(12.0, 4.0)),
                (Vec2::new(0.0, 12.0), 0.0, Collider::cuboid(16.0, 4.0)),
            ];
            insert_collider(commands, entity, shape, tag, tile_enum_tag);
        }
        "CollideInnerLR" => {
            let shape: Vec<(Vect, Rot, Collider)> = vec![
                (Vec2::new(12.0, 4.0), degrees, Collider::cuboid(12.0, 4.0)),
                (Vec2::new(0.0, -12.0), 0.0, Collider::cuboid(16.0, 4.0)),
            ];
            insert_collider(commands, entity, shape, tag, tile_enum_tag);
        }
        "DoubleWallVertical" => {
            let shape: Vec<(Vect, Rot, Collider)> = vec![
                (Vec2::new(12.0, 4.0), degrees, Collider::cuboid(16.0, 4.0)),
                (Vec2::new(-12.0, 4.0), degrees, Collider::cuboid(16.0, 4.0)),
            ];
            insert_collider(commands, entity, shape, tag, tile_enum_tag);
        }
        "CollideInnerWall" | "CollideOuterWall" => {
            let shape: Vec<(Vect, Rot, Collider)> =
                vec![(Vec2::new(0.0, 14.0), 0.0, Collider::cuboid(16.0, 4.0))];
            insert_collider(commands, entity, shape, tag, tile_enum_tag);
        }
        "RoomExit" => {
            // TODO: atm it is unused
            // in the future this might be expanded on
        }
        unknown => {
            println!("ERROR: Unknown Tile Enum Tag on this entity: {unknown}");
        }
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
    commands.entity(entity).insert(LdtkCollisionBundle {
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

/// returns a point inside the rect with -`inset`. `inset` is multiplied by `TILE_SIZE`
fn random_point_inside(rect: &Rect, inset: f32) -> Option<Vec2> {
    let mut rng = ThreadRng::default();
    let useable_space = rect.inset(-(TILE_SIZE * inset));
    let Rect {
        min: usable_min,
        max: usable_max,
    } = useable_space;
    let vertical_range = usable_min.y..usable_max.y;
    let horizontal_range = usable_min.x..usable_max.x;

    if rect.is_empty() {
        return None;
    }

    Some(Vec2 {
        x: if horizontal_range.is_empty() {
            warn!("using center x of {:?}", rect);
            rect.center().x
        } else {
            warn!("using range x");
            rng.gen_range(horizontal_range)
        },
        y: if vertical_range.is_empty() {
            warn!("using center y of {:?}", rect);
            rect.center().y
        } else {
            warn!("using range y");
            rng.gen_range(vertical_range)
        },
    })
}
