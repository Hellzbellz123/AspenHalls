use bevy::prelude::*;

use crate::{game::GameStage, loading::assets::UiTextureHandles, utilities::game::SystemLabels};

// This plugin will display a splash screen with Bevy logo for 1 second before switching to the menu
pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        // As this plugin is managing the splash screen, it will focus on the state `GameState::Splash`

        app
            // When entering the state, spawn everything needed for this screen
            .add_system_set(
                SystemSet::on_enter(GameStage::Splash)
                    .with_system(
                        spawn_main_camera
                            .label(SystemLabels::InitSettings)
                            .before(SystemLabels::UpdateSettings),
                    )
                    .with_system(splash_setup.label(SystemLabels::UpdateSettings)),
            )
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
pub struct MainCamera {
    #[allow(dead_code)]
    pub is_active: bool,
}

// Newtype to use a `Timer` for this screen as a resource
#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

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
        .insert(MainCamera { is_active: true });
    info!("Main Camera Spawned");
}

fn splash_setup(mut commands: Commands, textures: Res<UiTextureHandles>) {
    info!("loading splash");
    // Display the logo
    commands
        .spawn(ImageBundle {
            style: Style {
                margin: UiRect::all(Val::Auto),
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..default()
            },
            image: UiImage(textures.splash_image.clone()),
            ..default()
        })
        .insert(OnSplashScreen);
    commands.insert_resource(SplashTimer(Timer::from_seconds(1.0, TimerMode::Once)));
    info!("spawning splash ImageBundle");
}

// Tick the timer, and change state when finished
fn countdown(
    mut game_state: ResMut<State<GameStage>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(GameStage::Playing).unwrap(); //TODO:change back too menu when updated too new kayakui version
    }
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in to_despawn.iter() {
        info!("despawning entity: {:#?}", entity);
        commands.entity(entity).despawn_recursive();
    }
}
