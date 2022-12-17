use bevy::{prelude::*, sprite::Anchor};
use bevy_rapier2d::prelude::*;

use crate::{
    actors::weapons::components::{
        BarrelPointTag, DamageType, WeaponBarrelEndPoint, WeaponBundle, WeaponColliderBundle,
        WeaponColliderTag, WeaponStats, WeaponTag,
    },
    components::actors::{
        bundles::RigidBodyBundle,
        spawners::{SpawnWeaponEvent, WeaponType},
    },
    loading::assets::ActorTextureHandles,
    utilities::game::{ACTOR_PHYSICS_Z_INDEX, ACTOR_SIZE, PLAYER_PROJECTILE_LAYER},
};

//TODO: setup so i can load the guns from a ron file in assets directory. can probably use UntypedCollection
// too allow adding in custom guns.

pub fn spawn_smallsmg(
    game_assets: ActorTextureHandles,
    cmds: &mut Commands,
    event: &SpawnWeaponEvent,
) {
    cmds.spawn((WeaponBundle {
        name: Name::new("Small SMG"),
        tag: WeaponTag {
            parent: None,
            stored_weapon_slot: None,
        },
        weapontype: WeaponType::SmallSMG,
        weaponstats: WeaponStats {
            barreloffset: Vec3 {
                x: 10.0,
                y: 0.0,
                z: 0.0,
            },
            damage: 2.0,
            firing_speed: 0.03,
            bullet_speed: 7.0,
            projectile_size: 2.0,
        },
        damagetype: DamageType::KineticRanged,
        sprite: TextureAtlasSprite {
            flip_x: false,
            flip_y: true,
            anchor: Anchor::Custom(Vec2::new(0.2, 0.2)),
            custom_size: Some(ACTOR_SIZE), //character is 1 tile wide by 2 tiles wide
            ..default()
        },
        texture: game_assets.small_smg,
        spatial: SpatialBundle {
            visibility: Visibility::VISIBLE,
            transform: Transform {
                translation: event.spawn_position,
                rotation: Quat::IDENTITY,
                scale: Vec3::ONE,
            },
            ..default()
        },
        rigidbodybundle: RigidBodyBundle {
            rigidbody: RigidBody::Dynamic,
            velocity: Velocity::default(),
            friction: Friction::coefficient(0.9),
            howbouncy: Restitution::new(20.0),
            massprop: ColliderMassProperties::Density(1.5),
            rotationlocks: LockedAxes::empty(),
            dampingprop: Damping {
                linear_damping: 0.7,
                angular_damping: 0.2,
            },
        },
    },))
        .with_children(|child| {
            child.spawn(WeaponColliderBundle {
                name: Name::new("SMGCollider"),
                collider: Collider::capsule(
                    Vec2 { x: 0.0, y: -20.0 },
                    Vec2 { x: 0.0, y: 20.0 },
                    4.0,
                ),
                cgroups: CollisionGroups::new(Group::ALL, PLAYER_PROJECTILE_LAYER),
                transformbundle: TransformBundle {
                    local: Transform {
                        translation: Vec3 {
                            x: -4.5,
                            y: -5.5,
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
                            x: -5.5,
                            y: -33.0,
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

pub fn spawn_smallpistol(
    game_assets: ActorTextureHandles,
    cmds: &mut Commands,
    event: &SpawnWeaponEvent,
) {
    cmds.spawn((WeaponBundle {
        name: Name::new("Small Pistol"),
        tag: WeaponTag {
            parent: None,
            stored_weapon_slot: None,
        },
        weapontype: WeaponType::SmallPistol,
        weaponstats: WeaponStats {
            barreloffset: Vec3 {
                x: 6.0,
                y: 0.0,
                z: 0.0,
            },
            damage: 22.0,
            firing_speed: 1.2,
            projectile_size: 3.0,
            bullet_speed: 10.0,
        },
        damagetype: DamageType::KineticRanged,
        sprite: TextureAtlasSprite {
            flip_x: true,
            flip_y: true,
            anchor: Anchor::Custom(Vec2::new(0.2, 0.5)),
            custom_size: Some(ACTOR_SIZE), //character is 1 tile wide by 2 tiles wide
            ..default()
        },
        texture: game_assets.small_pistol,
        spatial: SpatialBundle {
            visibility: Visibility::VISIBLE,
            transform: Transform {
                translation: event.spawn_position,
                rotation: Quat::IDENTITY,
                scale: Vec3::ONE,
            },
            ..default()
        },
        rigidbodybundle: RigidBodyBundle {
            rigidbody: RigidBody::Dynamic,
            velocity: Velocity::default(),
            friction: Friction::coefficient(0.9),
            howbouncy: Restitution::new(20.0),
            massprop: ColliderMassProperties::Density(1.1),
            rotationlocks: LockedAxes::empty(),
            dampingprop: Damping {
                linear_damping: 0.4,
                angular_damping: 0.1,
            },
        },
    },))
        .with_children(|child| {
            child.spawn(WeaponColliderBundle {
                name: Name::new("PistolCollider"),
                collider: Collider::capsule(
                    Vec2 { x: -1.8, y: -18.3 },
                    Vec2 { x: -6.0, y: -8.0 },
                    10.0,
                ),
                cgroups: CollisionGroups::new(Group::ALL, PLAYER_PROJECTILE_LAYER),
                transformbundle: TransformBundle {
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
                            y: -27.0,
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
