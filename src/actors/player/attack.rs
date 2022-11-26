use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
    action_manager::actions::PlayerBindables,
    components::actors::{
        ai::{AIAttackTimer, AIEnemy},
        bundles::{ActorColliderBundle, ProjectileBundle, RigidBodyBundle},
        general::{ActorState, Player, TimeToLive},
    },
    game::TimeInfo,
    loading::assets::PlayerTextureHandles,
    utilities::game::ACTOR_PHYSICS_LAYER,
};

pub fn player_shoot_at_position(
    mut input_query: Query<&ActionState<PlayerBindables>, With<ActorState>>,
    mut player_query: Query<&mut ActorState, With<Player>>,
    _player_transform: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    let action_state = input_query.single_mut();
    let _player = player_query.single_mut();

    if action_state.pressed(PlayerBindables::Shoot) {}

    if action_state.released(PlayerBindables::Dash) {}
}

pub fn on_shoot(
    mut commands: Commands,
    timeinfo: Res<TimeInfo>,
    assets: ResMut<PlayerTextureHandles>,
    player_query: Query<&Transform, With<Player>>,
    mut query: Query<(&Transform, &mut AIAttackTimer), With<AIEnemy>>,
) {
    // let rconstraints = RotationConstraints::allow();

    if !timeinfo.game_paused {
        if let Ok(player_transform) = player_query.get_single() {
            for (transform, mut _attacking) in query.iter_mut() {
                let direction: Vec3 =
                    (player_transform.translation - transform.translation).normalize_or_zero();

                // Make sure that the projectiles spawn outside of the body so that it doesn't collide
                let beyond_body_diff = direction * 36.;
                let mut new_transform = *transform;
                new_transform.translation = transform.translation + beyond_body_diff;

                commands
                    .spawn((
                        ProjectileBundle {
                            name: Name::new("PlayerProjectile"),
                            sprite_bundle: SpriteBundle {
                                texture: assets.rex_attack.clone(),
                                transform: new_transform,
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
                            ttl: TimeToLive(Timer::from_seconds(5.0, TimerMode::Once)),
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
                            Name::new("PlayerProjectile"),
                            TimeToLive(Timer::from_seconds(5.0, TimerMode::Repeating)),
                            ActiveEvents::COLLISION_EVENTS,
                        ));
                    });
            }
        } else {
            info!("cant attack, game paused")
        }
    }
}
