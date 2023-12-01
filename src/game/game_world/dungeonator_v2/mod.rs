use bevy::{
    asset::Assets,
    ecs::{bundle::Bundle, entity::Entity, query::With, schedule::States, system::Resource},
    log::info,
    prelude::{
        default, resource_exists, run_once, state_exists_and_equals, BuildChildren, Commands,
        Component, Condition, Handle, IntoSystemConfigs, Name, Plugin, Query, Res, SpatialBundle,
        Transform, Update,
    },
    reflect::Reflect,
};
use bevy_ecs_ldtk::{
    assets::LdtkExternalLevel,
    prelude::{ldtk::loaded_level::LoadedLevel, LdtkProject},
    LevelIid,
};
use rand::prelude::{Rng, SliceRandom};

use crate::{loading::assets::MapAssetHandles, AppState};

/// are we in dungeon yet?
#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Resource, Default, Reflect)]
pub enum DungeonGeneratorState {
    /// no dungeon is spawned
    #[default]
    NoDungeon,
    /// CreatingDungeon
    GeneratingDungeon,
    /// finished making dunegon
    FinishedDungeonGen,
}

/// generates dungeons from ldtk level files
pub struct DungeonGeneratorPlugin;

impl Plugin for DungeonGeneratorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<DungeonSettings>()
            .add_state::<DungeonGeneratorState>();
        app.add_systems(
            Update,
            ((
                spawn_dungeon_root
                    .run_if(state_exists_and_equals(AppState::StartMenu).and_then(run_once())),
                spawn_rooms.run_if(
                    state_exists_and_equals(DungeonGeneratorState::GeneratingDungeon)
                        .and_then(run_once()),
                ),
            ),),
        );
    }
}

/// spawns dungeon root
fn spawn_dungeon_root(mut cmds: Commands, ldtk_project_handles: Res<MapAssetHandles>) {
    info!("spawning dungeon container");
    cmds.spawn((DungeonContainerBundle {
        tag: DungeonContainerTag,
        name: "DungeonContainer".into(),
        settings: DungeonSettings {
            dungeon_room_amount: 10,
            looped_hallway_percentage: 0.0,
            grid_too_room_percentage: 0.0,
            min_space_between_rooms: 0.0,
        },
        ldtk_project: ldtk_project_handles.dungeons.clone(),
        spatial: SpatialBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
    },));
}

// place rooms down sequentially
// for previous room, choose new xy values using rand::gen_range, starting at previous room value + offset


// TODO:
// 0. create ?hashmap? of room positions/size
// 1. generate room position with rand
// 2. clamp room position to factor of 32
// 3. check if room position is valid: no intersections with other rooms inside hashmap, not already in hashmap
// 4. add room too hashmap. repeat untill hashmap length equal too dungeon_settings.room_amount

fn spawn_rooms(
    mut cmds: Commands,
    dungeon_root: Query<
        (Entity, &DungeonSettings, &Handle<LdtkProject>),
        With<DungeonContainerTag>,
    >,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    level_assets: Res<Assets<LdtkExternalLevel>>,
) {
    let (dungeon_root, dungeon_settings, dungeon_project_handle) = dungeon_root.single();
    let dungeon_project = ldtk_project_assets
        .get(dungeon_project_handle)
        .unwrap()
        .as_parent();

    let mut too_spawn_dungeons: Vec<DungeonRoomBundle> = Vec::new();
    let start_room = dungeon_project
        .iter_external_levels(&level_assets)
        .find(|asset| asset.identifier() == "SmallStartRoom")
        .expect("Dungeon Assets resource should ALWAYS have a room called `SmallStartRoom`")
        .iid()
        .clone();
    let useable_rooms: Vec<String> = dungeon_project
        .iter_external_levels(&level_assets)
        .filter(|level| level.identifier() != "SmallStartRoom")
        .map(|f| f.iid().clone())
        .collect();
    for i in 1..=dungeon_settings.dungeon_room_amount {
        let dungeon_name = format!("DungeonRoom {i}");
        let mut rng = rand::thread_rng();
        let room_iid = useable_rooms
            .choose(&mut rng)
            .expect("rand failed too choose a value");
        let new_room = DungeonRoomBundle {
            tag: DungeonRoomTag,
            name: Name::new(dungeon_name),
            id: LevelIid::new(room_iid),
            spatial: SpatialBundle::from_transform(Transform::from_xyz(
                rng.gen_range(-1000.0..=1000.0),
                rng.gen_range(-1000.0..=1000.0),
                0.0,
            )),
        };
        too_spawn_dungeons.insert(0, new_room);
    }
    let start_room_bundle = DungeonRoomBundle {
        tag: DungeonRoomTag,
        name: Name::new("StartRoom"),
        id: LevelIid::new(start_room),
        spatial: SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.0, 0.0)),
    };
    too_spawn_dungeons.insert(0, start_room_bundle);

    cmds.entity(dungeon_root).with_children(|dungeons| {
        for _ in 0..=dungeon_settings.dungeon_room_amount {
            let a = too_spawn_dungeons.remove(0);
            dungeons.spawn(a);
        }
    });
}

/// settings to configure the dungeon generator,
/// `useable_rooms` and hallways are filled by other systems
#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct DungeonSettings {
    /// amount of rooms
    dungeon_room_amount: i32,
    /// looped hallway percentage
    looped_hallway_percentage: f32,
    /// grids too room percentage
    grid_too_room_percentage: f32,
    /// minimum space between dungeon rooms, in tiles
    min_space_between_rooms: f32,
}

/// tag too identify dungeons
#[derive(Component)]
pub struct DungeonContainerTag;

/// tag too identify dungeon hallways
#[derive(Component)]
pub struct DungeonHallwayTag;

/// tag too identify dungeon rooms
#[derive(Component)]
pub struct DungeonRoomTag;

/// bundle for easy spawning of dungeon
/// always 1 per dungeon, all dungeon rooms are children
#[derive(Bundle)]
pub struct DungeonContainerBundle {
    /// identifies dungeon root entity
    tag: DungeonContainerTag,
    /// identified dungeon root in hierarchy
    name: Name,
    /// configures spawning of child rooms and hallways
    settings: DungeonSettings,
    /// data used too spawn with
    ldtk_project: Handle<LdtkProject>,
    /// gives dungeons a position
    spatial: SpatialBundle,
}

/// bundle for easy spawning of Dungeon Rooms
#[derive(Bundle)]
pub struct DungeonRoomBundle {
    /// identifies dungeon rooms
    tag: DungeonRoomTag,
    /// basically just `LevelIid`
    name: Name,
    /// id from `LdtkProject`
    id: LevelIid,
    /// spatial data
    spatial: SpatialBundle,
}

/// bundle for easy spawning of Dungeon Hallways
#[derive(Bundle)]
pub struct DungeonHallWayBundle {
    /// identifies dungeon hallways
    tag: DungeonHallwayTag,
    /// Hallway# from-to
    name: Name,
    /// spatial data
    spatial: SpatialBundle,
}
