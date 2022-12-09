use bevy::prelude::{info, Plugin, SystemSet};
use bevy_ecs_ldtk::prelude::RegisterLdtkObjects;

use crate::{
    game::GameStage,
    game_world::homeworld::{
        map_components::{LdtkCollisionBundle, LdtkSensorBundle},
        systems::{enter_the_dungeon, homeworld_teleport},
    },
};

pub mod map_components;
pub mod systems;

pub struct PlayerTeleportEvent;
pub struct HomeWorldPlugin;

impl Plugin for HomeWorldPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        info!("registering ldtk map cells and adding teleport event");
        app
            // .register_ldtk_int_cell_for_layer::<RapierSensorBundle>("Physics_Layer", 2)
            .register_ldtk_int_cell_for_layer_optional::<LdtkCollisionBundle>(
                Some("Collision_Layer".to_string()),
                None,
            )
            .register_ldtk_entity::<LdtkSensorBundle>("TeleportSensor")
            .add_event::<PlayerTeleportEvent>()
            .add_system_set(
                SystemSet::on_enter(GameStage::Playing)
                    .with_system(systems::spawn_mapbundle) //TODO: Change back to menu when kayakui new menu is done
                    .with_system(systems::spawn_level_0),
            )
            .add_system_set(
                SystemSet::on_update(GameStage::Playing)
                    .with_system(homeworld_teleport)
                    .with_system(enter_the_dungeon),
            );
    }
}
