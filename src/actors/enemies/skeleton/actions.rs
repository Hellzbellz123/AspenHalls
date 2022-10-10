use bevy::prelude::*;
use heron::{
    CollisionLayers, CollisionShape, PhysicMaterial, RigidBody, RotationConstraints, Velocity,
};

use crate::{
    actors::{
        components::{Attacking, Enemy, Player, ProjectileBundle, TimeToLive},
        RigidBodyBundle,
    },
    game::TimeInfo,
    loading::assets::PlayerTextureHandles,
    utilities::game::PhysicsLayers,
};

pub fn on_shoot(
    mut commands: Commands,
    timeinfo: Res<TimeInfo>,
    time: Res<Time>,
    assets: ResMut<PlayerTextureHandles>,
    player_query: Query<&Transform, With<Player>>,
    mut query: Query<(&Transform, &mut Attacking), With<Enemy>>,
) {
    let rconstraints = RotationConstraints::allow();

    if !timeinfo.game_paused {
        if let Ok(player_transform) = player_query.get_single() {
            for (transform, mut attacking) in query.iter_mut() {
                // Only shoot when the cooldown is over
                if !attacking.is_attacking || !attacking.timer.tick(time.delta()).just_finished() {
                    continue;
                }

                let direction =
                    (player_transform.translation - transform.translation).normalize_or_zero();

                // Make sure that the projectiles spawn outside of the body so that it doesn't collide
                let beyond_body_diff = direction * 36.;
                let mut new_transform = *transform;
                new_transform.translation = transform.translation + beyond_body_diff;

                commands
                    .spawn_bundle(ProjectileBundle {
                        sprite_bundle: SpriteBundle {
                            texture: assets.rex_attack.clone(),
                            transform: new_transform,
                            sprite: Sprite {
                                custom_size: Some(Vec2::new(32.0, 32.0)),
                                ..default()
                            },
                            ..default()
                        },

                        collider_bundle: RigidBodyBundle {
                            collision_layers: CollisionLayers::none()
                                .with_groups(vec![PhysicsLayers::EnemyAttack, PhysicsLayers::Enemy])
                                .with_masks(&[
                                    // PhysicsLayers::World,
                                    PhysicsLayers::Player,
                                    PhysicsLayers::PlayerAttack,
                                ]),
                            rigidbody: RigidBody::Dynamic,
                            rconstraints,
                            physicsmat: PhysicMaterial {
                                restitution: 0.7,
                                density: 1.,
                                friction: 0.5,
                            },
                            velocity: Velocity::from_linear(direction * 150.),
                        },

                        ttl: TimeToLive(Timer::from_seconds(5.0, false)),
                    })
                    .insert(CollisionShape::Cuboid {
                        half_extends: Vec3::new(4.0, 4.0, 0.0),
                        border_radius: None,
                    });
            }
        }
    } else {
        info!("cant attack, game paused")
    }
}
