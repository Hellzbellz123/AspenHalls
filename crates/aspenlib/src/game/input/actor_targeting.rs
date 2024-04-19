use bevy::{prelude::*, window::PrimaryWindow};
use leafwing_input_manager::action_state::ActionState;

use crate::{
    game::input::{action_maps, AspenInputSystemSet},
    loading::splashscreen::MainCamera,
    AppState,
};

pub struct ActorTargetingPlugin;

impl Plugin for ActorTargetingPlugin {
    fn build(&self, app: &mut App) {
        // TODO: brainstorm actor targeting system
    }
}

#[derive(Debug, Component, Reflect, Default)]
#[reflect(Component)]
pub struct AspenTargetingReticle {
    current_target: Option<Entity>,
}
