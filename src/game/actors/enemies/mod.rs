use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    bundles::{ProjectileBundle, ProjectileColliderBundle, RigidBodyBundle},
    consts::{
        ACTOR_PHYSICS_Z_INDEX, ACTOR_Z_INDEX, BULLET_SPEED_MODIFIER, PLAYER_PROJECTILE_LAYER,
    },
    loading::assets::ActorTextureHandles,
};

/// shooting and graphics for enemies
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (on_shoot, update_enemy_graphics).run_if(resource_exists::<ActorTextureHandles>()));
    }
}

use bevy::{
    prelude::{Entity, Query, ResMut, Vec2, With},
    sprite::TextureAtlasSprite,
};

use super::{
    ai::components::{AIAttackState, Enemy},
    animation::components::{ActorAnimationType, AnimState},
    components::{
        EnemyProjectileColliderTag, EnemyProjectileTag, Player, ProjectileStats, TimeToLive,
    },
};
use crate::game::TimeInfo;

/// updates enemy's animation depending on velocity
pub fn update_enemy_graphics(
    time_info: ResMut<TimeInfo>,
    mut enemy_query: Query<(
        &mut Velocity,
        &mut AnimState,
        &mut TextureAtlasSprite,
        Entity,
        With<Enemy>,
    )>,
) {
    if !time_info.game_paused {
        enemy_query.for_each_mut(|(velocity, mut anim_state, mut sprite, _ent, ())| {
            if velocity.linvel == Vec2::ZERO {
                anim_state.facing = ActorAnimationType::Idle;
            } else if velocity.linvel.x > 5.0 {
                sprite.flip_x = false;
                anim_state.facing = ActorAnimationType::Right;
            } else if velocity.linvel.x < -5.0 {
                sprite.flip_x = true;
                anim_state.facing = ActorAnimationType::Left;
            } else if velocity.linvel.y < -5.0 {
                anim_state.facing = ActorAnimationType::Down;
            } else if velocity.linvel.y > 2.0 {
                anim_state.facing = ActorAnimationType::Up;
            }
        });
    }
}

use bevy_rapier2d::prelude::{RigidBody, Velocity};

/// timer for shooting
#[derive(Resource, Deref, DerefMut)]
pub struct ShootTimer(pub Timer);

/// checks if enemy can shoot and shoots if check is true
pub fn on_shoot(
    mut cmds: Commands,
    time: Res<Time>,
    assets: ResMut<ActorTextureHandles>,
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&Transform, &mut AIAttackState), With<Enemy>>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };

    enemy_query.for_each_mut(|(enemy_transform, mut attacking)| {
        let enemy_loc = enemy_transform.translation.truncate();
        let player_loc = player_transform.translation.truncate();
        let direction: Vec2 = (player_loc - enemy_loc).normalize_or_zero();

        // Make sure that the projectiles spawn outside of the body so that it doesn't collide
        let beyond_body_diff: Vec2 = direction * 36.;
        let modified_spawn_location: Vec2 = enemy_loc + beyond_body_diff;

        if attacking.should_shoot && attacking.timer.tick(time.delta()).finished() {
            info!("should shoot");
            create_enemy_projectile(
                &mut cmds,
                assets.bevy_icon.clone(),
                direction,
                modified_spawn_location,
            );
            attacking.timer.reset();
        }
    });
}

/// spawns enemy projectile
pub fn create_enemy_projectile(
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
                    PLAYER_PROJECTILE_LAYER,
                    Group::from_bits_truncate(0b00101),
                ),
                ttl: TimeToLive(Timer::from_seconds(2.0, TimerMode::Repeating)),
            },
            ActiveEvents::COLLISION_EVENTS,
        ));
    });
}
