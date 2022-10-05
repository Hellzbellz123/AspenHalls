// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(lint_reasons)]

use audio::{Ambience, Music, Sound};
use bevy::prelude::{Camera2d, OrthographicProjection, ParallelSystemDescriptorCoercion};
use bevy::{
    prelude::{info, App, ClearColor, Color, Query, Res, ResMut, With},
    render::texture::ImageSettings,
    window::{WindowDescriptor, Windows},
};
use bevy_kira_audio::{AudioChannel, AudioControl};

use game::TimeInfo;

use splashscreen::MainCamera;
use utilities::game::{AppSettings, SystemLabels};

pub mod action_manager;
pub mod actors;
pub mod audio;
mod dev_tools;
pub mod game;
pub mod game_world;
pub mod loading;
pub mod splashscreen;
pub mod ui;
pub mod utilities;

pub fn main() {
    App::new()
        //TODO: break all settings out into plugin that loads settings from disk, if settings dont exist create them
        // use commmands from args too insert windowdescriptor with default if no file exists but use whats in file if its thier.
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(ImageSettings::default_nearest())
        .insert_resource(WindowDescriptor {
            width: 1200.0,
            height: 800.0,
            title: "Project Kira".to_string(), // ToDo
            ..Default::default()
        })
        .add_plugins(bevy::DefaultPlugins)
        .add_plugin(utilities::UtilitiesPlugin)
        .add_state(game::GameStage::Loading)
        .add_plugin(loading::AssetLoadPlugin)
        .insert_resource(TimeInfo {
            time_step: 0.0,
            game_paused: true,
            pause_menu: false,
        })
        .add_plugin(splashscreen::SplashPlugin)
        .add_plugin(game::GamePlugin)
        .add_plugin(dev_tools::DebugPlugin)
        .add_system(update_settings.after(SystemLabels::InitSettings))
        //     SystemSet::on_enter(GameStage::Splash)
        //         .with_system(update_settings.label(SystemLabels::UpdateSettings)),
        // )
        .run();
}

//TODO: move this to loading plugin and only run it when the settings resource changes, or on game load.
// (system ordering is imporatant here) the camera needs to be spawned first or we get a panic
fn update_settings(
    mut windows: ResMut<Windows>,
    settings: Res<AppSettings>,
    bgm: Res<AudioChannel<Music>>,
    bga: Res<AudioChannel<Ambience>>,
    bgs: Res<AudioChannel<Sound>>,
    mut camera: Query<(&mut OrthographicProjection, &Camera2d, With<MainCamera>)>,
) {
    let window = windows.primary_mut();

    if !camera.is_empty() {
        camera.get_single_mut().expect("no camera?").0.scale = settings.camera_zoom;
    }

    if settings.is_changed() {
        window.set_resolution(settings.resolution.x, settings.resolution.y);
        //camera zoom
        //sound settings
        info!("volumes changed, applying settings");
        let mastervolume = &settings.sound_settings.mastervolume;
        bgm.set_volume(settings.sound_settings.musicvolume * mastervolume);
        bga.set_volume(settings.sound_settings.ambiencevolume * mastervolume);
        bgs.set_volume(settings.sound_settings.soundvolume * mastervolume);
    }
}
