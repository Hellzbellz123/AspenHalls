use bevy::prelude::*;

mod generator;
// mod utils;

pub struct DungeonGeneratorPlugin;

impl Plugin for DungeonGeneratorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_state::<GeneratorStage>()
            .insert_resource(DungeonGeneratorSettings {
                dungeon_room_amount: 20,
                dungeon_map_origin: Vec3::ZERO,
                dungeon_map_halfextent: 2000.0,
                dungeons_space_between: 600.0,
            })
            .add_systems((
                generator::create_dungeon_container
                    .in_set(OnUpdate(GeneratorStage::Initialization)),
                generator::place_dungeon_skeleton.in_set(OnUpdate(GeneratorStage::PlaceRooms)),
                generator::build_dungeons.in_set(OnUpdate(GeneratorStage::BuildDungeonRooms)),
            ));
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Resource, Default, Reflect)]
pub enum GeneratorStage {
    #[default]
    NoDungeon,
    Initialization,
    PlaceRooms,
    GenerateConnections,
    BuildDungeonRooms,
    DoneGenerating,
}

#[derive(Debug, Clone, Resource, Default, Reflect)]
pub struct DungeonGeneratorSettings {
    dungeon_room_amount: i32,
    dungeon_map_origin: Vec3,
    dungeon_map_halfextent: f32,
    dungeons_space_between: f32,
}
