use bevy::prelude::*;
use bevy_ecs_ldtk::LdtkPlugin;

pub mod components;
mod dungeon_generator;
pub mod sanctuary;

pub struct GameWorldPlugin;

impl Plugin for GameWorldPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugin(LdtkPlugin)
            .add_plugin(sanctuary::HideOutPlugin)
            .add_plugin(dungeon_generator::DungeonGeneratorPlugin);
    }
}
