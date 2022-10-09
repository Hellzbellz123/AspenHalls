use bevy::prelude::*;
use bevy_ecs_ldtk::LdtkPlugin;

use crate::game::GameStage;

use self::homeworld::systems::{homeworld_teleport, enter_the_dungeon};

pub mod homeworld;
pub mod world_components;

pub struct MapSystemPlugin;

impl Plugin for MapSystemPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugin(LdtkPlugin)
            .add_plugin(homeworld::HomeWorldPlugin)
            .add_system_set(
                SystemSet::on_update(GameStage::Playing).with_system(homeworld_teleport).with_system(enter_the_dungeon),
            );
    }
}
