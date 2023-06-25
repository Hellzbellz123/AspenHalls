use std::collections::HashMap;

use bevy::{math::vec3, prelude::*};
use bevy_ecs_ldtk::LdtkLevel;
use leafwing_input_manager::prelude::ActionState;

use crate::{
    components::actors::spawners::{SpawnWeaponEvent, WeaponType},
    consts::ACTOR_Z_INDEX,
    game::input::actions,
};

use self::generator::{DungeonContainerTag, RoomID, RoomInstance};

mod generator;
mod utils;
// mod test;
// mod grid2d;
// mod utils;

pub struct DungeonGeneratorPlugin;

impl Plugin for DungeonGeneratorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(DungeonGeneratorSettings {
            grid_too_room_ratio: 1.6,
            dungeon_room_amount: 30,
            dungeons_space_between: (32.0 * 6.0), // width of tiles in pixels * tiles amount
            useable_rooms: None,
        })
        .register_type::<RoomInstance>()
        .register_type::<DungeonGeneratorSettings>()
        .add_systems((
            generator::setup_dungeon_environment.in_set(OnUpdate(GeneratorStage::Initialization)),
            generator::create_dungeons_list.in_set(OnUpdate(GeneratorStage::Initialization)),
            generator::layout_dungeon_and_place_skeleton
                .in_set(OnUpdate(GeneratorStage::PlaceRooms)),
            generator::build_dungeons.in_set(OnUpdate(GeneratorStage::BuildDungeonRooms)),
            self::spawn_some_weapons.in_schedule(OnEnter(GeneratorStage::Finished)),
            self::regeneration_system.in_set(OnUpdate(GeneratorStage::Finished)),
        ));
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Resource, Default, Reflect)]
pub enum GeneratorStage {
    #[default]
    NoDungeon,
    Initialization,
    CreateMap,
    PlaceRooms,
    GenerateConnections,
    BuildDungeonRooms,
    Finished,
}

#[derive(Debug, Clone, Resource, Default, Reflect, FromReflect)]
pub struct DungeonGeneratorSettings {
    dungeon_room_amount: i32,
    grid_too_room_ratio: f32,
    dungeons_space_between: f32,
    #[reflect(ignore)]
    useable_rooms: Option<HashMap<RoomID, LdtkLevel>>,
}

fn spawn_some_weapons(mut ew: EventWriter<SpawnWeaponEvent>) {
    ew.send(SpawnWeaponEvent {
        weapon_to_spawn: WeaponType::SmallSMG,
        spawn_position: vec3(255.0, 257.0, ACTOR_Z_INDEX),
        spawn_count: 1,
    });

    ew.send(SpawnWeaponEvent {
        weapon_to_spawn: WeaponType::SmallPistol,
        spawn_position: vec3(500.0, 257.0, ACTOR_Z_INDEX),
        spawn_count: 1,
    });
}

fn regeneration_system(
    mut cmds: Commands,
    query_action_state: Query<&ActionState<actions::Combat>>,
    dungeon_container: Query<Entity, &DungeonContainerTag>,
) {
    let input = query_action_state.single();
    let dungeon = dungeon_container.single();

    if input.pressed(actions::Combat::DebugF2) {
        info!("regenerate dungeon pressed");
        cmds.entity(dungeon).despawn_recursive();
        cmds.insert_resource(NextState(Some(GeneratorStage::Initialization)));
    }
}

// we can probably create a plugin for this,
// TODO: use physics for level placement, this may give some weirdness but it should be doable
// spawn points equal to Max dungeon amount
// assign levels and colliders to points, give colliders size equal to map width/height whichever is greater,
// link levels together and try to pull them onto the center using joints/springs from rapier,
// when level velocity is less than a certain number, remove physics components and clamp them too the nearest multiple of 32px
