// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(lint_reasons)]

use audio::{Ambience, Music, Sound, SoundSettings};
use bevy::prelude::*;
use bevy::render::texture::ImageSettings;
use bevy::DefaultPlugins;

use bevy_inspector_egui::Inspectable;
use bevy_kira_audio::AudioChannel;
use bevy_kira_audio::AudioControl;
// use dev_tools::{debug_dirs, DebugPlugin};
use game::TimeInfo;
use heron::PhysicsLayer;
use loading::AssetLoadPlugin;
use splashscreen::SplashPlugin;
use utilities::UtilitiesPlugin;

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

#[derive(Inspectable)]
pub struct AppSettings {
    sound_settings: SoundSettings,
    resolution: Vec2,
    // control_settings: PlayerInput,
}

//TODO: default app settings if its a setting it goes here, move this too settings plugin
impl FromWorld for AppSettings {
    fn from_world(_: &mut World) -> Self {
        AppSettings {
            sound_settings: SoundSettings {
                mastervolume: 0.5,
                ambiencevolume: 0.5,
                musicvolume: 0.5,
                soundvolume: 0.5,
            },
            resolution: Vec2 {
                x: 1200.0,
                y: 800.0,
            },
        }
    }
}

pub const TILE_SIZE: Vec2 = Vec2 { x: 32.0, y: 32.0 };
pub const PLAYER_SIZE: Vec2 = Vec2::new(TILE_SIZE.x, TILE_SIZE.y * 2.0);

#[derive(PhysicsLayer)]
enum Layer {
    World,
    Player,
    // Enemies,
}

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
        .add_plugins(DefaultPlugins)
        .add_plugin(UtilitiesPlugin)
        .add_state(game::GameStage::Loading)
        .add_plugin(AssetLoadPlugin)
        .insert_resource(TimeInfo {
            time_step: 0.0,
            game_paused: true,
            pause_menu: false,
        })
        .add_plugin(SplashPlugin)
        .add_plugin(game::GamePlugin)
        .add_plugin(dev_tools::DebugPlugin)
        .add_system(update_settings)
        .run();
}

//TODO: move this to loading plugin
fn update_settings(
    mut windows: ResMut<Windows>,
    settings: Res<AppSettings>,
    bgm: Res<AudioChannel<Music>>,
    bga: Res<AudioChannel<Ambience>>,
    bgs: Res<AudioChannel<Sound>>,
) {
    let window = windows.primary_mut();
    if settings.is_changed() {
        window.set_resolution(settings.resolution.x, settings.resolution.y);

        let sound_settings = &settings.sound_settings;
        info!("volumes changed, applying settings");
        bgm.set_volume(sound_settings.musicvolume * sound_settings.mastervolume);
        bga.set_volume(sound_settings.ambiencevolume * sound_settings.mastervolume);
        bgs.set_volume(sound_settings.soundvolume * sound_settings.mastervolume);
    }
}
