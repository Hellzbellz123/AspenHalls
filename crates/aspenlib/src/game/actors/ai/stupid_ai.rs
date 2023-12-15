/*
all credit for this goes to Shane Satterfield @ https://github.com/shanesatterfield
for being the only real useful example of big-brain as far as im concerned
*/
use bevy::prelude::*;
use bevy_rapier2d::prelude::{QueryFilter, RapierContext, Velocity};
use big_brain::{
    prelude::{ActionState, Actor, Score},
    // BigBrainStage,
    BigBrainSet,
};
use rand::{thread_rng, Rng};

use crate::{
    consts::TILE_SIZE,
    game::{
        actors::{
            ai::components::{
                AIChaseAction, AIChaseConfig, AIWanderAction, AIWanderConfig, AttackScore,
                ChaseScore, Enemy, WanderScore,
            },
            animation::components::{ActorAnimationType, AnimState},
            components::{Player, PlayerColliderTag},
        },
        AppState,
    },
};

use super::components::{AIShootAction, AIShootConfig};

/// stupid ai systems and functions
pub struct StupidAiPlugin;

impl Plugin for StupidAiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            (chase_score_system, attack_score_system)
                .run_if(in_state(AppState::PlayingGame))
                .in_set(BigBrainSet::Scorers),
        )
        .add_systems(
            Update,
            (wander_action, chase_action, attack_action)
                .run_if(in_state(AppState::PlayingGame)) // shoot_action,
                .in_set(BigBrainSet::Actions),
        );
    }
}

//TODO: rework ai

/// chase score system, if player is in range, aggro score is 1
fn chase_score_system(
    player_query: Query<&Transform, With<Player>>, // player
    enemy_query: Query<(&Transform, &AIChaseConfig), With<Enemy>>, // enemies that can aggro
    mut chase_scorer_query: Query<(&Actor, &mut Score), With<ChaseScore>>, // aggro scorer
) {
    for (Actor(actor), mut chase_score) in &mut chase_scorer_query {
        let mut closest_player_as_distance = f32::INFINITY;
        let mut closest_player_transform: Option<&Transform> = None;

        // Iterate over available player queries and find the closest player to the enemy
        for player_transform in &player_query {
            let distance = player_transform.translation.truncate().distance(
                enemy_query
                    .get_component::<Transform>(*actor)
                    // TODO! this is an error
                    .unwrap_or(&Transform::IDENTITY)
                    .translation
                    .truncate(),
            );
            if distance > closest_player_as_distance {
                closest_player_as_distance = distance;
                closest_player_transform = Some(player_transform);
            }
        }

        if let Some(_player_transform) = closest_player_transform {
            let (_enemy_transform, chase_able) = enemy_query.get(*actor).unwrap();
            if closest_player_as_distance < chase_able.aggro_distance.abs() {
                chase_score.set(0.8);
            } else {
                chase_score.set(0.0);
            }
        } else {
            chase_score.set(0.0);
        }
    }
}

// /// wander able enemy's scorer
// /// if player is not within range, score is 1
// fn wander_score_system(
//     player_query: Query<&Transform, With<Player>>, //player
//     enemy_query: Query<(&Transform, &AIChaseConfig), With<Enemy>>, //enemy's that can aggro
//     mut wander_score_query: Query<
//         (&Actor, &mut Score),
//         (With<WanderScore>, Without<ChaseScore>, With<AttackScore>),
//     >,
// ) {
//     let Ok(player_transform) = player_query.get_single() else {
//         return;
//     };
//     for (Actor(actor), mut wander_score) in &mut wander_score_query {
//         if let Ok((transform, chase_config)) = enemy_query.get(*actor) {
//             let distance = player_transform
//                 .translation
//                 .truncate()
//                 .distance(transform.translation.truncate());

//             if distance > chase_config.aggro_distance {
//                 wander_score.set(1.0);
//             } else {
//                 wander_score.set(0.0);
//             }
//         }
//     }
// }

/// TODO: add attacks
/// check if player is in LOS with raycast
/// if player is within attack range, shoot at player
#[allow(clippy::type_complexity)]
fn attack_score_system(
    player_query: Query<&Transform, With<Player>>, //player
    mut enemy_query: Query<(&Transform, &mut AIShootConfig), With<Enemy>>, //enemy's that can aggro
    mut attack_score_query: Query<
        (&Actor, &mut Score),
        (With<AttackScore>, Without<ChaseScore>, Without<WanderScore>),
    >,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };

    for (Actor(actor), mut attack_score) in &mut attack_score_query {
        if let Ok((transform, mut attack_config)) = enemy_query.get_mut(*actor) {
            let distance_too_player = player_transform
                .translation
                .truncate()
                .distance(transform.translation.truncate())
                .abs();

            if distance_too_player <= attack_config.find_target_range {
                attack_score.set(1.0);
            } else {
                attack_config.can_shoot = false;
                attack_score.set(0.0);
            }
        }
    }
}

/// handles enemy's that can chase
fn chase_action(
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(
        &Transform,
        &mut Velocity,
        &AIChaseConfig,
        &mut AnimState,
        With<Enemy>,
    )>,
    mut chasing_enemies: Query<(&Actor, &mut ActionState), With<AIChaseAction>>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };

    for (Actor(actor), mut state) in &mut chasing_enemies {
        if let Ok((enemy_transform, mut velocity, chase_able, mut anim_state, ())) =
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
                    if distance <= chase_able.aggro_distance {
                        *velocity = Velocity::linear(direction * 50.);
                    }
                    // chase target escaped, failed to chase
                    else if distance < 100.0 || distance >= chase_able.aggro_distance {
                        *velocity = Velocity::linear(velocity.linvel.lerp(Vec2::ZERO, 0.3));
                        anim_state.animation_type = ActorAnimationType::Idle;
                        *state = ActionState::Failure;
                    }
                    // we really should not hit this block
                    else {
                        *velocity = Velocity::linear(velocity.linvel.lerp(Vec2::ZERO, 0.3));
                        anim_state.animation_type = ActorAnimationType::Idle;
                        warn!("AI CHASE ACTION HIT UNKNOWN CIRCUMSTANCES");
                        *state = ActionState::Failure;
                    }
                }
                ActionState::Cancelled => {
                    *velocity = Velocity::linear(Vec2::ZERO);
                    anim_state.animation_type = ActorAnimationType::Idle;
                    *state = ActionState::Failure;
                }
                ActionState::Success => {
                    *velocity = Velocity::linear(velocity.linvel.lerp(Vec2::ZERO, 0.3)); // Velocity::linear(Vec2::ZERO);
                    anim_state.animation_type = ActorAnimationType::Idle;
                }
                ActionState::Failure => {
                    *velocity = Velocity::linear(velocity.linvel.lerp(Vec2::ZERO, 1.0));
                    anim_state.animation_type = ActorAnimationType::Idle;
                }
            }
        }
    }
}

/// handles enemy's that are doing the wander action
fn wander_action(
    mut enemy_query: Query<(
        &Transform,
        &mut Velocity,
        &mut TextureAtlasSprite,
        &mut AIWanderConfig,
        With<Enemy>,
    )>,
    mut thinker_query: Query<(&Actor, &mut ActionState), With<AIWanderAction>>,
    rapier_context: Res<RapierContext>,
) {
    for (Actor(actor), mut state) in &mut thinker_query {
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
                        let direction_too_target = (target_pos
                            - enemy_transform.translation.truncate())
                        .normalize_or_zero();

                        let distance_to_target = enemy_transform
                            .translation
                            .truncate()
                            .distance(target_pos)
                            .abs();

                        let ray_origin = enemy_transform.translation.truncate()
                            + (direction_too_target * (TILE_SIZE / 2.0));
                        let ray_dir = direction_too_target;
                        let max_toi = distance_to_target;
                        let solid = true;
                        let filter = QueryFilter::new();

                        let ray =
                            rapier_context.cast_ray(ray_origin, ray_dir, max_toi, solid, filter);
                        let can_reach_target: bool = match ray {
                            None => true,
                            Some((_entity, _distance)) => false,
                        };

                        let c_target_pos: Vec2 = target_pos;
                        let distance = c_target_pos - cur_pos;
                        if distance.length().abs() <= 60.0 || !can_reach_target {
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
    }
}

/// handles enemy's that can chase
fn attack_action(
    rapier_context: Res<RapierContext>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    player_collider_query: Query<Entity, With<PlayerColliderTag>>,
    mut enemy_query: Query<(&Transform, &mut AIShootConfig, &AnimState), With<Enemy>>,
    mut shooting_enemies: Query<(&Actor, &mut ActionState), With<AIShootAction>>,
) {
    let Ok((_, player_transform)) = player_query.get_single() else {
        return;
    };

    for (Actor(actor), mut state) in &mut shooting_enemies {
        if let Ok((enemy_transform, mut shoot_cfg, _anim_state)) = enemy_query.get_mut(*actor) {
            let direction_too_player = (player_transform.translation.truncate()
                - enemy_transform.translation.truncate())
            .normalize_or_zero();

            let distance_to_player = enemy_transform
                .translation
                .truncate()
                .distance(player_transform.translation.truncate())
                .abs();

            let ray_origin =
                enemy_transform.translation.truncate() + (direction_too_player * (TILE_SIZE / 2.0));
            let ray_dir = direction_too_player;
            let max_toi = shoot_cfg.find_target_range;
            let solid = true;
            let filter = QueryFilter::new();

            let player_too_far_away = distance_to_player > shoot_cfg.find_target_range;
            let ray = rapier_context.cast_ray(ray_origin, ray_dir, max_toi, solid, filter);
            let raycast_hit_player: bool = match ray {
                None => false,
                Some((entity, _distance)) => entity == player_collider_query.single(),
            };

            match *state {
                ActionState::Init => {}
                ActionState::Requested => {
                    if player_too_far_away || !raycast_hit_player {
                        *state = ActionState::Failure;
                    } else if raycast_hit_player && !player_too_far_away {
                        shoot_cfg.should_shoot = true;
                        *state = ActionState::Executing;
                    }
                }
                ActionState::Executing | ActionState::Cancelled => {
                    if player_too_far_away || !raycast_hit_player {
                        *state = ActionState::Failure;
                    } else {
                        *state = ActionState::Success;
                    }
                }
                ActionState::Success | ActionState::Failure => {
                    shoot_cfg.should_shoot = false;
                }
            }
        }
    }
}
