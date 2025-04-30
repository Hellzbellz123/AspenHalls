/*
all credit for this goes to Shane Satterfield @ https://github.com/shanesatterfield
for being the only real useful example of big-brain as far as im concerned
*/
use rand::{thread_rng, Rng};

use avian2d::prelude::{
    Collider, LayerMask, LinearVelocity, ShapeCastConfig, ShapeHitData, SpatialQuery, SpatialQueryFilter
};
use bevy::{ecs::entity::{EntityHash, EntityHashSet}, prelude::*};
use big_brain::{
    prelude::{ActionState, Actor, Score},
    thinker::ThinkerBuilder,
    BigBrainSet,
};

use crate::{
    consts::TILE_SIZE, game::{
        characters::{
            ai::components::{
                AIAutoShootConfig, AIChaseAction, AICombatAggroConfig, AIShootAction,
                AIWanderAction, AIWanderConfig, AttackScorer, ChaseScorer,
            },
            player::PlayerSelectedHero,
        },
        combat::{AttackDirection, EventRequestAttack},
    }, loading::splashscreen::MainCamera, playing_game, utilities::tiles_to_f32
};

/// stupid ai systems and functions
pub struct StupidAiPlugin;

impl Plugin for StupidAiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (stupid_ai_aggro_manager)
                .run_if(playing_game())
                .in_set(BigBrainSet::Scorers),
        )
        .add_systems(
            FixedUpdate,
            (wander_action, chase_action, attack_action)
                .run_if(playing_game()) // shoot_action,
                .in_set(BigBrainSet::Actions),
        );
    }
}

/// All Components needed for `stupid_ai` functionality
#[derive(Bundle)]
pub struct BasicAiBundle {
    /// ai chase/attack config
    pub combat_config: AICombatAggroConfig,
    /// stupid wander action
    pub wander_config: AIWanderConfig,
    /// stupid shoot action
    pub shoot_config: AIAutoShootConfig,
    /// chooses action
    pub thinker: ThinkerBuilder,
}

/// updates character attack/chase score
#[allow(clippy::type_complexity)]
fn stupid_ai_aggro_manager(
    physics_query: SpatialQuery,
    // player
    player_query: Query<(Entity, &Transform), With<PlayerSelectedHero>>,
    // enemies that can aggro
    can_attack_query: Query<(Entity, &Transform, &AICombatAggroConfig)>,
    // scorers
    mut scorers: ParamSet<(
        Query<(&Actor, &mut Score), With<ChaseScorer>>,
        Query<(&Actor, &mut Score), With<AttackScorer>>,
    )>,
    children: Query<&Children>,
    colliders: Query<&Collider>,
) {
    let Ok((player, player_transform)) = player_query.single() else {
        warn!("no player for stupid-ai-manager too use");
        return;
    };

    let Some(player_collider_ent) = children
        .iter_descendants(player)
        .find(|f| colliders.get(*f).is_ok())
    else {
        warn!("could not get collider for player");
        return;
    };

    for (this_actor, enemy_transform, combat_cfg) in &can_attack_query {
        let player_pos = player_transform.translation.truncate();
        let enemy_pos = enemy_transform.translation.truncate();
        let distance_to_target = enemy_pos.distance(player_pos).abs();
        let direction_to_target = Vec2::normalize(player_pos - enemy_pos);

        let Some(actor_collider) = children
            .iter_descendants(this_actor)
            .find(|f| colliders.get(*f).is_ok())
        else {
            continue;
        };

        if distance_to_target <= 0.0 || distance_to_target >= TILE_SIZE * 32.0 {
            continue;
        }

        let mut excluded_entities = EntityHashSet::with_capacity(15);

        excluded_entities.insert(actor_collider);
        excluded_entities.insert(this_actor);

        let ray = physics_query.cast_shape(
            &Collider::circle(TILE_SIZE),
            enemy_pos,
            0.0,
            Dir2::new_unchecked(direction_to_target),
            &ShapeCastConfig::from_target_distance(distance_to_target),
            &SpatialQueryFilter {
                mask: LayerMask::ALL,
                excluded_entities,
            },
        );

        let target_in_shoot_range = distance_to_target <= tiles_to_f32(combat_cfg.shoot_range);
        let target_in_chase_range = distance_to_target <= tiles_to_f32(combat_cfg.chase_start);
        let target_in_personalspace = distance_to_target <= tiles_to_f32(combat_cfg.personal_space);

        let can_reach_target: bool = match ray {
            None => false,
            Some(ShapeHitData { entity, .. }) => entity == player_collider_ent,
        };

        if can_reach_target {
            if target_in_chase_range {
                trace!("target in chase range");
                set_chase_score(&mut scorers, this_actor, 0.7);
            } else {
                trace!("target out of chase range");
                set_chase_score(&mut scorers, this_actor, 0.5);
            }
            if target_in_shoot_range && !target_in_personalspace {
                trace!("target in shoot range");
                set_attack_score(&mut scorers, this_actor, 0.9);
            } else {
                trace!("target not in shoot range");
                set_attack_score(&mut scorers, this_actor, 0.4);
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
    mut enemy_query: Query<(&Transform, &mut LinearVelocity, &AICombatAggroConfig)>,
    mut chasing_enemies: Query<(&Actor, &mut ActionState), With<AIChaseAction>>,
) {
    let Ok(player_transform) = player_query.single() else {
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
                        *velocity = LinearVelocity(direction * 50.);
                    } else if actor_in_personal_space {
                        // move away from player if too close
                        trace!("actor in personal space, moving away");
                        *velocity = LinearVelocity(-direction * 50.);
                    }
                }
                ActionState::Cancelled => {
                    trace!("chase cancelled");
                    *state = ActionState::Failure;
                }
                ActionState::Failure | ActionState::Success => {
                    trace!("chase finished/failed");
                    *velocity = LinearVelocity::ZERO;
                }
            }
        }
    }
}

// this action should handle determining if the requested
// ai actor has a weapon or not and either send melee or weapon attack
/// handles enemy's that can attack
fn attack_action(
    time: Res<Time>,
    player_query: Query<(Entity, &Transform), With<PlayerSelectedHero>>,
    mut enemy_query: Query<(&Transform, &mut AIAutoShootConfig)>,
    mut ai_with_attacks: Query<(&Actor, &mut ActionState), With<AIShootAction>>,
    mut attack_requests: EventWriter<EventRequestAttack>,
) {
    let Ok((_, player_transform)) = player_query.single() else {
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
                        attack_requests.write(EventRequestAttack {
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
    time: Res<Time>,
    rapier_context: SpatialQuery,
    camera_pos: Query<&Transform, With<MainCamera>>,
    mut enemy_query: Query<(
        &Transform,
        &mut LinearVelocity,
        &mut Sprite,
        &mut AIWanderConfig,
    ), Without<MainCamera>>,
    mut thinker_query: Query<(&Actor, &mut ActionState), With<AIWanderAction>>,
    children: Query<&Children>,
    colliders: Query<&Collider>,
) {
    for (Actor(actor), mut state) in &mut thinker_query {
        if let Ok((enemy_transform, mut velocity, _sprite, mut can_meander_tag)) =
            enemy_query.get_mut(*actor)
        {
            let Ok(camera_pos) = camera_pos.single() else {
                return
            };
            let camera_pos = camera_pos.translation.truncate();
            let enemy_pos = enemy_transform.translation.truncate();

            if camera_pos.distance(enemy_pos) >= 32.0 * TILE_SIZE {
                continue;
            }

            let mut rng = thread_rng();
            let target_pos = can_meander_tag.wander_target;
            let wander_distance = can_meander_tag.wander_distance as f32 * TILE_SIZE;
            let target_deviation = rng.gen_range(25.0..=75.0);

            let spawn_pos = can_meander_tag
                .spawn_position
                .expect("theres always a spawn position, this can be expected");

            match *state {
                ActionState::Init => {}
                ActionState::Cancelled => {
                    *state = ActionState::Failure;
                }
                ActionState::Requested => {
                    if can_meander_tag.idle_timer.finished() {
                        // TODO: scale this based on some 'enemy_jumpiness' setting
                        // make the max idle lower as difficulty increases
                        can_meander_tag.idle_timer =
                            Timer::from_seconds(rng.gen_range(0.0..6.0), TimerMode::Once);
                        if target_pos
                            .is_some_and(|f| f.distance(enemy_pos).abs() <= target_deviation)
                            || target_pos.is_none()
                        {
                            can_meander_tag.wander_target = Some(Vec2 {
                                x: (spawn_pos.x
                                    + rng.gen_range(-wander_distance..=wander_distance)),
                                y: (spawn_pos.y
                                    + rng.gen_range(-wander_distance..=wander_distance)),
                            });
                            *state = ActionState::Requested;
                        }
                        *state = ActionState::Executing;
                    } else {
                        can_meander_tag.idle_timer.tick(time.delta());
                    }
                }
                ActionState::Executing => {
                    let Some(target_pos) = target_pos else {
                        *state = ActionState::Requested;
                        continue;
                    };
                    let direction = (target_pos - enemy_pos).normalize_or_zero();
                    let distance = enemy_pos.distance(target_pos).abs();

                    let Some(actor_collider) = children
                        .iter_descendants(*actor)
                        .find(|f| colliders.get(*f).is_ok())
                    else {
                        continue;
                    };

                    if distance <= 0.0 {
                        continue;
                    }

                    let mut excluded_entities = EntityHashSet::with_capacity(15);
                    excluded_entities.insert(actor_collider);
                    excluded_entities.insert(*actor);

                    let ray = rapier_context.cast_shape(
                        &Collider::circle(TILE_SIZE),
                        enemy_pos,
                        0.0,
                        Dir2::new_unchecked(direction),
                        &ShapeCastConfig::from_max_distance(distance),
                        &SpatialQueryFilter {
                            mask: LayerMask::ALL,
                            excluded_entities,
                        },
                    );

                    if ray.is_some_and(|f| f.distance <= distance * 0.1) {
                        can_meander_tag.wander_target = None;
                        *state = ActionState::Requested;
                    }

                    if distance <= target_deviation {
                        *state = ActionState::Requested;
                    } else {
                        *velocity = LinearVelocity(direction * 100.);
                    }
                }
                ActionState::Success | ActionState::Failure => {
                    // clear target, set velocity to None  // we actually don't want too succeed at this action because then the ai will just do nothing. if i set it too not be last resort action i bet it would work
                    *velocity = LinearVelocity::ZERO;
                    can_meander_tag.wander_target = None;
                    *state = ActionState::Requested;
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
