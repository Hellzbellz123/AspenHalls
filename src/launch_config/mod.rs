/// functions for loading `ConfigFile` from filesystem, returns `DefaultSettings` from the `ConfigFile`
mod load_file;




use bevy::{
    log::LogPlugin,
    prelude::{
        default, App, AssetPlugin, Camera2d, ClearColor, Color, Commands, DefaultPlugins,
        DetectChanges, Entity, EventReader, Handle, ImagePlugin, OrthographicProjection, Parent,
        Plugin, PluginGroup, Query, Res, ResMut, Update, Vec2, Window, WindowPlugin,
        WindowPosition, With, Resource, info, Component,
    },
    ecs::reflect::ReflectResource,
    window::{PresentMode, WindowMode, WindowResized, WindowResolution}, reflect::Reflect, diagnostic::DiagnosticsPlugin,
};
use bevy_ecs_ldtk::prelude::LdtkLevel;
use bevy_framepace::{FramepaceSettings, Limiter};
use bevy_inspector_egui::{InspectorOptions, inspector_options::ReflectInspectorOptions};
use bevy_kira_audio::{AudioChannel, AudioControl};
use serde::{Deserialize, Serialize};

use crate::{
    game::audio::{Ambience, Music, Sound},
    loading::splashscreen::MainCameraTag,
    utilities::logging::VCLogPlugin,
};

/// Holds game settings deserialized from the config.toml
#[derive(Reflect, Resource, Serialize, Deserialize, Clone, Copy, Default)]
#[reflect(Resource)]
pub struct ConfigFile {
    /// game window settings
    window_settings: WindowSettings,
    /// sound settings
    sound_settings: SoundSettings,
    /// general settings like zoom and difficulty
    general_settings: GeneralSettings,
}

/// make sure tables are AFTER single fields
#[derive(Reflect, Resource, Serialize, Deserialize, Copy, Clone)]
#[reflect(Resource)]
pub struct WindowSettings {
    /// enable v_sync if true
    pub v_sync: bool,
    /// framerate
    pub frame_rate_target: f64,
    /// full screen yes/no
    pub full_screen: bool,
    /// display resolution
    pub resolution: Vec2,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Reflect)]
/// game difficulty enum
pub enum GameDifficulty {
    /// 0.75 scale on enemy damage/hp, 1.25 on player
    Easy,
    /// 1.0 scale on player/enemy damage/hp
    Medium,
    /// enemy's are a little faster, more enemy's, more rooms
    Hard,
    /// enemy's are faster, even more enemy's/rooms, plus enemy's do more damage
    Insane,
    /// lots of enemy's/rooms, like a lot. 3x enemy hp/damage
    MegaDeath,
}

/// Settings like zoom and difficulty
/// maybe controls
#[derive(Reflect, Resource, InspectorOptions, Serialize, Deserialize, Copy, Clone)]
#[reflect(Resource, InspectorOptions)]
pub struct GeneralSettings {
    /// camera zoom
    #[inspector(min = 0.0, max = 150.0)]
    pub camera_zoom: f32,
    /// game difficulty,
    /// value ranging from 1-4, 1 being easiest, 4 being hardest
    pub game_difficulty: GameDifficulty,
}

/// modify to change sound volume settings
#[derive(Reflect, InspectorOptions, Debug, Serialize, Deserialize, Resource, Copy, Clone, Component)]
#[reflect(Resource, InspectorOptions)]
pub struct SoundSettings {
    /// Total Sound Scale for game
    #[inspector(min = 0.0, max = 1.0)]
    pub master_volume: f64,
    /// sound effects from environment
    #[inspector(min = 0.0, max = 1.0)]
    pub ambience_volume: f64,
    /// game soundtrack volume
    #[inspector(min = 0.0, max = 1.0)]
    pub music_volume: f64,
    /// important sounds from game
    #[inspector(min = 0.0, max = 1.0)]
    pub sound_volume: f64,
}

// TODO: refactor actors module to use this global difficulty resource
// add a system that takes GeneralSettings.difficulty_settings and matches
// that i32 and inserts this configured
#[derive(Reflect, Debug, Serialize, Deserialize, Resource, Copy, Clone)]
#[reflect(Resource)]
/// difficulty resource used globally for settings
pub struct DifficultyScale {
    /// not a scale, just an amount
    pub max_enemies_per_room: i32,
    /// f32 used too scale
    pub max_dungeon_amount: i32,

    /// f32 used too scale
    pub player_health_scale: f32,
    /// f32 used too scale
    pub player_damage_scale: f32,
    /// f32 used too scale
    pub player_speed_scale: f32,

    /// f32 used too scale
    pub enemy_health_scale: f32,
    /// f32 used too scale
    pub enemy_damage_scale: f32,
    /// f32 used too scale
    pub enemy_speed_scale: f32,
}

impl Default for DifficultyScale {
    fn default() -> Self {
        Self {
            max_enemies_per_room: 20,
            max_dungeon_amount: 5,
            player_health_scale: 1.0,
            player_damage_scale: 1.0,
            enemy_health_scale: 1.0,
            enemy_damage_scale: 1.0,
            enemy_speed_scale: 1.0,
            player_speed_scale: 1.0,
        }
    }
}

impl Default for GeneralSettings {
    fn default() -> Self {
        Self {
            camera_zoom: 3.5,
            game_difficulty: GameDifficulty::Medium,
        }
    }
}

//TODO: default app settings if its a setting it goes here, move this too settings plugin
impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            v_sync: true,
            frame_rate_target: 60.0,
            full_screen: false,
            resolution: Vec2 {
                x: 1200.0,
                y: 720.0,
            },
        }
    }
}

impl Default for SoundSettings {
    fn default() -> Self {
        Self {
            master_volume: 0.2,
            ambience_volume: 0.2,
            music_volume: 0.2,
            sound_volume: 0.2,
        }
    }
}

/// initial setup of the app, load settings and add default plugins
pub struct InitAppPlugin;

impl Plugin for InitAppPlugin {
    fn build(&self, app: &mut App) {
        let cfg_file = load_file::load_settings();
        let difficulty_settings = DifficultyScale::default();

        app.add_plugins({
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: if cfg_file.window_settings.v_sync {
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
                            if cfg_file.window_settings.full_screen {
                                // if full screen is true, use borderless full screen
                                // cursor mode is confined to the window so it cant
                                // leave without alt tab
                                WindowMode::BorderlessFullscreen
                            } else {
                                WindowMode::Windowed
                            }
                        },
                        window_level: bevy::window::WindowLevel::AlwaysOnBottom,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    asset_folder: "gamedata".to_string(),
                    watch_for_changes: None,
                })
                .disable::<LogPlugin>()
        })
        .insert_resource(ClearColor(Color::Hsla {
            hue: 294.0,
            saturation: 0.71,
            lightness: 0.08,
            alpha: 1.0,
        }))
        .insert_resource(cfg_file.window_settings)
        .insert_resource(cfg_file.sound_settings)
        .insert_resource(cfg_file.general_settings)
        .insert_resource(difficulty_settings);

        app.add_systems(
            Update,
            (
                apply_window_settings,
                apply_sound_settings,
                apply_camera_zoom,
                update_difficulty_settings,
                on_resize_system,
            ),
        );
    }
}

/// creates an App
/// if feature == "trace" then default logger, else custom logger that logs too file minimally
pub fn app_with_logging() -> App {
    let mut vanillacoffee = App::new();
    #[cfg(all(not(feature = "trace"), not(target_os = "android")))]
    {
        println!("Logging without tracing requested");
        vanillacoffee.add_plugins(VCLogPlugin {
            // filters for anything that makes it through the default log level. quiet big loggers
            // filter: "".into(), // an empty filter
            filter:
            "bevy_ecs=warn,naga=error,wgpu_core=error,wgpu_hal=error,symphonia=warn,big_brain=warn,bevy_rapier2d=error"
            .into(),
            level: bevy::log::Level::TRACE,
        });
        info!("Logging Initialized");
    }

    #[cfg(feature = "trace")]
    {
        println!("Logging with tracing requested");
        vanillacoffee.add_plugins(bevy::log::LogPlugin {
            filter: "".into(),
            level: bevy::log::Level::TRACE,
        });
        info!("Logging Initialized");
    }

    #[cfg(target_os="android")]
    {
        println!("Logging with tracing requested");
        vanillacoffee.add_plugins(bevy::log::LogPlugin {
            filter: "".into(),
            level: bevy::log::Level::TRACE,
        });
        info!("Logging Initialized");
    }
    // add bevy plugins
    vanillacoffee.add_plugins(InitAppPlugin);
    vanillacoffee
}

//TODO: move this to loading plugin and only run it when the settings resource changes (clicking apply in the settings menu, or reacting to OS changes), or on game load.
// (system ordering is important here) the camera needs to be spawned first or we get a panic
// #[bevycheck::system]
/// updates window settings if changed
fn apply_window_settings(
    // winit: NonSend<bevy::winit::WinitWindows>,
    window_settings: Res<WindowSettings>,
    mut frame_limiter_cfg: ResMut<FramepaceSettings>,
    mut mut_window_entity: Query<(Entity, &mut Window)>,
) {
    if window_settings.is_changed() || window_settings.is_added() {
        let requested_limiter = Limiter::from_framerate(window_settings.frame_rate_target);

        let (_w_ent, mut b_window) = mut_window_entity.single_mut();
        // let w_window = winit.get_window(w_ent).unwrap();

        if frame_limiter_cfg.limiter != requested_limiter {
            frame_limiter_cfg.limiter = requested_limiter;
        }

        if window_settings.full_screen && b_window.mode != WindowMode::BorderlessFullscreen {
            b_window.mode = WindowMode::BorderlessFullscreen;
        }
        if !window_settings.full_screen && b_window.mode == WindowMode::BorderlessFullscreen {
            b_window.mode = WindowMode::Windowed;
            b_window.resolution = window_settings.resolution.into();
        }

        info!(
            "Requested Window Resolution {}, Actual Resolution {:?}",
            window_settings.resolution, b_window.resolution
        );
    }
}

/// modifies `AudioChannel` volume if `SoundSettings` changes
fn apply_sound_settings(
    sound_settings: Res<SoundSettings>,
    music_channel: Res<AudioChannel<Music>>,
    ambience_channel: Res<AudioChannel<Ambience>>,
    sound_channel: Res<AudioChannel<Sound>>,
) {
    if sound_settings.is_changed() {
        //sound settings
        info!("volumes changed, applying settings");
        let mastervolume = sound_settings.master_volume;
        music_channel.set_volume(sound_settings.music_volume * mastervolume);
        ambience_channel.set_volume(sound_settings.ambience_volume * mastervolume);
        sound_channel.set_volume(sound_settings.sound_volume * mastervolume);
    }
}

/// applies camera zoom setting
fn apply_camera_zoom(
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

/// sets settings window size too actual size if resized
/// doesn't run if fullscreen
fn on_resize_system(
    mut settings: ResMut<WindowSettings>,
    mut resize_reader: EventReader<WindowResized>,
) {
    if !settings.full_screen {
        resize_reader.iter().for_each(|event| {
            settings.resolution.x = event.width;
            settings.resolution.y = event.height;
        });
        resize_reader.clear();
    }
}

/// updates `DifficultySettings` if player changes difficulty settings
fn update_difficulty_settings(
    levels: Query<(Entity, &Handle<LdtkLevel>), With<Parent>>,
    general_settings: Res<GeneralSettings>,
    mut cmds: Commands,
) {
    if general_settings.is_changed() || levels.iter().len() >= 1 {
        let level_amount = i32::try_from(levels.iter().len()).unwrap_or(1000);
        let difficulty_settings: DifficultyScale = match general_settings.game_difficulty {
            GameDifficulty::Easy => DifficultyScale {
                max_enemies_per_room: 10 * level_amount,
                player_health_scale: 1.25,
                player_damage_scale: 1.25,
                enemy_health_scale: 0.75,
                enemy_damage_scale: 0.75,
                max_dungeon_amount: 5,
                enemy_speed_scale: 0.9,
                player_speed_scale: 1.2,
            },
            GameDifficulty::Medium => DifficultyScale {
                max_enemies_per_room: 20 * level_amount,
                player_health_scale: 1.00,
                player_damage_scale: 1.00,
                enemy_health_scale: 1.0,
                enemy_damage_scale: 1.0,
                max_dungeon_amount: 7,
                enemy_speed_scale: 1.0,
                player_speed_scale: 1.0,
            },
            GameDifficulty::Hard => DifficultyScale {
                max_enemies_per_room: 30 * level_amount,
                player_health_scale: 1.0,
                player_damage_scale: 1.0,
                enemy_health_scale: 1.0,
                enemy_damage_scale: 1.0,
                max_dungeon_amount: 9,
                enemy_speed_scale: 1.2,
                player_speed_scale: 1.0,
            },
            GameDifficulty::Insane => DifficultyScale {
                max_enemies_per_room: 35 * level_amount,
                player_health_scale: 1.25,
                player_damage_scale: 1.25,
                enemy_health_scale: 1.0,
                enemy_damage_scale: 1.0,
                max_dungeon_amount: 15,
                enemy_speed_scale: 1.5,
                player_speed_scale: 1.0,
            },
            GameDifficulty::MegaDeath => DifficultyScale {
                max_enemies_per_room: 50 * level_amount,
                player_health_scale: 1.25,
                player_damage_scale: 1.25,
                enemy_health_scale: 1.0,
                enemy_damage_scale: 1.0,
                max_dungeon_amount: 25,
                enemy_speed_scale: 1.7,
                player_speed_scale: 0.8,
            },
        };
        cmds.insert_resource(difficulty_settings);
    }
}
