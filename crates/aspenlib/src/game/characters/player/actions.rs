use bevy::prelude::*;
use bevy_rapier2d::geometry::{Collider, CollisionGroups};
use leafwing_input_manager::prelude::ActionState;

use crate::{
    bundles::ItemColliderBundle,
    consts::TILE_SIZE,
    game::{
        characters::{player::PlayerSelectedHero, EventSpawnCharacter, components::WeaponSlot},
        combat::EventRequestAttack,
        components::ActorColliderType,
        input::action_maps,
        items::weapons::components::{
            CurrentlyDrawnWeapon, WeaponCarrier, WeaponHolder,
        },
    },
    loading::{config::GeneralSettings, registry::RegistryIdentifier},
};

/// zoom control
pub fn zoom_control(
    mut settings: ResMut<GeneralSettings>,
    actions: Res<ActionState<action_maps::Gameplay>>,
) {
    let multiplier = if actions.pressed(&action_maps::Gameplay::Sprint) {
        10.0
    } else {
        1.0
    };

    if actions.pressed(&action_maps::Gameplay::ZoomIn) {
        settings.camera_zoom += 0.05 * multiplier;
    } else if actions.pressed(&action_maps::Gameplay::ZoomOut) {
        settings.camera_zoom -= 0.05 * multiplier;
    }
}

/// spawns skeleton near player if `Gameplay::DebugF1` is pressed
pub fn spawn_custom(
    mut spawn_event_writer: EventWriter<EventSpawnCharacter>,
    player_query: Query<(Entity, &Transform), With<PlayerSelectedHero>>,
    actions: Res<ActionState<action_maps::Gameplay>>,
) {
    if actions.just_released(&action_maps::Gameplay::DebugF1) {
        let Ok((player, _)) = player_query.get_single() else {
            warn!("no player too spawn custom near");
            return;
        };

        debug!("pressed spawn_skeleton_button: Spawning Skeleton near player");
        spawn_event_writer.send(EventSpawnCharacter {
            requester: player,
            spawn_data: (RegistryIdentifier("skeleton".to_owned()), 1),
        });
    };
}

/// send shoot request to gun control system.
#[allow(clippy::type_complexity)]
pub fn player_attack(
    weapon_query: Query<Entity, (With<Parent>, With<CurrentlyDrawnWeapon>)>,
    player_query: Query<Entity, With<PlayerSelectedHero>>,
    actions: Res<ActionState<action_maps::Gameplay>>,
    mut shoot_event_writer: EventWriter<EventRequestAttack>,
) {
    let weapon_entity = weapon_query.iter().next();
    let player = player_query.single();

    if actions.pressed(&action_maps::Gameplay::Attack) {
        shoot_event_writer.send(EventRequestAttack {
            requester: player,
            weapon: weapon_entity,
        });
    }
}

/// rotates weapon too face wherever the players mouse is
#[allow(clippy::type_complexity)]
pub fn aim_weapon(
    player_query: Query<Entity, With<PlayerSelectedHero>>,
    mut weapon_query: Query<
        // this is equivalent to if player has a weapon equipped and out
        (&WeaponHolder, &GlobalTransform, &mut Transform),
        (With<Parent>, With<CurrentlyDrawnWeapon>),
    >,
    actions: Res<ActionState<action_maps::Gameplay>>,
) {
    let Ok(player) = player_query.get_single() else {
        return;
    };

    for (weapon_holder, weapon_global_transform, mut weapon_transform) in &mut weapon_query {
        if weapon_holder.is_some_and(|f| f.1 == player) {
            let global_mouse_pos = actions
                .action_data(&action_maps::Gameplay::CursorWorld).expect("always exists")
                .axis_pair
                .unwrap()
                .xy();
            let global_weapon_pos: Vec2 = weapon_global_transform.translation().truncate();
            let look_direction: Vec2 = (global_weapon_pos - global_mouse_pos).normalize_or_zero();
            let aim_angle = (-look_direction.y).atan2(-look_direction.x);

            weapon_transform.rotation = Quat::from_euler(EulerRot::ZYX, aim_angle, 0.0, 0.0);
        }
    }
}

// TODO: move too player actions file
/// updates players equipped weapon based on input
#[allow(clippy::type_complexity)]
pub fn change_weapon(
    actions: Res<ActionState<action_maps::Gameplay>>,
    mut player_query: Query<&mut WeaponCarrier>,
) {
    let mut player_weapon_socket = player_query.single_mut();

    if actions.pressed(&action_maps::Gameplay::CycleWeapon)
        && actions
            .current_duration(&action_maps::Gameplay::CycleWeapon)
            .as_secs()
            == 1
    {
        info!("hiding weapons");
        player_weapon_socket.drawn_slot = None;
    }

    let player_slots = &player_weapon_socket.weapon_slots;
    let drawn_slot = player_weapon_socket.drawn_slot.unwrap_or(WeaponSlot::Slot4);

    if actions.just_pressed(&action_maps::Gameplay::CycleWeapon) {
        info!("selecting next weapon slot");
        match drawn_slot {
            WeaponSlot::Slot1 => {
                player_weapon_socket.drawn_slot =
                    if player_slots.get(&WeaponSlot::Slot2).unwrap().is_some() {
                        Some(WeaponSlot::Slot2)
                    } else if player_slots.get(&WeaponSlot::Slot3).unwrap().is_some() {
                        Some(WeaponSlot::Slot3)
                    } else if player_slots.get(&WeaponSlot::Slot4).unwrap().is_some() {
                        Some(WeaponSlot::Slot4)
                    } else {
                        Some(WeaponSlot::Slot1)
                    }
            }
            WeaponSlot::Slot2 => {
                player_weapon_socket.drawn_slot = {
                    if player_slots.get(&WeaponSlot::Slot3).unwrap().is_some() {
                        Some(WeaponSlot::Slot3)
                    } else if player_slots.get(&WeaponSlot::Slot4).unwrap().is_some() {
                        Some(WeaponSlot::Slot4)
                    } else if player_slots.get(&WeaponSlot::Slot1).unwrap().is_some() {
                        Some(WeaponSlot::Slot1)
                    } else {
                        Some(WeaponSlot::Slot2)
                    }
                }
            }
            WeaponSlot::Slot3 => {
                player_weapon_socket.drawn_slot = {
                    if player_slots.get(&WeaponSlot::Slot4).unwrap().is_some() {
                        Some(WeaponSlot::Slot4)
                    } else if player_slots.get(&WeaponSlot::Slot1).unwrap().is_some() {
                        Some(WeaponSlot::Slot1)
                    } else if player_slots.get(&WeaponSlot::Slot2).unwrap().is_some() {
                        Some(WeaponSlot::Slot2)
                    } else {
                        Some(WeaponSlot::Slot3)
                    }
                }
            }
            WeaponSlot::Slot4 => {
                player_weapon_socket.drawn_slot = {
                    if player_slots.get(&WeaponSlot::Slot1).unwrap().is_some() {
                        Some(WeaponSlot::Slot1)
                    } else if player_slots.get(&WeaponSlot::Slot2).unwrap().is_some() {
                        Some(WeaponSlot::Slot2)
                    } else if player_slots.get(&WeaponSlot::Slot3).unwrap().is_some() {
                        Some(WeaponSlot::Slot3)
                    } else {
                        Some(WeaponSlot::Slot4)
                    }
                }
            }
        }
    }
}

// TODO:
// TODO:
// make picking up thing an event
// depending on event send new event
/// equips closest weapon too player if `WeaponSlots` is not full
#[allow(clippy::type_complexity)]
pub fn equip_closest_weapon(
    mut cmds: Commands,
    actions: Res<ActionState<action_maps::Gameplay>>,
    mut player_query: Query<(Entity, &mut WeaponCarrier, &mut Transform), With<PlayerSelectedHero>>,
    query_child_weapon_collider: Query<(Entity, &Parent), With<ActorColliderType>>,
    mut weapon_query: Query<
        (Entity, &mut WeaponHolder, &mut Transform),
        (Without<Parent>, Without<WeaponCarrier>),
    >,
) {
    let (player_entity, mut weapon_socket_on_player, p_transform) = player_query.single_mut();

    if !actions.just_pressed(&action_maps::Gameplay::Interact) {
        // TODO: equip multiple weapons by replacing currently equipped weapon with new one
        // if interact isn't pressed or weapon_socket_on_player.weapon_slots is "full" we can early exit the fn
        return;
    }
    info!("interact pressed");

    let weapon_slots = weapon_socket_on_player.weapon_slots.clone();
    let drawn_weapon = weapon_socket_on_player.drawn_slot;

    let slots_full = weapon_slots.values().all(std::option::Option::is_some);

    let Some((closest_weapon, mut weapon_holder, mut weapon_pos)) = weapon_query
        .iter_mut()
        .filter(|f| f.2.translation.distance(p_transform.translation) < TILE_SIZE * 2.0)
        .min_by(|lhs, rhs| {
            let da = (lhs.2.translation.truncate() - p_transform.translation.truncate())
                .length_squared();
            let db = (rhs.2.translation.truncate() - p_transform.translation.truncate())
                .length_squared();
            da.partial_cmp(&db).unwrap()
        })
    else {
        warn!("No weapons too equip");
        return;
    };

    let (too_update_slot, slot_value) = weapon_slots
        .iter()
        .find(|(_slot, value)| value.is_none())
        .unwrap_or_else(|| {
            weapon_slots
                .get_key_value(&drawn_weapon.unwrap_or(WeaponSlot::Slot1))
                .expect("weapon slot did not exist on player")
        });

    // equipping weapon too player
    if slots_full {
        if drawn_weapon.is_some() {
            warn!("slots full, unequipping drawn weapon");
            // TODO: recreate weapon collider properly?
            let weapon_ent = slot_value.unwrap();
            weapon_pos.translation = Vec3 {
                x: 50.0,
                y: 0.0,
                z: 0.0,
            };
            cmds.entity(weapon_ent).remove_parent();
            cmds.entity(weapon_ent).with_children(|f| {
                f.spawn(ItemColliderBundle {
                    name: Name::new("DroppedWeaponCollider"),
                    tag: ActorColliderType::Item,
                    collider: Collider::default(),
                    collision_groups: CollisionGroups::default(),
                    transform_bundle: TransformBundle::default(),
                });
            });
        } else {
            warn!("no weapon selected too replace");
        }
    }
    debug!(
        "the weapon slot is empty, parenting weapon: {:?} too player: {:?}",
        closest_weapon, player_entity
    );
    cmds.entity(player_entity).push_children(&[closest_weapon]);

    for (ent, parent) in query_child_weapon_collider.iter() {
        if parent.get() == closest_weapon {
            info!("despawning collider for {:?}", parent.get());
            cmds.entity(ent).despawn();
        }
    }

    // set equipped weapon too drawn weapon
    weapon_socket_on_player.drawn_slot = Some(*too_update_slot);
    *weapon_holder = WeaponHolder(Some((*too_update_slot, player_entity)));

    let socket_value = weapon_socket_on_player
        .weapon_slots
        .entry(*too_update_slot)
        .or_insert(None);
    *socket_value = Some(closest_weapon);

    weapon_pos.translation = Vec3::ZERO;
}
