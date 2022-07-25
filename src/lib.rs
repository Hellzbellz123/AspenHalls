mod action_manager;
mod audio;
mod loading;
mod menu;
mod player;
mod splashscreen;

use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::main_menu::*;
use crate::player::PlayerPlugin;
use crate::splashscreen::splash::SplashPlugin;
use action_manager::bindings::ActionsPlugin;
use action_manager::gamepad::GamepadPlugin;
use bevy::app::App;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use menu::MenuPlugin;
use std::time::Duration;
// use kayak_ui::bevy::BevyKayakUIPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Component, Inspectable)]
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

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum GamePaused {
    Paused,
    Unpaused,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Component, Inspectable)]
pub enum MenuState {
    MainMenu,
    Options,
    Play,
}

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        let _registry = app
            .world
            .get_resource_or_insert_with(bevy_inspector_egui::InspectableRegistry::default);

        app.add_plugin(LoadingPlugin)
            .insert_resource(GamePaused::Paused)
            .add_plugin(SplashPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(GamepadPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(PlayerPlugin)
            .register_inspectable::<player::Player>() // tells bevy-inspector-egui how to display the struct in the world inspector
            .add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(LogDiagnosticsPlugin {
                wait_duration: Duration::from_secs(20),
                ..Default::default()
            })
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(unpause_game));
    }
}

fn unpause_game(mut paused: ResMut<GamePaused>) {
    *paused = GamePaused::Unpaused;
}
