// use bevy::prelude::*;
// use bevy_rapier2d::prelude::{
//     ActiveEvents, Collider, ColliderMassProperties, CollisionGroups, Damping, Friction, Group,
//     LockedAxes, Restitution, RigidBody, Sensor, Velocity,
// };

// use crate::{
//     components::actors::{
//         ai::{AIAttackTimer, AIEnemy},
//         bundles::{
//             EnemyProjectileBundle, EnemyProjectileColliderBundle, EnemyProjectileColliderTag,
//             EnemyProjectileTag, RigidBodyBundle,
//         },
//         general::{Player, ProjectileStats, TimeToLive},
//     },
//     game::TimeInfo,
//     loading::assets::ActorTextureHandles,
//     // utilities::game::PhysicsLayers,
//     utilities::game::{ACTOR_PHYSICS_Z_INDEX, PLAYER_PROJECTILE_LAYER},
// };

// pub fn on_shoot(
//     mut commands: Commands,
//     _timeinfo: Res<TimeInfo>,
//     time: Res<Time>,
//     assets: ResMut<ActorTextureHandles>,
//     player_query: Query<&Transform, With<Player>>,
//     mut query: Query<(&Transform, &mut AIAttackTimer), With<AIEnemy>>,
// ) {
//     if let Ok(player_transform) = player_query.get_single() {
//         for (transform, mut attacking) in query.iter_mut() {
//             // Only shoot when the cooldown is over
//             if !attacking.should_shoot || !attacking.timer.tick(time.delta()).just_finished() {
//                 break;
//             }

//             let direction: Vec3 =
//                 (player_transform.translation - transform.translation).normalize_or_zero();

//             // Make sure that the projectiles spawn outside of the body so that it doesn't collide
//             let beyond_body_diff = direction * 36.;
//             let mut new_transform = *transform;
//             new_transform.translation = transform.translation + beyond_body_diff;

//             commands
//                 .spawn((
//                     EnemyProjectileBundle {
//                         name: Name::new("EnemyProjectile"),
//                         sprite_bundle: SpriteBundle {
//                             texture: assets.bevy_icon.clone(),
//                             transform: new_transform,
//                             sprite: Sprite {
//                                 custom_size: Some(Vec2::new(32.0, 32.0)),
//                                 ..default()
//                             },
//                             ..default()
//                         },

//                         rigidbody_bundle: RigidBodyBundle {
//                             velocity: Velocity::linear(direction.truncate() * 250.),
//                             rigidbody: RigidBody::Dynamic,
//                             friction: Friction::coefficient(0.7),
//                             howbouncy: Restitution::coefficient(0.3),
//                             massprop: ColliderMassProperties::Density(0.3),
//                             rotationlocks: LockedAxes::ROTATION_LOCKED,
//                             dampingprop: Damping {
//                                 linear_damping: 1.0,
//                                 angular_damping: 1.0,
//                             },
//                         },
//                         ttl: TimeToLive(Timer::from_seconds(5.0, TimerMode::Repeating)),
//                         tag: EnemyProjectileTag,
//                         projectile_stats: ProjectileStats {
//                             damage: 5.0,
//                             speed: 5.0,
//                             size: 5.0,
//                         },
//                     },
//                     Sensor,
//                 ))
//                 .with_children(|child| {
//                     child.spawn((
//                         EnemyProjectileColliderBundle {
//                             collider: Collider::ball(10.0),
//                             name: Name::new("EnemyProjectileCollider"),
//                             tag: EnemyProjectileColliderTag,
//                             ttl: TimeToLive(Timer::from_seconds(5.0, TimerMode::Repeating)),
//                             transformbundle: TransformBundle {
//                                 local: (Transform {
//                                     translation: (Vec3 {
//                                         x: 0.,
//                                         y: 0.,
//                                         z: ACTOR_PHYSICS_Z_INDEX,
//                                     }),
//                                     ..default()
//                                 }),
//                                 ..default()
//                             },
//                             collisiongroups: CollisionGroups::new(
//                                 PLAYER_PROJECTILE_LAYER,
//                                 Group::from_bits_truncate(0b00101),
//                             ),
//                         },
//                         ActiveEvents::COLLISION_EVENTS,
//                     ));
//                 });
//         }
//     } else {
//         warn!("cant attack, game paused")
//     }
// }
