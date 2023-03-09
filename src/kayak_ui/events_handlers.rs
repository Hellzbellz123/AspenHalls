use bevy::prelude::*;

use crate::game::{self, TimeInfo};

pub struct PlayButtonEvent;
pub struct AppExitEvent;

pub fn play_button_event(
    events: EventReader<PlayButtonEvent>,
    mut current_state: ResMut<bevy::prelude::State<game::GameStage>>,
    // mut commands: Commands,
    mut timeinfo: ResMut<TimeInfo>,
) {
    if !events.is_empty() {
        if *current_state.current() == game::GameStage::StartMenu {
            current_state
                .push(game::GameStage::PlaySubStage)
                .expect("couldnt set state, weird");
            info!(
                "play button was pressed, current state: {:?}",
                current_state
            )
        }

        if *current_state.current() == game::GameStage::PlaySubStage {
            info!("already playing, menu probably open, assuming close menu resume game");
            // commands.remove_resource::<BevyContext>();
            timeinfo.pause_menu = false;
            timeinfo.game_paused = false;
            timeinfo.time_step = 1.0;
        }
        events.clear();
    }
}
