use bevy::{
    asset::AssetMetaCheck,
    log::LogPlugin,
    prelude::*,
    window::{Cursor, PresentMode, WindowMode, WindowResized, WindowResolution},
};
use bevy_ecs_ldtk::assets::LdtkProject;
use bevy_kira_audio::{AudioChannel, AudioControl};

use serde::{Deserialize, Serialize};

use crate::{
    game::audio::{AmbienceSoundChannel, GameSoundChannel, MusicSoundChannel},
    loading::splashscreen::MainCamera,
    AppState,
};

#[cfg(feature = "develop")]
use bevy_inspector_egui::prelude::*;

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
    pub sound_settings: SoundSettings,
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
            sound_settings: SoundSettings::default(),
            general_settings: GeneralSettings::default(),
        }
    }
}

/// make sure tables are AFTER single fields
#[derive(Reflect, Resource, Serialize, Deserialize, Copy, Clone, Debug)]
#[reflect(Resource)]
pub struct WindowSettings {
    /// enable `v_sync` if true
    pub v_sync: bool,
    /// framerate
    pub frame_rate_target: f64,
    /// full screen yes/no
    pub full_screen: bool,
    /// window scale factor, only set upon start
    pub window_scale_override: f64,
    /// display resolution
    pub resolution: Vec2,
}

/// make sure tables are AFTER single fields
#[derive(Reflect, Resource, Serialize, Deserialize, Copy, Clone, Default, Debug)]
#[reflect(Resource)]
pub struct RenderSettings {
    /// enable `v_sync` if true
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
    Custom(DifficultyScales),
}

/// Settings like zoom and difficulty
/// maybe controls
#[derive(Reflect, Resource, Serialize, Deserialize, Copy, Clone, Debug)]
#[reflect(Resource)]
#[cfg_attr(
    feature = "develop",
    derive(InspectorOptions),
    reflect(InspectorOptions)
)]
pub struct GeneralSettings {
    /// camera zoom
    #[cfg(feature = "develop")]
    #[inspector(min = 0.0, max = 150.0)]
    pub camera_zoom: f32,
    /// camera zoom
    #[cfg(not(feature = "develop"))]
    pub camera_zoom: f32,
    /// game difficulty,
    /// value ranging from 1-4, 1 being easiest, 4 being hardest
    pub game_difficulty: GameDifficulty,
}

/// modify to change sound volume settings
#[derive(Reflect, Debug, Serialize, Deserialize, Resource, Copy, Clone, Component)]
#[reflect(Resource)]
#[cfg_attr(
    feature = "develop",
    derive(InspectorOptions),
    reflect(InspectorOptions)
)]
pub struct SoundSettings {
    /// Total sound scale for game
    #[cfg(feature = "develop")]
    #[inspector(min = 0.0, max = 1.0)]
    pub master_volume: f64,
    /// Total sound scale for game
    #[cfg(not(feature = "develop"))]
    pub master_volume: f64,

    /// Sound effects from environment
    #[cfg(feature = "develop")]
    #[inspector(min = 0.0, max = 1.0)]
    pub ambience_volume: f64,
    /// Sound effects from environment
    #[cfg(not(feature = "develop"))]
    pub ambience_volume: f64,

    /// Game soundtrack volume
    #[cfg(feature = "develop")]
    #[inspector(min = 0.0, max = 1.0)]
    pub music_volume: f64,
    /// Game soundtrack volume
    #[cfg(not(feature = "develop"))]
    pub music_volume: f64,

    /// Important sounds from game
    #[cfg(feature = "develop")]
    #[inspector(min = 0.0, max = 1.0)]
    pub sound_volume: f64,
    /// Important sounds from game
    #[cfg(not(feature = "develop"))]
    pub sound_volume: f64,
}

// TODO: refactor actors module to use this global difficulty resource
// add a system that takes GeneralSettings.difficulty_settings and matches
// that i32 and inserts this configured
#[derive(Reflect, Debug, Serialize, Deserialize, Resource, Copy, Clone, PartialEq, PartialOrd)]
#[reflect(Resource, Default)]
/// difficulty resource used globally for configuring actors and dungeons
pub struct DifficultyScales {
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

impl Default for DifficultyScales {
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
            game_difficulty: GameDifficulty::Custom(DifficultyScales::default()),
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
            window_scale_override: 1.0,
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
            custom_layer: |_| None,
        },
        AssetPlugin {
            file_path: "assets".to_string(),
            processed_file_path: "streamed_assets/Default".to_string(),
            meta_check: AssetMetaCheck::Never,
            watch_for_changes_override: None,
            mode: AssetMode::Unprocessed,
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
                    ), // .with_scale_factor_override(
                    //     cfg_file.window_settings.window_scale_override,
                    // )
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
                    window_level: bevy::window::WindowLevel::Normal,
                    cursor: Cursor {
                        icon: CursorIcon::Crosshair,
                        visible: true,
                        ..default()
                    },
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest())
            .disable::<LogPlugin>()
            .disable::<AssetPlugin>()
    })
    .insert_resource(if cfg_file.render_settings.msaa {
        Msaa::Sample4
    } else {
        Msaa::Off
    })
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

    asha.init_state::<AppState>();
    asha.register_type::<ConfigFile>();

    asha.add_systems(
        Update,
        (
            apply_window_settings.run_if(resource_changed::<WindowSettings>),
            apply_sound_settings.run_if(resource_changed::<SoundSettings>),
            apply_camera_zoom.run_if(resource_changed::<GeneralSettings>),
            update_difficulty_settings.run_if(resource_changed::<GeneralSettings>),
            on_resize_system.run_if(on_event::<WindowResized>()),
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
    // mut frame_limiter_cfg: ResMut<FramepaceSettings>,
    mut mut_window_entity: Query<(Entity, &mut Window)>,
    mut last_resolution: Local<Vec2>,
) {
    let (_w_ent, mut b_window) = mut_window_entity.single_mut();

    // TODO: fix this system too work better?
    if window_settings.resolution == *last_resolution
        && *last_resolution
            == (Vec2 {
                x: b_window.width(),
                y: b_window.height(),
            })
    {
        return;
    }
    *last_resolution = window_settings.resolution;

    // let w_window = winit.get_window(w_ent).unwrap();

    // let requested_limiter = Limiter::from_framerate(window_settings.frame_rate_target);
    // if frame_limiter_cfg.limiter != requested_limiter {
    //     frame_limiter_cfg.limiter = requested_limiter;
    // }

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

/// modifies `AudioChannel` volume if `SoundSettings` changes
fn apply_sound_settings(
    sound_settings: Res<SoundSettings>,
    music_channel: Res<AudioChannel<MusicSoundChannel>>,
    ambience_channel: Res<AudioChannel<AmbienceSoundChannel>>,
    sound_channel: Res<AudioChannel<GameSoundChannel>>,
) {
    //sound settings
    info!("volumes changed, applying settings");
    let mastervolume = sound_settings.master_volume;
    music_channel.set_volume(sound_settings.music_volume * mastervolume);
    ambience_channel.set_volume(sound_settings.ambience_volume * mastervolume);
    sound_channel.set_volume(sound_settings.sound_volume * mastervolume);
}

/// applies camera zoom setting
fn apply_camera_zoom(
    general_settings: Res<GeneralSettings>,
    mut camera: Query<&mut OrthographicProjection, With<MainCamera>>,
) {
    if camera.is_empty() {
        return;
    }

    //camera zoom
    match camera.get_single_mut() {
        Ok(mut projection) => {
            projection.scale = general_settings.camera_zoom;
        }
        Err(e) => {
            warn!("issue getting camera: {e}");
        }
    };
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
    levels: Query<(Entity, &Handle<LdtkProject>), With<Parent>>,
    general_settings: Res<GeneralSettings>,
    mut cmds: Commands,
) {
    let level_amount = i32::try_from(levels.iter().len()).unwrap_or(1);
    if let GameDifficulty::Custom(scales) = general_settings.game_difficulty {
        cmds.insert_resource(scales);
    } else {
        let difficulty_settings: DifficultyScales =
            create_difficulty_scales(*general_settings, Some(level_amount));
        cmds.insert_resource(difficulty_settings);
    }
}

/// converts `GeneralSettings.game_difficulty` too `DifficultyScales` too be used elsewhere
fn create_difficulty_scales(
    general_settings: GeneralSettings,
    level_amount: Option<i32>,
) -> DifficultyScales {
    let level_amount = level_amount.unwrap_or(1);

    match general_settings.game_difficulty {
        GameDifficulty::Custom(a) => a,
        GameDifficulty::Debug => DifficultyScales {
            max_enemies_per_room: 1,
            max_dungeon_amount: 1,
            player_health_scale: 1.0,
            player_damage_scale: 1.0,
            player_speed_scale: 1.0,
            enemy_health_scale: 1.0,
            enemy_damage_scale: 1.0,
            enemy_speed_scale: 1.0,
        },
        GameDifficulty::Easy => DifficultyScales {
            max_enemies_per_room: 10 * level_amount,
            player_health_scale: 1.25,
            player_damage_scale: 1.25,
            enemy_health_scale: 0.75,
            enemy_damage_scale: 0.75,
            max_dungeon_amount: 5,
            enemy_speed_scale: 0.9,
            player_speed_scale: 1.2,
        },
        GameDifficulty::Medium => DifficultyScales {
            max_enemies_per_room: 20 * level_amount,
            player_health_scale: 1.00,
            player_damage_scale: 1.00,
            enemy_health_scale: 1.0,
            enemy_damage_scale: 1.0,
            max_dungeon_amount: 7,
            enemy_speed_scale: 1.0,
            player_speed_scale: 1.0,
        },
        GameDifficulty::Hard => DifficultyScales {
            max_enemies_per_room: 30 * level_amount,
            player_health_scale: 1.0,
            player_damage_scale: 1.0,
            enemy_health_scale: 1.0,
            enemy_damage_scale: 1.0,
            max_dungeon_amount: 9,
            enemy_speed_scale: 1.2,
            player_speed_scale: 1.0,
        },
        GameDifficulty::Insane => DifficultyScales {
            max_enemies_per_room: 35 * level_amount,
            player_health_scale: 1.25,
            player_damage_scale: 1.25,
            enemy_health_scale: 1.0,
            enemy_damage_scale: 1.0,
            max_dungeon_amount: 15,
            enemy_speed_scale: 1.5,
            player_speed_scale: 1.0,
        },
        GameDifficulty::MegaDeath => DifficultyScales {
            max_enemies_per_room: 50 * level_amount,
            player_health_scale: 1.25,
            player_damage_scale: 1.25,
            enemy_health_scale: 1.0,
            enemy_damage_scale: 1.0,
            max_dungeon_amount: 25,
            enemy_speed_scale: 1.7,
            player_speed_scale: 0.8,
        },
    }
}
