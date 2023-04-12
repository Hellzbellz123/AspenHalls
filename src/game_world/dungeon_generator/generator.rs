use bevy::prelude::*;
use bevy_ecs_ldtk::{
    app::{LdtkEntityMap, LdtkIntCellMap},
    prelude::TilesetDefinition,
    utils::{create_entity_definition_map, create_layer_definition_map},
    LdtkAsset, LdtkLevel, LdtkSettings, LevelEvent, Respawn, Worldly,
};
use bevy_prototype_lyon::{
    prelude::{Fill, FillOptions, GeometryBuilder, ShapeBundle},
    shapes,
};
use rand::prelude::*;
use std::collections::HashMap;

use crate::{app_config::DifficultySettings, loading::assets::MapAssetHandles};

use super::{DungeonGeneratorSettings, GeneratorStage};

#[derive(Component, Default)]
pub struct DungeonContainerTag;

#[derive(Component, Default)]
pub struct DungeonTag;

#[derive(Bundle, Default)]
pub struct DungeonContainerBundle {
    pub ldtk_handle: Handle<LdtkAsset>,
    pub tag: DungeonContainerTag,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}

// TODO: pre place a "start room" for the dungeon at 0,0 and teleport the player too it
// it would be useful too have the startroom predefined in the dungeons asset
// spec for dungeon maps should be laid out,
// rooms should always have a SmallStartRoom, to make assumptions explainable?
pub fn create_dungeon_container(
    gen_settings: Res<DungeonGeneratorSettings>,
    dungeon_container: Query<Entity, &DungeonContainerTag>,
    mut cmds: Commands,
    dungeons: Res<MapAssetHandles>,
) {
    if dungeon_container.is_empty() {
        info!("No Dungeon Container, Creating Now.....");
        cmds.spawn((
            DungeonContainerBundle {
                tag: DungeonContainerTag,
                ldtk_handle: dungeons.dungeon_set_two.clone(),
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

// place ldtk levels at random places on the map
pub fn place_dungeon_skeleton(
    mut cmds: Commands,
    dungeon_container: Query<(Entity, &DungeonContainerTag, &Handle<LdtkAsset>)>,
    gen_settings: Res<DungeonGeneratorSettings>,
    mut ldtk_assets: ResMut<Assets<LdtkAsset>>,
    mut level_assets: ResMut<Assets<LdtkLevel>>,
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

    let spotsfn = dungeon_spots.iter().for_each(|spot| {
        info!("{:?}", spot);
    });

    info!(
        "Generated {} points within distance {} from origin {:?} and at least {} units apart\n {:#?}",
        dungeon_count, map_half_extent, dungeon_origin, space_between_dungeons, spotsfn
    );

    let dungeons: Vec<Handle<LdtkLevel>> = dungeon_asset.level_map.clone().into_values().collect(); //.collect();

    for _current_dungeon in 0..=dungeon_count {
        dungeon_spots.shuffle(&mut thread_rng);
        let Some(spot) = dungeon_spots.pop() else  {
            // if dungeon spots is empty, pop returns none,
            // dungeon spots should be empty when we have spawned all dungeons
            // we can insert the next state
            info!("Done spawning template");
            cmds.insert_resource(NextState(Some(GeneratorStage::BuildDungeonRooms)));
            return;
        }; //{warn!("Couldnt choose Vec3 from dungeon_spots array"); return;};
        let Some(levelasset) = level_assets.get_mut(dungeons.choose(&mut thread_rng).expect("error choosing from map")).cloned() else {info!("couldnt get the level from the hashmap"); return;};
        let name = format!("dungeon: {}", levelasset.level.identifier);

        info!("Creating Dungeon Skeleton: {name}");

        let spawner_box_visual = shapes::Rectangle {
            extents: Vec2 { x: 5.0, y: 5.0 },
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
                DungeonTag,
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

fn generate_points_within_distance(
    origin: Vec3,
    total_spread: f32,
    space_between: f32,
    num_points: usize,
) -> Vec<Vec3> {
    warn!("Generating postions too place dungeons at");

    let mut rng = rand::thread_rng();
    let mut points: Vec<Vec3> = Vec::with_capacity(num_points);

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
        (With<DungeonTag>, Without<Children>),
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
        With<DungeonTag>,
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
