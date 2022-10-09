use bevy::prelude::{Plugin, SystemSet};
use bevy_ecs_ldtk::prelude::RegisterLdtkObjects;

use crate::game::GameStage;

use self::{
    components::HeronCollisonBundle,
    systems::{enter_the_dungeon, homeworld_teleport},
};

pub mod components;
pub mod systems;

pub struct PlayerTeleportEvent;
pub struct HomeWorldPlugin;

impl Plugin for HomeWorldPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.register_ldtk_int_cell_for_layer::<HeronCollisonBundle>("CollisionGrid", 1)
            .register_ldtk_int_cell_for_layer::<HeronCollisonBundle>("CollisionGrid", 2)
            .add_event::<PlayerTeleportEvent>()
            .add_system_set(
                SystemSet::on_enter(GameStage::Menu).with_system(systems::spawn_mapbundle),
            )
            .add_system_set(
                SystemSet::on_enter(GameStage::Playing).with_system(systems::spawn_level_0),
            )
            .add_system_set(
                SystemSet::on_update(GameStage::Playing)
                    .with_system(homeworld_teleport)
                    .with_system(enter_the_dungeon),
            );
    }
}
