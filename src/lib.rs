use bevy::{
    core_pipeline::clear_color::ClearColorConfig, log::LogPlugin, prelude::*,
    render::camera::ScalingMode, window::WindowMode,
};
#[cfg(feature = "inspect")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::{DefaultPickingPlugins, backends::raycast::RaycastPickCamera};
use bevy_sprite3d::Sprite3dPlugin;
use bevy_tweening::TweeningPlugin;
use lazy_static::lazy_static;

//
// Game Environment Variables
//
lazy_static! {
    pub(crate) static ref TIME_WAIT_TO_START: u64 = {
        let time: u64 = 15;
        time
    };
    pub(crate) static ref MAX_BUGS_ON_SCREEN: usize = {
        let time: usize = 20;
        time
    };
}

#[derive(Resource)]
pub(crate) struct OpenLinkResource(pub Box<dyn Fn(&str) + Sync + Send + 'static>);

#[derive(Clone, Default, Debug, Hash, States, PartialEq, Eq)]
pub(crate) enum GameState {
    #[default]
    MainPage,
    Game,
}

pub fn app(fullscreen: bool) -> App {
    let mode = if fullscreen {
        WindowMode::BorderlessFullscreen
    } else {
        WindowMode::Windowed
    };

    let mut app = App::new();
    app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        mode,
                        title: "Aspen Halls".to_string(),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        present_mode: bevy::window::PresentMode::AutoVsync,
                        decorations: false,
                        ..default()
                    }),
                    ..default()
                })
                .disable::<LogPlugin>(),
        )
        .add_plugins(DefaultPickingPlugins)
        .add_plugins((Sprite3dPlugin, TweeningPlugin));
    #[cfg(feature = "inspect")]
    app.add_plugins(WorldInspectorPlugin::new());
    app.add_state::<GameState>()
        .add_systems(Startup, setup_camera);

    app
}

fn setup_camera(mut cmd: Commands) {
    cmd.spawn((
        Camera3dBundle {
            projection: Projection::Orthographic(OrthographicProjection {
                scaling_mode: ScalingMode::FixedVertical(25.),
                ..default()
            }),
            transform: Transform::from_xyz(0., 0., 25.),
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::rgb_u8(227, 227, 227)),
                ..default()
            },
            ..default()
        },
        RaycastPickCamera,
    ));

    cmd.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(8., 16., 8.),
        ..default()
    });
}
