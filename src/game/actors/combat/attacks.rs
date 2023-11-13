use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    bundles::{
        ProjectileBundle, ProjectileColliderBundle, RigidBodyBundle,
    },
    consts::{
        AspenCollisionLayer, ACTOR_PHYSICS_Z_INDEX, ACTOR_Z_INDEX,
        BULLET_SPEED_MODIFIER,
    },
    game::actors::{
        components::{
            PlayerProjectileColliderTag, PlayerProjectileTag,
            ProjectileStats, TimeToLive,
        },
        player::actions::ShootEvent,
    },
    loading::assets::ActorTextureHandles,
};

use super::components::WeaponStats;

/// creates player bullet
pub fn create_bullet(
    cmds: &mut Commands,
    assets: &ResMut<ActorTextureHandles>,
    event: &ShootEvent,
    weapon_stats: &WeaponStats,
) {
    cmds.spawn((
        PlayerProjectileTag,
        ProjectileBundle {
            name: Name::new("PlayerProjectile"),
            projectile_stats: ProjectileStats {
                damage: weapon_stats.damage,
                speed: weapon_stats.bullet_speed,
                size: weapon_stats.projectile_size,
            },
            ttl: TimeToLive(Timer::from_seconds(
                2.0,
                TimerMode::Repeating,
            )),
            sprite_bundle: SpriteBundle {
                texture: assets.bevy_icon.clone(),
                transform: Transform::from_translation(
                    event.bullet_spawn_loc.extend(ACTOR_Z_INDEX), //- Vec3 { x: 0.0, y: -5.0, z: 0.0 },
                ),
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(
                        weapon_stats.projectile_size,
                    )),
                    ..default()
                },
                ..default()
            },
            rigidbody_bundle: RigidBodyBundle {
                velocity: Velocity::linear(
                    event.travel_dir
                        * (weapon_stats.bullet_speed
                            * BULLET_SPEED_MODIFIER),
                ),
                rigidbody: RigidBody::Dynamic,
                friction: Friction::coefficient(0.2),
                how_bouncy: Restitution::coefficient(0.8),
                mass_prop: ColliderMassProperties::Density(2.1),
                rotation_locks: LockedAxes::ROTATION_LOCKED,
                damping_prop: Damping {
                    linear_damping: 0.1,
                    angular_damping: 0.1,
                },
            },
        },
        Sensor,
    ))
    .with_children(|child| {
        child.spawn((
            PlayerProjectileColliderTag,
            ProjectileColliderBundle {
                name: Name::new("PlayerProjectileCollider"),
                transform_bundle: TransformBundle {
                    local: (Transform {
                        translation: Vec2::ZERO
                            .extend(ACTOR_PHYSICS_Z_INDEX),
                        ..default()
                    }),
                    ..default()
                },
                collider: Collider::ball(3.0),
                collision_groups: CollisionGroups::new(
                    AspenCollisionLayer::PROJECTILE,
                    AspenCollisionLayer::WORLD
                        | AspenCollisionLayer::ACTOR
                        | AspenCollisionLayer::PROJECTILE,
                ),
                ttl: TimeToLive(Timer::from_seconds(
                    2.0,
                    TimerMode::Repeating,
                )),
            },
            ActiveEvents::COLLISION_EVENTS,
        ));
    });
}
