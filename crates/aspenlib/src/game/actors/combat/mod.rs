use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;
use leafwing_input_manager::prelude::ActionState;

use crate::{
    consts::ACTOR_Z_INDEX,
    game::{
        actors::{
            attributes_stats::{CharacterStats, DamageQueue, EquipmentStats},
            combat::components::{
                AttackDamage, CurrentlyDrawnWeapon, WeaponDescriptor, WeaponHolder, WeaponSlots,
                WeaponSocket,
            },
            player::actions::ShootEvent,
        },
        input::action_maps,
        AppState,
    },
    prelude::{engine, game::AspenInitHandles},
};

/// different attacks that can exist in the game
mod attacks;
/// combat related components
pub mod components;
/// hit detection for bullets
mod hit_detection;

/// information tracked for current run
#[derive(Debug, Clone, Copy, Resource)]
pub struct CurrentRunInformation {
    /// damage dealt by player this run
    pub enemy_physical_damage_taken: f32,
    /// damage dealt too player this run
    pub player_physical_damage_taken: f32,
    /// enemies killed by player this run
    pub enemies_deaths: i32,
    /// times player has died
    pub player_deaths: i32,
    /// amount of damage enemy's have fired that hit player and didn't get counted
    pub enemy_damage_sent: f32,
    /// amount of damage player have fired that hit enemy and didn't get counted
    pub player_damage_sent: f32,
}

//TODO: save this too file, load from file when rebooting game
/// information tracked for player save state
#[derive(Debug, Clone, Copy, Resource)]
pub struct PlayerSaveInformation {
    /// damage player has cause with this save
    pub all_time_damage: f32,
    /// amount of times player has finishes a run
    pub runs_completed: i32,
    /// amount of times play has started a run
    pub runs_started: i32,
    /// amount of money player has earned
    pub player_money: i32,
    /// total amount of player deaths
    pub total_deaths: i32,
    /// total amonut of items player has collected
    pub items_got: i32,
}

/// plugin for all actor weapon functionality
pub struct ActorWeaponPlugin;

impl Plugin for ActorWeaponPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentRunInformation {
            player_physical_damage_taken: 0.0,
            enemy_physical_damage_taken: 0.0,
            player_damage_sent: 0.0,
            enemies_deaths: 0,
            enemy_damage_sent: 0.0,
            player_deaths: 0,
        })
        .register_type::<AttackDamage>()
        .register_type::<EquipmentStats>()
        .insert_resource(WeaponFiringTimer::default())
        .add_systems(
            PreUpdate,
            (update_selected_weapon, apply_damage_system)
                .run_if(state_exists_and_equals(AppState::PlayingGame)),
        )
        .add_systems(
            Update,
            (
                update_player_equipped_weapon,
                handle_death_system,
                hit_detection::projectile_hits,
                // hit_detection::hits_on_enemy,
                // hit_detection::hits_on_player,
                flip_weapon_sprites,
                rotate_player_weapon,
                equipped_weapon_positioning,
                weapon_visibility_system,
                receive_shoot_weapon,
            )
                .run_if(state_exists_and_equals(AppState::PlayingGame)),
        );
    }
}

/// weapon firing timer
/// basically firing hammer time
#[derive(Component, Default, Reflect, Deref, DerefMut, Resource)]
#[reflect(Component)]
pub struct WeaponFiringTimer(pub Timer);

/// rotates weapon too face wherever the players mouse is
#[allow(clippy::type_complexity)]
fn rotate_player_weapon(
    mut player_query: Query<(&WeaponSocket, &ActionState<action_maps::Gameplay>)>,
    mut weapon_query: Query<
        // this is equivalent to if player has a weapon equipped and out
        (&WeaponHolder, &GlobalTransform, &mut Transform),
        (With<Parent>, With<CurrentlyDrawnWeapon>),
    >,
) {
    if weapon_query.is_empty() {
        return;
    }

    for (weapon_holder, weapon_global_transform, mut weapon_transform) in &mut weapon_query {
        if weapon_holder.is_some() {
            let (_player_animation_state, player_input) = player_query.single_mut();
            let global_mouse_pos = player_input
                .action_data(action_maps::Gameplay::CursorWorld)
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

/// flips weapon sprite if aim angle is not between -90 and 90 degrees
fn flip_weapon_sprites(
    // all weapons with a sprite
    mut weapon_query: Query<(&WeaponHolder, &Transform, &mut TextureAtlasSprite)>,
) {
    for (weapon_holder, weapon_transform, mut sprite) in &mut weapon_query {
        if weapon_holder.is_some() {
            let (_aim_angle, radians) = weapon_transform.rotation.to_axis_angle();
            let degrees = radians.to_degrees();

            sprite.flip_y = !(-90.0..=90.0).contains(&degrees);
        }
    }
}

/// keeps all weapons centered too parented entity
#[allow(clippy::type_complexity)]
fn equipped_weapon_positioning(
    children: Query<&Children>,
    // actors that can equip weapons
    characters: Query<Entity, With<WeaponSocket>>,
    mut weapon_query: Query<
        // all weapons equipped too entity
        (&mut Transform, &mut Velocity),
        (With<WeaponHolder>, Without<WeaponSocket>),
    >,
) {
    for character in &characters {
        children.iter_descendants(character).for_each(|f| {
            if let Ok((mut weapon_transform, mut weapon_velocity)) = weapon_query.get_mut(f) {
                if weapon_velocity.linvel != Vec2::ZERO {
                    weapon_velocity.linvel = Vec2::ZERO;
                }
                if weapon_velocity.angvel != 0.0 {
                    weapon_velocity.angvel = 0.0;
                }
                weapon_transform.translation = Vec3 {
                    x: 0.0,
                    y: 12.0,
                    z: 1.0,
                }
            }
        });
    }
}

/// check if the weapon is supposed to be visible
fn weapon_visibility_system(
    weapon_sockets: Query<&WeaponSocket>,
    mut weapon_query: Query<(&WeaponHolder, &mut Visibility, Option<&Parent>)>,
) {
    if weapon_sockets.is_empty() || weapon_query.is_empty() {
        return;
    }

    for (weapon_holder, mut weapon_visibility, parent) in &mut weapon_query {
        if let Some((weapon_holder, weapon_slot)) = weapon_holder.0
            && let Some(parent) = parent
        {
            let parent = parent.get();
            if parent != weapon_holder {
                warn!("weapon is parented incorrectly");
            }
            let Ok(weapon_socket) = weapon_sockets.get(parent) else {
                continue;
            };
            if weapon_socket.drawn_slot.is_some_and(|f| f == weapon_slot) {
                *weapon_visibility = Visibility::Inherited;
            } else {
                *weapon_visibility = Visibility::Hidden;
            }
        } else {
            //TODO: if we want too spawn weapons before player should grab we should make this changed or remove this
            *weapon_visibility = Visibility::Inherited;
        }
    }
}

/// removes `CurrentlyDrawnWeapon` from entity's not in `WeaponSocket.drawn_weapon`
fn update_selected_weapon(
    mut cmds: Commands,
    weapon_carry_actors: Query<(Entity, &WeaponSocket), Changed<WeaponSocket>>,
    selected_weapon: Query<&CurrentlyDrawnWeapon>,
) {
    for (_actor, socket) in &weapon_carry_actors {
        if socket.drawn_slot.is_none() {
            info!("actor should not display any weapons");
            let equipped_and_drawn_weapons = socket
                .weapon_slots
                .values()
                .flatten()
                .filter(|f| selected_weapon.get(**f).is_ok());
            // .filter(|f| f.is_some())
            // .map(|f| f.unwrap())
            equipped_and_drawn_weapons.for_each(|f| {
                cmds.entity(*f).remove::<CurrentlyDrawnWeapon>();
            });
        } else {
            let drawn_slot = socket.drawn_slot.unwrap();
            let Some(drawn_weapon) = socket.weapon_slots.get(&drawn_slot).unwrap() else {
                // no weapons exist for this actor
                continue;
            };

            // get slots with values != None
            let equipped_weapons = socket.weapon_slots.values().flatten();

            for weapon in equipped_weapons {
                if weapon != drawn_weapon {
                    if selected_weapon.get(*weapon).is_ok() {
                        cmds.entity(*weapon).remove::<CurrentlyDrawnWeapon>();
                    }
                } else if selected_weapon.get(*weapon).is_err() {
                    cmds.entity(*weapon).insert(CurrentlyDrawnWeapon);
                }
            }
        }
    }
}

// TODO: move too player actions file
/// updates players equipped weapon based on input
#[allow(clippy::type_complexity)]
fn update_player_equipped_weapon(
    mut player_query: Query<(&mut WeaponSocket, &ActionState<action_maps::Gameplay>)>,
) {
    let (mut player_weapon_socket, actions) = player_query.single_mut();

    if actions.pressed(action_maps::Gameplay::CycleWeapon)
        && actions.current_duration(action_maps::Gameplay::CycleWeapon) >= Duration::from_secs(2)
    {
        info!("setting equipped slot None");
        player_weapon_socket.drawn_slot = None;
    } else if actions.just_pressed(action_maps::Gameplay::CycleWeapon) {
        let player_slots = &player_weapon_socket.weapon_slots;

        info!("selecting next weapon slot");
        match player_weapon_socket
            .drawn_slot
            .unwrap_or(WeaponSlots::Slot4)
        {
            WeaponSlots::Slot1 => {
                player_weapon_socket.drawn_slot =
                    if player_slots.get(&WeaponSlots::Slot2).unwrap().is_some() {
                        Some(WeaponSlots::Slot2)
                    } else if player_slots.get(&WeaponSlots::Slot3).unwrap().is_some() {
                        Some(WeaponSlots::Slot3)
                    } else if player_slots.get(&WeaponSlots::Slot4).unwrap().is_some() {
                        Some(WeaponSlots::Slot4)
                    } else {
                        Some(WeaponSlots::Slot1)
                    }
            }
            WeaponSlots::Slot2 => {
                player_weapon_socket.drawn_slot = {
                    if player_slots.get(&WeaponSlots::Slot3).unwrap().is_some() {
                        Some(WeaponSlots::Slot3)
                    } else if player_slots.get(&WeaponSlots::Slot4).unwrap().is_some() {
                        Some(WeaponSlots::Slot4)
                    } else if player_slots.get(&WeaponSlots::Slot1).unwrap().is_some() {
                        Some(WeaponSlots::Slot1)
                    } else {
                        Some(WeaponSlots::Slot2)
                    }
                }
            }
            WeaponSlots::Slot3 => {
                player_weapon_socket.drawn_slot = {
                    if player_slots.get(&WeaponSlots::Slot4).unwrap().is_some() {
                        Some(WeaponSlots::Slot4)
                    } else if player_slots.get(&WeaponSlots::Slot1).unwrap().is_some() {
                        Some(WeaponSlots::Slot1)
                    } else if player_slots.get(&WeaponSlots::Slot2).unwrap().is_some() {
                        Some(WeaponSlots::Slot2)
                    } else {
                        Some(WeaponSlots::Slot3)
                    }
                }
            }
            WeaponSlots::Slot4 => {
                player_weapon_socket.drawn_slot = {
                    if player_slots.get(&WeaponSlots::Slot1).unwrap().is_some() {
                        Some(WeaponSlots::Slot1)
                    } else if player_slots.get(&WeaponSlots::Slot2).unwrap().is_some() {
                        Some(WeaponSlots::Slot2)
                    } else if player_slots.get(&WeaponSlots::Slot3).unwrap().is_some() {
                        Some(WeaponSlots::Slot3)
                    } else {
                        Some(WeaponSlots::Slot4)
                    }
                }
            }
        }
    }
}

// TODO: refactor this system and related systems into a module for weapons, add ammo management too new module
/// receives shoot events and creates bullet at location
#[allow(clippy::type_complexity)]
pub fn receive_shoot_weapon(
    mut cmds: Commands,
    time: Res<Time>,
    assets: Res<AspenInitHandles>,
    mut firing_timer: ResMut<WeaponFiringTimer>,
    mut attack_event_reader: EventReader<ShootEvent>,
    weapon_query: Query<
        // this is equivalent to if player has a weapon equipped and out
        (&mut WeaponHolder, &AttackDamage, &WeaponDescriptor),
        (With<Parent>, With<CurrentlyDrawnWeapon>),
    >,
) {
    firing_timer.tick(time.delta());

    if weapon_query.is_empty() {
        return;
    }

    let firing_timer = &mut firing_timer.0;
    let (_weapon_tag, weapon_damage, weapon_type) = weapon_query.single();

    //TODO: this system needs too handle weapon reload properly
    firing_timer.set_mode(TimerMode::Once);
    firing_timer.set_duration(Duration::from_secs_f32(1.0));

    for event in &mut attack_event_reader.read() {
        // info!("firing duration: {:#?}", firing_timer.duration());
        if firing_timer.finished() {
            attacks::create_bullet(&mut cmds, &assets, event, weapon_type, weapon_damage);
            firing_timer.reset();
            // info!("fire timer finished");
            return;
        }
    }
}

// TODO: have damaged characters use particle effect or red tint when damaged
/// applys
#[allow(clippy::type_complexity)]
fn apply_damage_system(
    mut game_info: ResMut<CurrentRunInformation>,
    mut damaged_characters: Query<
        (&mut CharacterStats, Entity, &DamageQueue),
        Changed<DamageQueue>,
    >,
    player_controlled: Query<&engine::ActionState<action_maps::Gameplay>>,
) {
    for (mut character_stats, character, damage_queue) in &mut damaged_characters {
        for damage in damage_queue.iter_queue() {
            if character_stats.get_current_health() <= 0.0 {
                return;
            }
            if player_controlled.get(character).is_ok() {
                game_info.player_physical_damage_taken += damage.physical.0;
            } else {
                game_info.enemy_physical_damage_taken += damage.physical.0;
            }
            character_stats.apply_damage(*damage);
        }
    }
}

/// gathers entitys that have damage and despawns them if have no remaining health
#[allow(clippy::type_complexity)]
fn handle_death_system(
    mut game_info: ResMut<CurrentRunInformation>,
    mut cmds: Commands,
    mut damaged_query: Query<
        (
            Entity,
            &mut CharacterStats,
            &mut Transform,
            Option<&ActionState<action_maps::Gameplay>>,
        ),
        Changed<CharacterStats>,
    >,
) {
    for (ent, mut stats, mut transform, player_control) in &mut damaged_query {
        if stats.get_current_health() <= 0.0 {
            if player_control.is_some() {
                error!("player died, moving player");
                // player is entity that died
                stats.set_health(150.0);
                *transform = Transform::from_translation(Vec3::new(0.0, 0.0, ACTOR_Z_INDEX));
                game_info.player_deaths += 1;
            } else {
                // entity that died is not player
                error!("despawning entity");
                game_info.enemies_deaths += 1;
                cmds.entity(ent).despawn_recursive();
            }
        }
    }
}

/// gets ent id of weapon in weapon slot
fn get_current_weapon(
    weapon_slots: &mut bevy::utils::hashbrown::HashMap<WeaponSlots, Option<Entity>>,
    weapon_socket: &WeaponSocket,
) -> Option<Entity> {
    let entity_in_drawn_slot = weapon_slots
        .entry(
            weapon_socket
                .drawn_slot
                .expect("failed to unwrap WeaponSocket.drawn_slot"),
        )
        .or_insert(None);
    let currently_equipped_from_hashmap: Option<Entity> = entity_in_drawn_slot
        .as_mut()
        .map(|current_equipped_weapon| *current_equipped_weapon);

    currently_equipped_from_hashmap.map_or_else(
        || {
            warn!("no currently equipped weapon");
            None
        },
        Some,
    )
}
