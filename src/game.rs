use bevy::{app::App, prelude::*};

use leafwing_input_manager::prelude::ActionState;

use crate::{
    actions::{bindings::ActionsPlugin, PlayerActions},
    actors::ActorPlugin,
    audio::InternalAudioPlugin,
    components::actors::general::TimeToLive,
    game_world::MapSystemPlugin,
    // ui::MenuPlugin,
    utilities::game::AppSettings,
};

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
    PlaySubStage, //(PlaySubStage),
    /// game failed to load an asset
    FailedLoading,
}

/// play substate, homeworld, in dungeon, or some other activity
// #[derive(Debug, Clone, Eq, PartialEq, Hash, Component, Reflect, Resource, Default)]
// pub enum PlaySubStage {
//     #[default]
//     /// not loaded yet
//     NotLoaded,
//     /// homeworld for selecting your character and other things
//     InHomeWorld,
//     /// actual random levels and other shit
//     InDungeon,
// }

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_plugin(MenuPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(MapSystemPlugin)
            .add_plugin(ActorPlugin)
            .add_system(setup_time_state.in_schedule(OnEnter(GameStage::PlaySubStage)))
            .add_systems((time_to_live, zoom_control).in_set(OnUpdate(GameStage::PlaySubStage)));
    }
}

pub fn setup_time_state(mut timeinfo: ResMut<TimeInfo>) {
    *timeinfo = TimeInfo {
        time_step: 1.0,
        game_paused: false,
        pause_menu: false,
    }
}

pub fn zoom_control(
    mut settings: ResMut<AppSettings>,
    query_action_state: Query<&ActionState<PlayerActions>>,
) {
    if query_action_state.is_empty() {
        return;
    }

    let actions = query_action_state.get_single().expect("no player?");

    if actions.pressed(PlayerActions::ZoomIn) {
        settings.camera_zoom += 0.01;
    } else if actions.pressed(PlayerActions::ZoomOut) {
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
