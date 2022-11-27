// all credit for this goes to Shane Satterfield @ https://github.com/shanesatterfield
// for being the only real useful example of big-brain as far as im concerned
use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;
use big_brain::{
    prelude::{ActionState, Actor, Score},
    BigBrainStage,
};

use crate::{
    components::actors::{
        ai::{AIAggroDistance, AIAttackAction, AIAttackTimer, AICanMeander, AIEnemy, AIIsAggroed},
        animation::FacingDirection,
        general::{ActorState, Player},
    },
    game::TimeInfo,
};

pub struct SimpleAIPlugin;

impl Plugin for SimpleAIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(BigBrainStage::Actions, aggro_system)
            // .add_system_to_stage(BigBrainStage::Actions, random_wander_system)
            .add_system_to_stage(BigBrainStage::Scorers, aggro_score_system);
    }
}

fn aggro_system(
    timeinfo: ResMut<TimeInfo>,
    player_query: Query<&Transform, With<Player>>,
    #[allow(clippy::type_complexity)] mut enemy_query: Query<(
        &Transform,
        &mut Velocity,
        &AIAggroDistance,
        &mut AIAttackTimer,
        &mut ActorState,
        &mut TextureAtlasSprite,
        With<AIEnemy>,
    )>,
    mut query: Query<(&Actor, &mut ActionState), With<AIAttackAction>>,
) {
    if !timeinfo.game_paused {
        if let Ok(player_transform) = player_query.get_single() {
            for (Actor(actor), mut state) in query.iter_mut() {
                if let Ok((
                    enemy_transform,
                    mut velocity,
                    aggroable,
                    mut attacking,
                    mut enemystate,
                    _sprite,
                    _,
                )) = enemy_query.get_mut(*actor)
                {
                    match *state {
                        ActionState::Requested => {
                            attacking.is_attacking = true;
                            info!("shaman ai attack requested");
                            *state = ActionState::Executing;
                        }
                        ActionState::Executing => {
                            info!("shaman ai attack executing");
                            let distance =
                                player_transform.translation - enemy_transform.translation;
                            if distance.length().abs() < aggroable.distance.abs() {
                                *velocity =
                                    Velocity::linear(distance.normalize_or_zero().truncate() * 50.);
                            } else {
                                info!("enemy_go_idle");
                                *velocity = Velocity::linear(Vec2::ZERO);
                                attacking.is_attacking = false;
                                enemystate.facing = FacingDirection::Idle;
                                *state = ActionState::Success;
                            }
                        }
                        ActionState::Cancelled => {
                            info!("shaman ai attack cancelled/failed");
                            attacking.is_attacking = false;
                            *state = ActionState::Failure;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

fn random_wander_system(
    timeinfo: ResMut<TimeInfo>,
    _player_query: Query<&Transform, With<Player>>,
    #[allow(clippy::type_complexity)] enemy_query: Query<(
        &Transform,
        &mut Velocity,
        // &AIMeanderAction,
        &mut ActorState,
        &mut TextureAtlasSprite,
        &AICanMeander,
        With<AIEnemy>,
    )>,
    mut query: Query<(&Actor, &mut ActionState), With<AICanMeander>>,
) {
    if timeinfo.game_paused {
        for (Actor(actor), mut state) in query.iter_mut() {
            if let Ok((_enemy_transform, _velocity, _meandering, _enemystate, _sprite, _a)) =
                enemy_query.get(*actor)
            {
                match *state {
                    ActionState::Requested => {
                        info!("meandering action requested");
                        *state = ActionState::Executing;
                    }
                    ActionState::Executing => {
                        info!("executing meandering");
                        *state = ActionState::Success;
                    }
                    ActionState::Cancelled => {
                        info!("cancelling meandering action");
                        *state = ActionState::Failure;
                    }
                    _ => {}
                }
            }
        }
    }
}

fn aggro_score_system(
    player_query: Query<&Transform, With<Player>>, //player
    enemy_query: Query<(&Transform, &AIAggroDistance), With<AIEnemy>>, //enemys that can aggro
    mut query: Query<(&Actor, &mut Score), With<AIIsAggroed>>, //enemy brain?
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (Actor(actor), mut score) in query.iter_mut() {
            if let Ok((transform, aggroable)) = enemy_query.get(*actor) {
                let distance = player_transform.translation - transform.translation;

                let score_value = if distance.length().abs() < aggroable.distance.abs() {
                    1.
                } else {
                    0.
                };
                score.set(score_value);
            }
        }
    }
}
