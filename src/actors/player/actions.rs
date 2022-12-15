use crate::actors::weapons::components::WeaponSlots;
use bevy::prelude::*;

use bevy_debug_text_overlay::screen_print;
use bevy_mouse_tracking_plugin::MousePosWorld;
use leafwing_input_manager::prelude::ActionState as leafwingActionState;

use crate::{
    action_manager::actions::PlayerActions,
    actors::weapons::components::{WeaponSocket, WeaponTag},
    components::actors::{
        general::Player,
        spawners::{EnemyType, SpawnEnemyEvent},
    },
    utilities::game::ACTOR_LAYER,
};

pub fn spawn_skeleton_button(
    mut eventwriter: EventWriter<SpawnEnemyEvent>,
    mouse: Res<MousePosWorld>,
    query_action_state: Query<&leafwingActionState<PlayerActions>>,
    player_query: Query<(&Transform, With<Player>)>,
) {
    if query_action_state.is_empty() {
        return;
    }
    let actions = query_action_state.get_single().expect("no ents?");

    if actions.just_released(PlayerActions::DebugF1) {
        debug!("pressed spawn_skeleton_button: Spawning Skeleton near player");
        let player_transform = player_query.single().0.translation.truncate();
        let direction: Vec2 = (player_transform - Vec2::new(mouse.x, mouse.y))
            .abs()
            .normalize_or_zero();

        eventwriter.send(SpawnEnemyEvent {
            enemy_to_spawn: EnemyType::Skeleton,
            spawn_position: (player_transform + (direction)).extend(ACTOR_LAYER),
            spawn_count: 1,
        })
    };
}

pub fn equip_closest_weapon(
    mut cmds: Commands,
    query_action_state: Query<&leafwingActionState<PlayerActions>>,
    mut player_query: Query<(Entity, &mut WeaponSocket, &mut Transform), With<Player>>,
    #[allow(clippy::type_complexity)] mut weapon_query: Query<
        (Entity, &mut WeaponTag, &mut Transform),
        Without<Player>,
    >,
) {
    if player_query.is_empty() | query_action_state.is_empty() {
        return;
    }
    let _actions = query_action_state.single();
    let (playerentity, mut weaponsocket_on_player, ptransform) = player_query.single_mut();
    screen_print!("{:#?}", weaponsocket_on_player.weapon_slots);
    screen_print!("{:?}", weaponsocket_on_player.drawn_slot);

    if !query_action_state
        .single()
        .just_pressed(PlayerActions::Interact)
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

                            weaponsocket_on_player.drawn_slot = slot; // this should make the most recently picked up weapon the currently drawn weapoin
                            cmds.entity(weapon)
                                // .insert(CurrentlyDrawnWeapon)
                                .despawn_descendants(); //despawn weapon collider when we parent it

                            weapontag.parent = Some(playerentity);
                            weapontag.stored_weapon_slot = Some(slot);

                            let socket_value = weaponsocket_on_player
                                .weapon_slots
                                .entry(slot)
                                .or_insert(None);
                            *socket_value = Some(weapon);

                            wtransform.translation = Vec3::ZERO
                                + Vec3 {
                                    x: 0.0,
                                    y: 1.5,
                                    z: 1.0,
                                };
                            return;
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
