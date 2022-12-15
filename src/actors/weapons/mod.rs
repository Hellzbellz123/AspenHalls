pub mod components;

use std::f32::consts::FRAC_PI_2;

use bevy::{math::vec2, prelude::*};

use bevy_rapier2d::prelude::{
    ActiveEvents, Collider, ColliderMassProperties, CollisionGroups, Damping, Friction, Group,
    LockedAxes, Restitution, RigidBody, Sensor, Velocity,
};
use leafwing_input_manager::prelude::ActionState;

use crate::{
    action_manager::actions::PlayerActions,
    actors::weapons::components::CurrentlyDrawnWeapon,
    components::actors::{
        animation::FacingDirection,
        bundles::{ActorColliderBundle, ProjectileBundle, RigidBodyBundle},
        general::{MovementState, Player, TimeToLive},
    },
    game::{GameStage, TimeInfo},
    loading::assets::ActorTextureHandles,
    utilities::{
        game::{SystemLabels, ACTOR_LAYER, ACTOR_PHYSICS_LAYER},
        EagerMousePos,
    },
};

use self::components::{WeaponSlots, WeaponSocket, WeaponTag};

use super::player::attack::PlayerShootEvent;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameStage::Playing)
                .with_system(rotate_player_weapon)
                .with_system(weapon_visiblity_system)
                .with_system(update_equipped_weapon)
                .with_system(keep_player_weapons_centered)
                .with_system(shoot_weapon)
                .after(SystemLabels::Spawn),
        );
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
    if gametime.game_paused || weapon_query.is_empty() {
        return;
    }
    let gmouse = eager_mouse.world;

    for (wtag, wgtransform, mut wtransform) in weapon_query.iter_mut() {
        if wtag.parent.is_some() {
            let (_playerstate, _) = player_query.single_mut();
            let gmousepos = vec2(gmouse.x, gmouse.y);
            let gweaponpos: Vec2 = wgtransform.compute_transform().translation.truncate();
            let lookdir: Vec2 = (gmousepos - gweaponpos).normalize_or_zero();
            let aimangle = lookdir.y.atan2(lookdir.x) + FRAC_PI_2; // add offset too rotation here

            // mirror whole entity by oppositing the scale when were looking left,
            if aimangle.to_degrees() > 180.0 || aimangle.to_degrees() < -0.0 {
                wtransform.scale.x = -1.0
            } else {
                wtransform.scale.x = 1.0
            }
            *wtransform.rotation = *(Quat::from_euler(EulerRot::ZYX, aimangle, 0.0, 0.0));
        }
    }
}

fn keep_player_weapons_centered(
    mut player_query: Query<(&mut MovementState, With<Player>)>,

    #[allow(clippy::type_complexity)]
    // trunk-ignore(clippy/type_complexity)
    mut weapon_query: Query<
        // this is equivelent to if player has a weapon equipped and out
        (&mut WeaponTag, &mut Transform),
        (With<Parent>, Without<Player>),
    >,
) {
    if weapon_query.is_empty() {
        return;
    }

    for (wtag, mut wtransform) in weapon_query.iter_mut() {
        if wtag.parent.is_some() {
            let (playerstate, _) = player_query.single_mut();
            // modify weapon sprite to be below player when facing up, this still looks strange but looks better than a back mounted smg
            if playerstate.facing == FacingDirection::Up {
                wtransform.translation = Vec3 {
                    x: 0.0,
                    y: 1.5,
                    z: -1.0,
                }
            } else {
                wtransform.translation = Vec3 {
                    x: 0.0,
                    y: 1.5,
                    z: ACTOR_LAYER,
                }
            }
        }
    }
}

// check if the weapon is supposed to be visible
fn weapon_visiblity_system(
    player_query: Query<(&WeaponSocket, &Transform), With<Player>>,
    mut weapon_query: Query<(&WeaponTag, &mut Visibility), With<Parent>>, // query weapons parented to entitys
) {
    let (p_weaponsocket, _ptransform) = player_query.single();
    for (wtag, mut wvisiblity) in weapon_query.iter_mut() {
        if wtag.stored_weapon_slot == Some(p_weaponsocket.drawn_slot) {
            wvisiblity.is_visible = true;
        } else {
            wvisiblity.is_visible = false;
        }
    }
}

fn update_equipped_weapon(
    mut cmds: Commands,
    query_action_state: Query<&ActionState<PlayerActions>>,
    mut player_query: Query<(Entity, &mut WeaponSocket, &mut Transform), With<Player>>,

    drawn_weapon: Query<&CurrentlyDrawnWeapon>,
    #[allow(clippy::type_complexity)]
    // trunk-ignore(clippy/type_complexity)
    weapon_query: Query<
        (Entity, &mut WeaponTag, &mut Transform),
        (With<Parent>, Without<Player>),
    >,
) {
    if player_query.is_empty() | weapon_query.is_empty() | query_action_state.is_empty() {
        return;
    }

    let (_ent, mut wsocket, _transform) = player_query.single_mut();
    let actions = query_action_state.single();

    // TODO: this mostly works, but we need to have a system that checks if the current equippped weapon has a CurrentlyDrawnWeapon and adds it if it doesnt
    // a default is basically what we need, so a weapon is always out if available i guess
    if actions.just_pressed(PlayerActions::EquipSlot1) {
        // set whatever weapon is in slot 1 as CurrentlyDrawnWeapon and remove CurrentlyDrawnWeapon from old weapon
        wsocket.drawn_slot = WeaponSlots::Slot1;
        let current_weapon_slots = &mut wsocket.weapon_slots.clone();
        let newwep = currently_equipped_from_hashmap(current_weapon_slots, &wsocket, &drawn_weapon, &mut cmds);

        if let Some(ent) = newwep {
            cmds.entity(ent).insert(CurrentlyDrawnWeapon);
            info!("equipping slot 1")
        }
    } else if actions.just_pressed(PlayerActions::EquipSlot2) {
        wsocket.drawn_slot = WeaponSlots::Slot2;
        let current_weapon_slots = &mut wsocket.weapon_slots.clone();
        let newwep = currently_equipped_from_hashmap(current_weapon_slots, &wsocket, &drawn_weapon, &mut cmds);

        if let Some(ent) = newwep {
            cmds.entity(ent).insert(CurrentlyDrawnWeapon);
            info!("equipping slot 2")
        }
    } else if actions.just_pressed(PlayerActions::EquipSlot3) {
        wsocket.drawn_slot = WeaponSlots::Slot3;
        let current_weapon_slots = &mut wsocket.weapon_slots.clone();
        let newwep = currently_equipped_from_hashmap(current_weapon_slots, &wsocket, &drawn_weapon, &mut cmds);

        if let Some(ent) = newwep {
            cmds.entity(ent).insert(CurrentlyDrawnWeapon);
            info!("equipping slot 3")
        }
    } else if actions.just_pressed(PlayerActions::EquipSlot4) {
        wsocket.drawn_slot = WeaponSlots::Slot4;
        let current_weapon_slots = &mut wsocket.weapon_slots.clone();
        let newwep = currently_equipped_from_hashmap(current_weapon_slots, &wsocket, &drawn_weapon, &mut cmds);

        if let Some(ent) = newwep {
            cmds.entity(ent).insert(CurrentlyDrawnWeapon);
            info!("equipping slot 4")
        }
    }
}

fn currently_equipped_from_hashmap(
    weaponslots: &mut bevy::utils::hashbrown::HashMap<WeaponSlots, Option<Entity>>,
    wsocket: &Mut<WeaponSocket>,
    drawn_weapon: &Query<&CurrentlyDrawnWeapon>,
    cmds: &mut Commands,
) -> Option<Entity> {
    let entity_in_drawn_slot = weaponslots.entry(wsocket.drawn_slot).or_insert(None);
    let currently_equipped_from_hashmap: Option<Entity> = if let Some(current_equiped_weapon) = entity_in_drawn_slot {
        let result = drawn_weapon.get(*current_equiped_weapon);
        info!("get equiped from hasmap: {:?}", result);
        match result {
            Ok(_a) => {
                info!("huh current equipped weapon match result OK")
            },
            Err(_a) => {
                info!("huh current equipped weapon match result Err, make it ok");
                cmds.entity(*current_equiped_weapon).insert(CurrentlyDrawnWeapon);
            },
        }
        Some(*current_equiped_weapon)
    } else {
        None
    };

    match currently_equipped_from_hashmap {
        Some(weapon) => Some(weapon),
        None => {
            warn!("no currently equipped weapon");
            None
        }
    }
}

pub fn shoot_weapon(
    mut attackreader: EventReader<PlayerShootEvent>,
    mouse: Res<EagerMousePos>,
    player: Query<(&mut Player, &mut Transform), With<MovementState>>,
    assets: ResMut<ActorTextureHandles>,
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
    for _event in attackreader.iter() {
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
