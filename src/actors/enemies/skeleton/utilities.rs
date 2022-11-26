use bevy::{
    prelude::{
        default, info, BuildChildren, Commands, Entity, Name, Query, ResMut, Transform, Vec2, Vec3,
        With,
    },
    sprite::{SpriteSheetBundle, TextureAtlasSprite},
    time::Timer,
    transform::TransformBundle,
};

use bevy_rapier2d::prelude::*;
use big_brain::{prelude::FirstToScore, thinker::Thinker};

// use heron::{CollisionShape, PhysicMaterial, RotationConstraints, Velocity};
use leafwing_input_manager::prelude::ActionState as LeafActionState;

use crate::{
    action_manager::actions::PlayerBindables,
    actors::enemies::skeleton::SkeletonBundle,
    components::actors::{
        ai::{AIAttackAction, AIAttackTimer, AIEnemy, AIIsAggroed},
        animation::{AnimState, AnimationSheet, FacingDirection},
        bundles::{ActorColliderBundle, RigidBodyBundle},
        general::{ActorState, Player},
    },
    game::TimeInfo,
    loading::assets::EnemyTextureHandles,
    utilities::game::{ACTOR_PHYSICS_LAYER, PLAYER_SIZE},
};

pub fn spawn_skeleton_button(
    mut commands: Commands,
    enemyassets: ResMut<EnemyTextureHandles>,
    query_action_state: Query<&LeafActionState<PlayerBindables>>,
    player_query: Query<(&Transform, With<Player>)>,
) {
    if !query_action_state.is_empty() {
        let actions = query_action_state.get_single().expect("no ents?");

        if actions.just_released(PlayerBindables::Heal) {
            info!("Pressed devtest button");

            if !player_query.is_empty() {
                commands
                    .spawn((
                        SkeletonBundle {
                            name: Name::new("Skeleton"),
                            actortype: AIEnemy,
                            actorstate: ActorState {
                                speed: 100.0,
                                sprint_available: false,
                                facing: FacingDirection::Idle,
                                just_moved: false,
                            },
                            animation_state: AnimState {
                                timer: Timer::from_seconds(0.2, bevy::time::TimerMode::Repeating),
                                current_frames: vec![0, 1, 2, 3, 4],
                                current_frame: 0,
                            },
                            available_animations: AnimationSheet {
                                handle: enemyassets.skele_full_sheet.clone(),
                                idle_animation: [0, 1, 2, 3, 4],
                                down_animation: [5, 6, 7, 8, 9],
                                up_animation: [10, 11, 12, 13, 14],
                                right_animation: [15, 16, 17, 18, 19],
                            },
                            sprite: SpriteSheetBundle {
                                sprite: TextureAtlasSprite {
                                    custom_size: Some(PLAYER_SIZE), //character is 1 tile wide by 2 tiles wide
                                    ..default()
                                },
                                texture_atlas: enemyassets.skele_full_sheet.clone(),
                                transform: Transform {
                                    translation: player_query
                                        .get_single()
                                        .expect("always a player, hopefully")
                                        .0
                                        .translation,
                                    ..default()
                                },
                                ..default()
                            },
                            rigidbody: RigidBodyBundle {
                                rigidbody: RigidBody::Dynamic,
                                velocity: Velocity::zero(),
                                friction: Friction::coefficient(0.7),
                                howbouncy: Restitution::coefficient(0.3),
                                massprop: ColliderMassProperties::Density(0.3),
                                rotationlocks: LockedAxes::ROTATION_LOCKED,
                                dampingprop: Damping {
                                    linear_damping: 1.0,
                                    angular_damping: 1.0,
                                },
                            },
                            // aggroable: AIAggroDistance { distance: 200.0 },
                        },
                        AIAttackTimer {
                            timer: Timer::from_seconds(5., bevy::time::TimerMode::Repeating),
                            is_attacking: false,
                            is_near: false,
                        },
                        Thinker::build()
                            .picker(FirstToScore { threshold: 0.8 })
                            .when(AIIsAggroed, AIAttackAction),
                    ))
                    .with_children(|child| {
                        child.spawn(ActorColliderBundle {
                            transform_bundle: TransformBundle {
                                local: (Transform {
                                    translation: (Vec3 {
                                        x: 0.,
                                        y: -5.,
                                        z: ACTOR_PHYSICS_LAYER,
                                    }),
                                    ..default()
                                }),
                                ..default()
                            },
                            collider: Collider::capsule_y(10.4, 13.12),
                        });
                    });
            }
        };
    }
}

pub fn update_skeleton_graphics(
    timeinfo: ResMut<TimeInfo>,
    mut enemy_query: Query<(
        &mut Velocity,
        &mut ActorState,
        &mut TextureAtlasSprite,
        Entity,
        With<AIEnemy>,
    )>,
) {
    if !timeinfo.game_paused {
        enemy_query.for_each_mut(|(velocity, mut enemystate, mut sprite, _ent, _)| {
            if velocity.linvel == Vec2::ZERO {
                enemystate.facing = FacingDirection::Idle;
            } else if velocity.linvel.x > 5.0 {
                sprite.flip_x = false;
                enemystate.facing = FacingDirection::Right;
            } else if velocity.linvel.x < -5.0 {
                sprite.flip_x = true;
                enemystate.facing = FacingDirection::Left;
            } else if velocity.linvel.y < -5.0 {
                enemystate.facing = FacingDirection::Down;
            } else if velocity.linvel.y > 5.0 {
                enemystate.facing = FacingDirection::Up;
            }
        })
    }
}
