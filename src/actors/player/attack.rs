use bevy::{math::vec2, prelude::*};
use bevy_mouse_tracking_plugin::{MousePos, MousePosWorld};
use bevy_rapier2d::prelude::{
    ActiveEvents, Collider, ColliderMassProperties, CollisionGroups, Damping, Friction, Group,
    LockedAxes, Restitution, RigidBody, Sensor, Velocity,
};
use leafwing_input_manager::prelude::*;

use crate::{
    action_manager::actions::PlayerBindables,
    components::actors::{
        bundles::{ActorColliderBundle, ProjectileBundle, RigidBodyBundle},
        general::{MovementState, Player, TimeToLive},
    },
    loading::assets::GameTextureHandles,
    utilities::game::{ACTOR_LAYER, ACTOR_PHYSICS_LAYER},
};

pub enum AttackEventType {
    Melee,
    Ranged,
}

pub struct PlayerAttackEvent {
    eventtype: AttackEventType,
}

// this should be an event

pub fn player_attack_sender(
    mut input_query: Query<&ActionState<PlayerBindables>, With<MovementState>>,
    mut attackewriter: EventWriter<PlayerAttackEvent>,
) {
    let action_state = input_query.single_mut();

    if action_state.pressed(PlayerBindables::Shoot) {
        attackewriter.send(PlayerAttackEvent {
            eventtype: AttackEventType::Ranged,
        })
    }
    if action_state.pressed(PlayerBindables::Melee) {
        attackewriter.send(PlayerAttackEvent {
            eventtype: AttackEventType::Melee,
        })
    }
}

pub fn player_melee(
    mouse: Res<MousePos>,
    attackreader: EventReader<PlayerAttackEvent>,
    _player: Query<(&mut Player, &Transform), With<MovementState>>,
) {
    if !attackreader.is_empty() {
        info!("meleeing towards: {:?}", mouse);
    }
}

pub fn player_shoot(
    attackreader: EventReader<PlayerAttackEvent>,
    mouse: Res<MousePosWorld>,
    player: Query<(&mut Player, &mut Transform), With<MovementState>>,
    assets: ResMut<GameTextureHandles>,
    mut cmds: Commands,
) {
    let playerpos = player.single().1.translation.truncate();
    let mousepos = vec2(mouse.x, mouse.y);
    let direction: Vec2 = (mousepos - playerpos).normalize_or_zero();

    let new_transform = (playerpos + (direction * 36.0)).extend(ACTOR_LAYER);
    if !attackreader.is_empty() {
        cmds.spawn((
            ProjectileBundle {
                name: Name::new("PlayerProjectile"),
                sprite_bundle: SpriteBundle {
                    texture: assets.bevy_icon.clone(),
                    transform: Transform::from_translation(new_transform),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(16.0, 16.0)),
                        ..default()
                    },
                    ..default()
                },

                rigidbody_bundle: RigidBodyBundle {
                    velocity: Velocity::linear(direction * 1500.),
                    rigidbody: RigidBody::Dynamic,
                    friction: Friction::coefficient(0.7),
                    howbouncy: Restitution::coefficient(0.3),
                    massprop: ColliderMassProperties::Density(0.3),
                    rotationlocks: LockedAxes::ROTATION_LOCKED,
                    dampingprop: Damping {
                        linear_damping: 1.0,
                        angular_damping: 1.0,
                    },
                },
                ttl: TimeToLive(Timer::from_seconds(5.0, TimerMode::Repeating)),
            },
            Sensor,
        ))
        .with_children(|child| {
            child.spawn((
                ActorColliderBundle {
                    transform_bundle: TransformBundle {
                        local: (Transform {
                            translation: (Vec3 {
                                x: 0.,
                                y: 0.,
                                z: ACTOR_PHYSICS_LAYER,
                            }),
                            ..default()
                        }),
                        ..default()
                    },
                    collider: Collider::ball(4.0),
                },
                TimeToLive(Timer::from_seconds(5.0, TimerMode::Repeating)),
                ActiveEvents::COLLISION_EVENTS,
                Name::new("EnemyProjectileCollider"),
                CollisionGroups::new(Group::GROUP_30, Group::NONE),
            ));
        });
    }
}
