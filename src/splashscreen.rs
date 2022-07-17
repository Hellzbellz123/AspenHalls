use bevy::prelude::*;

pub mod splash {
    use crate::loading::UiTextureAssets;
    use crate::splashscreen::despawn_screen;
    use crate::GameState;
    use bevy::prelude::*;

    // This plugin will display a splash screen with Bevy logo for 1 second before switching to the menu
    pub struct SplashPlugin;

    impl Plugin for SplashPlugin {
        fn build(&self, app: &mut App) {
            // As this plugin is managing the splash screen, it will focus on the state `GameState::Splash`
            app
                // When entering the state, spawn everything needed for this screen
                .add_system_set(SystemSet::on_enter(GameState::Splash).with_system(splash_setup))
                .add_system_set(SystemSet::on_update(GameState::Splash).with_system(countdown))
                // When exiting the state, despawn everything that was spawned for this screen
                .add_system_set(
                    SystemSet::on_exit(GameState::Splash)
                        .with_system(despawn_screen::<OnSplashScreen>),
                );
        }
    }

    // Tag component used to tag entities added on the splash screen
    #[derive(Component)]
    struct OnSplashScreen;

    // Newtype to use a `Timer` for this screen as a resource
    #[derive(Deref, DerefMut)]
    struct SplashTimer(Timer);

    fn splash_setup(mut commands: Commands, textures: Res<UiTextureAssets>) {
        info!("loading splash");
        commands.spawn_bundle(UiCameraBundle::default());
        // Display the logo
        commands
            .spawn_bundle(ImageBundle {
                style: Style {
                    margin: Rect::all(Val::Auto),
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
        mut game_state: ResMut<State<GameState>>,
        time: Res<Time>,
        mut timer: ResMut<SplashTimer>,
    ) {
        if timer.tick(time.delta()).finished() {
            game_state.set(GameState::Menu).unwrap();
        }
    }
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in to_despawn.iter() {
        info!("despawning entity: {:#?}", entity);
        commands.entity(entity).despawn_recursive();
    }
}