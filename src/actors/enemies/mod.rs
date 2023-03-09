use crate::components::actors::bundles::{
    EnemyProjectileBundle, EnemyProjectileColliderBundle, EnemyProjectileColliderTag,
    EnemyProjectileTag, RigidBodyBundle,
};
use crate::components::actors::general::{ProjectileStats, TimeToLive};
use crate::game::GameStage;
use crate::utilities::game::{
    ACTOR_PHYSICS_Z_INDEX, BULLET_SPEED_MODIFIER, PLAYER_PROJECTILE_LAYER,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub mod skeleton;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (on_shoot, update_enemy_graphics).in_set(OnUpdate(GameStage::PlaySubStage)),
        );
    }
}

use bevy::{
    prelude::{Entity, Query, ResMut, Vec2, With},
    sprite::TextureAtlasSprite,
};

use crate::{
    components::actors::{ai::AIEnemy, animation::FacingDirection, general::MovementState},
    game::TimeInfo,
};

pub fn update_enemy_graphics(
    timeinfo: ResMut<TimeInfo>,
    mut enemy_query: Query<(
        &mut Velocity,
        &mut MovementState,
        &mut TextureAtlasSprite,
        Entity,
        With<AIEnemy>,
    )>,
) {
    if !timeinfo.game_paused {
        enemy_query.for_each_mut(|(velocity, mut enemystate, mut sprite, _ent, _)| {
            if velocity.linvel == Vec2::ZERO {
                enemystate.facing = FacingDirection::Idle;
            } else if velocity.linvel.x > 5.0 {
                sprite.flip_x = false;
                enemystate.facing = FacingDirection::Right;
            } else if velocity.linvel.x < -5.0 {
                sprite.flip_x = true;
                enemystate.facing = FacingDirection::Left;
            } else if velocity.linvel.y < -5.0 {
                enemystate.facing = FacingDirection::Down;
            } else if velocity.linvel.y > 2.0 {
                enemystate.facing = FacingDirection::Up;
            }
        })
    }
}

use bevy_rapier2d::prelude::{RigidBody, Velocity};

use crate::{
    components::actors::{ai::AIAttackState, general::Player},
    loading::assets::ActorTextureHandles,
};

#[derive(Resource, Deref, DerefMut)]
pub struct ShootTimer(pub Timer);

pub fn on_shoot(
    mut cmds: Commands,
    _time: Res<Time>,
    assets: ResMut<ActorTextureHandles>,
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&Transform, &mut AIAttackState), With<AIEnemy>>,
) {
    let Ok(player_transform) = player_query.get_single() else {return;};

    enemy_query.for_each_mut(|(enemytransform, mut attacking)| {
        let direction: Vec3 =
            (player_transform.translation - enemytransform.translation).normalize_or_zero();

        // Make sure that the projectiles spawn outside of the body so that it doesn't collide
        let beyond_body_diff = direction * 36.;
        let mut new_transform = *enemytransform;
        new_transform.translation = enemytransform.translation + beyond_body_diff;

        if attacking.should_shoot && attacking.timer.tick(_time.delta()).finished() {
            info!("should shoot");
            create_enemy_projectile(
                &mut cmds,
                assets.bevy_icon.clone(),
                direction,
                new_transform,
            );
            attacking.timer.reset();
        }
    });
}

pub fn create_enemy_projectile(
    cmds: &mut Commands,
    projtexture: Handle<Image>,
    direction: Vec3,
    location: Transform,
) {
    cmds.spawn((
        EnemyProjectileBundle {
            name: Name::new("EnemyProjectile"),
            tag: EnemyProjectileTag,
            projectile_stats: ProjectileStats {
                damage: 10.0,
                speed: 100.0,
                size: 5.0,
            },
            ttl: TimeToLive(Timer::from_seconds(2.0, TimerMode::Repeating)),
            sprite_bundle: SpriteBundle {
                texture: projtexture,
                transform: location,
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(5.0)),
                    ..default()
                },
                ..default()
            },
            rigidbody_bundle: RigidBodyBundle {
                velocity: Velocity::linear(direction.truncate() * (BULLET_SPEED_MODIFIER * 5.0)),
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
            EnemyProjectileColliderBundle {
                name: Name::new("EnemyProjectileCollider"),
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
                tag: EnemyProjectileColliderTag,
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
