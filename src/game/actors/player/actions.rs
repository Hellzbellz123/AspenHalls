#![allow(clippy::type_complexity)]

use crate::{
    game::{
        actors::{
            combat::components::{
                BarrelPointTag, CurrentlySelectedWeapon, WeaponColliderTag, WeaponSlots,
                WeaponSocket, WeaponTag,
            },
            components::Player,
            spawners::components::{EnemyType, SpawnEnemyEvent},
        },
        input::actions,
    },
    utilities::EagerMousePos,
};
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState as lfActionState;

/// spawns skeleton near player if debugf1 is pressed
pub fn spawn_skeleton_button(
    mut eventwriter: EventWriter<SpawnEnemyEvent>,
    mouse: Res<EagerMousePos>,
    query_action_state: Query<&lfActionState<actions::Combat>>,
    player_query: Query<(&Transform, With<Player>)>,
) {
    if query_action_state.is_empty() {
        return;
    }
    let actions = query_action_state.get_single().expect("no ents?");

    if actions.just_released(actions::Combat::DebugF1) {
        debug!("pressed spawn_skeleton_button: Spawning Skeleton near player");
        let player_transform = player_query.single().0.translation.truncate();
        let direction: Vec2 = (player_transform - Vec2::new(mouse.world.x, mouse.world.y))
            .abs()
            .normalize_or_zero();

        eventwriter.send(SpawnEnemyEvent {
            enemy_to_spawn: EnemyType::Skeleton,
            spawn_position: (player_transform + (direction)),
            spawn_count: 1,
        })
    };
}



/// event too spawn bullets
#[derive(Event, Debug)]
pub struct ShootEvent {
    /// where too spawn
    pub bullet_spawn_loc: Vec2,
    /// direction too shoot
    pub travel_dir: Vec2,
}

/// send shoot request to gun control system.
pub fn player_attack_sender(
    weapon_query: Query<
        (
            Entity,
            &Children,
            &Parent,
            &CurrentlySelectedWeapon, // this can probably be single()
            &Transform,
        ),
        (With<Parent>, Without<Player>),
    >,
    query_childbarrelpoint: Query<
        (Entity, &Parent, &GlobalTransform),
        (With<BarrelPointTag>, Without<Player>),
    >,

    // query_childweaponcollider: Query<(Entity, &Parent), With<WeaponColliderTag>>,
    // mut weapon_query2: Query<(Entity, &mut WeaponTag, &mut Transform), Without<Player>>,
    player_query: Query<(&mut Player, &mut Transform)>,
    mut input_query: Query<&lfActionState<actions::Combat>>,
    mouse_pos: Res<EagerMousePos>,
    mut shootwriter: EventWriter<ShootEvent>,
) {
    if player_query.is_empty()
        | weapon_query.is_empty()
        | input_query.is_empty()
        | query_childbarrelpoint.is_empty()
    {
        return;
    }

    // weapon_query.for_each(|(went, _wchildren, _wparent, _wactivetag, _wtransform)| {
    // });

    let (went, _wchild, _wparent, _wactive, _wtrans) = weapon_query.single();

    query_childbarrelpoint.for_each(|(_ent, parent, barrel_trans)| {
        if parent.get() == went {
            let barrel_loc = barrel_trans.translation().truncate();
            let playerpos = player_query.single().1.translation.truncate();
            let direction: Vec2 = (mouse_pos.world - playerpos).normalize_or_zero();
            let action_state = input_query.single_mut();

            if action_state.pressed(actions::Combat::Shoot) {
                shootwriter.send(ShootEvent {
                    bullet_spawn_loc: barrel_loc,
                    travel_dir: direction,
                })
            }
            if action_state.pressed(actions::Combat::Melee) {
                // TODO: setup melee system and weapons
                info!("meleee not implemented yet")
            }
        }
    });
}

/// equips closest weapon too player if weaponslots is not full
pub fn equip_closest_weapon(
    mut cmds: Commands,
    mut player_query: Query<
        (
            Entity,
            &mut WeaponSocket,
            &mut Transform,
            &lfActionState<actions::Combat>,
        ),
        With<Player>,
    >,
    query_childweaponcollider: Query<(Entity, &Parent), With<WeaponColliderTag>>,
    mut weapon_query: Query<(Entity, &mut WeaponTag, &mut Transform), Without<Player>>,
) {
    if player_query.is_empty() {
        return;
    }

    let (playerentity, mut weaponsocket_on_player, ptransform, actions) = player_query.single_mut();

    // screen_print!(
    //     "{:#?} \n selected slot: {:?}",
    //     weaponsocket_on_player.weapon_slots,
    //     weaponsocket_on_player.drawn_slot
    // );

    if !actions
        .just_pressed(actions::Combat::Interact)
        | // if interact isnt pressed BitXor weaponsockets.weaponslots is "full" we can early exit the fn
        weaponsocket_on_player
            .weapon_slots
            .values()
            .all(|&x| x.is_some())
    {
        return;
    }

    debug!("player interact pressed, testing weapon distance");
    for (weapon, mut weapontag, mut wtransform) in weapon_query.iter_mut() {
        // pretty sure i should leave this as an iterator, we want it sequential
        let distance_to_player = (ptransform.translation - wtransform.translation)
            .length()
            .abs();
        if distance_to_player < 70.0 {
            // check if player has available weapon sockets
            let player_weapon_slots = &weaponsocket_on_player.weapon_slots;
            let slots_to_check = [
                WeaponSlots::Slot1,
                WeaponSlots::Slot2,
                WeaponSlots::Slot3,
                WeaponSlots::Slot4,
            ];
            for &slot in &slots_to_check {
                match player_weapon_slots.get(&slot) {
                    Some(slot_value) => {
                        if slot_value.is_none() {
                            // the slot is empty, we can add the weapon too it and then return
                            debug!(
                                "the weapon slot is empty, parenting weapon: {:?} too player: {:?}",
                                weapon, slot
                            );
                            cmds.entity(playerentity).push_children(&[weapon]);

                            for (ent, parent) in query_childweaponcollider.iter() {
                                if parent.get() == weapon {
                                    info!("despawning collider for {:?}", parent.get());
                                    cmds.entity(ent).despawn()
                                }
                            }

                            weaponsocket_on_player.drawn_slot = Some(slot); // this should make the most recently picked up weapon the currently drawn weapoin

                            cmds.entity(weapon).insert(CurrentlySelectedWeapon);
                            weapontag.parent = Some(playerentity);
                            weapontag.stored_weapon_slot = Some(slot);

                            let socket_value = weaponsocket_on_player
                                .weapon_slots
                                .entry(slot)
                                .or_insert(None);
                            *socket_value = Some(weapon);

                            wtransform.translation = Vec3::ZERO;
                            return;
                        }
                        if slot_value.is_some() {
                            debug!("this slot is full")
                        }
                    }
                    None => {
                        warn!("This slot doesnt exist?");
                        return;
                    }
                }
            }
        }
    }
}
