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

#[derive(Reflect, Resource, Serialize, Deserialize, Clone, Default)]
#[reflect(Resource)]
pub struct ConfigFile {
    window_settings: Box<WindowSettings>,
    sound_settings: Box<SoundSettings>,
    general_settings: Box<GeneralSettings>,
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Reflect)]
pub enum GameDifficulty {
    Easy,
    Medium,
    Hard,
    Insane,
    MegaDeath,
}

#[derive(Reflect, Resource, InspectorOptions, Serialize, Deserialize, Copy, Clone)]
#[reflect(Resource, InspectorOptions)]
pub struct GeneralSettings {
    /// camera zooom
    #[inspector(min = 0.0, max = 50.0)]
    pub camera_zoom: f32,
    /// game difficulty,
    /// value ranging from 1-4, 1 being easiest, 4 being hardest
    pub game_difficulty: GameDifficulty,
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

// TODO: refactor actors module to use this global difficulty resource
// add a system that takes GeneralSettings.difficulty_settings and matches
// that i32 and inserts this configured
#[derive(Reflect, Debug, Serialize, Deserialize, Resource, Copy, Clone)]
#[reflect(Resource)]
pub struct DifficultySettings {
    pub max_enemies: i32,
    pub player_health_modifier: f32,
    pub player_damage_modifier: f32,
    pub enemy_health_modifier: f32,
    pub enemy_damage_modifier: f32,
    pub max_dungeon_amount: i32,
    pub enemy_speed: f32,
    pub player_speed: f32,
}

impl Default for DifficultySettings {
    fn default() -> Self {
        Self {
            max_enemies: 100,
            max_dungeon_amount: 5,
            player_health_modifier: 1.0,
            player_damage_modifier: 1.0,
            enemy_health_modifier: 1.0,
            enemy_damage_modifier: 1.0,
            enemy_speed: 1.0,
            player_speed: 1.0,
        }
    }
}

impl Default for GeneralSettings {
    fn default() -> Self {
        GeneralSettings {
            camera_zoom: 1.0,
            game_difficulty: GameDifficulty::Medium,
        }
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
        let cfg_file = load_file::load_settings();
        let difficulty_settings = DifficultySettings::default();

        app.add_plugins(
            bevy::DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: if cfg_file.window_settings.vsync {
                            PresentMode::AutoVsync
                        } else {
                            PresentMode::AutoNoVsync
                        },
                        position: WindowPosition::Automatic,
                        title: "Vanilla Coffee".to_string(),
                        resolution: WindowResolution::new(
                            cfg_file.window_settings.resolution.x,
                            cfg_file.window_settings.resolution.y,
                        ),
                        mode: {
                            if cfg_file.window_settings.fullscreen {
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
        .insert_resource(*cfg_file.window_settings)
        .insert_resource(*cfg_file.sound_settings)
        .insert_resource(*cfg_file.general_settings)
        .insert_resource(difficulty_settings);

        app.add_systems((
            apply_window_settings,
            apply_sound_settings,
            apply_general_settings,
            update_difficulty_settings,
            on_resize_system,
        ));
    }
}

pub fn configure_and_build() -> App {
    let mut vanillacoffee = App::new();

    vanillacoffee.add_plugin(VCLogPlugin {
        // filters for anything that makies it through the default log level. quiet big loggers
        // filter: "".into(), // an empty filter
        filter:
            "bevy_ecs=warn,naga=error,wgpu_core=error,wgpu_hal=error,symphonia=warn,big_brain=warn"
                .into(),
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

fn update_difficulty_settings(
    general_settings: Res<GeneralSettings>,
    _dif_settings: Res<DifficultySettings>,
    mut cmds: Commands,
) {
    // if !general_settings.is_changed() | !dif_settings.is_added() {
    //     return;
    // }

    let difficulty_settings: DifficultySettings = match general_settings.game_difficulty {
        GameDifficulty::Easy => DifficultySettings {
            max_enemies: 25,
            player_health_modifier: 1.25,
            player_damage_modifier: 1.25,
            enemy_health_modifier: 0.75,
            enemy_damage_modifier: 0.75,
            max_dungeon_amount: 5,
            enemy_speed: 0.9,
            player_speed: 1.2,
        },
        GameDifficulty::Medium => DifficultySettings {
            max_enemies: 50,
            player_health_modifier: 1.00,
            player_damage_modifier: 1.00,
            enemy_health_modifier: 1.0,
            enemy_damage_modifier: 1.0,
            max_dungeon_amount: 7,
            enemy_speed: 1.0,
            player_speed: 1.0,
        },
        GameDifficulty::Hard => DifficultySettings {
            max_enemies: 100,
            player_health_modifier: 1.25,
            player_damage_modifier: 1.25,
            enemy_health_modifier: 1.0,
            enemy_damage_modifier: 1.0,
            max_dungeon_amount: 9,
            enemy_speed: 1.2,
            player_speed: 1.0,
        },
        GameDifficulty::Insane => DifficultySettings {
            max_enemies: 150,
            player_health_modifier: 1.25,
            player_damage_modifier: 1.25,
            enemy_health_modifier: 1.0,
            enemy_damage_modifier: 1.0,
            max_dungeon_amount: 15,
            enemy_speed: 1.5,
            player_speed: 1.0,
        },
        GameDifficulty::MegaDeath => DifficultySettings {
            max_enemies: 2000,
            player_health_modifier: 1.25,
            player_damage_modifier: 1.25,
            enemy_health_modifier: 1.0,
            enemy_damage_modifier: 1.0,
            max_dungeon_amount: 25,
            enemy_speed: 1.7,
            player_speed: 0.8,
        },
    };
    cmds.insert_resource(difficulty_settings);
}
