use crate::prelude::{engine::*, game::*};

/// Identifies the Main Camera
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct MainCamera {
    /// is this main camera used over some other camera?
    pub in_use: bool,

    /// how fast should this camera move towards the player?
    pub camera_speed: f32,

    /// multiplied by player velocity then added too the camera target
    pub look_ahead_factor: f32,

    /// change camera lerp speed when player velocity is below this magnitude
    pub lerp_change_magnitude: f32,

    /// camera speed when player is standing still
    pub changed_speed: f32,
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
        app.add_systems(Startup, spawn_main_camera);
        app.add_systems(OnEnter(AppState::Loading), splash_setup);
        app.add_systems(OnExit(AppState::Loading), despawn_with::<OnlySplashScreen>);
    }
}

/// spawns main camera
fn spawn_main_camera(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.9,
    });
    commands.spawn((
        Name::new("MainCamera"),
        MainCamera {
            in_use: true,
            look_ahead_factor: 1.0,
            camera_speed: 5.0,
            lerp_change_magnitude: 0.5,
            changed_speed: 0.2,
        },
        Camera2dBundle {
            camera: Camera {
                is_active: true,
                order: 1,
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
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
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Default,
            },
            transform: Transform::from_xyz(200.0, 100.0, 999.0),
            ..default()
        },
        FogSettings {
            color: Color::RED,
            // directional_light_color: todo!(),
            // directional_light_exponent: todo!(),
            falloff: FogFalloff::from_visibility_contrast_colors(
                100.0,
                1.0,
                Color::RED,
                Color::GRAY,
            ),
            ..default()
        },
        UiCameraConfig { show_ui: true },
    ));
    info!("Main Camera Spawned");
}

/// spawns splash, inserts splash timer
fn splash_setup(
    // mut images: ResMut<Assets<Image>>,
    // window: Query<&Window>,
    mut commands: Commands,
    init_assets: Res<AspenInitHandles>,
) {
    info!("loading splash");
    // let window = window.single();
    // let img_bytes = include_bytes!("../../assets-build/splashL.png");
    // let splash_image = Image::from_buffer(
    //     img_bytes,
    //     ImageType::Extension("png"),
    //     CompressedImageFormats::empty(),
    //     true,
    // )
    // .unwrap();
    // let img: Handle<Image> = images.add(Image::from_dynamic(
    //     splash_image
    //         .try_into_dynamic()
    //         .unwrap_or_default()
    //         .resize_exact(window.physical_width(), window.physical_height(), FilterType::Nearest),
    //     true,
    // ));
    // Display the logo
    info!("spawning splash ImageBundle");
    commands.spawn((
        Name::new("SplashScreenImage"),
        OnlySplashScreen,
        ImageBundle {
            style: Style {
                margin: UiRect::all(Val::Px(0.0)),
                min_width: Val::Percent(100.0),
                min_height: Val::Percent(100.0),
                max_width: Val::Percent(100.0),
                max_height: Val::Percent(100.0),
                ..default()
            },
            image: UiImage {
                texture: init_assets.img_splashscreen.clone(),
                ..default()
            },
            ..default()
        },
    ));
}
