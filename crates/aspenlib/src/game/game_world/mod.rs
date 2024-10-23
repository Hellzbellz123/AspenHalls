use bevy::prelude::*;
use bevy_ecs_ldtk::{
    prelude::{EntityIid, LdtkEntityAppExt},
    TileEnumTags,
};

use leafwing_input_manager::action_state::ActionState;
use rand::prelude::{Rng, ThreadRng};

use crate::{
    consts::{ACTOR_Z_INDEX, TILE_SIZE},
    game::{
        characters::{
            components::{CharacterMoveState, CharacterType, TeleportStatus},
            player::PlayerSelectedHero,
        },
        game_world::{
            self,
            collisions::handle_and_removed_collider_tag,
            components::{
                ActorTeleportEvent, CharacterSpawner, HeroLocation, PlayerStartLocation,
                RoomBoundryTile, RoomExitTile, SpawnerTimer, SpawnerWave, Teleporter,
                TpTriggerEffect, WeaponSpawner,
            },
            dungeonator_v2::{components::Dungeon, GeneratorState},
            world_objects::{
                LdtkCharacterSpawner, LdtkHeroLocation, LdtkSpawnerWave, LdtkStartLocation,
                LdtkTeleporter, LdtkWeaponSpawner,
            },
        },
        input::action_maps,
        items::EventSpawnItem,
    },
    loading::registry::RegistryIdentifier,
    register_types, AppState,
};

/// tile collider creation systems/tools
mod collisions;
/// shared components for dungeon and home
pub mod components;
/// holds dungeon generator plugin
pub mod dungeonator_v2;
/// hideout plugin, spawns home area for before and after dungeons
pub mod hideout;
/// player progression module
pub mod progress;
/// bundles for entities that are defined inside ldtk
mod world_objects;
/// game world plugin handles home area and dungeon generator functions
pub struct GameWorldPlugin;

// TODO: for faster/more effecient dungeon generation?
// Take main ldtk asset, create 3 tile layers from the many tile layers on the ldtk rooms
// spawn these 3 tile layers on large grid, rooms will have their own entity for holding data about room
// tiles will be set too the main dungeon grid.
// this should all be done using bevy_ecs_tilemap data structures,

impl Plugin for GameWorldPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        register_types!(
            app,
            [
                SpawnerWave,
                SpawnerTimer,
                Teleporter,
                CharacterSpawner,
                WeaponSpawner,
                PlayerStartLocation,
                HeroLocation
            ]
        );

        app.add_event::<RegenerateDungeonEvent>()
            .add_event::<ActorTeleportEvent>()
            .add_plugins((
                progress::GameProgressPlugin,
                hideout::HideOutPlugin,
                dungeonator_v2::DungeonGeneratorPlugin,
            ))
            .register_ldtk_entity::<LdtkSpawnerWave>("SpawnerWave")
            .register_ldtk_entity::<LdtkTeleporter>("Teleporter")
            .register_ldtk_entity::<LdtkCharacterSpawner>("CharacterSpawner")
            .register_ldtk_entity::<LdtkWeaponSpawner>("WeaponSpawner")
            .register_ldtk_entity::<LdtkStartLocation>("StartLocation")
            .register_ldtk_entity::<LdtkHeroLocation>("HeroLocation")
            .add_systems(
                Update,
                (
                    process_tile_enum_tags.run_if(any_with_component::<TileEnumTags>),
                    handle_teleport_events.run_if(on_event::<ActorTeleportEvent>()),
                    (
                        listen_rebuild_dungeon_request,
                        debug_regen_dungeon,
                        game_world::world_objects::character_spawners_system.run_if(
                            in_state(GeneratorState::NoDungeon)
                                .or_else(in_state(GeneratorState::FinishedDungeonGen)),
                        ),
                    )
                        .run_if(in_state(AppState::PlayingGame)),
                ),
            )
            .add_systems(
                OnExit(GeneratorState::FinalizeHallways),
                populate_start_room,
            );
    }
}

/// listens for dungeon rebuild request if dungeon is finished spawning
#[allow(clippy::type_complexity)]
fn listen_rebuild_dungeon_request(
    mut regen_events: EventReader<RegenerateDungeonEvent>,
    mut cmds: Commands,
    _generator_state: Res<State<GeneratorState>>,
    actors: Query<
        Entity,
        (
            With<RegistryIdentifier>,
            Without<PlayerSelectedHero>,
            Without<Parent>,
        ),
    >,
) {
    if let Some(regen_event) = regen_events.read().next() {
        if regen_event.reason == RegenReason::FirstGeneration {
            warn!("laying out first dungeon");
            cmds.insert_resource(NextState(Some(GeneratorState::LayoutDungeon)));
            return;
        }
    
        info!("despawning old actors");
        actors.iter().for_each(|f| {
            cmds.entity(f).despawn_recursive();
        });
    
        cmds.insert_resource(NextState(Some(GeneratorState::LayoutDungeon)));
    }
    regen_events.clear();
}

/// send dungeon regen event for debug purposes
fn debug_regen_dungeon(
    actions: Res<ActionState<action_maps::Gameplay>>,
    mut regen_event: EventWriter<RegenerateDungeonEvent>,
) {
    if actions.just_pressed(&action_maps::Gameplay::DebugF2) {
        regen_event.send(RegenerateDungeonEvent {
            reason: RegenReason::ManualRegen,
        });
    }
}

/// event for rebuilding dungeon layout
#[derive(Event, Debug)]
pub struct RegenerateDungeonEvent {
    /// reason dungeon should regenerate
    pub reason: RegenReason,
}

/// why should dungeon be rebuilt
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum RegenReason {
    /// regen dungeon because player has defeated dungeon
    BossDefeat,
    /// player has requested a dungeon reset
    ManualRegen,
    /// player has died so regenerate dungeon
    PlayerDeath,
    /// player started game and dungeon should be generated
    FirstGeneration,
}

// /// holds all things related too game data for heirarchy, this might change
// #[derive(Debug, Component)]
// pub struct GameContainerTag;

// fn create_world_container(mut cmds: Commands) {
//     cmds.spawn((Name::new("GameContainer"), GameContainerTag));
// }

/// Holds `NavGrid`, for easier query
#[derive(Component)]
pub struct GridContainerTag;

/// handles passed player teleport events
/// warns if event is unknown
fn handle_teleport_events(
    mut regen_event: EventWriter<RegenerateDungeonEvent>,
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
                        // TODO: reset dungeon before changing state.
                        regen_event.send(RegenerateDungeonEvent {
                            reason: RegenReason::FirstGeneration,
                        });
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
    dungeon_root: Query<Entity, With<Dungeon>>,
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

/// Takes `TileEnumTags` that is added from ldtk editor
fn process_tile_enum_tags(
    mut commands: Commands,
    mut tiles_with_enums: Query<(Entity, &mut TileEnumTags)>,
) {
    if tiles_with_enums.is_empty() {
        return;
    }

    for (entity, mut tile_enum_tag) in &mut tiles_with_enums {
        let tags = tile_enum_tag.tags.clone();
        if tags.is_empty() {
            // info!("Tile has no more tags");
            if let Some(mut cmds) = commands.get_entity(entity) {
                cmds.remove::<TileEnumTags>();
            } else {
                warn!("tag entity was despawned");
            }
        }
        for tag in tags {
            if handle_and_removed_collider_tag(&tag, &mut commands, entity, &mut tile_enum_tag) {
                continue;
            }
            if handle_and_removed_misc_tag(&tag, &mut commands, entity, &mut tile_enum_tag) {
                continue;
            }
            info!("Unknown Tile Enum Tag on entity: {tag}");
        }
    }
}

/// checks for tags unrelated too collision
fn handle_and_removed_misc_tag(
    tag: &str,
    cmds: &mut Commands<'_, '_>,
    entity: Entity,
    tag_info: &mut Mut<'_, TileEnumTags>,
) -> bool {
    let tag_was_handled = match tag {
        "RoomExit" => {
            cmds.entity(entity)
                .insert((Name::new("RoomExit"), RoomExitTile));
            tag_info.tags.retain(|f| f != tag);
            true
        }
        "HallwayBoundry" => {
            cmds.entity(entity)
                .insert((Name::new("RoomBoundry"), RoomBoundryTile));
            true
        }
        _ => false,
    };

    if tag_was_handled {
        tag_info.tags.retain(|f| f != tag);
    }
    tag_was_handled
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
