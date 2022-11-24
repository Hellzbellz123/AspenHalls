use bevy::prelude::{info, Plugin, SystemSet};
use bevy_ecs_ldtk::prelude::RegisterLdtkObjects;

use crate::{
    game::GameStage,
    game_world::homeworld::{
        components::{RapierCollisionBundle, RapierSensorBundle},
        systems::{enter_the_dungeon, homeworld_teleport},
    },
};

pub mod components;
pub mod systems;

pub struct PlayerTeleportEvent;
pub struct HomeWorldPlugin;

impl Plugin for HomeWorldPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        info!("spawning ldtklevels");
        app.register_ldtk_int_cell_for_layer::<RapierCollisionBundle>("CollisionGrid", 1)
            .register_ldtk_int_cell_for_layer::<RapierSensorBundle>("CollisionGrid", 2)
            .add_event::<PlayerTeleportEvent>()
            .add_system_set(
                SystemSet::on_enter(GameStage::Playing).with_system(systems::spawn_mapbundle), //TODO: Change back to menu when kayakui new menu is done
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
