use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    bundles::{ItemColliderBundle, ProjectileBundle, RigidBodyBundle},
    consts::{AspenCollisionLayer, ACTOR_PHYSICS_Z_INDEX, ACTOR_Z_INDEX},
    game::{
        attributes_stats::{Damage, ElementalEffect, PhysicalDamage, ProjectileStats},
        characters::ai::components::AIShootConfig,
        components::{ActorColliderType, TimeToLive},
    },
    loading::assets::AspenInitHandles,
    AppState,
};

/// handles character attacks if they have no weapons or did not use an action
pub struct UnArmedPlugin;

impl Plugin for UnArmedPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<EventAttackUnarmed>();
        app.add_systems(
            Update,
            delegate_unarmed_attacks.run_if(state_exists_and_equals(AppState::PlayingGame)),
        );
    }
}

/// collects attack requests and passes them too unarmed tool
pub fn delegate_unarmed_attacks(
    mut cmds: Commands,
    mut attack_requests: EventReader<EventAttackUnarmed>,
    ai_shoot_cfg: Query<&AIShootConfig>,
    assets: Res<AspenInitHandles>,
    locations: Query<&GlobalTransform>,
) {
    for attack in attack_requests.read() {
        let is_unarmed_shoot = ai_shoot_cfg.get(attack.requester).is_ok();

        let location = locations
            .get(attack.requester)
            .expect("msg")
            .translation()
            .truncate();

        if is_unarmed_shoot {
            // create projectile in attack direction.
            cmds.spawn((
                Sensor,
                ProjectileBundle {
                    name: Name::new("MonsterProjectile"),
                    projectile_stats: ProjectileStats {
                        entity_that_shot: attack.requester,
                        damage: Damage {
                            physical: PhysicalDamage(5.0),
                            elemental: ElementalEffect::Fire(1.0),
                        },
                    },
                    ttl: TimeToLive(Timer::from_seconds(2.0, TimerMode::Once)),
                    sprite_bundle: SpriteBundle {
                        texture: assets.img_favicon.clone(),
                        transform: Transform::from_translation(
                            (location + (attack.direction * 12.0)).extend(ACTOR_Z_INDEX),
                        ),
                        sprite: Sprite {
                            custom_size: Some(Vec2::splat(10.0)),
                            ..default()
                        },
                        ..default()
                    },
                    rigidbody_bundle: RigidBodyBundle {
                        velocity: Velocity::linear(attack.direction * 250.0),
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
            ))
            .with_children(|bullet_parts| {
                bullet_parts.spawn((
                    ActiveEvents::COLLISION_EVENTS,
                    ItemColliderBundle {
                        name: Name::new("MonsterProjectileCollider"),
                        transform_bundle: TransformBundle {
                            local: (Transform {
                                translation: Vec2::ZERO.extend(ACTOR_PHYSICS_Z_INDEX),
                                ..default()
                            }),
                            ..default()
                        },
                        tag: ActorColliderType::Projectile,
                        collider: Collider::ball(10.0),
                        collision_groups: CollisionGroups::new(
                            AspenCollisionLayer::PROJECTILE,
                            AspenCollisionLayer::WORLD | AspenCollisionLayer::ACTOR,
                        ),
                    },
                ));
            });

            // TODO: brainstorm possible delegations
        }
    }
}

// TODO:
// implement fist attack as an ability?
// possibly a hidden ability
/// attacked without weapon using fist
#[derive(Debug, Event)]
pub struct EventAttackUnarmed {
    /// who requested fist attack
    pub requester: Entity,
    /// vector to aim attack
    pub direction: Vec2,
}
