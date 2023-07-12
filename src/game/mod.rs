/// actors enemy, chests, anything
pub mod actors;
/// audio data for game
pub mod audio;
/// homeworld and dungeon generator
pub mod game_world;
/// input from player
pub mod input;
/// ui related functionality
pub mod ui;

use crate::{
    app_config::GeneralSettings,
    game::{
        actors::{
            components::{Player, TimeToLive},
            ActorPlugin,
        },
        audio::InternalAudioPlugin,
        game_world::GameWorldPlugin,
        input::{actions, ActionsPlugin},
        ui::BevyUiPlugin,
    },
};

use bevy::{app::App, prelude::*};
use bevy_rapier2d::prelude::{RapierConfiguration, TimestepMode};
use leafwing_input_manager::prelude::ActionState;

/// time info for game,
#[derive(Debug, Clone, Component, Default, Resource, Reflect)]
pub struct TimeInfo {
    /// set rapier timestep
    pub time_step: f32,
    /// pause check
    pub game_paused: bool,
    /// in pause menu
    pub pause_menu: bool,
}

/// main game state loop
#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Resource, Default, Reflect)]
pub enum GameStage {
    /// During the loading State the [`loading::LoadingPlugin`] will load our assets and display splash?!
    #[default]
    Loading,
    /// Here the menu is drawn and waiting for player interaction
    StartMenu,
    /// this is technically a [`PlaySubStage`] substate. not fully implemented yet however u,
    PlayingGame, //(PlaySubStage),
    /// Game Paused in this state, rapier timestep set too 0.0, no physics, ai is also stopped
    PauseMenu,
    /// game failed to load an asset
    FailedLoading,
}

/// are we in dungeon yet?
#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Resource, Default, Reflect)]
pub enum GameProgress {
    /// homeroom
    #[default]
    Sanctuary,
    /// in dungeon now
    Dungeon,
}

/// plugin for all game functionality
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TimeInfo {
            time_step: 1.0,
            game_paused: false,
            pause_menu: false,
        })
        //game stuff after initial Game State setup
        .add_plugin(BevyUiPlugin)
        .add_plugin(ActionsPlugin)
        .add_plugin(InternalAudioPlugin)
        .add_plugin(GameWorldPlugin)
        .add_plugin(ActorPlugin)
        .add_system(pause_game)
        .add_system(setup_time_state.in_schedule(OnEnter(GameStage::PlayingGame)))
        .add_systems((time_to_live, zoom_control).in_set(OnUpdate(GameStage::PlayingGame)));
    }
}

/// setup initial time state
pub fn setup_time_state(mut timeinfo: ResMut<TimeInfo>) {
    *timeinfo = TimeInfo {
        time_step: 1.0,
        game_paused: false,
        pause_menu: false,
    }
}

/// pause game and modify timestate
pub fn pause_game(
    mut cmds: Commands,
    gamestate: Res<State<GameStage>>,
    mut rapiercfg: ResMut<RapierConfiguration>,
    input_query: Query<&ActionState<actions::Combat>, With<Player>>,
) {
    if input_query.is_empty() {
        return;
    }

    let input = input_query
        .get_single()
        .expect("should always only be one input");

    match gamestate.0 {
        GameStage::PlayingGame => {
            rapiercfg.timestep_mode = TimestepMode::Variable {
                max_dt: 1.0 / 60.0,
                time_scale: 1.0,
                substeps: 1,
            };
        }
        GameStage::PauseMenu => {
            rapiercfg.timestep_mode = TimestepMode::Variable {
                max_dt: 1.0 / 60.0,
                time_scale: 0.0,
                substeps: 1,
            };
        }
        _ => {
            return;
        }
    }

    if input.just_pressed(actions::Combat::Pause) {
        match gamestate.0 {
            GameStage::PlayingGame => {
                cmds.insert_resource(NextState(Some(GameStage::PauseMenu)));
            }
            GameStage::PauseMenu => {
                cmds.insert_resource(NextState(Some(GameStage::PlayingGame)));
            }
            _ => {}
        }
    }
}

/// zoom control
pub fn zoom_control(
    mut settings: ResMut<GeneralSettings>,
    query_action_state: Query<&ActionState<actions::Combat>>,
) {
    if query_action_state.is_empty() {
        return;
    }

    let actions = query_action_state.get_single().expect("no player?");

    let mut multiplier = 1.0;
    if actions.pressed(actions::Combat::ZoomIn) {
        if actions.pressed(actions::Combat::Sprint) {
            multiplier = 10.0;
        }
        settings.camera_zoom += 0.01 * multiplier;
    } else if actions.pressed(actions::Combat::ZoomOut) {
        if actions.pressed(actions::Combat::Sprint) {
            multiplier = 10.0;
        }
        settings.camera_zoom -= 0.01 * multiplier;
    }
}

/// despawns any entity with TimeToLive timer thats finished
fn time_to_live(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut TimeToLive)>,
) {
    query.for_each_mut(|(entity, mut timer)| {
        if timer.0.tick(time.delta()).finished() {
            commands.entity(entity).despawn();
        }
    });
}
