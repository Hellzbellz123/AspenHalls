/*
all credit for this goes to Shane Satterfield @ https://github.com/shanesatterfield
for being the only real useful example of big-brain as far as im concerned
*/

use bevy::prelude::*;
use bevy_rapier2d::{
    math::Rot,
    prelude::{Collider, CollisionGroups, Group, QueryFilter, RapierContext, Velocity},
};
use big_brain::{
    prelude::{ActionState, Actor, Score},
    // BigBrainStage,
    BigBrainSet,
};
use rand::{thread_rng, Rng};

use crate::{
    consts::{actor_collider, AspenCollisionLayer, BACKUP_DISTANCE, TILE_SIZE},
    game::{
        actors::{
            ai::components::{
                AIChaseAction, AIChaseConfig, AICombatConfig, AIWanderAction, AIWanderConfig,
                AttackScorer, ChaseScorer, Enemy,
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
            (stupid_ai_aggro_manager)
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

pub fn tiles_to_f32(distance: i32) -> f32 {
    distance as f32 * TILE_SIZE
}

use bevy::hierarchy::HierarchyQueryExt;

//TODO: rework ai
fn stupid_ai_aggro_manager(
    names: Query<&Name>,
    rapier_context: Res<RapierContext>,
    // player
    player_query: Query<(Entity, &Transform), With<Player>>,
    // enemies that can aggro
    enemy_query: Query<(Entity, &Transform, &AICombatConfig), With<Enemy>>,
    // scorers
    mut scorers: ParamSet<(
        Query<(&Actor, &mut Score), With<ChaseScorer>>,
        Query<(&Actor, &mut Score), With<AttackScorer>>,
    )>,
    children: Query<&Children>,
    colliders: Query<&Collider>,
) {
    let Ok((player, player_transform)) = player_query.get_single() else {
        warn!("no player for stupid-ai-manager too use");
        return;
    };

    let player_collider = children
        .iter_descendants(player)
        .find(|f| colliders.get(*f).is_ok())
        .expect("player should always have a collider");

    for (this_actor, enemy_transform, combat_cfg) in &enemy_query {
        let player_pos = player_transform.translation.truncate();
        let enemy_pos = enemy_transform.translation.truncate();
        let distance_to_target = enemy_pos.distance(player_pos).abs();
        let direction_to_target = Vec2::normalize_or_zero(player_pos - enemy_pos);

        let ray = rapier_context.cast_shape(
            enemy_pos,
            Rot::MIN,
            direction_to_target,
            &actor_collider(),
            distance_to_target,
            false,
            QueryFilter::new()
                .exclude_sensors()
                .exclude_rigid_body(this_actor),
        );

        let target_in_shoot_range = distance_to_target <= tiles_to_f32(combat_cfg.shoot_range);
        let target_in_chase_range = distance_to_target <= tiles_to_f32(combat_cfg.chase_start);
        let target_in_personalspace = distance_to_target <= tiles_to_f32(combat_cfg.personal_space);

        // TODO: this raycast is not working properly
        // it seems too always be true
        let can_reach_target: bool = match ray {
            None => false,
            Some((entity, _distance)) => {
                if entity == player_collider {
                    true
                } else {
                    false
                }
            }
        };

        if ray.is_some() {
            let (ent, _distance) = ray.unwrap();
            let name = names.get(ent).unwrap_or(&Name::new("NONAME")).clone();
            trace!("name of ent ray is hitting: {}", name);
        }

        if can_reach_target {
            if target_in_chase_range {
                trace!("target in chase range");
                set_chase_score(&mut scorers, this_actor, 0.7);
            } else {
                trace!("target out of chase range");
                set_chase_score(&mut scorers, this_actor, 0.0);
            }
            if target_in_shoot_range && !target_in_personalspace {
                trace!("target in shoot range");
                set_attack_score(&mut scorers, this_actor, 0.9);
            } else {
                trace!("target not in shoot range");
                set_attack_score(&mut scorers, this_actor, 0.0);
            }
        } else {
            set_attack_score(&mut scorers, this_actor, 0.0);
            set_chase_score(&mut scorers, this_actor, 0.0);
        }
    }
}

/// handles enemy's that can chase
fn chase_action(
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(
        &Transform,
        &mut Velocity,
        &AICombatConfig,
        &mut AnimState,
        With<Enemy>,
    )>,
    mut chasing_enemies: Query<(&Actor, &mut ActionState), With<AIChaseAction>>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        warn!("no player for ai too chase");
        return;
    };

    for (Actor(actor), mut state) in &mut chasing_enemies {
        if let Ok((enemy_transform, mut velocity, combat_cfg, mut anim_state, ())) =
            enemy_query.get_mut(*actor)
        {
            let player_pos = player_transform.translation.truncate();
            let enemy_pos = enemy_transform.translation.truncate();
            let direction = (player_pos - enemy_pos).normalize_or_zero();
            let distance = player_pos.distance(enemy_pos).abs();

            let actor_in_personal_space = distance <= tiles_to_f32(combat_cfg.personal_space);
            let actor_in_chase_range = distance <= tiles_to_f32(combat_cfg.chase_start);
            let actor_in_shoot_range = distance <= tiles_to_f32(combat_cfg.shoot_range);

            match *state {
                ActionState::Init => {}
                ActionState::Requested => {
                    trace!("chase requested");
                    *state = ActionState::Executing;
                }
                ActionState::Executing => {
                    trace!("chase executing");
                    if !actor_in_personal_space && actor_in_shoot_range {
                        trace!("actor is close enough too attack: chase sucsessful");
                        *state = ActionState::Success
                    }
                    if !actor_in_chase_range {
                        trace!("actor not in range, failed chase");
                        *state = ActionState::Failure
                    }

                    if !actor_in_personal_space {
                        // move towards the player if player is close enough
                        trace!("actor not in shoot range, moving closer");
                        *velocity = Velocity::linear(direction * 50.);
                    } else if actor_in_personal_space {
                        // move away from player if too close
                        trace!("actor in personal space, moving away");
                        *velocity = Velocity::linear(-direction * 50.);
                    }
                }
                ActionState::Cancelled => {
                    trace!("chase cancelled");
                    *state = ActionState::Failure;
                }
                ActionState::Failure | ActionState::Success => {
                    trace!("chase finished/failed");
                    *velocity = Velocity::linear(Vec2::ZERO);
                    anim_state.animation_type = ActorAnimationType::Idle;
                }
            }
        }
    }
}

/// handles enemy's that can attack
fn attack_action(
    // rapier_context: Res<RapierContext>,
    // player_collider_query: Query<Entity, With<PlayerColliderTag>>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    mut enemy_query: Query<(&Transform, &mut AIShootConfig, &AnimState), With<Enemy>>,
    mut shooting_enemies: Query<(&Actor, &mut ActionState), With<AIShootAction>>,
) {
    let Ok((_, _player_transform)) = player_query.get_single() else {
        return;
    };

    for (Actor(actor), mut state) in &mut shooting_enemies {
        if let Ok((_enemy_transform, mut shoot_cfg, _anim_state)) = enemy_query.get_mut(*actor) {
            // let player_pos = player_transform.translation.truncate();
            // let enemy_pos = enemy_transform.translation.truncate();

            // let direction_too_player = (enemy_pos - player_pos).normalize_or_zero();
            // let distance_too_player = enemy_pos.distance(player_pos).abs();

            match *state {
                ActionState::Init => {}
                ActionState::Requested => {
                    *state = ActionState::Executing;
                }
                ActionState::Executing => {
                    if shoot_cfg.should_shoot {
                        trace!("stop shoot");
                        shoot_cfg.should_shoot = false;
                        *state = ActionState::Success
                    } else {
                        trace!("start shoot");
                        shoot_cfg.should_shoot = true;
                        *state = ActionState::Success;
                    }
                }
                ActionState::Success | ActionState::Failure => {
                    shoot_cfg.should_shoot = false;
                }
                ActionState::Cancelled => {
                    *state = ActionState::Failure;
                }
            }
        }
    }
}

/// handles enemy's that are doing the wander action
fn wander_action(
    mut enemy_query: Query<
        (
            &Transform,
            &mut Velocity,
            &mut TextureAtlasSprite,
            &mut AIWanderConfig,
        ),
        With<Enemy>,
    >,
    mut thinker_query: Query<(&Actor, &mut ActionState), With<AIWanderAction>>,
    rapier_context: Res<RapierContext>,
) {
    for (Actor(actor), mut state) in &mut thinker_query {
        if let Ok((enemy_transform, mut velocity, _sprite, mut can_meander_tag)) =
            enemy_query.get_mut(*actor)
        {
            let spawn_pos = can_meander_tag
                .spawn_position
                .expect("theres always a spawn position, this can be expected");
            let mut rng = thread_rng();

            let target_pos = can_meander_tag.wander_target;
            let enemy_pos = enemy_transform.translation.truncate();
            match *state {
                ActionState::Init => {}
                ActionState::Cancelled => {
                    *state = ActionState::Failure;
                }
                ActionState::Requested => {
                    // pick a random target within range of home and current position
                    if let Some(target_pos) = target_pos {
                        let target_deviation = rng.gen_range(0.0..=50.0);
                        let distance = enemy_pos.distance(target_pos).abs();
                        if distance <= target_deviation {
                            can_meander_tag.wander_target = None;
                            *state = ActionState::Success;
                        } else {
                            *state = ActionState::Executing;
                        }
                    } else {
                        can_meander_tag.wander_target = Some(Vec2 {
                            x: (spawn_pos.x + rng.gen_range(-300.0..=300.0)),
                            y: (spawn_pos.y + rng.gen_range(-300.0..=300.0)),
                        });
                        *state = ActionState::Executing;
                    }
                }
                ActionState::Executing => {
                    let Some(target_pos) = target_pos else {
                        *state = ActionState::Failure;
                        return;
                    };
                    let direction = (target_pos - enemy_pos).normalize_or_zero();
                    let distance = enemy_pos.distance(target_pos).abs();

                    // let ray = rapier_context.cast_ray(
                    //     enemy_pos,
                    //     direction,
                    //     tiles_to_f32(can_meander_tag.wander_distance),
                    //     false,
                    //     QueryFilter::new().exclude_rigid_body(*actor)
                    //         .exclude_sensors(),
                    //         // .exclude_rigid_body(*actor),
                    // );
                    let ray = rapier_context.cast_shape(
                        enemy_pos,
                        Rot::MIN,
                        direction,
                        &actor_collider(),
                        distance,
                        false,
                        QueryFilter::new()
                            .exclude_sensors()
                            .exclude_rigid_body(*actor),
                    );

                    if ray.is_some() {
                        can_meander_tag.wander_target = None;
                        *state = ActionState::Requested;
                    }
                    if distance <= 60.0 {
                        can_meander_tag.wander_target = None;
                        *state = ActionState::Requested;
                    } else {
                        *velocity = Velocity::linear(direction * 100.);
                    }
                }
                ActionState::Success | ActionState::Failure => {
                    // clear target, set velocity to None  // we actually don't want too succeed at this action because then the ai will just do nothing. if i set it too not be last resort action i bet it would work
                    *velocity = Velocity::linear(velocity.linvel.lerp(Vec2::ZERO, 1.0));
                    can_meander_tag.wander_target = None;
                }
            }
        }
    }
}

fn set_chase_score(
    scorers: &mut ParamSet<
        '_,
        '_,
        (
            Query<'_, '_, (&Actor, &mut Score), With<ChaseScorer>>,
            Query<'_, '_, (&Actor, &mut Score), With<AttackScorer>>,
        ),
    >,
    enemy: Entity,
    new_score: f32,
) {
    scorers
        .p0()
        .iter_mut()
        .filter(|f| f.0 .0 == enemy)
        .for_each(|(_, mut score)| score.set(new_score));
}

///set `AttackScore` value for actor
fn set_attack_score(
    scorers: &mut ParamSet<
        '_,
        '_,
        (
            Query<'_, '_, (&Actor, &mut Score), With<ChaseScorer>>,
            Query<'_, '_, (&Actor, &mut Score), With<AttackScorer>>,
        ),
    >,
    actor: Entity,
    new_score: f32,
) {
    scorers
        .p1()
        .iter_mut()
        .filter(|f| f.0 .0 == actor)
        .for_each(|(_, mut score)| score.set(new_score));
}

// /// chase score system, if player is in range, aggro score is 1
// fn chase_score_system(
//     player_query: Query<&Transform, With<Player>>, // player
//     enemy_query: Query<(&Transform, &AIChaseConfig), With<Enemy>>, // enemies that can aggro
//     mut chase_scorer_query: Query<(&Actor, &mut Score), With<ChaseScore>>, // aggro scorer
// ) {
//     for (Actor(actor), mut chase_score) in &mut chase_scorer_query {
//         let mut closest_player_as_distance = f32::INFINITY;
//         let mut closest_player_transform: Option<&Transform> = None;

//         // Iterate over available player queries and find the closest player to the enemy
//         for player_transform in &player_query {
//             let distance = player_transform.translation.truncate().distance(
//                 enemy_query
//                     .get_component::<Transform>(*actor)
//                     // TODO! this is an error
//                     .unwrap_or(&Transform::IDENTITY)
//                     .translation
//                     .truncate(),
//             );
//             if distance > closest_player_as_distance {
//                 closest_player_as_distance = distance;
//                 closest_player_transform = Some(player_transform);
//             }
//         }

//         if let Some(_player_transform) = closest_player_transform {
//             let (_enemy_transform, chase_able) = enemy_query.get(*actor).unwrap();
//             if closest_player_as_distance < chase_able.aggro_distance.abs() {
//                 chase_score.set(0.8);
//             } else {
//                 chase_score.set(0.0);
//             }
//         } else {
//             chase_score.set(0.0);
//         }
//     }
// }

// /// if player is within attack range, shoot at player
// #[allow(clippy::type_complexity)]
// fn attack_score_system(
//     player_query: Query<&Transform, With<Player>>, //player
//     mut enemy_query: Query<(&Transform, &mut AIShootConfig), With<Enemy>>, //enemy's that can aggro
//     mut attack_score_query: Query<
//         (&Actor, &mut Score),
//         (With<AttackScore>, Without<ChaseScore>, Without<WanderScore>),
//     >,
// ) {
//     let Ok(player_transform) = player_query.get_single() else {
//         return;
//     };

//     for (Actor(actor), mut attack_score) in &mut attack_score_query {
//         if let Ok((transform, mut attack_config)) = enemy_query.get_mut(*actor) {
//             let distance_too_player = player_transform
//                 .translation
//                 .truncate()
//                 .distance(transform.translation.truncate())
//                 .abs();

//             if distance_too_player <= attack_config.find_target_range {
//                 attack_score.set(1.0);
//             } else {
//                 attack_config.can_shoot = false;
//                 attack_score.set(0.0);
//             }
//         }
//     }
// }

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
