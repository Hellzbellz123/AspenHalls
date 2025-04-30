
use serde::{Deserialize, Serialize};

use bevy::{
    asset::AssetMetaCheck, log::LogPlugin, prelude::*, sprite::SpritePlugin, window::{PresentMode, WindowMode, WindowResized, WindowResolution}
};
use bevy_ecs_ldtk::LdtkProjectHandle;
use bevy_inspector_egui::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl};

use crate::{
    game::audio::{AmbienceSoundChannel, GameSoundChannel, MusicSoundChannel},
    loading::splashscreen::MainCamera,
    AppStage, GameStage,
};

/// functions too create default file and save file
pub mod save_load;

/// Holds game settings deserialized from the config.toml
#[derive(Reflect, Resource, Serialize, Deserialize, Clone, Debug)]
#[reflect(Resource)]
pub struct ConfigFile {
    /// preset log filter from cfg
    pub log_filter: Option<String>,
    /// game window settings
    pub window_settings: WindowSettings,
    /// rendering settings
    pub render_settings: RenderSettings,
    /// sound settings
    pub sound_settings: AudioSettings,
    /// general settings like zoom and difficulty
    pub general_settings: GeneralSettings,
}

impl Default for ConfigFile {
    fn default() -> Self {
        Self {
            log_filter: Some("info,symphonia=warn,blocking=warn,wgpu=error,naga=warn,gilrs=warn,bevy_ecs_tilemap=debug,big_brain=warn".into()),
            // log_filter: Some("trace,log=warn,wgpu=error,naga=warn,gilrs=warn,bevy_ecs_tilemap=debug".into()),
            window_settings: WindowSettings::default(),
            render_settings: RenderSettings::default(),
            sound_settings: AudioSettings::default(),
            general_settings: GeneralSettings::default(),
        }
    }
}

/// make sure tables are AFTER single fields
#[derive(Reflect, Resource, Serialize, Deserialize, Copy, Clone, Debug, InspectorOptions)]
#[reflect(Resource)]
pub struct WindowSettings {
    /// disable software cursor systems
    pub software_cursor_enabled: bool,
    /// enable `v_sync` if true
    pub v_sync: bool,
    /// framerate
    pub frame_rate_target: Option<f32>,
    /// full screen yes/no
    pub full_screen: bool,
    /// window scale factor, only set upon start
    #[inspector(min = 0.2, max = 100.0)]
    pub window_scale: f64,
    /// game ui scale
    #[inspector(min = 0.2, max = 100.0)]
    pub ui_scale: f64,
    /// display resolution
    pub resolution: Vec2,
}

/// make sure tables are AFTER single fields
#[derive(Reflect, Resource, Serialize, Deserialize, Copy, Clone, Default, Debug)]
#[reflect(Resource)]
pub struct RenderSettings {
    /// enable `msaa` if true
    pub msaa: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Reflect, PartialEq, PartialOrd, Default)]
#[reflect(Default)]
/// game difficulty enum
pub enum GameDifficulty {
    /// 0.75 scale on enemy damage/hp, 1.25 on player
    Easy,
    /// 1.0 scale on player/enemy damage/hp
    #[default]
    Medium,
    /// enemy's are a little faster, more enemy's, more rooms
    Hard,
    /// enemy's are faster, even more enemy's/rooms, plus enemy's do more damage
    Insane,
    /// lots of enemy's/rooms, like a lot. 3x enemy hp/damage
    MegaDeath,
    /// only 1 dungeon and if more than 1 enemy is spawned
    Debug,
    /// custom
    Custom(DifficultySettings),
}

/// Settings like zoom and difficulty
/// maybe controls
#[derive(Reflect, Resource, Serialize, Deserialize, Copy, Clone, Debug, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct GeneralSettings {
    /// spawns touch gamepad ui for devices with touch screens
    pub enable_touch_controls: bool,
    /// enable debug/development tools and utils
    pub enable_debug: bool,
    /// camera zoom
    #[inspector(min = 0.0, max = 150.0)]
    pub camera_zoom: f32,
    /// master game difficulty,
    /// configures the actual difficulty settings
    /// value ranging from 1-4, 1 being easiest, 4 being hardest
    pub game_difficulty: GameDifficulty,
}

/// modify to change sound volume settings
#[derive(
    Reflect, Debug, Serialize, Deserialize, Resource, Copy, Clone, Component, InspectorOptions,
)]
#[reflect(Resource, InspectorOptions)]
pub struct AudioSettings {
    /// maximum distance at which audio can be heard
    #[inspector(min = 32.0, max = 10000.0)]
    pub max_distance: f32,
    /// game max sound amount
    #[inspector(min = 20, max = 5000)]
    pub max_sounds: i32,
    /// sound volume config resource
    pub volume_config: VolumeConfig,
}

#[derive(
    Reflect, Debug, Serialize, Deserialize, Resource, Copy, Clone, Component, InspectorOptions,
)]
#[reflect(InspectorOptions)]
/// sound decible scales
pub struct VolumeConfig {
    /// global sound scales
    #[inspector(min = 0.0, max = 1.0)]
    pub master: f64,
    /// weapon sounds, monster sounds footsteps
    #[inspector(min = 0.0, max = 1.0)]
    pub gameplay: f64,
    /// ambience sound settings like creaking howling etc
    #[inspector(min = 0.0, max = 1.0)]
    pub ambience: f64,
    /// ingame music settings
    #[inspector(min = 0.0, max = 1.0)]
    pub music: f64,
}

// TODO: refactor actors module to use this global difficulty resource
// add a system that takes GeneralSettings.difficulty_settings and matches
// that i32 and inserts this configured
#[derive(Reflect, Debug, Serialize, Deserialize, Resource, Copy, Clone, PartialEq, PartialOrd)]
#[reflect(Resource, Default)]
/// difficulty resource used globally for configuring actors and dungeons
pub struct DifficultySettings {
    /// can actors of the same `AI_TYPE` cause damage too eachother
    pub friendly_fire_enabled: bool,
    /// not a scale, just an amount multiplied by total rooms
    pub max_enemies_per_room: i32,
    /// i32 used too scale, multiples dungeon amount
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

impl Default for DifficultySettings {
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
            friendly_fire_enabled: false,
        }
    }
}

impl Default for GeneralSettings {
    fn default() -> Self {
        Self {
            camera_zoom: 0.6,
            game_difficulty: GameDifficulty::Custom(DifficultySettings::default()),
            enable_debug: cfg!(feature = "develop"),
            enable_touch_controls: cfg!(target_os = "android") || cfg!(target_os = "ios"),
        }
    }
}

//TODO: default app settings if its a setting it goes here, move this too settings plugin
impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            software_cursor_enabled: true,
            v_sync: true,
            frame_rate_target: None,
            full_screen: false,
            resolution: Vec2 {
                x: 1200.0,
                y: 720.0,
            },
            window_scale: 1.0,
            ui_scale: 1.0,
        }
    }
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            max_distance: 350.0,
            max_sounds: 200,
            volume_config: VolumeConfig {
                master: 0.2,
                gameplay: 0.2,
                ambience: 0.2,
                music: 0.2,
            },
        }
    }
}

/// creates an `App` with logging and initialization assets
pub fn create_configured_app(cfg_file: ConfigFile) -> App {
    let mut asha = App::new();

    asha.add_plugins((
        LogPlugin {
            filter: if cfg!(feature = "trace") {
                String::new()
            } else {
                cfg_file.log_filter.unwrap_or_default()
            },
            level: bevy::log::Level::TRACE,
            custom_layer: crate::dev_tools::console::init_log_layers,
        },
        AssetPlugin {
            file_path: "assets".to_string(),
            processed_file_path: "streamed_assets/Default".to_string(),
            meta_check: AssetMetaCheck::Never,
            watch_for_changes_override: None,
            mode: AssetMode::Unprocessed,
            unapproved_path_mode: bevy::asset::UnapprovedPathMode::Deny,
        },
    ));

    info!("Logging and Asset Server Initialized");
    // add vanillacoffee stuff

    let difficulty_settings = create_difficulty_scales(cfg_file.general_settings, None);

    asha.add_plugins({
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: if cfg_file.window_settings.v_sync {
                        PresentMode::AutoVsync
                    } else {
                        PresentMode::AutoNoVsync
                    },
                    position: WindowPosition::Automatic,
                    title: "Aspen Halls".to_string(),
                    resolution: WindowResolution::new(
                        cfg_file.window_settings.resolution.x,
                        cfg_file.window_settings.resolution.y,
                    )
                    .with_scale_factor_override(cfg_file.window_settings.window_scale as f32),
                    mode: {
                        if cfg_file.window_settings.full_screen {
                            // if full screen is true, use borderless full screen
                            // cursor mode is confined to the window so it cant
                            // leave without alt tab
                            WindowMode::BorderlessFullscreen(MonitorSelection::Primary)
                        } else {
                            WindowMode::Windowed
                        }
                    },
                    window_level: bevy::window::WindowLevel::Normal,
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest())
            .disable::<LogPlugin>()
            .disable::<AssetPlugin>()
    })
    // .insert_resource(if cfg_file.render_settings.msaa {
    //     Msaa::Sample4
    // } else {
    //     Msaa::Off
    // })
    .insert_resource(ClearColor(Color::srgba(
        26.0 / 255.0,
        25.0 / 255.0,
        25.0 / 255.0,
        1.0,
    )))
    .insert_resource(cfg_file.window_settings)
    .insert_resource(cfg_file.sound_settings)
    .insert_resource(cfg_file.general_settings)
    .insert_resource(difficulty_settings);

    asha.init_state::<AppStage>().add_sub_state::<GameStage>();
    asha.register_type::<ConfigFile>();

    asha.add_systems(
        Update,
        (
            apply_window_settings.run_if(resource_changed::<WindowSettings>),
            apply_sound_settings.run_if(resource_changed::<AudioSettings>),
            apply_camera_zoom.run_if(resource_changed::<GeneralSettings>),
            update_difficulty_settings.run_if(resource_changed::<GeneralSettings>),
            on_resize_system.run_if(on_event::<WindowResized>),
        ),
    );

    // add bevy plugins
    asha
}

//TODO: move this to loading plugin and only run it when the settings resource changes (clicking apply in the settings menu, or reacting to OS changes), or on game load.
// (system ordering is important here) the camera needs to be spawned first or we get a panic
// #[bevycheck::system]
/// updates window settings if changed
fn apply_window_settings(
    // winit: NonSend<bevy::winit::WinitWindows>,
    window_settings: Res<WindowSettings>,
    mut ui_scale: ResMut<UiScale>,
    // frame_limiter_cfg: ResMut<FramepaceSettings>,
    mut mut_window_entity: Query<(Entity, &mut Window)>,
    mut last_resolution: Local<Vec2>,
) {
    let Ok((_w_ent, b_window)) = mut_window_entity.single_mut() else {
        return
    };

    // TODO: fix this system too work better?
    if window_settings.resolution != *last_resolution
        && *last_resolution
            != (Vec2 {
                x: b_window.width(),
                y: b_window.height(),
            })
    {
        *last_resolution = window_settings.resolution;
    }

    *ui_scale = UiScale(window_settings.ui_scale as f32);


    // TODO: fix config file application
    // if let Some(requested_fps_target) = window_settings.frame_rate_target {
    //     let requested_fps_target = requested_fps_target.clamp(16.0, 999.0);
    //     let requested_limiter = Limiter::from_framerate(f64::from(requested_fps_target));
    //     if frame_limiter_cfg.limiter != requested_limiter {
    //         frame_limiter_cfg.limiter = requested_limiter;
    //     }
    // }

    // if window_settings.full_screen && b_window.mode != WindowMode::BorderlessFullscreen {
    //     b_window.mode = WindowMode::BorderlessFullscreen;
    // }
    // if !window_settings.full_screen && b_window.mode == WindowMode::BorderlessFullscreen {
    //     b_window.mode = WindowMode::Windowed;
    //     b_window.resolution = window_settings.resolution.into();
    // }

    info!(
        "Requested Window Resolution {}, Actual Resolution {:?}",
        window_settings.resolution, b_window.resolution
    );
}

/// modifies `AudioChannel` volume if `SoundSettings` changes
fn apply_sound_settings(
    sound_settings: Res<AudioSettings>,
    music_channel: Res<AudioChannel<MusicSoundChannel>>,
    ambience_channel: Res<AudioChannel<AmbienceSoundChannel>>,
    sound_channel: Res<AudioChannel<GameSoundChannel>>,
) {
    //sound settings
    info!("volumes changed, applying settings");
    let sound_settings = sound_settings.volume_config;

    let mastervolume = sound_settings.master;
    music_channel.set_volume(sound_settings.music * mastervolume);
    ambience_channel.set_volume(sound_settings.ambience * mastervolume);
    sound_channel.set_volume(sound_settings.gameplay * mastervolume);
}

/// applies camera zoom setting
fn apply_camera_zoom(
    general_settings: Res<GeneralSettings>,
    mut camera: Query<&mut Projection, With<MainCamera>>,
) {
    if camera.is_empty() {
        return;
    }

    let Ok(projection) = camera.single_mut() else {
        return
    };

    match projection.into_inner() {
        Projection::Orthographic(orthographic_projection) => orthographic_projection.scale = general_settings.camera_zoom,
        _ => warn!("wrong camera projection"),
    }
    //camera zoom
    // match camera.single_mut() {
    //     Ok(projection) => {
    //     }
    //     Err(e) => {
    //         warn!("issue getting camera: {e}");
    //     }
    // };
}

// TODO: fix logical pixels
/// sets settings window size too actual size if resized
/// doesn't run if fullscreen
fn on_resize_system(
    mut settings: ResMut<WindowSettings>,
    mut resize_reader: EventReader<WindowResized>,
) {
    if !settings.full_screen {
        for resize in resize_reader.read() {
            let (width, height) = (resize.width, resize.height);
            settings.resolution.x = width;
            settings.resolution.y = height;
        }
    }
}

/// updates `DifficultySettings` if player changes difficulty settings
fn update_difficulty_settings(
    // TODO: this is not correct
    levels: Query<(Entity, &LdtkProjectHandle), With<ChildOf>>,
    general_settings: Res<GeneralSettings>,
    mut cmds: Commands,
) {
    let level_amount = i32::try_from(levels.iter().len()).unwrap_or(1);
    if let GameDifficulty::Custom(scales) = general_settings.game_difficulty {
        cmds.insert_resource(scales);
    } else {
        let difficulty_settings: DifficultySettings =
            create_difficulty_scales(*general_settings, Some(level_amount));
        cmds.insert_resource(difficulty_settings);
    }
}

/// converts `GeneralSettings.game_difficulty` too `DifficultyScales` too be used elsewhere
fn create_difficulty_scales(
    general_settings: GeneralSettings,
    level_amount: Option<i32>,
) -> DifficultySettings {
    let level_amount = level_amount.unwrap_or(1);

    match general_settings.game_difficulty {
        GameDifficulty::Custom(a) => a,
        GameDifficulty::Debug => DifficultySettings {
            max_enemies_per_room: 1,
            max_dungeon_amount: 1,
            player_health_scale: 1.0,
            player_damage_scale: 1.0,
            player_speed_scale: 1.0,
            enemy_health_scale: 1.0,
            enemy_damage_scale: 1.0,
            enemy_speed_scale: 1.0,
            friendly_fire_enabled: false,
        },
        GameDifficulty::Easy => DifficultySettings {
            max_enemies_per_room: 10 * level_amount,
            player_health_scale: 1.25,
            player_damage_scale: 1.25,
            enemy_health_scale: 0.75,
            enemy_damage_scale: 0.75,
            max_dungeon_amount: 5,
            enemy_speed_scale: 0.9,
            player_speed_scale: 1.2,
            friendly_fire_enabled: false,
        },
        GameDifficulty::Medium => DifficultySettings {
            max_enemies_per_room: 20 * level_amount,
            player_health_scale: 1.00,
            player_damage_scale: 1.00,
            enemy_health_scale: 1.0,
            enemy_damage_scale: 1.0,
            max_dungeon_amount: 7,
            enemy_speed_scale: 1.0,
            player_speed_scale: 1.0,
            friendly_fire_enabled: false,
        },
        GameDifficulty::Hard => DifficultySettings {
            max_enemies_per_room: 30 * level_amount,
            player_health_scale: 1.0,
            player_damage_scale: 1.0,
            enemy_health_scale: 1.0,
            enemy_damage_scale: 1.0,
            max_dungeon_amount: 9,
            enemy_speed_scale: 1.2,
            player_speed_scale: 1.0,
            friendly_fire_enabled: false,
        },
        GameDifficulty::Insane => DifficultySettings {
            max_enemies_per_room: 35 * level_amount,
            player_health_scale: 1.25,
            player_damage_scale: 1.25,
            enemy_health_scale: 1.0,
            enemy_damage_scale: 1.0,
            max_dungeon_amount: 15,
            enemy_speed_scale: 1.5,
            player_speed_scale: 1.0,
            friendly_fire_enabled: false,
        },
        GameDifficulty::MegaDeath => DifficultySettings {
            max_enemies_per_room: 50 * level_amount,
            player_health_scale: 1.25,
            player_damage_scale: 1.25,
            enemy_health_scale: 1.0,
            enemy_damage_scale: 1.0,
            max_dungeon_amount: 25,
            enemy_speed_scale: 1.7,
            player_speed_scale: 0.8,
            friendly_fire_enabled: false,
        },
    }
}
