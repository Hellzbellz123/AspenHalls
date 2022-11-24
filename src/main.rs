// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(stmt_expr_attributes)]
#![feature(lint_reasons)]

use audio::{Ambience, Music, Sound};
use bevy::prelude::{default, Camera2d, ImagePlugin, OrthographicProjection, PluginGroup, Vec2};
use bevy::window::{
    MonitorSelection, PresentMode, WindowPlugin, WindowPosition, WindowResizeConstraints,
};
use bevy::{
    prelude::{info, App, Query, Res, ResMut, With},
    window::{WindowDescriptor, Windows},
};
use bevy_kira_audio::{AudioChannel, AudioControl};
use bevy_rapier2d::prelude::RapierPhysicsPlugin;
use bevy_rapier2d::prelude::{NoUserData, RapierConfiguration};

#[cfg(feature = "dev")]
use crate::dev_tools::debug_plugin::DebugPlugin;

use game::TimeInfo;

use splashscreen::MainCamera;
use utilities::game::AppSettings;

pub mod action_manager;
pub mod actors;
pub mod audio;
mod dev_tools;
pub mod game;
pub mod game_world;
pub mod loading;
pub mod splashscreen;
// pub mod ui;
pub mod utilities;

pub fn main() {
    let mut vanillacoffee = App::new();

    vanillacoffee
        //TODO: break all settings out into plugin that loads settings from disk, if settings dont exist create them
        // use commmands from args too insert windowdescriptor with default if no file exists but use whats in file if its thier.
        .add_plugins(
            bevy::DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        width: 1200.0,
                        height: 800.0,
                        position: WindowPosition::Centered,
                        monitor: MonitorSelection::Primary,
                        resize_constraints: WindowResizeConstraints {
                            min_width: 300.0,
                            min_height: 200.0,
                            ..default()
                        },
                        // scale_factor_override: Some(1.0),
                        title: "Vanilla Coffee".to_string(),
                        present_mode: PresentMode::Fifo,
                        // resizable: true,
                        // decorations: false,
                        // cursor_visible: true,
                        // cursor_grab_mode: bevy::window::CursorGrabMode::Confined,
                        // mode: WindowMode::BorderlessFullscreen,
                        // transparent: false,
                        // alpha_mode: bevy::window::CompositeAlphaMode::PreMultiplied,
                        ..default()
                    },
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(16.0))
        .add_plugin(utilities::UtilitiesPlugin)
        .add_state(game::GameStage::Loading)
        .add_plugin(loading::AssetLoadPlugin)
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })
        .insert_resource(TimeInfo {
            time_step: 0.0,
            game_paused: true,
            pause_menu: false,
        })
        .add_plugin(splashscreen::SplashPlugin)
        .add_plugin(game::GamePlugin)
        .add_system(update_settings);

    #[cfg(feature = "dev")]
    vanillacoffee.add_plugin(DebugPlugin);

    #[cfg(not(feature = "dev"))]
    vanillacoffee.init_resource::<AppSettings>();

    vanillacoffee.run()
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
