pub mod actors;
pub mod audio;
pub mod game_world;
pub mod input;
pub mod ui;

use crate::{
    app_config::GeneralSettings,
    components::actors::general::{MovementState, TimeToLive},
    game::{
        actors::ActorPlugin, audio::InternalAudioPlugin, game_world::GameWorldPlugin,
        input::ActionsPlugin, ui::BevyUiPlugin,
    },
};
use bevy::{app::App, prelude::*};

use bevy_rapier2d::prelude::{RapierConfiguration, TimestepMode};
use leafwing_input_manager::prelude::ActionState;

use self::input::actions;

#[derive(Debug, Clone, Component, Default, Resource, Reflect)]
pub struct TimeInfo {
    pub time_step: f32,
    pub game_paused: bool,
    pub pause_menu: bool,
}

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

// TODO: use this
#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Resource, Default, Reflect)]
pub enum GameProgress {
    #[default]
    Sanctuary,
    Dungeon,
}

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

pub fn setup_time_state(mut timeinfo: ResMut<TimeInfo>) {
    *timeinfo = TimeInfo {
        time_step: 1.0,
        game_paused: false,
        pause_menu: false,
    }
}

pub fn pause_game(
    mut cmds: Commands,
    gamestate: Res<State<GameStage>>,
    mut rapiercfg: ResMut<RapierConfiguration>,
    input_query: Query<&ActionState<actions::Combat>, With<MovementState>>,
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

pub fn zoom_control(
    mut settings: ResMut<GeneralSettings>,
    query_action_state: Query<&ActionState<actions::Combat>>,
) {
    if query_action_state.is_empty() {
        return;
    }

    let actions = query_action_state.get_single().expect("no player?");

    if actions.pressed(actions::Combat::ZoomIn) {
        settings.camera_zoom += 0.01;
    } else if actions.pressed(actions::Combat::ZoomOut) {
        settings.camera_zoom -= 0.01;
    }
}

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