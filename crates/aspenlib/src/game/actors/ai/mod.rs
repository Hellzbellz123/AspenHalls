use std::time::Duration;

use bevy::prelude::*;
use big_brain::{
    actions::ActionState,
    prelude::{Action, Score, Thinker},
    thinker::Actor,
    BigBrainPlugin,
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
    bundles::StupidAiBundle,
    game::actors::ai::components::{
        AIChaseAction, AICombatConfig, AIShootAction, AIShootConfig, AIWanderAction, AIWanderConfig,
    },
    loading::custom_assets::actor_definitions::AiSetupConfig,
    prelude::game::{AttackScorer, ChaseScorer},
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
                AIShootConfig,
                AIChaseAction,
                AIWanderAction,
                AIShootAction
            ]
        );

        app.add_plugins((BigBrainPlugin::new(Update), StupidAiPlugin))
            .add_systems(Update, initialize_ai);
    }
}

/// entity tag for all `big_brain` entity's too be parented too
/// clean hierarchy plz
#[derive(Debug, Component)]
struct BigBrainContainerTag;

fn initialize_ai(
    mut commands: Commands,
    ai_controlled: Query<(Entity, &AiSetupConfig, &GlobalTransform)>,
) {
    for (character, who_should_control, pos) in &ai_controlled {
        match who_should_control {
            AiSetupConfig::Player => {}
            AiSetupConfig::GameAI(ai_type) => match ai_type {
                components::AiType::Stupid => {
                    //TODO: get definition and use values from definition
                    commands.entity(character).insert(StupidAiBundle {
                        combat_config: AICombatConfig {
                            chase_start: 10,
                            chase_end: 16,
                            shoot_range: 6,
                            personal_space: 3,
                            runaway_hp: 20.0,
                        },
                        wander_config: AIWanderConfig {
                            wander_target: None,
                            spawn_position: Some(pos.translation().truncate()),
                            wander_distance: 15,
                        },
                        shoot_config: AIShootConfig {
                            find_target_range: 6,
                            timer: Timer::new(Duration::from_secs_f32(0.5), TimerMode::Once),
                            should_shoot: false,
                            can_shoot: false,
                        },
                        thinker: Thinker::build()
                            .picker(big_brain::pickers::Highest)
                            .when(ChaseScorer, AIChaseAction)
                            .when(AttackScorer, AIShootAction)
                            .otherwise(AIWanderAction),
                    });
                }
                a => {
                    warn!("AI type unimplemented! {:?}", a)
                }
            },
        }
        commands.entity(character).remove::<AiSetupConfig>();
    }
}
