use bevy::prelude::*;
use bevy_ecs_ldtk::LdtkPlugin;

pub mod components;
pub mod homeworld;

pub struct MapSystemPlugin;

impl Plugin for MapSystemPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugin(LdtkPlugin)
            .add_plugin(homeworld::HomeWorldPlugin);
    }
}
