use bevy::prelude::{Plugin, SystemSet};
use bevy_ecs_ldtk::{
    prelude::RegisterLdtkObjects, IntGridRendering, LdtkPlugin, LdtkSettings, LevelBackground,
    LevelSpawnBehavior, SetClearColor,
};

use crate::game::GameStage;

use super::world_components::ColliderBundle;

pub mod components;
pub mod ldtk;

pub struct HomeWorldPlugin;

impl Plugin for HomeWorldPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugin(LdtkPlugin)
            .register_ldtk_int_cell_for_layer::<ColliderBundle>("CollisionGrid", 1)
            .insert_resource(LdtkSettings {
                level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                    load_level_neighbors: true,
                },
                set_clear_color: SetClearColor::No,
                int_grid_rendering: IntGridRendering::Invisible,
                level_background: LevelBackground::Nonexistent,
            })
            .add_system_set(
                SystemSet::on_enter(GameStage::Menu)
                    .with_system(ldtk::spawn_mapbundle)
            )
            .add_system_set(
                SystemSet::on_enter(GameStage::Playing)
                    .with_system(ldtk::spawn_level_0),
            )
            .add_system(ldtk::name_colliders);
    }
}
