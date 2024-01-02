use bevy_rapier2d::geometry::{Collider, CollisionGroups};

use crate::{
    bundles::ItemColliderBundle,
    game::actors::{combat::components::WeaponDescriptor, components::ActorColliderType},
    loading::registry::RegistryIdentifier,
    prelude::{
        engine::{
            debug, info, warn, ActionState, BuildChildren, Commands, Entity, Event, EventWriter,
            GlobalTransform, Name, Parent, Query, Transform, TransformBundle, Vec2, Vec3, With,
            Without,
        },
        game::{
            action_maps, ActorType, CurrentlyDrawnWeapon, NpcType, SpawnActorEvent, WeaponHolder,
            WeaponSlots, WeaponSocket, TILE_SIZE,
        },
    },
};

/// spawns skeleton near player if `Gameplay::DebugF1` is pressed
pub fn spawn_custom_on_button(
    mut spawn_event_writer: EventWriter<SpawnActorEvent>,
    player_query: Query<(&Transform, &ActionState<action_maps::Gameplay>)>,
) {
    if player_query.is_empty() {
        return;
    }
    let (player_transform, actions) = player_query.get_single().expect("no entities?");

    if actions.just_released(action_maps::Gameplay::DebugF1) {
        debug!("pressed spawn_skeleton_button: Spawning Skeleton near player");
        let mouse_world = actions
            .action_data(action_maps::Gameplay::CursorWorld)
            .axis_pair
            .expect("this should always have an axis pair, its data MAY be zero")
            .xy();
        let player_position = player_transform.translation.truncate();
        let direction_offset: Vec2 = (player_position - mouse_world).abs().normalize() * TILE_SIZE;

        spawn_event_writer.send(SpawnActorEvent {
            who_spawned: None,
            actor_type: ActorType::Npc(NpcType::Creep),
            what_to_spawn: RegistryIdentifier("skeleton".to_owned()),
            spawn_position: (player_position + (direction_offset)),
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
    weapon_query: Query<
        (Entity, &Transform, &GlobalTransform, &WeaponDescriptor),
        (With<Parent>, With<CurrentlyDrawnWeapon>),
    >,
    player_query: Query<(&mut Transform, &ActionState<action_maps::Gameplay>), Without<Parent>>,
    mut shoot_event_writer: EventWriter<ShootEvent>,
) {
    if player_query.is_empty() | weapon_query.is_empty() {
        return;
    }

    //TODO: split this into different systems
    // match on weapon type
    let (_weapon_entity, local_transform, global_transform, weapon_attack) = weapon_query.single();

    let player = player_query.single();
    let player_position = player.0.translation.truncate();
    let action_state = player.1;
    let cursor_world = action_state
        .action_data(action_maps::Gameplay::CursorWorld)
        .axis_pair
        .expect("no axis pair on Gameplay::LookWorld")
        .xy();
    let WeaponDescriptor::Gun {
        projectile_speed,
        projectile_size,
        barrel_end,
        ammo_amount,
        reload_time: reload_rate,
        fire_rate,
    } = weapon_attack
    else {
        return;
    };

    let barrel_loc =
        global_transform.transform_point(local_transform.translation + barrel_end.extend(0.0));
    let direction: Vec2 = (cursor_world - player_position).normalize_or_zero();

    if action_state.pressed(action_maps::Gameplay::Shoot) {
        info!("bang");
        shoot_event_writer.send(ShootEvent {
            bullet_spawn_loc: barrel_loc.truncate(),
            travel_dir: direction,
        });
    }
    // if action_state.pressed(action_maps::Gameplay::Melee) {
    //     // TODO: setup melee system and weapons
    //     info!("melee not implemented yet");
    // }
}

// TODO:
// make picking up thing an event
// depending on event send new event
/// equips closest weapon too player if `WeaponSlots` is not full
#[allow(clippy::type_complexity)]
pub fn equip_closest_weapon(
    mut cmds: Commands,
    mut player_query: Query<(
        Entity,
        &mut WeaponSocket,
        &mut Transform,
        &ActionState<action_maps::Gameplay>,
    )>,
    query_child_weapon_collider: Query<(Entity, &Parent), With<ActorColliderType>>,
    mut weapon_query: Query<
        (Entity, &mut WeaponHolder, &mut Transform),
        (Without<Parent>, Without<WeaponSocket>),
    >,
) {
    let (player_entity, mut weapon_socket_on_player, p_transform, actions) =
        player_query.single_mut();

    if !actions.just_pressed(action_maps::Gameplay::Interact) {
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
                .get_key_value(&drawn_weapon.unwrap_or(WeaponSlots::Slot1))
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
    *weapon_holder = WeaponHolder(Some((player_entity, *too_update_slot)));

    let socket_value = weapon_socket_on_player
        .weapon_slots
        .entry(*too_update_slot)
        .or_insert(None);
    *socket_value = Some(closest_weapon);

    weapon_pos.translation = Vec3::ZERO;
}
