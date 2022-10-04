use crate::game::GameStage;
use bevy::prelude::{App, Plugin, SystemSet, *};

mod skeleton;

const MAX_ENEMIES: i32 = 20;
const CAN_SPAWN: bool = true;

fn on_enter() {
    info!("this only runs when switching to gamestage::playing, setup enemys here")
}

fn on_update() {
    // info!("this runs every frame in gamestage::playing \"sorta\" ");
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameStage::Playing).with_system(on_enter))
            .add_system_set(SystemSet::on_update(GameStage::Playing).with_system(on_update));
    }
}
