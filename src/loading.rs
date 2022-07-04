use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

/// This plugin loads all assets using [AssetLoader] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at https://bevy-cheatbook.github.io/features/assets.html
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        AssetLoader::new(GameState::Loading)
            .with_collection::<FontAssets>()
            .with_collection::<AudioAssets>()
            .with_collection::<TextureAssets>()
            .continue_to_state(GameState::Menu)
            .build(app);
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see https://github.com/NiklasEi/bevy_asset_loader)

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "audio/ost/expansion.ogg")]
    pub gamesoundtrack: Handle<AudioSource>,
}

#[derive(AssetCollection)]
pub struct TextureAssets {
    #[asset(path = "textures/Player/player.png")]
    pub texture_player: Handle<Image>,
}

pub(crate) mod splash {
    use bevy::prelude::*;

    use super::{despawn_screen, GameState};

    // This plugin will display a splash screen with Bevy logo for 1 second before switching to the menu
    pub struct SplashPlugin;

    impl Plugin for SplashPlugin {
        fn build(&self, app: &mut App) {
            // As this plugin is managing the splash screen, it will focus on the state `GameState::Splash`
            app
                // When entering the state, spawn everything needed for this screen
                .add_system_set(SystemSet::on_enter(GameState::Loading).with_system(splash_setup))
                // When exiting the state, despawn everything that was spawned for this screen
                .add_system_set(
                    SystemSet::on_exit(GameState::Loading)
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

    fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        let icon = asset_server.load("textures/splashscreen.png");
        // Display the logo
        commands
            .spawn_bundle(ImageBundle {
                style: Style {
                    // This will center the logo
                    margin: Rect::all(Val::Auto),
                    // This will set the logo to be 200px wide, and auto adjust its height
                    size: Size::new(Val::Auto, Val::Auto),
                    ..default()
                },
                image: UiImage(icon),
                ..default()
            })
            .insert(OnSplashScreen);
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
        commands.entity(entity).despawn_recursive();
    }
}
