use bevy::{
    core_pipeline::{tonemapping::{DebandDither, Tonemapping}, clear_color::ClearColorConfig},
    prelude::*,
    render::{texture::{CompressedImageFormats, ImageType}, camera::ScalingMode, primitives::Frustum},
};
// use rust_embed::RustEmbed;
use crate::{game::AppStage, utilities::despawn_with};

/// Identifies the Main Camera
#[derive(Component, Reflect)]
pub struct MainCameraTag {
    /// true if active, false if not
    pub is_active: bool,
}

/// tag added too splashscreen entities that should be de-spawned after splashscreen
#[derive(Component)]
pub struct OnlySplashScreen;

/// `NewType` to use a `Timer` for splashscreen, if we need transitions we can use this
#[derive(Resource, Deref, DerefMut)]
pub struct SplashTimer(pub Timer);

/// This plugin will display a splash screen with Bevy logo for 1 second before switching to the menu
pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        // TODO: do some special trickery to make this system work awesome
        // As this plugin is managing the splash screen, it will focus on the state `GameState::Splash`

        app.add_systems(Startup, (spawn_main_camera, splash_setup).chain());
        app.add_systems(
            Update,
            despawn_with::<OnlySplashScreen>.run_if(not(in_state(AppStage::Loading))),
        );
    }
}

/// spawns main camera
fn spawn_main_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                is_active: true,
                order: 1,
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::AcesFitted,
            deband_dither: DebandDither::Enabled,
            projection: OrthographicProjection {
                near: 0.001,
                far: 999.0,
                viewport_origin: Vec2 { x: 0.5, y: 0.5 },
                scaling_mode: ScalingMode::WindowSize(10.0),
                scale: 3.5,
                ..default()
            },
            frustum: Frustum::default(),
            camera_2d: Camera2d { clear_color: ClearColorConfig::Custom(Color::GRAY) },
            // transform: todo!(),
            // global_transform: todo!(),
            // visible_entities: todo!(),
            ..default()
        },
        Name::new("MainCamera"),
        MainCameraTag { is_active: true },
        UiCameraConfig { show_ui: true },
    ));
    info!("Main Camera Spawned");
}

/// spawns splash, inserts splash timer
fn splash_setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    info!("loading splash");
    let img_bytes = include_bytes!("splashL.png");
    let splash_image = Image::from_buffer(
        img_bytes,
        ImageType::Extension("png"),
        CompressedImageFormats::empty(),
        true,
    )
    .unwrap();
    let img: Handle<Image> = images.add(splash_image);

    // Display the logo
    info!("spawning splash ImageBundle");
    commands
        .spawn(ImageBundle {
            style: Style {
                margin: UiRect::all(Val::Px(0.0)),
                min_width: Val::Percent(100.0),
                min_height: Val::Percent(100.0),
                max_width: Val::Percent(100.0),
                max_height: Val::Percent(100.0),
                ..default()
            },
            image: UiImage {
                texture: img,
                ..default()
            },
            ..default()
        })
        .insert(OnlySplashScreen);
}
