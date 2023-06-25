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

use crate::{
    components::actors::{
        ai::{
            AIAttackState, AICanChase, AICanShoot, AICanWander, AIChaseAction, AIEnemy,
            AIWanderAction, AggroScore, AttackScore, WanderScore,
        },
        animation::FacingDirection,
        general::{MovementState, Player},
    },
    consts::ACTOR_Z_INDEX,
    game::{GameStage, TimeInfo},
};

/// All Componenents needed for 'stupid_ai' functionality
#[derive(Bundle, Debug, Default)]
pub struct StuipidAiBundle {
    wander: AICanWander,
    chase: AICanChase,
    shoot: AICanShoot,
}

pub struct StupidAiPlugin;

impl Plugin for StupidAiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                wander_score_system.run_if(in_state(GameStage::PlayingGame)),
                aggro_score_system.run_if(in_state(GameStage::PlayingGame)),
            )
                .in_set(BigBrainSet::Scorers),
        )
        .add_systems(
            (
                wander_action.run_if(in_state(GameStage::PlayingGame)),
                chase_action.run_if(in_state(GameStage::PlayingGame)), // shoot_action,
            )
                .in_set(BigBrainSet::Actions),
        );
    }
}

fn aggro_score_system(
    player_query: Query<&Transform, With<Player>>, // player
    enemy_query: Query<(&Transform, &AICanChase), With<AIEnemy>>, // enemies that can aggro
    mut aggro_scorer_query: Query<(&Actor, &mut Score), With<AggroScore>>, // aggro scorer
) {
    for (Actor(actor), mut aggro_score) in aggro_scorer_query.iter_mut() {
        let mut closest_player_as_distance = f32::INFINITY;
        let mut closest_player_transform: Option<&Transform> = None;

        // Iterate over available player queries and find the closest player to the enemy
        player_query.for_each(|player_transform| {
            let distance = player_transform.translation.distance(
                enemy_query
                    .get_component::<Transform>(*actor)
                    .unwrap()
                    .translation,
            );
            if distance < closest_player_as_distance {
                closest_player_as_distance = distance;
                closest_player_transform = Some(player_transform);
            }
        });

        if let Some(_player_transform) = closest_player_transform {
            let (_enemy_transform, aggroable) = enemy_query.get(*actor).unwrap();
            if closest_player_as_distance < aggroable.aggro_distance.abs() {
                aggro_score.set(1.0);
            } else {
                aggro_score.set(0.0)
            }
        } else {
            aggro_score.set(0.0);
        }
    }
}

fn wander_score_system(
    player_query: Query<&Transform, With<Player>>, //player
    enemy_query: Query<(&Transform, &AICanChase), With<AIEnemy>>, //enemys that can aggro
    #[allow(clippy::type_complexity)]
    // trunk-ignore(clippy/type_complexity)
    mut wanderscore_query: Query<
        (&Actor, &mut Score),
        (With<WanderScore>, Without<AggroScore>, With<AttackScore>),
    >,
) {
    let Ok(player_transform) = player_query.get_single() else { return };
    wanderscore_query.for_each_mut(|(Actor(actor), mut wander_score)| {
        if let Ok((transform, aggroable)) = enemy_query.get(*actor) {
            let distance = player_transform.translation.distance(transform.translation);

            if distance > aggroable.aggro_distance {
                wander_score.set(1.0)
            } else {
                wander_score.set(0.0)
            }
        }
    });
}

fn attack_score_system(
    _player_query: Query<&Transform, With<Player>>, //player
    _enemy_query: Query<(&Transform, &AICanChase), With<AIEnemy>>, //enemys that can aggro
    #[allow(clippy::type_complexity)]
    // trunk-ignore(clippy/type_complexity)
    _attackscore_query: Query<
        (&Actor, &mut Score),
        (With<AttackScore>, Without<AggroScore>, With<WanderScore>),
    >,
) {
}

fn chase_action(
    _timeinfo: ResMut<TimeInfo>,
    player_query: Query<&Transform, With<Player>>,
    #[allow(clippy::type_complexity)]
    // trunk-ignore(clippy/type_complexity)
    mut enemy_query: Query<(
        &Transform,
        &mut Velocity,
        &AICanChase,
        &mut AIAttackState,
        &mut MovementState,
        &mut TextureAtlasSprite,
        With<AIEnemy>,
    )>,
    mut chasing_enemys: Query<(&Actor, &mut ActionState), With<AIChaseAction>>,
) {
    let Ok(player_transform) = player_query.get_single() else { return; };

    chasing_enemys.for_each_mut(|(Actor(actor), mut state)| {
        if let Ok((
            enemy_transform,
            mut velocity,
            aggroable,
            _attacking,
            mut enemystate,
            _sprite,
            _,
        )) = enemy_query.get_mut(*actor)
        {
            let direction = ((player_transform.translation) - enemy_transform.translation)
                .truncate()
                .normalize_or_zero();

            let distance = enemy_transform
                .translation
                .distance(player_transform.translation)
                .abs();

            match *state {
                ActionState::Init => {}
                ActionState::Requested => {
                    *state = ActionState::Executing;
                }
                ActionState::Executing => {
                    // move towards the player if player is close enough
                    if distance <= aggroable.aggro_distance {
                        *velocity = Velocity::linear(direction * 50.);
                    } else
                    // chase target escaped, failed to chase
                    if distance < 100.0 || distance >= aggroable.aggro_distance {
                        *velocity = Velocity::linear(velocity.linvel.lerp(Vec2::ZERO, 0.3));
                        enemystate.facing = FacingDirection::Idle;
                        *state = ActionState::Failure;
                    }
                    // we really shouldnt hit this block
                    else {
                        *velocity = Velocity::linear(velocity.linvel.lerp(Vec2::ZERO, 0.3));
                        enemystate.facing = FacingDirection::Idle;
                        warn!("AI CHASE ACTION HIT UNKNOWN CIRCUMSTANCES");
                        *state = ActionState::Failure;
                    }
                }
                ActionState::Cancelled => {
                    *velocity = Velocity::linear(Vec2::ZERO);
                    enemystate.facing = FacingDirection::Idle;
                    *state = ActionState::Failure;
                }
                ActionState::Success => {
                    *velocity = Velocity::linear(velocity.linvel.lerp(Vec2::ZERO, 0.3)); // Velocity::linear(Vec2::ZERO);
                    enemystate.facing = FacingDirection::Idle;
                }
                ActionState::Failure => {
                    *velocity = Velocity::linear(velocity.linvel.lerp(Vec2::ZERO, 1.0));
                    enemystate.facing = FacingDirection::Idle;
                }
            }
        }
    });
}

fn wander_action(
    #[allow(clippy::type_complexity)]
    // trunk-ignore(clippy/type_complexity)
    mut enemy_query: Query<(
        &Transform,
        &mut Velocity,
        &mut MovementState,
        &mut TextureAtlasSprite,
        &mut AICanWander,
        With<AIEnemy>,
    )>,
    mut thinker_query: Query<(&Actor, &mut ActionState), With<AIWanderAction>>,
) {
    thinker_query.for_each_mut(|(Actor(actor), mut state)| {
        if let Ok((enemy_transform, mut velocity, _enemystate, _prite, mut can_meander_tag, _a)) =
            enemy_query.get_mut(*actor)
        {
            let target_pos = can_meander_tag.wander_target;
            let spawn_pos = can_meander_tag
                .spawn_position
                .expect("theres always a spawn position, this can be expected");
            let cur_pos = enemy_transform.translation;
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
                    match target_pos {
                        Some(target_pos) => {
                            let c_target_pos: Vec3 = target_pos;
                            let distance = c_target_pos - cur_pos;
                            if distance.length().abs() <= t_deviation {
                                can_meander_tag.wander_target = None;
                                *state = ActionState::Success;
                            } else {
                                *state = ActionState::Executing;
                            }
                        }
                        None => {
                            can_meander_tag.wander_target = Some(Vec3 {
                                x: (spawn_pos.x + rng.gen_range(-300.0..=300.0)), //Rng::gen_range(&mut )),
                                y: (spawn_pos.y + rng.gen_range(-300.0..=300.0)),
                                z: ACTOR_Z_INDEX,
                            });
                            *state = ActionState::Executing;
                        }
                    }
                }
                ActionState::Executing => match target_pos {
                    Some(target_pos) => {
                        let c_target_pos: Vec3 = target_pos;
                        let distance = c_target_pos - cur_pos;
                        if distance.length().abs() <= 60.0 {
                            can_meander_tag.wander_target = None;
                            *state = ActionState::Requested;
                        } else {
                            *velocity =
                                Velocity::linear(distance.normalize_or_zero().truncate() * 100.);
                        }
                    }
                    None => {
                        *state = ActionState::Requested;
                    }
                },
                ActionState::Success => {
                    // clear target, set velocity to None  // we actually dont want too succeed at this action because then the ai will just do nothing. if i set it too not be last resort action i bet it would work
                    *velocity = Velocity::linear(velocity.linvel.lerp(Vec2::ZERO, 1.0));
                    can_meander_tag.wander_target = None;
                    *state = ActionState::Requested;
                }
                ActionState::Failure => {
                    // clear target, set velocity to None
                    *velocity = Velocity::linear(velocity.linvel.lerp(Vec2::ZERO, 1.0));
                    can_meander_tag.wander_target = None;
                    *state = ActionState::Requested;
                }
            }
        }
    });
}