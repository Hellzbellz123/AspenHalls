use bevy::{asset::LoadState, prelude::*};

use crate::{
    components::{OnSplashScreen, SplashTimer},
    game::GameStage,
};

//builds menus for vanillacoffee, both ingame and main menu
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_system_set(
            SystemSet::on_update(GameStage::Menu).with_system(pass_to_play::<OnSplashScreen>),
        );
        // .add_system_set(SystemSet::on_enter(GameStage::Menu).with_system(draw_mainmenu))
        // .add_system_set(SystemSet::on_exit(GameStage::Menu).with_system(destroy_menu))
        // .add_system_set(SystemSet::on_update(GameStage::Playing).with_system(pause_game));
    }
}

fn pass_to_play<T: Component>(
    assetserver: ResMut<AssetServer>,

    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
    to_despawn: Query<Entity, With<T>>,
    mut commands: Commands,
    mut game_state: ResMut<State<GameStage>>,
) {
    let img: Handle<Image> = assetserver.load("splash/splashL.png");
    let mut state_pushed;
    let imgloadstate = assetserver.get_load_state(img);

    if imgloadstate == LoadState::Loaded
        && timer.tick(time.delta()).finished()
        && game_state.current() == &GameStage::Menu
    {
        state_pushed = false;

        // info!("splash asset loaded");
        // game_state.set(GameStage::Menu).unwrap(); //TODO:change back too menu when updated too new kayakui version
        info!("pushing playing state too state stack");
        if !state_pushed {
            game_state
                .push(GameStage::Playing)
                .expect("couldnt push state to stack");

            state_pushed = true;

            for entity in to_despawn.iter() {
                info!("despawning entity: {:#?}", entity);
                commands.entity(entity).despawn_recursive();
            }
        } else if state_pushed {
            info!(" do nothing?")
        }
    }
}
