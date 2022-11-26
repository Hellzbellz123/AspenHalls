use bevy::prelude::{App, Plugin, SystemSet};

use crate::game::GameStage;

use self::animation::GraphicsPlugin;

pub mod animation;
// pub mod components;
pub mod enemies;
pub mod player;
pub mod spawners;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameStage::Playing)
                .with_system(Self::update_current_animation)
                .with_system(Self::frame_animation),
        );
    }
}
