mod load_file;

use bevy::{
    log::LogPlugin,
    prelude::*,
    window::{PresentMode, WindowMode, WindowResized, WindowResolution},
};
use bevy_framepace::{FramepaceSettings, Limiter};
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};
use bevy_kira_audio::{AudioChannel, AudioControl};
use serde::{Deserialize, Serialize};
use winit::dpi::LogicalSize;

use crate::{
    audio::{Ambience, Music, Sound},
    components::MainCameraTag,
    utilities::logging::VCLogPlugin,
};

#[derive(Reflect, Resource, Serialize, Deserialize, Copy, Clone, Default)]
#[reflect(Resource)]
pub struct ConfigFile {
    window_settings: WindowSettings,
    sound_settings: SoundSettings,
    general_settings: GeneralSettings,
}

/// make sure tables are AFTER single fields
#[derive(Reflect, Resource, InspectorOptions, Serialize, Deserialize, Copy, Clone)]
#[reflect(Resource, InspectorOptions)]
pub struct WindowSettings {
    /// enable vsync if true
    pub vsync: bool,
    /// framerate
    pub frame_rate_target: f64,
    // fullscreen yes/no
    pub fullscreen: bool,
    /// display resolution
    pub resolution: Vec2,
}

#[derive(Reflect, Resource, InspectorOptions, Serialize, Deserialize, Copy, Clone)]
#[reflect(Resource, InspectorOptions)]
pub struct GeneralSettings {
    /// camera zooom
    #[inspector(min = 0.0, max = 50.0)]
    pub camera_zoom: f32,
}

/// modify to change sound volume settings
#[derive(Reflect, InspectorOptions, Debug, Serialize, Deserialize, Resource, Copy, Clone)]
#[reflect(Resource, InspectorOptions)]
pub struct SoundSettings {
    #[inspector(min = 0.0, max = 1.0)]
    pub mastervolume: f64,
    #[inspector(min = 0.0, max = 1.0)]
    pub ambiencevolume: f64,
    #[inspector(min = 0.0, max = 1.0)]
    pub musicvolume: f64,
    #[inspector(min = 0.0, max = 1.0)]
    pub soundvolume: f64,
}

impl Default for GeneralSettings {
    fn default() -> Self {
        GeneralSettings { camera_zoom: 1.0 }
    }
}

//TODO: default app settings if its a setting it goes here, move this too settings plugin
impl Default for WindowSettings {
    fn default() -> Self {
        WindowSettings {
            vsync: true,
            frame_rate_target: 144.0,
            fullscreen: false,
            resolution: Vec2 {
                x: 1200.0,
                y: 720.0,
            },
        }
    }
}

impl Default for SoundSettings {
    fn default() -> Self {
        SoundSettings {
            mastervolume: 1.0,
            ambiencevolume: 0.3,
            musicvolume: 0.2,
            soundvolume: 0.7,
        }
    }
}

pub struct InitAppPlugin;

impl Plugin for InitAppPlugin {
    fn build(&self, app: &mut App) {
        let settings = load_file::load_settings();

        app.add_plugins(
            bevy::DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: if settings.window_settings.vsync {
                            PresentMode::AutoVsync
                        } else {
                            PresentMode::AutoNoVsync
                        },
                        position: WindowPosition::Automatic,
                        title: "Vanilla Coffee".to_string(),
                        resolution: WindowResolution::new(
                            settings.window_settings.resolution.x,
                            settings.window_settings.resolution.y,
                        ),
                        mode: {
                            if settings.window_settings.fullscreen {
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
        }))
        .insert_resource(settings.window_settings)
        .insert_resource(settings.sound_settings)
        .insert_resource(settings.general_settings);

        app.add_systems((
            apply_window_settings,
            apply_sound_settings,
            apply_general_settings,
            on_resize_system,
        ));
    }
}

pub fn configure_and_build() -> App {
    let mut vanillacoffee = App::new();

    vanillacoffee.add_plugin(VCLogPlugin {
        // filters for anything that makies it through the default log level. quiet big loggers
        // filter: "".into(), // an empty filter
        filter: "bevy_ecs=warn,naga=error,wgpu_core=error,wgpu_hal=error,symphonia=warn".into(),
        level: bevy::log::Level::DEBUG,
    });

    info!("Logging Initialized");

    // add bevy plugins
    vanillacoffee.add_plugin(InitAppPlugin);
    vanillacoffee
}

//TODO: move this to loading plugin and only run it when the settings resource changes (clicking apply in the settings menu, or reacting to OS changes), or on game load.
// (system ordering is imporatant here) the camera needs to be spawned first or we get a panic
// #[bevycheck::system]
fn apply_window_settings(
    window_settings: Res<WindowSettings>,
    mut framelimiter: ResMut<FramepaceSettings>,
    winitwindows: NonSend<bevy::winit::WinitWindows>,
    mutwindowent: Query<(Entity, &Window)>,
) {
    if window_settings.is_changed() {
        framelimiter.limiter = Limiter::from_framerate(window_settings.frame_rate_target);

        info!(
            "window resolution before update system runs {}, {}",
            window_settings.resolution.x, window_settings.resolution.y
        );

        let winitwindow = winitwindows
            .get_window(mutwindowent.single().0)
            .expect("always one window");

        winitwindow.set_inner_size(LogicalSize::new(
            window_settings.resolution.x,
            window_settings.resolution.y,
        ));
    }
}

fn apply_sound_settings(
    sound_settings: Res<SoundSettings>,
    bgm: Res<AudioChannel<Music>>,
    bga: Res<AudioChannel<Ambience>>,
    bgs: Res<AudioChannel<Sound>>,
) {
    if sound_settings.is_changed() {
        //sound settings
        info!("volumes changed, applying settings");
        let mastervolume = sound_settings.mastervolume;
        bgm.set_volume(sound_settings.musicvolume * mastervolume);
        bga.set_volume(sound_settings.ambiencevolume * mastervolume);
        bgs.set_volume(sound_settings.soundvolume * mastervolume);
    }
}

fn apply_general_settings(
    general_settings: Res<GeneralSettings>,
    mut camera: Query<(&mut OrthographicProjection, &Camera2d), With<MainCameraTag>>,
) {
    if camera.is_empty() {
        return;
    }

    if general_settings.is_changed() {
        //camera zoom
        camera.get_single_mut().expect("no camera?").0.scale = general_settings.camera_zoom;
    }
}

fn on_resize_system(
    mut settings: ResMut<WindowSettings>,
    mut resize_reader: EventReader<WindowResized>,
) {
    for resize_event in resize_reader.iter() {
        settings.resolution.x = resize_event.width;
        settings.resolution.y = resize_event.height;
    }
}
