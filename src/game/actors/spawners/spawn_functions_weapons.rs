use bevy::{prelude::*, sprite::Anchor};
use bevy_rapier2d::prelude::*;

use crate::{
    bundles::RigidBodyBundle,
    consts::{AspenCollisionLayer, ACTOR_PHYSICS_Z_INDEX, ACTOR_SIZE, ACTOR_Z_INDEX},
    game::actors::combat::components::{
        BarrelPointTag, DamageType, WeaponBarrelEndPoint, WeaponBundle, WeaponColliderBundle,
        WeaponColliderTag, WeaponStats, WeaponTag,
    },
    loading::assets::ActorTextureHandles,
};

use super::components::{SpawnActorEvent, WeaponType};

//TODO: setup so i can load the guns from a ron file in assets directory. can probably use UntypedCollection
// too allow adding in custom guns.

/// spawns small smg, called from spawner
pub fn spawn_small_smg(
    game_assets: ActorTextureHandles,
    cmds: &mut Commands,
    event: &SpawnActorEvent,
) {
    cmds.spawn((WeaponBundle {
        name: Name::new("Small SMG"),
        tag: WeaponTag {
            parent: None,
            stored_weapon_slot: None,
        },
        weapon_type: WeaponType::SmallSMG,
        weapon_stats: WeaponStats {
            barrel_offset: Vec3 {
                x: 10.0,
                y: 0.0,
                z: 0.0,
            },
            damage: 20.0,
            attack_speed: 0.03,
            bullet_speed: 6.0,
            projectile_size: 2.0,
        },
        damage_type: DamageType::KineticRanged,
        sprite: SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                flip_x: false,
                flip_y: true,
                anchor: Anchor::Custom(Vec2::new(0.2, 0.2)),
                custom_size: Some(ACTOR_SIZE), //character is 1 tile wide by 2 tiles wide
                ..default()
            },
            texture_atlas: game_assets.small_smg,
            transform: Transform {
                translation: event.spawn_position.extend(ACTOR_Z_INDEX),
                rotation: Quat::IDENTITY,
                scale: Vec3::ONE,
            },
            ..default()
        },
        rigidbody_bundle: RigidBodyBundle {
            rigidbody: RigidBody::Dynamic,
            velocity: Velocity::default(),
            friction: Friction::coefficient(0.9),
            how_bouncy: Restitution::new(20.0),
            mass_prop: ColliderMassProperties::Density(1.5),
            rotation_locks: LockedAxes::empty(),
            damping_prop: Damping {
                linear_damping: 0.7,
                angular_damping: 0.2,
            },
        },
    },))
        .with_children(|child| {
            child.spawn(WeaponColliderBundle {
                name: Name::new("SMGCollider"),
                collider: Collider::capsule(
                    Vec2 { x: 0.0, y: -10.0 },
                    Vec2 { x: 0.0, y: 10.0 },
                    2.0,
                ),
                collision_groups: CollisionGroups::new(
                    AspenCollisionLayer::ACTOR,
                    AspenCollisionLayer::EVERYTHING,
                ),
                transform_bundle: TransformBundle {
                    local: Transform {
                        translation: Vec3 {
                            x: -2.25,
                            y: -2.525,
                            z: ACTOR_PHYSICS_Z_INDEX,
                        },
                        rotation: Quat::IDENTITY,
                        scale: Vec3::ONE,
                    },
                    global: GlobalTransform::IDENTITY,
                },
                tag: WeaponColliderTag,
            });
            child.spawn(WeaponBarrelEndPoint {
                name: "SMGBarrelEndPoint".into(),
                sprite: SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(1.0, 0.25, 0.35),
                        custom_size: Some(Vec2::new(1.0, 1.0)),
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3 {
                            x: -2.525,
                            y: -16.5,
                            z: 1.0,
                        },
                        ..default()
                    },
                    ..default()
                },
                tag: BarrelPointTag,
            });
        });
}

/// spawns small pistol, called from spawner
pub fn spawn_small_pistol(
    game_assets: ActorTextureHandles,
    cmds: &mut Commands,
    event: &SpawnActorEvent,
) {
    cmds.spawn((WeaponBundle {
        name: Name::new("Small Pistol"),
        tag: WeaponTag {
            parent: None,
            stored_weapon_slot: None,
        },
        weapon_type: WeaponType::SmallPistol,
        weapon_stats: WeaponStats {
            barrel_offset: Vec3 {
                x: 6.0,
                y: 0.0,
                z: 0.0,
            },
            damage: 100.0,
            attack_speed: 1.1,
            projectile_size: 3.0,
            bullet_speed: 8.0,
        },
        damage_type: DamageType::KineticRanged,
        sprite: SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                flip_x: true,
                flip_y: true,
                anchor: Anchor::Custom(Vec2::new(0.2, 0.5)),
                custom_size: Some(ACTOR_SIZE), //character is 1 tile wide by 2 tiles wide
                ..default()
            },
            texture_atlas: game_assets.small_pistol,
            transform: Transform {
                translation: event.spawn_position.extend(ACTOR_Z_INDEX),
                rotation: Quat::IDENTITY,
                scale: Vec3::ONE,
            },
            ..default()
        },
        rigidbody_bundle: RigidBodyBundle {
            rigidbody: RigidBody::Dynamic,
            velocity: Velocity::default(),
            friction: Friction::coefficient(0.9),
            how_bouncy: Restitution::new(20.0),
            mass_prop: ColliderMassProperties::Density(1.1),
            rotation_locks: LockedAxes::empty(),
            damping_prop: Damping {
                linear_damping: 0.4,
                angular_damping: 0.1,
            },
        },
    },))
        .with_children(|child| {
            child.spawn(WeaponColliderBundle {
                name: Name::new("PistolCollider"),
                collider: Collider::capsule(
                    Vec2 { x: -0.9, y: -9.15 },
                    Vec2 { x: -3.0, y: -4.0 },
                    5.0,
                ),
                collision_groups: CollisionGroups::new(
                    AspenCollisionLayer::ACTOR,
                    AspenCollisionLayer::EVERYTHING,
                ),
                transform_bundle: TransformBundle {
                    local: Transform {
                        translation: Vec3 {
                            x: 0.0,
                            y: 0.0,
                            z: ACTOR_PHYSICS_Z_INDEX,
                        },
                        rotation: Quat::IDENTITY,
                        scale: Vec3::ONE,
                    },
                    global: GlobalTransform::IDENTITY,
                },
                tag: WeaponColliderTag,
            });
            child.spawn(WeaponBarrelEndPoint {
                name: "PistolBarrelEndPoint".into(),
                sprite: SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(1.0, 0.25, 0.35),
                        custom_size: Some(Vec2::new(1.0, 1.0)),
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3 {
                            x: -1.0,
                            y: -13.5,
                            z: 1.0,
                        },
                        ..default()
                    },
                    ..default()
                },
                tag: BarrelPointTag,
            });
        });
}
