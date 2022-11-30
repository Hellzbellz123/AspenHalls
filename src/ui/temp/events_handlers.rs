use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::{
    action_manager::actions::PlayerBindables,
    components::actors::general::ActorState,
    game::{self, TimeInfo},
    loading::assets::{FontHandles, UiTextureHandles},
};

pub struct PlayButtonEvent;
pub struct AppExitEvent;

pub fn destroy_menu(_commands: Commands) {
    // commands.remove_resource::<BevyContext>();
}

pub fn play_button_event(
    events: EventReader<PlayButtonEvent>,
    mut current_state: ResMut<bevy::prelude::State<game::GameStage>>,
    // mut commands: Commands,
    mut timeinfo: ResMut<TimeInfo>,
) {
    if !events.is_empty() {
        if *current_state.current() == game::GameStage::Menu {
            current_state
                .push(game::GameStage::Playing)
                .expect("couldnt set state, weird");
            info!(
                "play button was pressed, current state: {:?}",
                current_state
            )
        }

        if *current_state.current() == game::GameStage::Playing {
            info!("already playing, menu probably open, assuming close menu resume game");
            // commands.remove_resource::<BevyContext>();
            timeinfo.pause_menu = false;
            timeinfo.game_paused = false;
            timeinfo.time_step = 1.0;
        }
        events.clear();
    }
}

pub fn pause_game(
    // bevytime: Res<Time>,
    mut timeinfo: ResMut<TimeInfo>,
    input_query: Query<&ActionState<PlayerBindables>, With<ActorState>>,
    commands: Commands,
    _font_assets: Res<FontHandles>,
    _ui_assets: Res<UiTextureHandles>,
    // image_manager: ResMut<ImageManager>,
    // font_mapping: ResMut<FontMapping>,
) {
    let action_state = input_query.single();
    let mut timeinfo = timeinfo.as_mut();

    if action_state.just_pressed(PlayerBindables::Pause) {
        if timeinfo.pause_menu {
            destroy_menu(commands);
            timeinfo.pause_menu = false;
            timeinfo.game_paused = false;
            timeinfo.time_step = 1.0;
        } else {
            timeinfo.pause_menu = true;
            timeinfo.game_paused = true;
            timeinfo.time_step = 0.;
        }
        // info!("pause action pressed, {:?} \n {:?}", timeinfo, bevytime);
    }
}
