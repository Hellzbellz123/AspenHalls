use belly::prelude::Elements;
use bevy::prelude::{
    info, state_exists, App, Commands, Event, EventReader, EventWriter,
    IntoSystemConfigs, NextState, OnEnter, Query, Res, ResMut, State,
    Update, With,
};
use bevy_rapier2d::prelude::{RapierConfiguration, TimestepMode};
use leafwing_input_manager::prelude::ActionState;

use crate::game::{
    actors::components::Player,
    input::action_maps::{self},
    interface::menus::pause_menu::PauseMenu,
    AppStage,
};

/// pause menu systems
pub mod pause_menu;
/// settings menu systems
pub mod settings_menu;
/// start menu systems
pub mod start_menu;

/// adds all
pub fn setup(app: &mut App) {
    app.add_event::<PausePlayEvent>().add_systems(
        Update,
        (menu_key_control, pause_game, control_rapier_timestep)
            .run_if(state_exists::<AppStage>()),
    );
    // app.add_systems(OnEnter(AppStage::PlayingGame), PauseMenu::hide);
    app.add_systems(OnEnter(AppStage::PauseMenu), PauseMenu::show);

    // TODO: Add menu systems
    start_menu::setup_menu(app);
    pause_menu::setup_menu(app);
    settings_menu::setup_menu(app);
}

/// updates menu state based on game stage
fn menu_key_control(
    game_state: Res<State<AppStage>>,
    input: Query<&ActionState<action_maps::Gameplay>, With<Player>>,
    mut ew: EventWriter<PausePlayEvent>,
) {
    if input.is_empty() {
        return;
    }
    let input = input.single();

    if input.just_pressed(action_maps::Gameplay::Pause) {
        match game_state.get() {
            AppStage::StartMenu => {
                ew.send(PausePlayEvent(EventType::Play))
            }
            AppStage::PauseMenu => {
                ew.send(PausePlayEvent(EventType::Resume))
            }
            AppStage::PlayingGame => {
                ew.send(PausePlayEvent(EventType::Pause))
            }
            _ => {}
        }
    }
}

/// event that starts game if in `AppStage::StartMenu`, or pauses/resumes game if in `AppStage::PlayingGame`
#[derive(Debug, Event)]
pub struct PausePlayEvent(EventType);

/// different events menu buttons can send
#[derive(Debug)]
pub enum EventType {
    /// event from start menu
    Play,
    /// event from pause menu
    Pause,
    /// event too pause menu
    Resume,
    /// event for go back menu
    Previous,
}

/// pauses rapier physics depending on `AppStage`
fn control_rapier_timestep(
    mut rapier_cfg: ResMut<RapierConfiguration>,
    game_state: Res<State<AppStage>>,
) {
    match game_state.get() {
        AppStage::PlayingGame => {
            rapier_cfg.timestep_mode = TimestepMode::Variable {
                max_dt: 1.0 / 60.0,
                time_scale: 1.0,
                substeps: 1,
            };
        }
        AppStage::PauseMenu | AppStage::StartMenu => {
            rapier_cfg.timestep_mode = TimestepMode::Variable {
                max_dt: 1.0 / 60.0,
                time_scale: 0.0,
                substeps: 1,
            };
        }
        _ => {}
    }
}

/// pause game and modify `TimeState`
pub fn pause_game(
    mut elements: Elements,
    mut cmds: Commands,
    mut pause_events: EventReader<PausePlayEvent>,
    game_state: Res<State<AppStage>>,
) {
    pause_events.iter().for_each(|event| match event.0 {
        EventType::Play => match game_state.get() {
            AppStage::StartMenu => {
                info!("in [StartMenu], setting [NextState] too [PlayingGame]");
                elements.select("div.start-menu-root").add_class("hidden");
                cmds.insert_resource(NextState(Some(AppStage::PlayingGame)));
            }
            _ => {
                info!("we should not hit this: Play");
            }
        },
        EventType::Pause => match game_state.get() {
            AppStage::PlayingGame => {
                info!("in [PlayingGame], setting [NextState] too [PauseGame]");
                elements
                    .select("div.pause-menu-root")
                    .remove_class("hidden");
                cmds.insert_resource(NextState(Some(AppStage::PauseMenu)));
            }
            _ => {
                info!("we should not hit this: PAUSE");
            }
        },
        EventType::Resume => match game_state.get() {
            AppStage::PauseMenu => {
                info!("in [PauseMenu], setting [NextState] too [PlayingGame]");
                elements.select("div.pause-menu-root").add_class("hidden");
                cmds.insert_resource(NextState(Some(AppStage::PlayingGame)));
            }
            _ => {
                info!("we should not hit this: RESUME");
            }
        },
        EventType::Previous => match game_state.get() {
            AppStage::PauseMenu => {
                info!("hiding settings menu and showing pause menu");
                elements
                    .select("div.settings-menu-root")
                    .add_class("hidden");
                elements
                    .select("div.pause-menu-root")
                    .remove_class("hidden");
            }
            AppStage::StartMenu => {
                info!("hiding settings menu and showing start menu");
                elements
                    .select("div.settings-menu-root")
                    .add_class("hidden");
                elements
                    .select("div.start-menu-root")
                    .remove_class("hidden");
            }
            AppStage::PlayingGame => {
                elements
                    .select("div.settings-menu-root")
                    .toggle_class("hidden");
            }
            _ => {
                info!("we should not hit this: RESUME");
            }
        },
    });
}
