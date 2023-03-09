// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(stmt_expr_attributes)]
#![feature(type_ascription)]
#![feature(lint_reasons)]
// #![forbid(missing_docs)]
#![allow(clippy::module_name_repetitions)]

// #![allow(dead_code)]
use audio::{Ambience, Music, Sound};

use bevy::log::LogPlugin;
use bevy::prelude::{
    default, info, warn, AssetPlugin, Camera2d, ClearColor, Color, DetectChanges, ImagePlugin,
    OrthographicProjection, PluginGroup, Vec2,
};
use bevy::prelude::{App, Query, Res, ResMut, With};
use bevy::window::{
    PresentMode, Window, WindowMode, WindowPlugin, WindowPosition, WindowResizeConstraints,
};

use bevy_framepace::{FramepaceSettings, Limiter};
use bevy_kira_audio::{AudioChannel, AudioControl};
use bevy_prototype_lyon::prelude::ShapePlugin;
use bevy_rapier2d::prelude::{NoUserData, RapierConfiguration, RapierPhysicsPlugin};

// use crate::console::VCConsolePlugin;
// use crate::console::VCConsolePlugin;
use crate::dev_tools::debug_plugin::debug_dump_graphs;
#[cfg(feature = "dev")]
use crate::dev_tools::debug_plugin::DebugPlugin;
use crate::utilities::logging::VCLogPlugin;

use components::MainCameraTag;
use game::TimeInfo;
use utilities::game::AppSettings;

/// module holds buttons that can be pressed
pub mod actions;
/// module holds actors (entitys that have spatialbundles and that can affect gameplay)
pub mod actors;
/// module for all game audio, internal audio plugin handles all sound
pub mod audio;
pub mod components;
// pub mod console;
mod dev_tools;
pub mod game;
pub mod game_world;
pub mod loading;
// pub mod kayak_ui;
pub mod bevy_ui;
pub mod utilities;

/// this translates too same folder as executable
pub const APP_SETTINGS_PATH: &str = "./config.toml";

fn main() {
    let mut vanillacoffee = App::new();

    vanillacoffee.add_state::<game::GameStage>();
    vanillacoffee.add_plugin(VCLogPlugin {
        // filter: "".into(),
        filter: "bevy_ecs=warn,naga=error,wgpu_core=error,wgpu_hal=error,symphonia=warn".into(), // filters for anything that makies it through the default log level. quiet big loggers
        level: bevy::log::Level::DEBUG,
    });
    info!("Starting Game");

    info!("Loading Game Settings from file and inserting settings");
    let settings: AppSettings = utilities::load_settings();
    vanillacoffee.world.insert_resource(settings);

    // add bevy plugins
    vanillacoffee
        .add_plugins(
            bevy::DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: if settings.vsync {
                            PresentMode::AutoVsync
                        } else {
                            PresentMode::AutoNoVsync
                        },
                        position: WindowPosition::Automatic,
                        title: "Vanilla Coffee".to_string(),
                        resize_constraints: WindowResizeConstraints {
                            min_width: 300.0,
                            min_height: 200.0,
                            ..default()
                        },
                        resolution: (settings.resolution.x, settings.resolution.y).into(),
                        mode: {
                            if settings.fullscreen {
                                // if fullscreen is true, use borderless fullscreen
                                // cursor mode is confined to the window so it cant
                                // leave without alt tab
                                WindowMode::BorderlessFullscreen
                            } else {
                                WindowMode::Windowed
                            }
                        },
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    asset_folder: "gamedata".to_string(),
                    watch_for_changes: false,
                })
                // .set(LogPlugin {
                //     filter: "trace".into(),
                //     level: bevy::log::Level::TRACE,
                // })
                .disable::<LogPlugin>(),
        )
        .insert_resource(ClearColor(Color::Hsla {
            hue: 294.0,
            saturation: 0.71,
            lightness: 0.08,
            alpha: 1.0,
        }));

    // add third party plugins
    vanillacoffee
        .add_plugin(bevy_framepace::FramepacePlugin)
        .add_plugin(ShapePlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(16.0))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })
        .insert_resource(FramepaceSettings {
            limiter: Limiter::from_framerate(settings.frame_rate_target),
        });

    // add vanillacoffee plugins
    vanillacoffee
        //TODO: break all settings out into plugin that loads settings from disk, if settings dont exist create them
        // use commmands from args too insert windowdescriptor with default if no file exists but use whats in file if its thier.
        .insert_resource(TimeInfo {
            time_step: 1.0, //TODO: change this back too false and 0.0 when we get the mainmenu back
            game_paused: false,
            pause_menu: false,
        })
        .add_plugin(loading::AssetLoadPlugin)
        .add_plugin(bevy_ui::BevyUiPlugin)
        // .add_plugin(kayak_ui::UIPlugin)
        // .add_plugin(VCConsolePlugin)
        .add_plugin(utilities::UtilitiesPlugin)
        .add_plugin(game::GamePlugin)
        .add_system(update_settings);

    #[cfg(feature = "dev")]
    vanillacoffee.add_plugin(DebugPlugin);

    #[cfg(feature = "dev")]
    debug_dump_graphs(&mut vanillacoffee);
    #[cfg(feature = "dev")]
    warn!("Dumping graphs");

    vanillacoffee.run();
}

//TODO: move this to loading plugin and only run it when the settings resource changes, or on game load.
// (system ordering is imporatant here) the camera needs to be spawned first or we get a panic
// #[bevycheck::system]
fn update_settings(
    settings: Res<AppSettings>,
    mut framelimiter: ResMut<FramepaceSettings>,
    mut windows: Query<&mut Window>,
    bgm: Res<AudioChannel<Music>>,
    bga: Res<AudioChannel<Ambience>>,
    bgs: Res<AudioChannel<Sound>>,
    mut camera: Query<(&mut OrthographicProjection, &Camera2d), With<MainCameraTag>>,
) {
    if camera.is_empty() //{
        || !settings.is_changed()
    {
        return;
    }

    framelimiter.limiter = Limiter::from_framerate(60.0);

    let mut window = windows.get_single_mut().expect("one window only");
    camera.get_single_mut().expect("no camera?").0.scale = settings.camera_zoom;

    window
        .resolution
        .set(settings.resolution.x, settings.resolution.y);
    //camera zoom
    //sound settings
    info!("volumes changed, applying settings");
    let mastervolume = &settings.sound_settings.mastervolume;
    bgm.set_volume(settings.sound_settings.musicvolume * mastervolume);
    bga.set_volume(settings.sound_settings.ambiencevolume * mastervolume);
    bgs.set_volume(settings.sound_settings.soundvolume * mastervolume);
}
