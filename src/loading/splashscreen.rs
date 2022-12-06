use bevy::prelude::*;
// use rust_embed::RustEmbed;
use crate::{
    components::{MainCameraTag, OnSplashScreen, SplashTimer},
    game::GameStage,
    utilities::game::SystemLabels,
};

// This plugin will display a splash screen with Bevy logo for 1 second before switching to the menu
pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        // As this plugin is managing the splash screen, it will focus on the state `GameState::Splash`

        app.add_system_set(
            SystemSet::on_enter(GameStage::Loading)
                .with_system(
                    spawn_main_camera
                        .label(SystemLabels::InitSettings)
                        .before(SystemLabels::UpdateSettings),
                )
                .with_system(splash_setup.label(SystemLabels::UpdateSettings)),
        );
    }
}

fn spawn_main_camera(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle {
            camera: Camera {
                priority: 1,
                is_active: true,
                ..default()
            },
            transform: Transform::from_xyz(-60.0, 1090.0, 8.0),
            ..default()
        })
        .insert(Name::new("Main Camera"))
        .insert(MainCameraTag { is_active: true });
    info!("Main Camera Spawned");
}

fn splash_setup(mut commands: Commands, assetserver: ResMut<AssetServer>) {
    info!("loading splash");
    let img = assetserver.load("images/splash/splashL.png");

    // Display the logo
    info!("spawning splash ImageBundle");
    commands
        .spawn(ImageBundle {
            style: Style {
                margin: UiRect::all(Val::Auto),
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..default()
            },
            image: UiImage(img),
            ..default()
        })
        .insert(OnSplashScreen);
    commands.insert_resource(SplashTimer(Timer::from_seconds(3.0, TimerMode::Once)));
}
