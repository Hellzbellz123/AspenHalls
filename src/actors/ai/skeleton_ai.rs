/*
all credit for this goes to Shane Satterfield @ https://github.com/shanesatterfield
for being the only real useful example of big-brain as far as im concerned
*/
use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;
use big_brain::{
    prelude::{ActionState, Actor, Score},
    BigBrainStage,
};
use rand::{thread_rng, Rng};

use crate::{
    components::actors::{
        ai::{
            AICanChase, AICanWander, AIChaseAction, AIEnemy, AIWanderAction, AggroScore,
            WanderScore,
        },
        animation::FacingDirection,
        general::{ActorState, Player},
    },
    game::TimeInfo,
    utilities::game::ACTOR_LAYER,
};

pub struct SkeletonAiPlugin;

impl Plugin for SkeletonAiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(BigBrainStage::Scorers, aggro_score_system)
            .add_system_to_stage(BigBrainStage::Scorers, wander_score_system)
            .add_system_to_stage(BigBrainStage::Actions, chase_action)
            .add_system_to_stage(BigBrainStage::Actions, random_wander_system);
    }
}

fn aggro_score_system(
    player_query: Query<&Transform, With<Player>>, //player
    enemy_query: Query<(&Transform, &AICanChase), With<AIEnemy>>, //enemys that can aggro
    mut aggro_scorer_query: Query<(&Actor, &mut Score), With<AggroScore>>, //enemy brain?
    mut wanderscore_query: Query<(&Actor, &mut Score), (With<WanderScore>, Without<AggroScore>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (Actor(actor), mut aggro_score) in aggro_scorer_query.iter_mut() {
            if let Ok((transform, aggroable)) = enemy_query.get(*actor) {
                let distance = player_transform.translation - transform.translation;

                if distance.length().abs() < aggroable.aggro_distance.abs() {
                    let score: f32 = i16::from(true).into(); //aggro is true
                    aggro_score.set(score);
                } else {
                    aggro_score.set(0.0);
                    for (Actor(wander_actor), mut wander_score) in wanderscore_query.iter_mut() {
                        if let Ok((_transform, _aggroable)) = enemy_query.get(*wander_actor) {
                            wander_score.set(i16::from(false).into());
                        }
                    }
                };
                // info!(
                //     "[AggroScore] for Entity: {:?} set to value {:?}",
                //     actor, score
                // );
            }
        }
    }
}

fn wander_score_system(
    player_query: Query<&Transform, With<Player>>, //player
    enemy_query: Query<(&Transform, &AICanChase), With<AIEnemy>>, //enemys that can aggro
    mut wanderscore_query: Query<(&Actor, &mut Score), (With<WanderScore>, Without<AggroScore>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (Actor(actor), mut wander_score) in wanderscore_query.iter_mut() {
            if let Ok((transform, aggroable)) = enemy_query.get(*actor) {
                let distance = player_transform.translation - transform.translation;

                if distance.length().abs() > aggroable.aggro_distance.abs() {
                    wander_score.set(1.0)
                } else {
                    wander_score.set(0.0)
                }
            }
        }
    }
}

fn chase_action(
    timeinfo: ResMut<TimeInfo>,
    player_query: Query<&Transform, With<Player>>,
    #[allow(clippy::type_complexity)] mut enemy_query: Query<(
        &Transform,
        &mut Velocity,
        &AICanChase,
        &mut ActorState,
        &mut TextureAtlasSprite,
        With<AIEnemy>,
    )>,
    mut query: Query<(&Actor, &mut ActionState), With<AIChaseAction>>,
) {
    if !timeinfo.game_paused {
        if let Ok(player_transform) = player_query.get_single() {
            for (Actor(actor), mut state) in query.iter_mut() {
                if let Ok((
                    enemy_transform,
                    mut velocity,
                    aggroable,
                    // mut attacking,
                    mut enemystate,
                    _sprite,
                    _,
                )) = enemy_query.get_mut(*actor)
                {
                    match *state {
                        ActionState::Cancelled => {
                            // info!("action chase: cancelled");
                            *state = ActionState::Failure;
                            *velocity = Velocity::linear(Vec2::ZERO);
                            enemystate.facing = FacingDirection::Idle;
                        }
                        ActionState::Requested => {
                            // info!("action chase: requested");
                            *state = ActionState::Executing;
                        }
                        ActionState::Executing => {
                            let distance =
                                player_transform.translation - enemy_transform.translation;
                            if distance.length().abs() <= 60.0 {
                                *state = ActionState::Success;
                                // info!("action chase: enemy near player")
                            } else if distance.length().abs() < aggroable.aggro_distance.abs() {
                                // info!("action chase: player is within aggro range");
                                *velocity =
                                    Velocity::linear(distance.normalize_or_zero().truncate() * 50.);
                            } else if distance.length().abs() > aggroable.aggro_distance.abs() {
                                // info!("action chase: player is out of aggro range");
                                *velocity = Velocity::linear(velocity.linvel.lerp(Vec2::ZERO, 0.3));
                                enemystate.facing = FacingDirection::Idle;
                                *state = ActionState::Failure;
                            } else {
                                *state = ActionState::Failure;
                                *velocity = Velocity::linear(velocity.linvel.lerp(Vec2::ZERO, 0.3));
                                enemystate.facing = FacingDirection::Idle;
                                // info!("we shouldnt have hit this code block")
                            }
                        }
                        ActionState::Success => {
                            // info!("action chase: success");
                            *velocity = Velocity::linear(velocity.linvel.lerp(Vec2::ZERO, 0.3)); // Velocity::linear(Vec2::ZERO);
                            enemystate.facing = FacingDirection::Idle;
                        }
                        ActionState::Failure => {
                            // info!("action chase: failure");
                            *velocity = Velocity::linear(velocity.linvel.lerp(Vec2::ZERO, 1.0));
                            enemystate.facing = FacingDirection::Idle;
                        }
                        ActionState::Init => {
                            // info!("action chase: init");
                        }
                    }
                }
            }
        }
    }
}

fn random_wander_system(
    timeinfo: ResMut<TimeInfo>,
    _player_query: Query<&Transform, With<Player>>,
    #[allow(clippy::type_complexity)] mut enemy_query: Query<(
        &Transform,
        &mut Velocity,
        &mut ActorState,
        &mut TextureAtlasSprite,
        &mut AICanWander,
        With<AIEnemy>,
    )>,
    mut thinker_query: Query<(&Actor, &mut ActionState), With<AIWanderAction>>,
) {
    if !timeinfo.game_paused {
        for (Actor(actor), mut state) in thinker_query.iter_mut() {
            if let Ok((
                enemy_transform,
                mut velocity,
                _enemystate,
                _prite,
                mut can_meander_tag,
                _a,
            )) = enemy_query.get_mut(*actor)
            {
                let target_pos = can_meander_tag.wander_target;
                let spawn_pos = can_meander_tag
                    .spawn_position
                    .expect("theres always a spawn position, this can be expected");
                let cur_pos = enemy_transform.translation;
                let mut rng = thread_rng();
                match *state {
                    ActionState::Cancelled => {
                        // clear target, set velocity to None
                        *velocity = Velocity::linear(velocity.linvel.lerp(Vec2::ZERO, 1.0));
                        *state = ActionState::Failure;
                    }
                    ActionState::Init => {
                        info!("action meander: init")
                    }
                    ActionState::Requested => {
                        // pick a random target within range of home and current position
                        let t_deviation = rng.gen_range(-50.0..=50.0);
                        if target_pos.is_some() {
                            let c_target_pos: Vec3 = target_pos.expect("");
                            let distance = c_target_pos - cur_pos;
                            info!("{:#?}", can_meander_tag.wander_target);
                            if distance.length().abs() <= t_deviation {
                                info!("enemy roughly at wander target");
                                can_meander_tag.wander_target = None;
                                *state = ActionState::Success;
                            } else {
                                info!("we arent at the target position yet");
                                *state = ActionState::Executing;
                            }
                        } else if target_pos.is_none() {
                            can_meander_tag.wander_target = Some(Vec3 {
                                x: (spawn_pos.x + rng.gen_range(-300.0..=300.0)), //Rng::gen_range(&mut )),
                                y: (spawn_pos.y + rng.gen_range(-300.0..=300.0)),
                                z: ACTOR_LAYER,
                            });
                            *state = ActionState::Executing;
                        }
                        info!("action meander: requested");
                        *state = ActionState::Executing;
                    }
                    ActionState::Executing => {
                        if target_pos.is_some() {
                            let c_target_pos: Vec3 = target_pos.expect("");
                            let distance = c_target_pos - cur_pos;
                            info!("{:#?}", can_meander_tag.wander_target);
                            if distance.length().abs() <= 60.0 {
                                can_meander_tag.wander_target = None;
                                *state = ActionState::Requested;
                            } else {
                                *velocity = Velocity::linear(
                                    distance.normalize_or_zero().truncate() * 100.,
                                );
                            }
                            info!("action meander: executing");
                        }
                    }
                    ActionState::Success => {
                        // clear target, set velocity to None  // we actually dont want too succeed at this action because then the ai will just do nothing. if i set it too not be last resort action i bet it would work
                        *velocity = Velocity::linear(velocity.linvel.lerp(Vec2::ZERO, 1.0));
                        info!("action meander: success")
                    }
                    ActionState::Failure => {
                        // clear target, set velocity to None
                        *velocity = Velocity::linear(velocity.linvel.lerp(Vec2::ZERO, 1.0));
                        info!("action meander: failure")
                    }
                }
            }
        }
    }
}

// let allowed_x_difference =
//     (cur_pos.x + t_deviation)..(cur_pos.x + t_deviation);
// let allowed_y_difference =
//     (cur_pos.y + t_deviation)..(cur_pos.y + t_deviation);

// if allowed_x_difference.contains(&c_target_pos.x)
//     && allowed_y_difference.contains(&c_target_pos.y)
// {
//     info!("entity roughly at target position");
//     can_meander_tag.wander_target = None;
//     *state = ActionState::Success;
// } else {
//     *state = ActionState::Executing;
//     info!("we arent at the target position yet");
// }