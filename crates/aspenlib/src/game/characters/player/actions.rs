use avian2d::prelude::CollisionLayers;
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::{
    bundles::{AspenColliderBundle, NeedsCollider},
    consts::{ACTOR_PHYSICS_Z_INDEX, ACTOR_Z_INDEX, TILE_SIZE},
    game::{
        characters::{components::WeaponSlot, player::PlayerSelectedHero, EventSpawnCharacter},
        combat::{AttackDirection, EventRequestAttack},
        components::ActorColliderType,
        input::{action_maps, AspenCursorPosition},
        items::weapons::components::{CurrentlyDrawnWeapon, WeaponCarrier, WeaponHolder},
    },
    loading::{config::GeneralSettings, registry::RegistryIdentifier},
};

/// zoom control
pub fn zoom_control(
    mut settings: ResMut<GeneralSettings>,
    actions: Res<ActionState<action_maps::Gameplay>>,
) {
    let base_zoom_speed = if actions.pressed(&action_maps::Gameplay::Sprint) {
        10.0
    } else {
        1.0
    };

    let final_speed = if settings.camera_zoom < 0.75 {
        base_zoom_speed / 10.0
    } else {
        base_zoom_speed
    };

    if actions.pressed(&action_maps::Gameplay::ZoomSubtract) {
        if settings.camera_zoom <= 0.050 {
            return;
        }

        settings.camera_zoom -= 0.05 * final_speed;
    } else if actions.pressed(&action_maps::Gameplay::ZoomAdd) {
        settings.camera_zoom += 0.05 * final_speed;
    }
}

/// spawns skeleton near player if `Gameplay::DebugF1` is pressed
pub fn spawn_custom(
    mut spawn_event_writer: EventWriter<EventSpawnCharacter>,
    player_query: Query<(Entity, &Transform), With<PlayerSelectedHero>>,
    actions: Res<ActionState<action_maps::Gameplay>>,
) {
    if actions.just_released(&action_maps::Gameplay::DebugF1) {
        let Ok((player, _)) = player_query.single() else {
            warn!("no player too spawn custom near");
            return;
        };

        debug!("pressed spawn_skeleton_button: Spawning Skeleton near player");
        spawn_event_writer.write(EventSpawnCharacter {
            requester: player,
            identifier: RegistryIdentifier("skeleton".to_owned()),
        });
    };
}

/// send attack request too combat systems.
#[allow(clippy::type_complexity)]
pub fn player_attack(
    weapon_query: Query<Entity, (With<ChildOf>, With<CurrentlyDrawnWeapon>)>,
    player_query: Query<Entity, With<PlayerSelectedHero>>,
    actions: Res<ActionState<action_maps::Gameplay>>,
    mut attack_event_writer: EventWriter<EventRequestAttack>,
) {
    let weapon_entity = weapon_query.iter().next();
    let Ok(player) = player_query.single() else {
        return
    };

    if actions.pressed(&action_maps::Gameplay::Attack) {
        match weapon_entity {
            Some(weapon) => {
                attack_event_writer.write(EventRequestAttack {
                    requester: player,
                    direction: AttackDirection::FromWeapon(weapon),
                });
            }
            None => {
                // TODO: calculate direction from virtual cursor position
                attack_event_writer.write(EventRequestAttack {
                    requester: player,
                    direction: AttackDirection::FromVector(Vec2 { x: 0.0, y: 1.0 }),
                });
            }
        }
    }
}

/// rotates weapon too face wherever the players mouse is
#[allow(clippy::type_complexity)]
pub fn aim_weapon(
    player_query: Query<Entity, With<PlayerSelectedHero>>,
    mut weapon_query: Query<
        // this is equivalent to if player has a weapon equipped and out
        (&WeaponHolder, &GlobalTransform, &mut Transform),
        (With<ChildOf>, With<CurrentlyDrawnWeapon>),
    >,
    cursor_positon: Res<AspenCursorPosition>,
) {
    let Ok(player) = player_query.single() else {
        return;
    };

    for (weapon_holder, weapon_global_transform, mut weapon_transform) in &mut weapon_query {
        if weapon_holder.is_some_and(|f| f.1 == player) {
            let look_direction = (cursor_positon.world
                - weapon_global_transform.translation().truncate())
            .normalize_or_zero();
            let aim_angle = (look_direction.y).atan2(look_direction.x);

            weapon_transform.rotation = Quat::from_euler(EulerRot::ZYX, aim_angle, 0.0, 0.0);
        }
    }
}

// TODO: move too player actions file
/// updates players equipped weapon based on input
#[allow(clippy::type_complexity)]
pub fn change_weapon(
    actions: Res<ActionState<action_maps::Gameplay>>,
    mut player_query: Query<&mut WeaponCarrier, With<PlayerSelectedHero>>,
) {
    let Ok(mut player_weapon_socket) = player_query.single_mut() else {
        return
    };

    let duration = actions
        .previous_duration(&action_maps::Gameplay::CycleWeapon)
        .as_secs_f32();

    if actions.just_released(&action_maps::Gameplay::CycleWeapon) {
        info!(
            "current duration: {}",
            actions
                .current_duration(&action_maps::Gameplay::CycleWeapon)
                .as_secs_f32()
        );
        info!(
            "previous duration: {}",
            actions
                .previous_duration(&action_maps::Gameplay::CycleWeapon)
                .as_secs_f32()
        );

        if duration >= 1.0 {
            info!("hiding weapons");
            player_weapon_socket.drawn_slot = None;
            return;
        }

        info!("Selecting next weapon slot");

        let player_slots = &player_weapon_socket.weapon_slots;
        let drawn_slot = player_weapon_socket.drawn_slot.unwrap_or(WeaponSlot::Slot4);

        let next_slot = match drawn_slot {
            WeaponSlot::Slot1 => vec![
                WeaponSlot::Slot2,
                WeaponSlot::Slot3,
                WeaponSlot::Slot4,
                WeaponSlot::Slot1,
            ],
            WeaponSlot::Slot2 => vec![
                WeaponSlot::Slot3,
                WeaponSlot::Slot4,
                WeaponSlot::Slot1,
                WeaponSlot::Slot2,
            ],
            WeaponSlot::Slot3 => vec![
                WeaponSlot::Slot4,
                WeaponSlot::Slot1,
                WeaponSlot::Slot2,
                WeaponSlot::Slot3,
            ],
            WeaponSlot::Slot4 => vec![
                WeaponSlot::Slot1,
                WeaponSlot::Slot2,
                WeaponSlot::Slot3,
                WeaponSlot::Slot4,
            ],
        };

        player_weapon_socket.drawn_slot = next_slot
            .into_iter()
            .find(|slot| player_slots.get(slot).unwrap().is_some());
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
    query_child_weapon_collider: Query<(Entity, &ChildOf), With<ActorColliderType>>,
    mut weapon_query: Query<
        (Entity, &mut WeaponHolder, &mut Transform),
        (Without<ChildOf>, Without<WeaponCarrier>),
    >,
) {
    let Ok((player_entity, mut weapon_socket_on_player, p_transform)) = player_query.single_mut() else {
        return
    };

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
            cmds.entity(weapon_ent).remove_parent_in_place();
            cmds.entity(weapon_ent).with_children(|f| {
                f.spawn(AspenColliderBundle {
                    name: Name::new("DroppedWeaponCollider"),
                    tag: ActorColliderType::Item,
                    collider: NeedsCollider::Aabb,
                    // TODO: standard weapon Colliderbundles
                    collision_groups: CollisionLayers::default(),
                    transform: Transform::from_translation(
                        Vec2::ZERO.extend(ACTOR_PHYSICS_Z_INDEX),
                    ),
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
    cmds.entity(player_entity).add_children(&[closest_weapon]);

    for (ent, parent) in query_child_weapon_collider.iter() {
        if parent.parent() == closest_weapon {
            info!("despawning collider for {:?}", parent.parent());
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

    weapon_pos.translation = Vec2::ZERO.extend(ACTOR_Z_INDEX + 1.0);
}
