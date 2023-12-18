use bevy::prelude::*;
use big_brain::{
    prelude::{Action, Score, Thinker},
    thinker::{Actor, ActionSpan},
    BigBrainPlugin, actions::ActionState,
};

use self::stupid_ai::StupidAiPlugin;

/// ai components
pub mod components;
/// stupid ai stuff
pub mod stupid_ai;
/// util functions
pub mod utility;

/// handles different AI classes
pub struct AIPlugin;

use crate::{
    ahp::game::{AttackScorer, ChaseScorer},
    game::actors::ai::components::{AICombatConfig, AIWanderConfig, AIChaseConfig, AIShootConfig, AIChaseAction, AIWanderAction, AIShootAction},
    register_types,
};

impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        register_types!(
            app,
            [
                Actor,
                ActionState,
                Thinker,
                Score,
                Action,
                ChaseScorer,
                AttackScorer,
                AICombatConfig,
                AIWanderConfig,
                AIChaseConfig,
                AIShootConfig,
                AIChaseAction,
                AIWanderAction,
                AIShootAction
            ]
        );

        app.add_plugins((BigBrainPlugin::new(Update), StupidAiPlugin))
        .add_systems(Update, parent_brains_to_target_actor);
    }
}

/// entity tag for all `big_brain` entity's too be parented too
/// clean hierarchy plz
#[derive(Debug, Component)]
struct BigBrainContainerTag;

//TODO: make this system insert `InspectorIgnore` component on each
/// this feels like a dirty hack but it totally works!!!
/// ALL HAIL THE ENTITY HIERARCHY!!!
fn parent_brains_to_target_actor(
    mut commands: Commands,
    brain_query: Query<(Entity, &Actor), Without<Parent>>,
    // brain_container_query: Query<Entity, With<BigBrainContainerTag>>,
    // thinkers: Query<&Thinker>,
) {
    // for thinker in &thinkers {
    //     info!("thinker: {:?}", thinker)
    // }

    // if brain_container_query.is_empty() {
    //     commands.spawn((BigBrainContainerTag, Name::new("BigBrainContainer")));
    //     return;
    // }

    for brain in &brain_query {
        match commands.get_entity(brain.0) {
            Some(mut e) => {
                e.set_parent(brain.1 .0);
            }
            None => {
                warn!("Could not parent Actor, did not exist");
                return;
            }
        }
    }
}
