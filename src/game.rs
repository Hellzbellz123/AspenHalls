use bevy::{app::App, prelude::*};

use bevy_inspector_egui::Inspectable;

use crate::{
    action_manager::bindings::ActionsPlugin, audio::InternalAudioPlugin,
    characters::player::PlayerPlugin, loading::LoadingPlugin, splashscreen::SplashPlugin,
    ui::MenuPlugin,
};

#[derive(Debug, Clone, PartialEq, Component, Inspectable, Reflect)]
pub struct TimeInfo {
    pub time_step: f32,
    pub game_paused: bool,
    pub pause_menu: bool,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Component, Inspectable)]
pub enum GameStage {
    //splash
    Splash,
    // During the loading State the LoadingPlugin will load our assets and display splash?!
    Loading,
    // Here the menu is drawn and waiting for player interaction
    Menu,
    // During this State the actual game logic is executed
    Playing,
}

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LoadingPlugin)
            .insert_resource(TimeInfo {
                time_step: 0.0,
                game_paused: true,
                pause_menu: false,
            })
            .add_plugin(SplashPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(PlayerPlugin)
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
