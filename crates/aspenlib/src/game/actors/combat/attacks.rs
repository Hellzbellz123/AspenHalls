use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    bundles::{ItemColliderBundle, ProjectileBundle, RigidBodyBundle},
    consts::{AspenCollisionLayer, ACTOR_PHYSICS_Z_INDEX, ACTOR_Z_INDEX, BULLET_SPEED_MODIFIER},
    game::actors::{
        attributes_stats::{Damage, ProjectileStats},
        combat::components::{AttackDamage, WeaponDescriptor},
        components::{ActorColliderType, TimeToLive},
        player::actions::ShootEvent,
    },
    prelude::game::AspenInitHandles,
};

// TODO: handle attacks as an event
// characters should send attack event
// AttackEvent {weapon: Entity, character: Entity}
// system takes attack event and sends event for weapon type
// WeaponTypeUsedEvent {weapon: Entity, character: Entity}
// systems too handle each weapon type

/// creates player bullet
pub fn create_bullet(
    cmds: &mut Commands,
    assets: &Res<AspenInitHandles>,
    event: &ShootEvent,
    weapon_stats: &WeaponDescriptor,
    weapon_damage: &AttackDamage,
) {
    let gun_data = weapon_stats;

    let WeaponDescriptor::Gun {
        projectile_speed,
        projectile_size,
        barrel_end: _,
        ammo_amount: _,
        reload_time: _,
        fire_rate: _,
    }: WeaponDescriptor = *gun_data
    else {
        panic!("NOT A GUN BUT WANTED TOO SHOOT?")
    };

    cmds.spawn((
        ProjectileBundle {
            name: Name::new("PlayerProjectile"),
            projectile_stats: ProjectileStats {
                damage: Damage {
                    physical: weapon_damage.physical,
                    elemental: weapon_damage.elemental,
                },
                is_player_projectile: false,
            },
            ttl: TimeToLive(Timer::from_seconds(2.0, TimerMode::Repeating)),
            sprite_bundle: SpriteBundle {
                texture: assets.img_favicon.clone(),
                transform: Transform::from_translation(
                    event.bullet_spawn_loc.extend(ACTOR_Z_INDEX), //- Vec3 { x: 0.0, y: -5.0, z: 0.0 },
                ),
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(projectile_size)),
                    ..default()
                },
                ..default()
            },
            rigidbody_bundle: RigidBodyBundle {
                velocity: Velocity::linear(
                    event.travel_dir * (projectile_speed * BULLET_SPEED_MODIFIER),
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
            ItemColliderBundle {
                name: Name::new("PlayerProjectileCollider"),
                transform_bundle: TransformBundle {
                    local: (Transform {
                        translation: Vec2::ZERO.extend(ACTOR_PHYSICS_Z_INDEX),
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
                tag: ActorColliderType::Projectile,
            },
            ActiveEvents::COLLISION_EVENTS,
        ));
    });
}
