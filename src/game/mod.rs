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
    ahp::{
        engine::{leafwing_input_manager::prelude::ActionState, *},
        game::{GeneralSettings, TimeToLive, *},
    },
    game::{
        actors::ActorPlugin,
        audio::InternalAudioPlugin,
        game_world::GameWorldPlugin,
        input::ActionsPlugin,
        interface::InterfacePlugin,
        // interface::InterfacePlugin,
    },
};

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

/// are we in dungeon yet?
#[derive(
    Debug, Clone, Eq, PartialEq, Hash, States, Resource, Default, Reflect,
)]
pub enum GameProgress {
    /// homeroom
    #[default]
    Sanctuary,
    /// in dungeon now
    Dungeon,
}

/// plugin that holds all game functionality as plugin modules
pub struct DungeonGamePlugin;

impl Plugin for DungeonGamePlugin {
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
                    state_exists_and_equals(AppState::PlayingGame)
                        .and_then(run_once()),
                ),
                (time_to_live, zoom_control)
                    .run_if(in_state(AppState::PlayingGame)),
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
    mut settings: ResMut<GeneralSettings>,
    query_action_state: Query<
        &ActionState<action_maps::Gameplay>,
        Changed<ActionState<Gameplay>>,
    >,
) {
    let actions = match query_action_state.get_single() {
        Ok(action_state) => action_state,
        Err(error) => {
            warn!("issue getting player `ActionState<Gameplay>`: {error}");
            return;
        }
    };

    let multiplier = if actions.pressed(action_maps::Gameplay::Sprint) {
        10.0
    } else {
        1.0
    };

    if actions.pressed(action_maps::Gameplay::ZoomIn) {
        settings.camera_zoom += 0.05 * multiplier;
    } else if actions.pressed(action_maps::Gameplay::ZoomOut) {
        settings.camera_zoom -= 0.05 * multiplier;
    }
}

/// despawn any entity with `TimeToLive` timer thats finished
fn time_to_live(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut TimeToLive)>,
) {
    query.for_each_mut(|(entity, mut timer)| {
        if timer.tick(time.delta()).finished() {
            commands.entity(entity).despawn();
        }
    });
}
