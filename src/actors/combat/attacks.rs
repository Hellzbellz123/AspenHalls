use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    actors::player::actions::ShootEvent,
    components::actors::{
        bundles::{
            PlayerProjectileBundle, PlayerProjectileColliderBundle, PlayerProjectileTag,
            RigidBodyBundle,
        },
        general::{ProjectileStats, TimeToLive},
    },
    consts::{ACTOR_PHYSICS_Z_INDEX, BULLET_SPEED_MODIFIER, PLAYER_PROJECTILE_LAYER},
    loading::assets::ActorTextureHandles,
};

use super::components::WeaponStats;

pub fn create_bullet(
    cmds: &mut Commands,
    assets: &ResMut<ActorTextureHandles>,
    event: &ShootEvent,
    wstats: &WeaponStats,
) {
    cmds.spawn((
        PlayerProjectileBundle {
            name: Name::new("PlayerProjectile"),
            tag: PlayerProjectileTag,
            projectile_stats: ProjectileStats {
                damage: wstats.damage,
                speed: wstats.bullet_speed,
                size: wstats.projectile_size,
            },
            ttl: TimeToLive(Timer::from_seconds(2.0, TimerMode::Repeating)),
            sprite_bundle: SpriteBundle {
                texture: assets.bevy_icon.clone(),
                transform: Transform::from_translation(
                    event.bullet_spawn_loc, //- Vec3 { x: 0.0, y: -5.0, z: 0.0 },
                ),
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(wstats.projectile_size)),
                    ..default()
                },
                ..default()
            },
            rigidbody_bundle: RigidBodyBundle {
                velocity: Velocity::linear(
                    event.travel_dir * (wstats.bullet_speed * BULLET_SPEED_MODIFIER),
                ),
                rigidbody: RigidBody::Dynamic,
                friction: Friction::coefficient(0.2),
                howbouncy: Restitution::coefficient(0.8),
                massprop: ColliderMassProperties::Density(2.1),
                rotationlocks: LockedAxes::ROTATION_LOCKED,
                dampingprop: Damping {
                    linear_damping: 0.1,
                    angular_damping: 0.1,
                },
            },
        },
        Sensor,
    ))
    .with_children(|child| {
        child.spawn((
            PlayerProjectileColliderBundle {
                name: Name::new("PlayerProjectileCollider"),
                transformbundle: TransformBundle {
                    local: (Transform {
                        translation: (Vec3 {
                            x: 0.,
                            y: 0.,
                            z: ACTOR_PHYSICS_Z_INDEX,
                        }),
                        ..default()
                    }),
                    ..default()
                },
                collider: Collider::ball(3.0),
                tag: crate::components::actors::bundles::PlayerProjectileColliderTag,
                collisiongroups: CollisionGroups::new(
                    PLAYER_PROJECTILE_LAYER,
                    Group::from_bits_truncate(0b00101),
                ),
                ttl: TimeToLive(Timer::from_seconds(2.0, TimerMode::Repeating)),
            },
            ActiveEvents::COLLISION_EVENTS,
        ));
    });
}
