use bevy::{app::App, prelude::*};

use bevy_inspector_egui::Inspectable;
use leafwing_input_manager::prelude::ActionState;

use crate::{
    action_manager::{actions::PlayerBindables, bindings::ActionsPlugin},
    actors::ActorPlugin,
    audio::InternalAudioPlugin,
    components::actors::general::TimeToLive,
    game_world::MapSystemPlugin,
    // ui::MenuPlugin,
    utilities::game::AppSettings,
};

#[derive(Debug, Clone, Component, Default, Resource, Inspectable)]
pub struct TimeInfo {
    pub time_step: f32,
    pub game_paused: bool,
    pub pause_menu: bool,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Component, Inspectable, Resource, Default)]
pub enum GameStage {
    /// During the loading State the [`loading::LoadingPlugin`] will load our assets and display splash?!
    Loading,
    /// Here the menu is drawn and waiting for player interaction
    #[default]
    Menu,
    /// settings state for menu
    Settings,
    /// During this State the actual game logic is executed
    Playing,
    /// game failed to load an asset
    FailedLoading,
}

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_plugin(MenuPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(MapSystemPlugin)
            .add_plugin(ActorPlugin)
            .add_system_set(
                SystemSet::on_enter(GameStage::Playing).with_system(setup_time_state), // .with_system(zoom_control),
            )
            .add_system_set(SystemSet::on_update(GameStage::Playing).with_system(time_to_live))
            .add_system(zoom_control);
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
    query_action_state: Query<&ActionState<PlayerBindables>>,
) {
    if !query_action_state.is_empty() {
        let actions = query_action_state.get_single().expect("no ents?");

        if actions.pressed(PlayerBindables::ZoomIn) {
            settings.camera_zoom += 0.01;
        } else if actions.pressed(PlayerBindables::ZoomOut) {
            settings.camera_zoom -= 0.01;
        }
    }
}

fn time_to_live(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut TimeToLive)>,
) {
    for (entity, mut timer) in query.iter_mut() {
        if timer.0.tick(time.delta()).finished() {
            commands.entity(entity).despawn();
        }
    }
}
