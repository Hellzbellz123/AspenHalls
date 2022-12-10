use bevy::{prelude::*, render::camera::RenderTarget};
use bevy_mouse_tracking_plugin::MainCamera;
use chrono::Utc;
use std::path::Path;

pub mod game;
pub mod window;

use self::game::AppSettings;
use crate::audio::SoundSettings;

/// holds general game utilities
/// not particularly related to gameplay
pub struct UtilitiesPlugin;

impl Plugin for UtilitiesPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "dev")]
        app.add_system(window::set_debug_title);

        app.add_startup_system(window::set_window_icon)
            .insert_resource(EagerMousePos {
                world: Vec2::ZERO,
                window: Vec2::ZERO,
            })
            .add_system(eager_cursor_pos);
    }
}

/// mouse position eagerly updated for latency focused stuff. probably bad, will find out
#[derive(Debug, Resource, Clone, Copy, PartialEq, Component)]
pub struct EagerMousePos {
    /// mouse pos in world space
    pub world: Vec2,
    /// mouse pos in window coords
    pub window: Vec2,
}
/// updates `EagerMousePos` resource with current mouse positon and mouse pos translated to worldspace. no change detection, always runs
fn eager_cursor_pos(
    mut fastmousepos: ResMut<EagerMousePos>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    windows: Res<Windows>,
) {
    if !q_camera.is_empty() {
        // get the camera info and transform
        // assuming there is exactly one main camera entity, so query::single() is OK
        let (camera, camera_transform) = q_camera.single();

        // // Games typically only have one window (the primary window).
        // // For multi-window applications, you need to use a specific window ID here.
        let wnd = if let RenderTarget::Window(id) = camera.target {
            windows.get(id).unwrap()
        } else {
            windows.get_primary().unwrap()
        };

        // check if the cursor is inside the window and get its position
        if let Some(screen_pos) = wnd.cursor_position() {
            // get the size of the window
            let window_size = Vec2::new(wnd.width(), wnd.height());

            // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
            let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

            // matrix for undoing the projection and camera transform
            let ndc_to_world =
                camera_transform.compute_matrix() * camera.projection_matrix().inverse();

            // use it to convert ndc to world-space coordinates
            let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

            // reduce it to a 2D value
            let world_pos: Vec2 = world_pos.truncate();
            fastmousepos.world = world_pos;
            fastmousepos.window = screen_pos;
            info!("eager mouse update {}", world_pos);
        } else {
            fastmousepos.world = Vec2::ZERO;
            fastmousepos.window = Vec2::ZERO;
            info!("eager mouse not in window");
        }
    }
}

pub fn despawn_with<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in to_despawn.iter() {
        info!("despawning entity recursively: {:#?}", entity);
        commands.entity(entity).despawn_recursive();
    }
}

#[must_use]
pub fn load_settings() -> AppSettings {
    let settings_path = Path::new("./config.toml");
    let target_settings = std::fs::read_to_string(settings_path); //File::open(settings_path);

    match target_settings {
        Ok(_) => {
            println!("{}", append_info("Game Settings loaded succesfully"));
            let toml_cfg: AppSettings = toml::from_str(target_settings.unwrap().as_str())
                .expect("error parsing config file");
            AppSettings {
                sound_settings: toml_cfg.sound_settings,
                resolution: toml_cfg.resolution,
                camera_zoom: toml_cfg.camera_zoom,
            }
        }
        Err(_) => {
            println!(
                "{} {:?}, target file: {:?}",
                append_info("there was an error reading app settings:"),
                target_settings,
                settings_path,
            );

            let app_settings = AppSettings {
                camera_zoom: 1.0,
                resolution: Vec2 {
                    x: 1200.0,
                    y: 900.0,
                },
                sound_settings: SoundSettings {
                    mastervolume: 1.0,
                    ambiencevolume: 1.0,
                    musicvolume: 1.0,
                    soundvolume: 1.0,
                },
            };

            let serial_cfg =
                toml::to_string(&app_settings).expect("error converting config to string");
            std::fs::write(settings_path, serial_cfg).expect("couldnt write file");

            app_settings
        }
    }
}

/// custom wrapper over format that outputs current time in rfc2822 and a green INFO tag
// #[allow(unused_variables)]
#[must_use]
pub fn append_info(content: &str) -> String {
    let time = Utc::now();
    format!("{} \x1b[32mINFO\x1b[0m {content}", time.to_rfc2822())
}
