pub mod components;

use std::f32::consts::FRAC_PI_2;

use bevy::{math::vec2, prelude::*, sprite::Anchor};

use bevy_rapier2d::prelude::{
    ActiveEvents, Collider, ColliderMassProperties, CollisionGroups, Damping, Friction, Group,
    LockedAxes, Restitution, RigidBody, Sensor, Velocity,
};
use leafwing_input_manager::prelude::ActionState;

use crate::{
    action_manager::actions::PlayerBindables,
    actors::weapons::components::{
        CurrentlyDrawnWeapon, DamageType, WeaponBundle, WeaponPhysicsBundle, WeaponStats,
    },
    components::actors::{
        animation::FacingDirection,
        bundles::{ActorColliderBundle, ProjectileBundle, RigidBodyBundle},
        general::{MovementState, Player, TimeToLive},
    },
    game::{GameStage, TimeInfo},
    loading::assets::GameTextureHandles,
    utilities::{
        game::{SystemLabels, ACTOR_LAYER, ACTOR_PHYSICS_LAYER, ACTOR_SIZE},
        EagerMousePos,
    },
};

use self::components::{WeaponSocket, WeaponTag};

use super::player::attack::{AttackEventType, PlayerShootEvent};

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameStage::Playing)
                .with_system(manage_weapon_sockets)
                .with_system(rotate_player_weapon)
                .with_system(shoot_weapon)
                .with_system(spawn_weapon)
                .after(SystemLabels::Spawn),
        );
    }
}

fn manage_weapon_sockets(
    mut cmds: Commands,
    mut player_query: Query<(Entity, &mut WeaponSocket, &mut Transform), With<Player>>,
    #[allow(clippy::type_complexity)]
    // trunk-ignore(clippy/type_complexity)
    mut weapon_query: Query<(Entity, &mut WeaponTag, &mut Transform), Without<Player>>,
) {
    if !player_query.is_empty() {
        let (playerentity, mut weaponsocket_on_player, ptransform) = player_query.single_mut();
        if weaponsocket_on_player.attached_weapon.is_none() {
            for (weapon, mut weapontag, mut wtransform) in weapon_query.iter_mut() {
                let distance = (ptransform.translation - wtransform.translation)
                    .length()
                    .abs();
                if distance < 50.0 {
                    info!("parenting weapon: {:?} to player", weapon);
                    cmds.entity(playerentity).push_children(&[weapon]);
                    weapontag.parent = Some(playerentity);
                    weaponsocket_on_player.attached_weapon = Some(weapon);
                    wtransform.translation = Vec3::ZERO
                        + Vec3 {
                            x: 0.0,
                            y: 1.5,
                            z: 1.0,
                        };
                    cmds.entity(weapon)
                        .insert(CurrentlyDrawnWeapon)
                        .despawn_descendants()
                    // cmds.entity(weapon).remove_children();
                } else {
                    info!("no weapon in range");
                };
            }
        }
    }
}

fn rotate_player_weapon(
    gametime: Res<TimeInfo>,
    eager_mouse: Res<EagerMousePos>,
    mut player_query: Query<(&mut MovementState, With<Player>)>,

    #[allow(clippy::type_complexity)]
    // trunk-ignore(clippy/type_complexity)
    mut weapon_query: Query<
        // this is equivelent to if player has a weapon equipped and out
        (&mut WeaponTag, &GlobalTransform, &mut Transform),
        (With<Parent>, With<CurrentlyDrawnWeapon>, Without<Player>),
    >,
) {
    let gmouse = eager_mouse.world;
    if !gametime.game_paused && !weapon_query.is_empty() {
        for (wtag, wgtransform, mut wtransform) in weapon_query.iter_mut() {
            if wtag.parent.is_some() {
                let (playerstate, _) = player_query.single_mut();
                let gmousepos = vec2(gmouse.x, gmouse.y);
                let gweaponpos: Vec2 = wgtransform.compute_transform().translation.truncate();
                let lookdir: Vec2 = (gmousepos - gweaponpos).normalize_or_zero();
                let aimangle = lookdir.y.atan2(lookdir.x) + FRAC_PI_2; // add offset too rotation here

                if aimangle.to_degrees() > 180.0 || aimangle.to_degrees() < -0.0 {
                    info!("weapon is upside down");
                    wtransform.scale.x = -1.0
                } else {
                    wtransform.scale.x = 1.0
                }

                info!("{}", aimangle.to_degrees());

                // modify weapon sprite to be below player when facing up, this still looks strange but looks better than a back mounted smg
                if playerstate.facing == FacingDirection::Up {
                    wtransform.translation.z = -1.0
                } else {
                    wtransform.translation.z = ACTOR_LAYER;
                }

                *wtransform.rotation = *(Quat::from_euler(EulerRot::ZYX, aimangle, 0.0, 0.0));
            }
        }
    }
}

pub fn spawn_weapon(
    game_assets: Res<GameTextureHandles>,
    query_action_state: Query<&ActionState<PlayerBindables>>,
    mut cmds: Commands,
) {
    if !query_action_state.is_empty() {
        let actions = query_action_state.get_single().expect("no ents?");

        if actions.just_released(PlayerBindables::DebugF2) {
            debug!("debug f2 action requested: spawn smg");

            cmds.spawn((WeaponBundle {
                name: Name::new("SmallSMG"),
                tag: WeaponTag {
                    parent: None,
                    stored_weapon_slot: None,
                },
                weaponstats: WeaponStats {
                    damage: 2.0,
                    speed: 0.2,
                },
                damagetype: DamageType::KineticRanged,
                sprite: TextureAtlasSprite {
                    flip_x: false,
                    flip_y: true,
                    anchor: Anchor::Custom(Vec2::new(0.2, 0.2)),
                    custom_size: Some(ACTOR_SIZE), //character is 1 tile wide by 2 tiles wide
                    ..default()
                },
                texture: game_assets.small_smg.clone(),
                spatial: SpatialBundle {
                    visibility: Visibility::VISIBLE,
                    transform: Transform {
                        translation: Vec3 {
                            x: -60.0,
                            y: 1090.0,
                            z: ACTOR_LAYER,
                        },
                        rotation: Quat::IDENTITY,
                        scale: Vec3::ONE,
                    },
                    ..default()
                },
            },))
                .with_children(|child| {
                    child.spawn(WeaponPhysicsBundle {
                        name: Name::new("SMGCollider"),
                        collider: Collider::capsule(
                            Vec2 { x: 0.0, y: -20.0 },
                            Vec2 { x: 0.0, y: 20.0 },
                            4.0,
                        ),
                        cgroups: CollisionGroups::new(Group::NONE, Group::GROUP_30),
                        rigidbodybundle: RigidBodyBundle {
                            rigidbody: RigidBody::Dynamic,
                            velocity: Velocity::default(),
                            friction: Friction::coefficient(0.7),
                            howbouncy: Restitution::coefficient(0.3),
                            massprop: ColliderMassProperties::Density(0.3),
                            rotationlocks: LockedAxes::empty(),
                            dampingprop: Damping {
                                linear_damping: 1.0,
                                angular_damping: 1.0,
                            },
                        },
                        transformbundle: TransformBundle {
                            local: Transform {
                                translation: Vec3 {
                                    x: 0.0,
                                    y: 10.0,
                                    z: 1.0,
                                },
                                rotation: Quat::IDENTITY,
                                scale: Vec3::ONE,
                            },
                            global: GlobalTransform::IDENTITY,
                        },
                    });
                });
        };
    }
}

pub fn shoot_weapon(
    mut attackreader: EventReader<PlayerShootEvent>,
    mouse: Res<EagerMousePos>,
    player: Query<(&mut Player, &mut Transform), With<MovementState>>,
    assets: ResMut<GameTextureHandles>,
    mut cmds: Commands,
) {
    let playerpos = player.single().1.translation.truncate();
    let mouse = mouse.world;
    let mousepos = vec2(mouse.x, mouse.y);
    let direction: Vec2 = (mousepos - playerpos).normalize_or_zero();

    let new_transform = (playerpos + (direction * 36.0)).extend(ACTOR_LAYER);
    if attackreader.is_empty() {
        return;
    }
    for event in attackreader.iter() {
        match event.eventtype {
            AttackEventType::Melee => {
                return;
            }
            AttackEventType::Ranged => {
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
                            name: Name::new("PlayerProjectileCollider"),
                            transformbundle: TransformBundle {
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
                        CollisionGroups::new(Group::GROUP_30, Group::NONE),
                    ));
                });
            }
        }
    }
}

// check if if the weapon is supposed to be visible
fn weapon_visiblity_system(
    _cmds: Commands,
    mut player_query: Query<(Entity, &mut WeaponSocket, &mut Transform), With<Player>>,
    #[allow(clippy::type_complexity)]
    // trunk-ignore(clippy/type_complexity)
    mut weapon_query: Query<
        (Entity, &mut WeaponTag, &mut Transform, &mut Visibility),
        (With<Parent>, Without<Player>),
    >, // query weapons
) {
    let (_pent, pweaponsocket, _ptransform) = player_query.single_mut();
    for (_wentity, wtag, _wtransform, mut wvisiblity) in weapon_query.iter_mut() {
        if wtag.stored_weapon_slot == pweaponsocket.currently_equipped {
            wvisiblity.is_visible = true;
        } else {
            wvisiblity.is_visible = false;
        }
    }
}

fn update_equipped_weapon(
    _cmds: Commands,
    _player_query: Query<(Entity, &mut WeaponSocket, &mut Transform), With<Player>>,
    #[allow(clippy::type_complexity)]
    // trunk-ignore(clippy/type_complexity)
    _weapon_query: Query<
        (Entity, &mut WeaponTag, &mut Transform),
        (Without<Parent>, Without<Player>),
    >,
) {
}
