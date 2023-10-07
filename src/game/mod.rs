/// actors enemy, chests, anything
pub mod actors;
/// audio data for game
pub mod audio;
/// sanctuary and dungeon generator
pub mod game_world;
/// input from player
pub mod input;
/// Game `UserInterface` Module, contains interface plugin
pub mod interface;

use crate::{
    game::{
        actors::{
            components::TimeToLive,
            ActorPlugin,
        },
        audio::InternalAudioPlugin,
        game_world::GameWorldPlugin,
        input::{actions, ActionsPlugin},
        interface::InterfacePlugin,
    },
    loading::config::GeneralSettings,
};

use bevy::{app::App, prelude::*};
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
pub enum AppStage {
    /// During the loading State the [`loading::LoadingPlugin`] will load our assets and display splash?!
    #[default]
    Loading,
    /// Here the menu is drawn and waiting for player interaction
    StartMenu,
    /// playing game, some States are inserted here
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
            .add_plugins((
                InterfacePlugin,
                ActionsPlugin,
                InternalAudioPlugin,
                GameWorldPlugin,
                ActorPlugin,
            ))
            .add_systems(
                Update,
                (
                    setup_time_state.run_if(
                        state_exists_and_equals(AppStage::PlayingGame).and_then(run_once()),
                    ),
                    (time_to_live, zoom_control).run_if(in_state(AppStage::PlayingGame)),
                ),
            );
    }
}

/// setup initial time state
pub fn setup_time_state(mut time_info: ResMut<TimeInfo>) {
    *time_info = TimeInfo {
        time_step: 1.0,
        game_paused: false,
        pause_menu: false,
    }
}

/// zoom control
pub fn zoom_control(
    mut multiplier: Local<f32>,
    mut settings: ResMut<GeneralSettings>,
    query_action_state: Query<&ActionState<actions::Combat>>,
) {
    if query_action_state.is_empty() {
        return;
    }
    let actions = query_action_state.get_single().expect("no player?");

    if actions.pressed(actions::Combat::Sprint) {
        *multiplier = 10.0;
    }

    if actions.pressed(actions::Combat::ZoomIn) {
        settings.camera_zoom += 0.01 * *multiplier;
    } else if actions.pressed(actions::Combat::ZoomOut) {
        settings.camera_zoom -= 0.01 * *multiplier;
    }
}

/// despawn any entity with `TimeToLive` timer thats finished
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
