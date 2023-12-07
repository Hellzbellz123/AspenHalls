// use crate::{
//     consts::TILE_SIZE,
use crate::ahp::{
    engine::{
        debug, info, warn, ActionState, BuildChildren, Commands, Entity, Event, EventWriter,
        GlobalTransform, Parent, Query, Transform, Vec2, Vec3, With, Without,
    },
    game::{
        action_maps, ActorType, BarrelPointTag, CurrentlySelectedWeapon, Player, SpawnActorEvent,
        Type, WeaponColliderTag, WeaponSlots, WeaponSocket, Weapon, TILE_SIZE,
    },
};

/// spawns skeleton near player if `Gameplay::DebugF1` is pressed
pub fn spawn_custom_on_button(
    mut spawn_event_writer: EventWriter<SpawnActorEvent>,
    query_action_state: Query<&ActionState<action_maps::Gameplay>>,
    player_query: Query<(&Transform, With<Player>)>,
) {
    if query_action_state.is_empty() {
        return;
    }
    let actions = query_action_state.get_single().expect("no entities?");

    if actions.just_released(action_maps::Gameplay::DebugF1) {
        debug!("pressed spawn_skeleton_button: Spawning Skeleton near player");
        let mouse_world = actions
            .action_data(action_maps::Gameplay::LookWorld)
            .axis_pair
            .expect("this should always have an axis pair, its data MAY be zero")
            .xy();
        let player_transform = player_query.single().0.translation.truncate();
        let direction_offset: Vec2 = (player_transform - mouse_world).abs().normalize() * TILE_SIZE;

        spawn_event_writer.send(SpawnActorEvent {
            spawner: None,
            actor_type: ActorType(Type::Enemy),
            what_to_spawn: "Skeleton".to_string(),
            spawn_position: (player_transform + (direction_offset)),
            spawn_count: 1,
        });
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
#[allow(clippy::type_complexity)]
pub fn player_attack_sender(
    weapon_query: Query<Entity, (With<Parent>, With<CurrentlySelectedWeapon>, Without<Player>)>,
    query_child_barrel_point: Query<
        (&Parent, &GlobalTransform),
        (With<BarrelPointTag>, Without<Player>),
    >,

    player_query: Query<(
        &mut Player,
        &mut Transform,
        &ActionState<action_maps::Gameplay>,
    )>,
    mut shoot_event_writer: EventWriter<ShootEvent>,
) {
    if player_query.is_empty() | weapon_query.is_empty() | query_child_barrel_point.is_empty() {
        return;
    }

    let weapon_entity = weapon_query.single();

    for (parent, barrel_trans) in &query_child_barrel_point {
        if parent.get() == weapon_entity {
            let player = player_query.single();
            let player_position = player.1.translation.truncate();
            let action_state = player.2;
            let cursor_world = action_state
                .action_data(action_maps::Gameplay::LookWorld)
                .axis_pair
                .expect("no axis pair on Gameplay::LookWorld")
                .xy();
            let barrel_loc = barrel_trans.translation().truncate();
            let direction: Vec2 = (cursor_world - player_position).normalize_or_zero();

            if action_state.pressed(action_maps::Gameplay::Shoot) {
                info!("bang");
                shoot_event_writer.send(ShootEvent {
                    bullet_spawn_loc: barrel_loc,
                    travel_dir: direction,
                });
            }
            if action_state.pressed(action_maps::Gameplay::Melee) {
                // TODO: setup melee system and weapons
                info!("melee not implemented yet");
            }
        }
    }
}

/// equips closest weapon too player if `WeaponSlots` is not full
pub fn equip_closest_weapon(
    mut cmds: Commands,
    mut player_query: Query<
        (
            Entity,
            &mut WeaponSocket,
            &mut Transform,
            &ActionState<action_maps::Gameplay>,
        ),
        With<Player>,
    >,
    query_child_weapon_collider: Query<(Entity, &Parent), With<WeaponColliderTag>>,
    mut weapon_query: Query<(Entity, &mut Weapon, &mut Transform), Without<Player>>,
) {
    if player_query.is_empty() {
        return;
    }

    let (player_entity, mut weapon_socket_on_player, p_transform, actions) =
        player_query.single_mut();

    // screen_print!(
    //     "{:#?} \n selected slot: {:?}",
    //     weapon_socket_on_player.weapon_slots,
    //     weapon_socket_on_player.drawn_slot
    // );

    if !actions
        .just_pressed(action_maps::Gameplay::Interact)
        || // if interact isn't pressed BitXor weapon_socket_on_player.weapon_slots is "full" we can early exit the fn
        weapon_socket_on_player
            .weapon_slots
            .values()
            .all(|&x| x.is_some())
    {
        return;
    }

    debug!("player interact pressed, testing weapon distance");
    for (weapon, mut weapon_tag, mut w_transform) in &mut weapon_query {
        // TODO: investigate distance_squared for performance reasons?
        // TODO: move pickup range into an actor stat for buffs like magnetism and stuffs
        // pretty sure i should leave this as an iterator, we want it sequential?
        let distance_to_player = p_transform.translation.distance(w_transform.translation);

        if distance_to_player > 70.0 {
            return;
        }

        // check if player has available weapon sockets
        let player_weapon_slots = &weapon_socket_on_player.weapon_slots;
        let slots_to_check = [
            WeaponSlots::Slot1,
            WeaponSlots::Slot2,
            WeaponSlots::Slot3,
            WeaponSlots::Slot4,
        ];
        for &slot in &slots_to_check {
            if let Some(slot_value) = player_weapon_slots.get(&slot) {
                if slot_value.is_none() {
                    // the slot is empty, we can add the weapon too it and then return
                    debug!(
                        "the weapon slot is empty, parenting weapon: {:?} too player: {:?}",
                        weapon, slot
                    );
                    cmds.entity(player_entity).push_children(&[weapon]);

                    for (ent, parent) in query_child_weapon_collider.iter() {
                        if parent.get() == weapon {
                            info!("despawning collider for {:?}", parent.get());
                            cmds.entity(ent).despawn();
                        }
                    }

                    // this should make the most recently picked up weapon the currently drawn weapon
                    weapon_socket_on_player.drawn_slot = Some(slot);

                    cmds.entity(weapon).insert(CurrentlySelectedWeapon);
                    weapon_tag.holder = Some(player_entity);
                    weapon_tag.holder_slot = Some(slot);

                    let socket_value = weapon_socket_on_player
                        .weapon_slots
                        .entry(slot)
                        .or_insert(None);
                    *socket_value = Some(weapon);

                    w_transform.translation = Vec3::ZERO;
                    return;
                }
                if slot_value.is_some() {
                    debug!("this slot is full");
                }
            } else {
                warn!("This slot doesn't exist?");
                return;
            }
        }
    }
}
