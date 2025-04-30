use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    bundles::{Aspen2dPhysicsBundle, AspenColliderBundle, NeedsCollider, ProjectileBundle}, consts::{AspenCollisionLayer, ACTOR_PHYSICS_Z_INDEX, ACTOR_Z_INDEX}, game::{
        attributes_stats::{Damage, ElementalEffect, PhysicalDamage, ProjectileStats},
        characters::ai::components::AIAutoShootConfig,
        components::{ActorColliderType, TimeToLive},
    }, loading::assets::AspenInitHandles, utilities::EntityCreator, AppStage
};

/// handles character attacks if they have no weapons or did not use an action
pub struct UnArmedPlugin;

impl Plugin for UnArmedPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<EventAttackUnarmed>();
        app.add_systems(
            Update,
            delegate_unarmed_attacks.run_if(in_state(AppStage::Running)),
        );
    }
}

/// collects attack requests and passes them too unarmed tool
pub fn delegate_unarmed_attacks(
    mut cmds: Commands,
    mut attack_requests: EventReader<EventAttackUnarmed>,
    ai_shoot_cfg: Query<&AIAutoShootConfig>,
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
                // TODO: get requesters stats and build projectile speed/damage from that
                ProjectileBundle {
                    name: Name::new("MonsterProjectile"),
                    projectile_stats: ProjectileStats {
                        bullet_creator: attack.requester,
                        damage: Damage {
                            physical: PhysicalDamage(5.0),
                            elemental: ElementalEffect::Fire(1.0),
                        },
                    },
                    ttl: TimeToLive(Timer::from_seconds(2.0, TimerMode::Once)),
                    rigidbody_bundle: Aspen2dPhysicsBundle::new_projectile(
                        attack.direction * 250.0,
                    ),
                },
                Sprite {
                    image: assets.img_favicon.clone(),
                    custom_size: Some(Vec2::splat(10.0)),
                    ..default()
                },
                Transform::from_translation(
                    (location + (attack.direction * 12.0)).extend(ACTOR_Z_INDEX),
                ),
                Sensor,
            ))
            .with_children(|bullet_parts| {
                bullet_parts.spawn((
                    EntityCreator(attack.requester),
                    CollisionEventsEnabled,
                    AspenColliderBundle {
                        name: Name::new("MonsterProjectileCollider"),
                        tag: ActorColliderType::Projectile,
                        collider: NeedsCollider::Aabb,
                        collision_groups: AspenCollisionLayer::projectile_actor(),
                        transform: Transform {
                            translation: Vec2::ZERO.extend(ACTOR_PHYSICS_Z_INDEX),
                            ..default()
                        },
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
