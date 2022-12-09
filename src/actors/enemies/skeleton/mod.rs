use bevy::{
    prelude::{Bundle, Name},
    sprite::SpriteSheetBundle,
};

use crate::components::actors::{
    ai::AIEnemy,
    animation::{AnimState, AnimationSheet},
    bundles::{RigidBodyBundle, SkeletonAiBundle},
    general::MovementState,
};

#[derive(Bundle)]
pub struct SkeletonBundle {
    pub name: Name,
    pub actortype: AIEnemy,
    pub actorstate: MovementState,
    pub animation_state: AnimState,
    pub available_animations: AnimationSheet,
    #[bundle]
    pub brain: SkeletonAiBundle,
    #[bundle]
    pub sprite: SpriteSheetBundle,
    #[bundle]
    pub rigidbody: RigidBodyBundle,
}

pub mod actions {
    use bevy::prelude::*;
    use bevy_rapier2d::prelude::{
        ActiveEvents, Collider, ColliderMassProperties, Damping, Friction, LockedAxes, Restitution,
        RigidBody, Sensor, Velocity,
    };

    use crate::{
        components::actors::{
            ai::{AIAttackTimer, AIEnemy},
            bundles::{ActorColliderBundle, ProjectileBundle, RigidBodyBundle},
            general::{Player, TimeToLive},
        },
        game::TimeInfo,
        loading::assets::GameTextureHandles,
        utilities::game::{ACTOR_LAYER, ACTOR_PHYSICS_LAYER},
    };

    pub fn on_shoot(
        mut commands: Commands,
        timeinfo: Res<TimeInfo>,
        time: Res<Time>,
        assets: ResMut<GameTextureHandles>,
        player_query: Query<&Transform, With<Player>>,
        mut query: Query<(&Transform, &mut AIAttackTimer), With<AIEnemy>>,
    ) {
        // let rconstraints = RotationConstraints::allow();

        if !timeinfo.game_paused {
            if let Ok(player_transform) = player_query.get_single() {
                for (transform, mut attacking) in query.iter_mut() {
                    // Only shoot when the cooldown is over
                    if !attacking.is_attacking
                        || !attacking.timer.tick(time.delta()).just_finished()
                    {
                        continue;
                    }

                    let direction: Vec3 =
                        (player_transform.translation - transform.translation).normalize_or_zero();

                    // Make sure that the projectiles spawn outside of the body so that it doesn't collide
                    let beyond_body_diff = direction * 36.;
                    let mut new_transform = *transform;
                    new_transform.translation = transform.translation + beyond_body_diff;

                    commands
                        .spawn((
                            ProjectileBundle {
                                name: Name::new("EnemyProjectile"),
                                sprite_bundle: SpriteBundle {
                                    texture: assets.bevy_icon.clone(),
                                    transform: Transform {
                                        translation: new_transform
                                            .translation
                                            .truncate()
                                            .extend(ACTOR_LAYER),
                                        ..default()
                                    }, //new_transform,
                                    sprite: Sprite {
                                        custom_size: Some(Vec2::new(32.0, 32.0)),
                                        ..default()
                                    },
                                    ..default()
                                },

                                rigidbody_bundle: RigidBodyBundle {
                                    velocity: Velocity::linear(direction.truncate() * 250.),
                                    rigidbody: RigidBody::Dynamic,
                                    friction: Friction::coefficient(0.7),
                                    howbouncy: Restitution::coefficient(0.3),
                                    massprop: ColliderMassProperties::Density(0.3),
                                    rotationlocks: LockedAxes::ROTATION_LOCKED,
                                    dampingprop: Damping {
                                        linear_damping: 1.0,
                                        angular_damping: 1.0,
                                    },
                                },
                                ttl: TimeToLive(Timer::from_seconds(5.0, TimerMode::Repeating)),
                            },
                            Sensor,
                        ))
                        .with_children(|child| {
                            child.spawn((
                                ActorColliderBundle {
                                    transform_bundle: TransformBundle {
                                        local: (Transform {
                                            translation: (Vec3 {
                                                x: 0.,
                                                y: 0.,
                                                z: ACTOR_PHYSICS_LAYER,
                                            }),
                                            ..default()
                                        }),
                                        ..default()
                                    },
                                    collider: Collider::ball(10.0),
                                },
                                TimeToLive(Timer::from_seconds(5.0, TimerMode::Repeating)),
                                ActiveEvents::COLLISION_EVENTS,
                                Name::new("EnemyProjectileCollider"),
                            ));
                        });
                }
            } else {
                info!("cant attack, game paused")
            }
        }
    }
}

pub mod utilities {}
