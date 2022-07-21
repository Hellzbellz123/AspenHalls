mod action_manager;
mod audio;
mod loading;
mod menu;
mod player;
mod splashscreen;

use crate::action_manager::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::main_menu::*;
use crate::player::PlayerPlugin;
use crate::splashscreen::splash::SplashPlugin;
use bevy::app::App;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use std::time::Duration;
// use kayak_ui::bevy::BevyKayakUIPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Component)]
pub enum GameState {
    //splash
    Splash,
    // During the loading State the LoadingPlugin will load our assets and display splash?!
    Loading,
    // Here the menu is drawn and waiting for player interaction
    Menu,
    // During this State the actual game logic is executed
    Playing,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Component)]
pub enum MenuState {
    MainMenu,
    Options,
    Play,
}

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LoadingPlugin)
            .add_plugin(SplashPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(LogDiagnosticsPlugin {
                wait_duration: Duration::from_secs(20),
                ..Default::default()
            });
    }
}
