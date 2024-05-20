/*
all credit for this goes to Shane Satterfield @ https://github.com/shanesatterfield
for being the only real useful example of big-brain as far as im concerned
*/

use bevy::hierarchy::HierarchyQueryExt;
use bevy::prelude::*;
use bevy_rapier2d::{
    math::Rot,
    prelude::{Collider, QueryFilter, RapierContext, Velocity},
};
use big_brain::{
    prelude::{ActionState, Actor, Score},
    thinker::ThinkerBuilder,
    // BigBrainStage,
    BigBrainSet,
};
use rand::{thread_rng, Rng};

use crate::{
    consts::default_actor_collider,
    game::{
        characters::{
            ai::components::{
                AIChaseAction, AICombatConfig, AIWanderAction, AIWanderConfig, AttackScorer,
                ChaseScorer,
            },
            player::PlayerSelectedHero,
        },
        combat::{AttackDirection, EventRequestAttack},
        AppState,
    },
    utilities::tiles_to_f32,
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

/// All Components needed for `stupid_ai` functionality
#[derive(Bundle)]
pub struct StupidAiBundle {
    /// ai chase/attack config
    pub combat_config: AICombatConfig,
    /// stupid wander action
    pub wander_config: AIWanderConfig,
    /// stupid shoot action
    pub shoot_config: AIShootConfig,
    /// chooses action
    pub thinker: ThinkerBuilder,
}

//TODO: rework ai
/// updates character attack/chase score
#[allow(clippy::type_complexity)]
fn stupid_ai_aggro_manager(
    names: Query<&Name>,
    rapier_context: Res<RapierContext>,
    // player
    player_query: Query<(Entity, &Transform), With<PlayerSelectedHero>>,
    // enemies that can aggro
    can_attack_query: Query<(Entity, &Transform, &AICombatConfig)>,
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

    for (this_actor, enemy_transform, combat_cfg) in &can_attack_query {
        let player_pos = player_transform.translation.truncate();
        let enemy_pos = enemy_transform.translation.truncate();
        let distance_to_target = enemy_pos.distance(player_pos).abs();
        let direction_to_target = Vec2::normalize_or_zero(player_pos - enemy_pos);

        let ray = rapier_context.cast_shape(
            enemy_pos,
            Rot::MIN,
            direction_to_target,
            &default_actor_collider(),
            distance_to_target,
            false,
            QueryFilter::new()
                .exclude_sensors()
                .exclude_rigid_body(this_actor),
        );

        let target_in_shoot_range = distance_to_target <= tiles_to_f32(combat_cfg.shoot_range);
        let target_in_chase_range = distance_to_target <= tiles_to_f32(combat_cfg.chase_start);
        let target_in_personalspace = distance_to_target <= tiles_to_f32(combat_cfg.personal_space);

        let can_reach_target: bool = match ray {
            None => false,
            Some((entity, _distance)) => entity == player_collider,
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
    player_query: Query<&Transform, With<PlayerSelectedHero>>,
    mut enemy_query: Query<(&Transform, &mut Velocity, &AICombatConfig)>,
    mut chasing_enemies: Query<(&Actor, &mut ActionState), With<AIChaseAction>>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        warn!("no player for ai too chase");
        return;
    };

    for (Actor(actor), mut state) in &mut chasing_enemies {
        if let Ok((enemy_transform, mut velocity, combat_cfg)) = enemy_query.get_mut(*actor) {
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
                        *state = ActionState::Success;
                    }
                    if !actor_in_chase_range {
                        trace!("actor not in range, failed chase");
                        *state = ActionState::Failure;
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
                }
            }
        }
    }
}

/// handles enemy's that can attack
fn attack_action(
    // rapier_context: Res<RapierContext>,
    // player_collider_query: Query<Entity, With<PlayerColliderTag>>,
    time: Res<Time>,
    player_query: Query<(Entity, &Transform), With<PlayerSelectedHero>>,
    mut enemy_query: Query<(&Transform, &mut AIShootConfig)>,
    mut ai_with_attacks: Query<(&Actor, &mut ActionState), With<AIShootAction>>,
    mut attack_requests: EventWriter<EventRequestAttack>,
) {
    let Ok((_, player_transform)) = player_query.get_single() else {
        return;
    };

    for (Actor(actor), mut state) in &mut ai_with_attacks {
        if let Ok((enemy_transform, mut shoot_cfg)) = enemy_query.get_mut(*actor) {
            let player_pos = player_transform.translation.truncate();
            let enemy_pos = enemy_transform.translation.truncate();

            let direction_too_player = (player_pos - enemy_pos).normalize_or_zero();
            let distance_too_player = enemy_pos.distance(player_pos).abs();

            match *state {
                ActionState::Init => {}
                ActionState::Requested => {
                    *state = ActionState::Executing;
                }
                ActionState::Executing => {
                    if distance_too_player > shoot_cfg.find_target_range as f32 {
                        *state = ActionState::Failure;
                    }
                    if shoot_cfg.timer.tick(time.delta()).finished() {
                        // TODO: get weapons on entity, if melee weapon attack with that, else use ranged
                        attack_requests.send(EventRequestAttack {
                            requester: *actor,
                            direction: AttackDirection::FromVector(direction_too_player),
                        });
                        shoot_cfg.should_shoot = true;
                        shoot_cfg.timer.reset();
                    } else {
                        shoot_cfg.should_shoot = false;
                        shoot_cfg.timer.tick(time.delta());
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
    mut enemy_query: Query<(&Transform, &mut Velocity, &mut Sprite, &mut AIWanderConfig)>,
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
                        &default_actor_collider(),
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

/// sets chase score for character
#[allow(clippy::type_complexity)]
fn set_chase_score(
    scorers: &mut ParamSet<(
        Query<(&Actor, &mut Score), With<ChaseScorer>>,
        Query<(&Actor, &mut Score), With<AttackScorer>>,
    )>,
    enemy: Entity,
    new_score: f32,
) {
    scorers
        .p0()
        .iter_mut()
        .filter(|f| f.0 .0 == enemy)
        .for_each(|(_, mut score)| score.set(new_score));
}

/// set attack score for character
#[allow(clippy::type_complexity)]
fn set_attack_score(
    scorers: &mut ParamSet<(
        Query<(&Actor, &mut Score), With<ChaseScorer>>,
        Query<(&Actor, &mut Score), With<AttackScorer>>,
    )>,
    actor: Entity,
    new_score: f32,
) {
    scorers
        .p1()
        .iter_mut()
        .filter(|f| f.0 .0 == actor)
        .for_each(|(_, mut score)| score.set(new_score));
}
