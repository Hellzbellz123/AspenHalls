use bevy::prelude::*;

use crate::{game::*, loading::assets::UiTextureAssets};

// This plugin will display a splash screen with Bevy logo for 1 second before switching to the menu
pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        // As this plugin is managing the splash screen, it will focus on the state `GameState::Splash`
        app
            // When entering the state, spawn everything needed for this screen
            .add_system_set(SystemSet::on_enter(GameStage::Splash).with_system(splash_setup))
            .add_system_set(SystemSet::on_update(GameStage::Splash).with_system(countdown))
            // When exiting the state, despawn everything that was spawned for this screen
            .add_system_set(
                SystemSet::on_exit(GameStage::Splash).with_system(despawn_screen::<OnSplashScreen>),
            );
    }
}

// Tag component used to tag entities added on the splash screen
#[derive(Component)]
struct OnSplashScreen;

#[derive(Component)]
struct MainCamera {
    #[allow(dead_code)]
    is_active: bool,
}

// Newtype to use a `Timer` for this screen as a resource
#[derive(Deref, DerefMut)]
struct SplashTimer(Timer);

fn splash_setup(mut commands: Commands, textures: Res<UiTextureAssets>) {
    info!("loading splash");

    commands
        .spawn_bundle(Camera2dBundle {
            camera: Camera {
                priority: 1,
                is_active: true,
                ..default()
            },
            ..default()
        })
        .insert(Name::new("main camera"))
        .insert(MainCamera { is_active: true });

    // Display the logo
    commands
        .spawn_bundle(ImageBundle {
            style: Style {
                margin: UiRect::all(Val::Auto),
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..default()
            },
            image: UiImage(textures.splash_image.clone()),
            ..default()
        })
        .insert(OnSplashScreen);
    commands.insert_resource(SplashTimer(Timer::from_seconds(1.0, false)));
    info!("spawning splash ImageBundle");
}

// Tick the timer, and change state when finished
fn countdown(
    mut game_state: ResMut<State<GameStage>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(GameStage::Menu).unwrap();
    }
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in to_despawn.iter() {
        info!("despawning entity: {:#?}", entity);
        commands.entity(entity).despawn_recursive();
    }
}
