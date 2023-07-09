use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use big_brain::thinker::Thinker;

use crate::{
    bundles::{
        ActorAttributesBundle, ActorBundle, ActorColliderBundle, RigidBodyBundle, StupidAiBundle,
    },
    consts::{
        ACTOR_COLLIDER, ACTOR_PHYSICS_Z_INDEX, ACTOR_SIZE, ACTOR_Z_INDEX, PLAYER_PROJECTILE_LAYER,
    },
    game::actors::{
        ai::components::{
            AIAttackState, AICanAggro, AICanShoot, AICanWander, AIChaseAction, AIWanderAction,
            ActorType, AggroScore, Enemy, Faction,
        },
        animation::components::{ActorAnimationType, AnimState, AnimationSheet},
        components::EnemyColliderTag,
    },
    loading::assets::ActorTextureHandles,
};

use super::components::SpawnEnemyEvent;

/// spawn skeleton fn, called from event handler
pub fn spawn_skeleton(
    enemycontainer: Entity,
    commands: &mut Commands,
    enemyassets: &ActorTextureHandles,
    event: &SpawnEnemyEvent,
) {
    commands
    .get_entity(enemycontainer)
    .expect("should always be atleast one entity container. if this panics we probably made more than 1")
        .with_children(|parent| {
            parent
            .spawn((
                Enemy,
                StupidAiBundle {
                    canaggro: AICanAggro { aggro_distance: 200.0 },
                    canmeander: AICanWander { wander_target: None, spawn_position: Some(event.spawn_position) },
                    aiattacktimer: AIAttackState {
                        timer: Timer::new(Duration::from_secs_f32(0.5), TimerMode::Once),
                        should_shoot: false,
                        is_near: false,
                    },
                    thinker: Thinker::build()
                        .picker(big_brain::pickers::Highest)
                        .when(AggroScore, AIChaseAction)
                        .otherwise(AIWanderAction),
                    canshoot: AICanShoot { shoot_range: 500.0},
                },
                ActorBundle {
                    name: Name::new("Skeleton"),
                    actortype: ActorType(Faction::Enemy),
                    stats: ActorAttributesBundle::default(),
                    animationstate: AnimState {
                        facing: ActorAnimationType::Idle,
                        timer: Timer::from_seconds(0.2, TimerMode::Repeating),
                        animation_frames: vec![0, 1, 2, 3, 4],
                        active_frame: 0,
                    },
                    available_animations: AnimationSheet {
                        handle: enemyassets.skeleton_sheet.clone(),
                        idle_animation: [0, 1, 2, 3, 4],
                        down_animation: [5, 6, 7, 8, 9],
                        up_animation: [10, 11, 12, 13, 14],
                        right_animation: [15, 16, 17, 18, 19],
                    },
                    sprite: SpriteSheetBundle {
                        sprite: TextureAtlasSprite {
                            custom_size: Some(ACTOR_SIZE), //character is 1 tile wide by 2 tiles wide
                            ..default()
                        },
                        texture_atlas: enemyassets.skeleton_sheet.clone(),
                        transform: Transform {
                            translation: event.spawn_position.extend(ACTOR_Z_INDEX),
                            rotation: Quat::default(),
                            scale: Vec3::ONE,
                        },
                        ..default()
                    },
                        rigidbody: RigidBodyBundle {
                            rigidbody: bevy_rapier2d::prelude::RigidBody::Dynamic,
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
                }))
                .with_children(|child| {
                    child.spawn((
                        EnemyColliderTag,
                        ActorColliderBundle {
                        name: Name::new("SkeletonCollider"),
                        transformbundle: TransformBundle {
                            local: (
                                Transform {
                                translation: (Vec3 {
                                    x: 0.,
                                    y: -5.,
                                    z: ACTOR_PHYSICS_Z_INDEX,
                            }),
                                ..default()
                            }),
                            ..default()
                        },
                        collider: Collider::capsule(ACTOR_COLLIDER.0, ACTOR_COLLIDER.1, ACTOR_COLLIDER.2),
                        //capsule_y(10.4, 13.12),
                        collisiongroups: CollisionGroups { memberships: Group::all(), filters: PLAYER_PROJECTILE_LAYER },
                    }));
                });
        });
}

/// spawn slime fn, called from spawner
pub fn spawn_slime(
    enemycontainer: Entity,
    commands: &mut Commands,
    enemyassets: &ActorTextureHandles,
    event: &SpawnEnemyEvent,
) {
    commands
            .get_entity(enemycontainer)
            .expect("should always be atleast one entity container. if this panics we probably made more than 1")
            .with_children(|parent| {
                parent
                        .spawn((
                            Enemy,
                            StupidAiBundle {
                                    canaggro: AICanAggro { aggro_distance: 200.0 },
                                    canmeander: AICanWander { wander_target: None, spawn_position: Some(event.spawn_position) },
                                    canshoot: AICanShoot { shoot_range: 500.0  },
                                    aiattacktimer: AIAttackState {
                                        should_shoot: false,
                                        is_near: false,
                                        timer: Timer::new(Duration::from_secs_f32(1.0), TimerMode::Once),
                                    },
                                    thinker: Thinker::build()
                                        .picker(big_brain::pickers::Highest)
                                        .when(AggroScore, AIChaseAction)
                                        .otherwise(AIWanderAction),
                                },
                            ActorBundle {
                                name: Name::new("Slime"),
                                actortype: ActorType(Faction::Enemy),
                                stats: ActorAttributesBundle::default(),
                                    animationstate: AnimState {
                                        facing: ActorAnimationType::Idle,
                                        timer: Timer::from_seconds(0.2, TimerMode::Repeating),
                                        animation_frames: vec![0, 1, 2, 3, 4],
                                        active_frame: 0,
                                    },
                                    available_animations: AnimationSheet {
                                        handle: enemyassets.slime_sheet.clone(),
                                        idle_animation: [0, 1, 2, 3, 4],
                                        down_animation: [5, 6, 7, 8, 9],
                                        up_animation: [10, 11, 12, 13, 14],
                                        right_animation: [15, 16, 17, 18, 19],
                                    },
                                    sprite: SpriteSheetBundle {
                                        sprite: TextureAtlasSprite {
                                            custom_size: Some(ACTOR_SIZE), //character is 1 tile wide by 2 tiles wide
                                            ..default()
                                        },
                                        texture_atlas: enemyassets.slime_sheet.clone(),
                                        transform: Transform {
                                            translation: event.spawn_position.extend(ACTOR_Z_INDEX),
                                            rotation: Quat::default(),
                                            scale: Vec3::ONE,
                                        },
                                        ..default()
                                    },
                                        rigidbody: RigidBodyBundle {
                                            rigidbody: bevy_rapier2d::prelude::RigidBody::Dynamic,
                                            velocity: Velocity::zero(),
                                            friction: Friction::coefficient(0.7),
                                            howbouncy: Restitution::coefficient(1.3),
                                            massprop: ColliderMassProperties::Density(0.6),
                                            rotationlocks: LockedAxes::ROTATION_LOCKED,
                                            dampingprop: Damping {
                                                linear_damping: 1.0,
                                                angular_damping: 1.0,
                                            },
                                        },
                            },
                        ))
                        .with_children(|child| {
                            child.spawn((
                                EnemyColliderTag,
                                ActorColliderBundle {
                                name: Name::new("SlimeCollider"),
                                transformbundle: TransformBundle {
                                    local: (
                                        Transform {
                                        translation: (Vec3 {
                                        x: 0.,
                                            y: -3.0,
                                            z: ACTOR_PHYSICS_Z_INDEX,
                                    }),
                                        ..default()
                                    }),
                                    ..default()
                                },
                                collider: Collider::capsule(ACTOR_COLLIDER.0 / 4.0, ACTOR_COLLIDER.1, ACTOR_COLLIDER.2),
                                collisiongroups: CollisionGroups { memberships: Group::all(), filters: PLAYER_PROJECTILE_LAYER},
                            }));
                        });
                });
}
