#![allow(clippy::type_complexity)]
/*
all credit for this goes to Shane Satterfield @ https://github.com/shanesatterfield
for being the only real useful example of big-brain as far as im concerned
*/
use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;
use big_brain::{
    prelude::{ActionState, Actor, Score},
    BigBrainSet,
    // BigBrainStage,
};
use rand::{thread_rng, Rng};

use crate::game::{
    actors::{
        ai::components::{
            AIAttackState, AICanAggro, AICanWander, AIChaseAction, AIWanderAction, AggroScore,
            AttackScore, Enemy, WanderScore,
        },
        animation::components::{ActorAnimationType, AnimState},
        components::Player,
    },
    AppStage, TimeInfo,
};

/// stupid ai systems and functions
pub struct StupidAiPlugin;

impl Plugin for StupidAiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            (
                wander_score_system.run_if(in_state(AppStage::PlayingGame)),
                aggro_score_system.run_if(in_state(AppStage::PlayingGame)),
            )
                .in_set(BigBrainSet::Scorers),
        )
        .add_systems(
            Update,
            (
                wander_action.run_if(in_state(AppStage::PlayingGame)),
                chase_action.run_if(in_state(AppStage::PlayingGame)), // shoot_action,
            )
                .in_set(BigBrainSet::Actions),
        );
    }
}

/// aggro score system, if player is in range, aggro score is 1
fn aggro_score_system(
    player_query: Query<&Transform, With<Player>>, // player
    enemy_query: Query<(&Transform, &AICanAggro), With<Enemy>>, // enemies that can aggro
    mut aggro_scorer_query: Query<(&Actor, &mut Score), With<AggroScore>>, // aggro scorer
) {
    for (Actor(actor), mut aggro_score) in &mut aggro_scorer_query {
        let mut closest_player_as_distance = f32::INFINITY;
        let mut closest_player_transform: Option<&Transform> = None;

        // Iterate over available player queries and find the closest player to the enemy
        player_query.for_each(|player_transform| {
            let distance = player_transform.translation.truncate().distance(
                enemy_query
                    .get_component::<Transform>(*actor)
                    // TODO! this is an error
                    .unwrap_or(&Transform::IDENTITY)
                    .translation
                    .truncate(),
            );
            if distance < closest_player_as_distance {
                closest_player_as_distance = distance;
                closest_player_transform = Some(player_transform);
            }
        });

        if let Some(_player_transform) = closest_player_transform {
            let (_enemy_transform, aggro_able) = enemy_query.get(*actor).unwrap();
            if closest_player_as_distance < aggro_able.aggro_distance.abs() {
                aggro_score.set(1.0);
            } else {
                aggro_score.set(0.0);
            }
        } else {
            aggro_score.set(0.0);
        }
    }
}

/// wander able enemy's scorer
/// if player is not within range, score is 1
fn wander_score_system(
    player_query: Query<&Transform, With<Player>>, //player
    enemy_query: Query<(&Transform, &AICanAggro), With<Enemy>>, //enemy's that can aggro
    mut wander_score_query: Query<
        (&Actor, &mut Score),
        (With<WanderScore>, Without<AggroScore>, With<AttackScore>),
    >,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };
    wander_score_query.for_each_mut(|(Actor(actor), mut wander_score)| {
        if let Ok((transform, aggro_able)) = enemy_query.get(*actor) {
            let distance = player_transform
                .translation
                .truncate()
                .distance(transform.translation.truncate());

            if distance > aggro_able.aggro_distance {
                wander_score.set(1.0);
            } else {
                wander_score.set(0.0);
            }
        }
    });
}

/// TODO: add attacks
/// if player is within attack range, shoot at player
#[allow(dead_code)]
fn attack_score_system(
    _player_query: Query<&Transform, With<Player>>, //player
    _enemy_query: Query<(&Transform, &AICanAggro), With<Enemy>>, //enemy's that can aggro
    _attack_score_query: Query<
        (&Actor, &mut Score),
        (With<AttackScore>, Without<AggroScore>, Without<WanderScore>),
    >,
) {
}

/// handles enemy's that can chase
fn chase_action(
    _time_info: ResMut<TimeInfo>,
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(
        &Transform,
        &mut Velocity,
        &AICanAggro,
        &mut AIAttackState,
        &mut AnimState,
        With<Enemy>,
    )>,
    mut chasing_enemies: Query<(&Actor, &mut ActionState), With<AIChaseAction>>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };

    chasing_enemies.for_each_mut(|(Actor(actor), mut state)| {
        if let Ok((enemy_transform, mut velocity, aggro_able, _attacking, mut anim_state, ())) =
            enemy_query.get_mut(*actor)
        {
            let direction = ((player_transform.translation.truncate())
                - enemy_transform.translation.truncate())
            .normalize_or_zero();

            let distance = enemy_transform
                .translation
                .truncate()
                .distance(player_transform.translation.truncate())
                .abs();

            match *state {
                ActionState::Init => {}
                ActionState::Requested => {
                    *state = ActionState::Executing;
                }
                ActionState::Executing => {
                    // move towards the player if player is close enough
                    if distance <= aggro_able.aggro_distance {
                        *velocity = Velocity::linear(direction * 50.);
                    }
                    // chase target escaped, failed to chase
                    else if distance < 100.0 || distance >= aggro_able.aggro_distance {
                        *velocity = Velocity::linear(velocity.linvel.lerp(Vec2::ZERO, 0.3));
                        anim_state.facing = ActorAnimationType::Idle;
                        *state = ActionState::Failure;
                    }
                    // we really should not hit this block
                    else {
                        *velocity = Velocity::linear(velocity.linvel.lerp(Vec2::ZERO, 0.3));
                        anim_state.facing = ActorAnimationType::Idle;
                        warn!("AI CHASE ACTION HIT UNKNOWN CIRCUMSTANCES");
                        *state = ActionState::Failure;
                    }
                }
                ActionState::Cancelled => {
                    *velocity = Velocity::linear(Vec2::ZERO);
                    anim_state.facing = ActorAnimationType::Idle;
                    *state = ActionState::Failure;
                }
                ActionState::Success => {
                    *velocity = Velocity::linear(velocity.linvel.lerp(Vec2::ZERO, 0.3)); // Velocity::linear(Vec2::ZERO);
                    anim_state.facing = ActorAnimationType::Idle;
                }
                ActionState::Failure => {
                    *velocity = Velocity::linear(velocity.linvel.lerp(Vec2::ZERO, 1.0));
                    anim_state.facing = ActorAnimationType::Idle;
                }
            }
        }
    });
}

/// handles enemy's that are doing the wander action
fn wander_action(
    #[allow(clippy::type_complexity)]
    // trunk-ignore(clippy/type_complexity)
    mut enemy_query: Query<(
        &Transform,
        &mut Velocity,
        &mut TextureAtlasSprite,
        &mut AICanWander,
        With<Enemy>,
    )>,
    mut thinker_query: Query<(&Actor, &mut ActionState), With<AIWanderAction>>,
) {
    thinker_query.for_each_mut(|(Actor(actor), mut state)| {
        if let Ok((enemy_transform, mut velocity, _sprite, mut can_meander_tag, _a)) =
            enemy_query.get_mut(*actor)
        {
            let target_pos = can_meander_tag.wander_target;
            let spawn_pos = can_meander_tag
                .spawn_position
                .expect("theres always a spawn position, this can be expected");
            let cur_pos = enemy_transform.translation.truncate();
            let mut rng = thread_rng();
            match *state {
                ActionState::Init => {}
                ActionState::Cancelled => {
                    // clear target, set velocity to None
                    can_meander_tag.wander_target = None;
                    *velocity = Velocity::linear(velocity.linvel.lerp(Vec2::ZERO, 0.6));
                    *state = ActionState::Failure;
                }
                ActionState::Requested => {
                    // pick a random target within range of home and current position
                    let t_deviation = rng.gen_range(-50.0..=50.0);
                    if let Some(target_pos) = target_pos {
                        let c_target_pos: Vec2 = target_pos;
                        let distance = c_target_pos - cur_pos;
                        if distance.length().abs() <= t_deviation {
                            can_meander_tag.wander_target = None;
                            *state = ActionState::Success;
                        } else {
                            *state = ActionState::Executing;
                        }
                    } else {
                        can_meander_tag.wander_target = Some(Vec2 {
                            x: (spawn_pos.x + rng.gen_range(-300.0..=300.0)), //Rng::gen_range(&mut )),
                            y: (spawn_pos.y + rng.gen_range(-300.0..=300.0)),
                        });
                        *state = ActionState::Executing;
                    }
                }
                ActionState::Executing => match target_pos {
                    Some(target_pos) => {
                        let c_target_pos: Vec2 = target_pos;
                        let distance = c_target_pos - cur_pos;
                        if distance.length().abs() <= 60.0 {
                            can_meander_tag.wander_target = None;
                            *state = ActionState::Requested;
                        } else {
                            *velocity = Velocity::linear(distance.normalize_or_zero() * 100.);
                        }
                    }
                    None => {
                        *state = ActionState::Requested;
                    }
                },
                ActionState::Success | ActionState::Failure => {
                    // clear target, set velocity to None  // we actually don't want too succeed at this action because then the ai will just do nothing. if i set it too not be last resort action i bet it would work
                    *velocity = Velocity::linear(velocity.linvel.lerp(Vec2::ZERO, 1.0));
                    can_meander_tag.wander_target = None;
                    *state = ActionState::Requested;
                }
            }
        }
    });
}
