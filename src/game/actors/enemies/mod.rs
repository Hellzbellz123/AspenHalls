use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    bundles::{ProjectileBundle, ProjectileColliderBundle, RigidBodyBundle},
    consts::{AspenCollisionLayer, ACTOR_PHYSICS_Z_INDEX, ACTOR_Z_INDEX, BULLET_SPEED_MODIFIER},
    loading::assets::ActorTextureHandles,
};

//TODO: on startup, load all ron files in assets/packs/asha/actors
//create hashmap of `(ActorBundle, ActorColliderBundle)` with key `String` as app resource
// spawn functions should pull from this resource

/// shooting and graphics for enemies
pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            enemy_can_shoot_check.run_if(resource_exists::<ActorTextureHandles>()),
        );
    }
}

use bevy::prelude::{Query, ResMut, Vec2, With};

use super::{
    ai::components::{AIShootConfig, Enemy},
    components::{
        EnemyProjectileColliderTag, EnemyProjectileTag, Player, ProjectileStats, TimeToLive,
    },
};

use bevy_rapier2d::prelude::{RigidBody, Velocity};

/// timer for shooting
#[derive(Resource, Deref, DerefMut)]
pub struct ShootTimer(pub Timer);

/// checks if enemy can shoot and shoots if check is true
pub fn enemy_can_shoot_check(
    mut cmds: Commands,
    time: Res<Time>,
    assets: ResMut<ActorTextureHandles>,
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&Transform, &mut AIShootConfig), With<Enemy>>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };

    for (enemy_transform, mut ai_attack_state) in &mut enemy_query {
        let enemy_loc = enemy_transform.translation.truncate();
        let player_loc = player_transform.translation.truncate();
        let direction: Vec2 = (player_loc - enemy_loc).normalize_or_zero();

        // Make sure that the projectiles spawn outside of the body so that it doesn't collide
        let beyond_body_diff: Vec2 = direction * 36.;
        let modified_spawn_location: Vec2 = enemy_loc + beyond_body_diff;

        if ai_attack_state.should_shoot {
            info!("should shoot");
            if ai_attack_state.timer.tick(time.delta()).finished() {
                spawn_enemy_projectile(
                    &mut cmds,
                    assets.bevy_icon.clone(),
                    direction,
                    modified_spawn_location,
                );
                ai_attack_state.timer.reset();
            }
        }
    }
}

/// spawns enemy projectile
pub fn spawn_enemy_projectile(
    cmds: &mut Commands,
    projectile_texture: Handle<Image>,
    direction: Vec2,
    location: Vec2,
) {
    cmds.spawn((
        EnemyProjectileTag,
        ProjectileBundle {
            name: Name::new("EnemyProjectile"),
            projectile_stats: ProjectileStats {
                damage: 10.0,
                speed: 100.0,
                size: 5.0,
            },
            ttl: TimeToLive(Timer::from_seconds(2.0, TimerMode::Repeating)),
            sprite_bundle: SpriteBundle {
                texture: projectile_texture,
                transform: Transform::from_translation(location.extend(ACTOR_Z_INDEX)),
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(5.0)),
                    ..default()
                },
                ..default()
            },
            rigidbody_bundle: RigidBodyBundle {
                velocity: Velocity::linear(direction * (BULLET_SPEED_MODIFIER * 5.0)),
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
            EnemyProjectileColliderTag,
            ProjectileColliderBundle {
                name: Name::new("EnemyProjectileCollider"),
                transform_bundle: TransformBundle {
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
                collision_groups: CollisionGroups::new(
                    AspenCollisionLayer::PROJECTILE,
                    AspenCollisionLayer::EVERYTHING,
                ),
                ttl: TimeToLive(Timer::from_seconds(2.0, TimerMode::Repeating)),
            },
            ActiveEvents::COLLISION_EVENTS,
        ));
    });
}
