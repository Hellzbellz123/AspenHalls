use bevy::{app::App, prelude::*};

use bevy_inspector_egui::Inspectable;
use heron::{Gravity, PhysicsPlugin};

use crate::{
    action_manager::bindings::ActionsPlugin,
    audio::InternalAudioPlugin,
    characters::player::{animation::GraphicsPlugin, PlayerPlugin},
    ui::MenuPlugin,
};

use crate::game_world::MapSystem;

#[derive(Debug, Clone, Component, Reflect)]
pub struct TimeInfo {
    pub time_step: f32,
    pub game_paused: bool,
    pub pause_menu: bool,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Component, Inspectable)]
pub enum GameStage {
    /// During the loading State the [`loading::LoadingPlugin`] will load our assets and display splash?!
    Loading,
    /// game "stage" for loading in splashscreen and spawning camera
    Splash,
    /// Here the menu is drawn and waiting for player interaction
    Menu,
    /// During this State the actual game logic is executed
    Playing,
}

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MenuPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(PhysicsPlugin::default())
            .insert_resource(Gravity::from(Vec3::new(0.0, 0.0, 0.0)))
            .add_plugin(MapSystem)
            .add_plugin(PlayerPlugin)
            .add_plugin(GraphicsPlugin)
            .add_system_set(SystemSet::on_enter(GameStage::Playing).with_system(setup_time_state));
    }
}

pub fn setup_time_state(mut timeinfo: ResMut<TimeInfo>) {
    *timeinfo = TimeInfo {
        time_step: 1.0,
        game_paused: false,
        pause_menu: false,
    }
}
