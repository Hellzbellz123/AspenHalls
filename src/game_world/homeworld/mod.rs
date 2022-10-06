use bevy::prelude::{Plugin, SystemSet};
use bevy_ecs_ldtk::{
    prelude::RegisterLdtkObjects, IntGridRendering, LdtkSettings, LevelBackground,
    LevelSpawnBehavior, SetClearColor,
};

use crate::game::GameStage;

use super::world_components::{HeronBundles};

pub mod components;
pub mod systems;

pub struct HomeWorldPlugin;

impl Plugin for HomeWorldPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app
            .register_ldtk_int_cell_for_layer::<HeronBundles>("CollisionGrid", 1)
            .register_ldtk_int_cell_for_layer::<HeronBundles>("CollisionGrid", 2)
            .insert_resource(LdtkSettings {
                level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                    load_level_neighbors: true,
                },
                set_clear_color: SetClearColor::No,
                int_grid_rendering: IntGridRendering::Invisible,
                level_background: LevelBackground::Nonexistent,
            })
            .add_system_set(
                SystemSet::on_enter(GameStage::Menu).with_system(systems::spawn_mapbundle),
            )
            .add_system_set(
                SystemSet::on_enter(GameStage::Playing).with_system(systems::spawn_level_0),
            );
    }
}
