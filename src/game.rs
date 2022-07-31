use bevy::{
    app::App,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};

use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use std::time::Duration;

use crate::{
    action_manager::bindings::ActionsPlugin,
    audio::InternalAudioPlugin,
    characters::player::{PlayerComponent, PlayerPlugin},
    loading::LoadingPlugin,
    splashscreen::SplashPlugin,
    ui::MenuPlugin,
};

#[derive(Debug, Clone, PartialEq, Component, Inspectable)]
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
        let _registry = app
            .world
            .get_resource_or_insert_with(bevy_inspector_egui::InspectableRegistry::default);

        app.add_plugin(LoadingPlugin)
            // .insert_resource(GamePaused::Paused)
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
            .register_inspectable::<PlayerComponent>()
            .register_inspectable::<TimeInfo>() // tells bevy-inspector-egui how to display the struct in the world inspector
            .add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(LogDiagnosticsPlugin {
                wait_duration: Duration::from_secs(20),
                ..Default::default()
            })
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
