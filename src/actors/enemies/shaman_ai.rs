// all credit for this goes to Shane Satterfield @ https://github.com/shanesatterfield
use bevy::prelude::*;
use big_brain::{
    prelude::{ActionState, Actor, Score},
    BigBrainStage,
};
use heron::Velocity;

use crate::actors::components::{Aggroable, Aggroed, AttackPlayer, Attacking, Enemy, Player};

pub struct ShamanAi;

impl Plugin for ShamanAi {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(BigBrainStage::Actions, aggro_system)
            .add_system_to_stage(BigBrainStage::Scorers, aggro_score_system);
    }
}

fn aggro_system(
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&Transform, &mut Velocity, &Aggroable, &mut Attacking)>,
    mut query: Query<(&Actor, &mut ActionState), With<AttackPlayer>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (Actor(actor), mut state) in query.iter_mut() {
            if let Ok((transform, mut velocity, aggroable, mut attacking)) =
                enemy_query.get_mut(*actor)
            {
                match *state {
                    ActionState::Requested => {
                        attacking.is_attacking = true;
                        *state = ActionState::Executing;
                    }
                    ActionState::Executing => {
                        let distance = player_transform.translation - transform.translation;
                        if distance.length().abs() < aggroable.distance.abs() {
                            *velocity = Velocity::from_linear(distance.normalize_or_zero() * 20.);
                        } else {
                            *velocity = Velocity::from_linear(Vec3::ZERO);
                            attacking.is_attacking = false;
                            *state = ActionState::Success;
                        }
                    }
                    ActionState::Cancelled => {
                        attacking.is_attacking = false;
                        *state = ActionState::Failure;
                    }
                    _ => {}
                }
            }
        }
    }
}

fn aggro_score_system(
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<(&Transform, &Aggroable), With<Enemy>>,
    mut query: Query<(&Actor, &mut Score), With<Aggroed>>,
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
