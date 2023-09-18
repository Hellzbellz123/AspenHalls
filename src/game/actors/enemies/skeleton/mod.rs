// pub mod actions;
// pub mod actions {
//     use bevy::prelude::*;
//     use bevy_rapier2d::prelude::{
//         ActiveEvents, Collider, ColliderMassProperties, CollisionGroups, Damping, Friction, Group,
//         LockedAxes, Restitution, RigidBody, Sensor, Velocity,
//     };

//     use crate::{
//         components::actors::{
//             ai::{AIAttackTimer, AIEnemy},
//             bundles::{
//                 EnemyColliderBundle, EnemyColliderTag, EnemyProjectileBundle, EnemyProjectileTag,
//                 RigidBodyBundle,
//             },
//             general::{Player, ProjectileStats, TimeToLive},
//         },
//         game::TimeInfo,
//         loading::assets::ActorTextureHandles,
//         utilities::game::{ACTOR_PHYSICS_Z_INDEX, ACTOR_Z_INDEX},
//     };

//     pub fn on_shoot(
//         mut commands: Commands,
//         timeinfo: Res<TimeInfo>,
//         time: Res<Time>,
//         assets: ResMut<ActorTextureHandles>,
//         player_query: Query<&Transform, With<Player>>,
//         mut query: Query<(&Transform, &mut AIAttackTimer), With<AIEnemy>>,
//     ) {
//         // let rconstraints = RotationConstraints::allow();

//         if !timeinfo.game_paused {
//             if let Ok(player_transform) = player_query.get_single() {
//                 for (transform, mut attacking) in query.iter_mut() {
//                     // Only shoot when the cooldown is over
//                     if !attacking.is_attacking
//                         || !attacking.timer.tick(time.delta()).just_finished()
//                     {
//                         continue;
//                     }

//                     let direction: Vec3 =
//                         (player_transform.translation - transform.translation).normalize_or_zero();

//                     // Make sure that the projectiles spawn outside of the body so that it doesn't collide
//                     let beyond_body_diff = direction * 36.;
//                     let mut new_transform = *transform;
//                     new_transform.translation = transform.translation + beyond_body_diff;

//                     commands
//                         .spawn((
//                             EnemyProjectileBundle {
//                                 name: Name::new("EnemyProjectile"),
//                                 sprite_bundle: SpriteBundle {
//                                     texture: assets.bevy_icon.clone(),
//                                     transform: Transform {
//                                         translation: new_transform
//                                             .translation
//                                             .truncate()
//                                             .extend(ACTOR_Z_INDEX),
//                                         ..default()
//                                     }, //new_transform,
//                                     sprite: Sprite {
//                                         custom_size: Some(Vec2::new(32.0, 32.0)),
//                                         ..default()
//                                     },
//                                     ..default()
//                                 },

//                                 rigidbody_bundle: RigidBodyBundle {
//                                     velocity: Velocity::linear(direction.truncate() * 250.),
//                                     rigidbody: RigidBody::Dynamic,
//                                     friction: Friction::coefficient(0.7),
//                                     howbouncy: Restitution::coefficient(0.3),
//                                     massprop: ColliderMassProperties::Density(0.3),
//                                     rotationlocks: LockedAxes::ROTATION_LOCKED,
//                                     dampingprop: Damping {
//                                         linear_damping: 1.0,
//                                         angular_damping: 1.0,
//                                     },
//                                 },
//                                 ttl: TimeToLive(Timer::from_seconds(5.0, TimerMode::Repeating)),
//                                 projectile_stats: ProjectileStats {
//                                     damage: 0.0,
//                                     speed: 0.0,
//                                     size: 8.0,
//                                 },
//                                 tag: EnemyProjectileTag,
//                             },
//                             Sensor,
//                         ))
//                         .with_children(|child| {
//                             child.spawn((
//                                 EnemyColliderBundle {
//                                     name: Name::new("EnemyProjectileCollider"),
//                                     transformbundle: TransformBundle {
//                                         local: (Transform {
//                                             translation: (Vec3 {
//                                                 x: 0.,
//                                                 y: 0.,
//                                                 z: ACTOR_PHYSICS_Z_INDEX,
//                                             }),
//                                             ..default()
//                                         }),
//                                         ..default()
//                                     },
//                                     collider: Collider::ball(10.0),
//                                     tag: EnemyColliderTag,
//                                     collisiongroups: CollisionGroups::new(
//                                         Group::ALL,
//                                         Group::GROUP_30,
//                                     ),
//                                 },
//                                 TimeToLive(Timer::from_seconds(5.0, TimerMode::Repeating)),
//                                 ActiveEvents::COLLISION_EVENTS,
//                             ));
//                         });
//                 }
//             } else {
//                 info!("cant attack, game paused")
//             }
//         }
//     }
// }

// pub mod utilities {}
