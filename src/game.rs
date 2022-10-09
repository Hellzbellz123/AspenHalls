use bevy::{app::App, prelude::*};

use bevy_inspector_egui::Inspectable;
use heron::{Gravity, PhysicsPlugin};
use leafwing_input_manager::prelude::ActionState;

use crate::{
    action_manager::{actions::PlayerBindables, bindings::ActionsPlugin},
    actors::{animation::GraphicsPlugin, enemies::EnemyPlugin, player::PlayerPlugin},
    audio::InternalAudioPlugin,
    game_world::MapSystemPlugin,
    ui::MenuPlugin,
    utilities::game::AppSettings,
};

#[derive(Debug, Clone, Component, Default, Reflect)]
#[reflect(Component)]
pub struct TimeInfo {
    pub time_step: f32,
    pub game_paused: bool,
    pub pause_menu: bool,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Component, Inspectable)]
pub enum GameStage {
    /// During the loading State the [`loading::LoadingPlugin`] will load our assets and display splash?!
    Loading,
    /// game "stage" for loading in splashscreen and spawning camera
    Splash,
    /// Here the menu is drawn and waiting for player interaction
    Menu,
    /// During this State the actual game logic is executed
    Playing,
}

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MenuPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(PhysicsPlugin::default())
            .insert_resource(Gravity::from(Vec3::new(0.0, 0.0, 0.0)))
            .add_plugin(MapSystemPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(EnemyPlugin)
            .add_plugin(GraphicsPlugin)
            .add_system_set(
                SystemSet::on_enter(GameStage::Playing).with_system(setup_time_state), // .with_system(zoom_control),
            )
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
