use std::collections::HashMap;

use bevy::{math::vec3, prelude::*};
use bevy_ecs_ldtk::LdtkLevel;

use crate::{
    components::actors::spawners::{SpawnWeaponEvent, WeaponType},
    consts::ACTOR_Z_INDEX,
};

use self::generator::{RoomID, RoomInstance};

mod generator;
mod utils;
// mod test;
// mod grid2d;
// mod utils;

pub struct DungeonGeneratorPlugin;

impl Plugin for DungeonGeneratorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(DungeonGeneratorSettings {
            dungeon_room_amount: 30,
            dungeon_map_origin: Vec3::ZERO,
            dungeon_map_halfextent: 1000.0,
            dungeons_space_between: 100.0,
            useable_rooms: None,
        })
        .register_type::<RoomInstance>()
        .add_systems((
            generator::setup_dungeon_environment.in_set(OnUpdate(GeneratorStage::Initialization)),
            generator::create_dungeons_list.in_set(OnUpdate(GeneratorStage::Initialization)),
            generator::layout_dungeon_and_place_skeleton
                .in_set(OnUpdate(GeneratorStage::PlaceRooms)),
            generator::build_dungeons.in_set(OnUpdate(GeneratorStage::BuildDungeonRooms)),
            self::spawn_some_weapons.in_schedule(OnEnter(GeneratorStage::Finished)),
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

#[derive(Debug, Clone, Resource, Default, Reflect)]
pub struct DungeonGeneratorSettings {
    dungeon_room_amount: i32,
    dungeon_map_origin: Vec3,
    dungeon_map_halfextent: f32,
    dungeons_space_between: f32,
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

// we can probably create a plugin for this,
// TODO: use physics for level placement, this may give some weirdness but it should be doable
// spawn points equal to Max dungeon amount
// assign levels and colliders to points, give colliders size equal to map width/height whichever is greater,
// link levels together and try to pull them onto the center using joints/springs from rapier,
// when level velocity is less than a certain number, remove physics components and clamp them too the nearest multiple of 32px
