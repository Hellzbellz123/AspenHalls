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
    enemy_container: Entity,
    commands: &mut Commands,
    enemy_assets: &ActorTextureHandles,
    event: &SpawnEnemyEvent,
) {
    commands
    .get_entity(enemy_container)
    .expect("should always be at least one entity container. if this panics we probably made more than 1")
        .with_children(|parent| {
            parent
            .spawn((
                Enemy,
                StupidAiBundle {
                    can_aggro: AICanAggro { aggro_distance: 200.0 },
                    can_meander: AICanWander { wander_target: None, spawn_position: Some(event.spawn_position) },
                    ai_attack_timer: AIAttackState {
                        timer: Timer::new(Duration::from_secs_f32(0.5), TimerMode::Once),
                        should_shoot: false,
                        is_near: false,
                    },
                    thinker: Thinker::build()
                        .picker(big_brain::pickers::Highest)
                        .when(AggroScore, AIChaseAction)
                        .otherwise(AIWanderAction),
                    can_shoot: AICanShoot { shoot_range: 500.0},
                },
                ActorBundle {
                    name: Name::new("Skeleton"),
                    actor_type: ActorType(Faction::Enemy),
                    stats: ActorAttributesBundle::default(),
                    animation_state: AnimState {
                        facing: ActorAnimationType::Idle,
                        timer: Timer::from_seconds(0.2, TimerMode::Repeating),
                        animation_frames: vec![0, 1, 2, 3, 4],
                        active_frame: 0,
                    },
                    available_animations: AnimationSheet {
                        handle: enemy_assets.skeleton_sheet.clone(),
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
                        texture_atlas: enemy_assets.skeleton_sheet.clone(),
                        transform: Transform {
                            translation: event.spawn_position.extend(ACTOR_Z_INDEX),
                            rotation: Quat::default(),
                            scale: Vec3::ONE,
                        },
                        ..default()
                    },
                        rigidbody_bundle: RigidBodyBundle {
                            rigidbody: bevy_rapier2d::prelude::RigidBody::Dynamic,
                            velocity: Velocity::zero(),
                            friction: Friction::coefficient(0.7),
                            how_bouncy: Restitution::coefficient(0.3),
                            mass_prop: ColliderMassProperties::Density(0.3),
                            rotation_locks: LockedAxes::ROTATION_LOCKED,
                            damping_prop: Damping {
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
                        transform_bundle: TransformBundle {
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
                        collision_groups: CollisionGroups { memberships: Group::all(), filters: PLAYER_PROJECTILE_LAYER },
                    }));
                });
        });
}

/// spawn slime fn, called from spawner
pub fn spawn_slime(
    enemy_container: Entity,
    commands: &mut Commands,
    enemy_assets: &ActorTextureHandles,
    event: &SpawnEnemyEvent,
) {
    commands
            .get_entity(enemy_container)
            .expect("should always be at least one entity container. if this panics we probably made more than 1")
            .with_children(|parent| {
                parent
                        .spawn((
                            Enemy,
                            StupidAiBundle {
                                    can_aggro: AICanAggro { aggro_distance: 200.0 },
                                    can_meander: AICanWander { wander_target: None, spawn_position: Some(event.spawn_position) },
                                    can_shoot: AICanShoot { shoot_range: 500.0  },
                                    ai_attack_timer: AIAttackState {
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
                                actor_type: ActorType(Faction::Enemy),
                                stats: ActorAttributesBundle::default(),
                                    animation_state: AnimState {
                                        facing: ActorAnimationType::Idle,
                                        timer: Timer::from_seconds(0.2, TimerMode::Repeating),
                                        animation_frames: vec![0, 1, 2, 3, 4],
                                        active_frame: 0,
                                    },
                                    available_animations: AnimationSheet {
                                        handle: enemy_assets.slime_sheet.clone(),
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
                                        texture_atlas: enemy_assets.slime_sheet.clone(),
                                        transform: Transform {
                                            translation: event.spawn_position.extend(ACTOR_Z_INDEX),
                                            rotation: Quat::default(),
                                            scale: Vec3::ONE,
                                        },
                                        ..default()
                                    },
                                        rigidbody_bundle: RigidBodyBundle {
                                            rigidbody: bevy_rapier2d::prelude::RigidBody::Dynamic,
                                            velocity: Velocity::zero(),
                                            friction: Friction::coefficient(0.7),
                                            how_bouncy: Restitution::coefficient(1.3),
                                            mass_prop: ColliderMassProperties::Density(0.6),
                                            rotation_locks: LockedAxes::ROTATION_LOCKED,
                                            damping_prop: Damping {
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
                                transform_bundle: TransformBundle {
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
                                collision_groups: CollisionGroups { memberships: Group::all(), filters: PLAYER_PROJECTILE_LAYER},
                            }));
                        });
                });
}