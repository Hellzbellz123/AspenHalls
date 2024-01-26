use bevy::prelude::*;
use big_brain::{prelude::*, BigBrainPlugin};
use std::time::Duration;

use crate::{
    game::characters::{
        ai::components::{
            AIChaseAction, AICombatConfig, AIShootAction, AIShootConfig, AIWanderAction,
            AIWanderConfig, AiType, AttackScorer, ChaseScorer,
        },
        player::PlayerSelectedHero,
    },
    register_types,
};

use self::stupid_ai::StupidAiPlugin;

/// ai components
pub mod components;
/// stupid ai stuff
pub mod stupid_ai;

/// handles different AI classes
pub struct AIPlugin;

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

/// finds all characters wanting ai and adds required ai components
#[allow(clippy::type_complexity)]
fn initialize_ai(
    mut commands: Commands,
    ai_controlled: Query<
        (Entity, &AiType, &GlobalTransform),
        (Added<AiType>, Without<PlayerSelectedHero>),
    >,
) {
    for (character, wanted_ai, pos) in &ai_controlled {
        match wanted_ai {
            AiType::Stupid => {
                //TODO: get definition and use values from definition
                commands
                    .entity(character)
                    .insert(stupid_ai::StupidAiBundle {
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
            AiType::Boss => error!("ai type not implemented"),
            AiType::Critter => error!("ai type not implemented"),
            AiType::PlayerPet => error!("ai type not implemented"),
            AiType::FollowerHero => error!("ai type not implemented"),
        }

        commands.entity(character).remove::<AiType>();
    }
}
