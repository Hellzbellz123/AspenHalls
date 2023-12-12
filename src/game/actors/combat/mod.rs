/// different attacks that can exist in the game
mod attacks;
/// combat related components
pub mod components;
/// hit detection for bullets
mod hit_detection;

use std::{f32::consts::FRAC_PI_2, time::Duration};

use bevy::prelude::*;

use bevy_debug_text_overlay::screen_print;

use bevy_rapier2d::prelude::Velocity;
use leafwing_input_manager::prelude::ActionState;

use crate::{
    consts::ACTOR_Z_INDEX,
    game::{
        actors::{
            combat::components::{
                CurrentlySelectedWeapon, Weapon, WeaponSlots, WeaponSocket, WeaponStats,
            },
            player::actions::ShootEvent,
        },
        input::action_maps,
        AppState, TimeInfo,
    },
    loading::assets::ActorTextureHandles,
};

use self::components::Damage;

use super::{
    ai::components::{ActorType, Enemy},
    animation::components::{ActorAnimationType, AnimState},
    components::{ActorCombatStats, Player},
};

/// stats tracked for game progress
#[derive(Debug, Clone, Copy, Resource)]
pub struct PlayerGameInformation {
    /// total damage dealt by player
    pub damage_dealt: f32,
    /// enemies killed by player
    pub enemies_killed: i32,
    /// damage enemy's deal too player
    pub damage_taken: f32,
    /// times player has died
    pub player_deaths: i32,
    /// amount of damage enemy's have fired that hit player and didn't get counted
    pub enemy_damage_sent: f32,
    /// amount of damage player have fired that hit enemy and didn't get counted
    pub player_damage_sent: f32,
}

/// plugin for all actor weapon functionality
pub struct ActorWeaponPlugin;

impl Plugin for ActorWeaponPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerGameInformation {
            damage_taken: 0.0,
            damage_dealt: 0.0,
            player_damage_sent: 0.0,
            enemies_killed: 0,
            enemy_damage_sent: 0.0,
            player_deaths: 0,
        })
        .insert_resource(WeaponFiringTimer::default())
        .add_systems(PreUpdate, (remove_cdw_component, deal_with_damaged))
        .add_systems(
            Update,
            (
                update_equipped_weapon,
                player_death_system,
                hit_detection::hits_on_enemy,
                hit_detection::hits_on_player,
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
    game_time: Res<TimeInfo>,
    mut player_query: Query<(
        (&AnimState, &ActionState<action_maps::Gameplay>),
        With<Player>,
    )>,
    mut weapon_query: Query<
        // this is equivalent to if player has a weapon equipped and out
        (&Weapon, &GlobalTransform, &mut Transform),
        (With<Parent>, With<CurrentlySelectedWeapon>),
    >,
) {
    if game_time.game_paused || weapon_query.is_empty() {
        return;
    }

    for (weapon_tag, weapon_global_transform, mut weapon_transform) in &mut weapon_query {
        if weapon_tag.holder.is_some() {
            let ((_player_animation_state, player_input), ()) = player_query.single_mut();
            let global_mouse_pos = player_input
                .action_data(action_maps::Gameplay::LookWorld)
                .axis_pair
                .unwrap()
                .xy();
            let global_weapon_pos: Vec2 = weapon_global_transform
                .compute_transform()
                .translation
                .truncate();
            let look_direction: Vec2 = (global_weapon_pos - global_mouse_pos).normalize_or_zero();
            let aim_angle = (-look_direction.y).atan2(-look_direction.x) + FRAC_PI_2; // add offset too rotation here

            // mirror whole entity by negating the scale when were looking left,
            if aim_angle.to_degrees() > 180.0 || aim_angle.to_degrees() < -0.0 {
                weapon_transform.scale.x = -1.0;
            } else {
                weapon_transform.scale.x = 1.0;
            }
            weapon_transform.rotation = Quat::from_euler(EulerRot::ZYX, aim_angle, 0.0, 0.0);
        }
    }
}

/// keeps all weapons centered too parented entity
#[allow(clippy::type_complexity)]
fn equipped_weapon_positioning(
    // actors that can equip weapons
    weapon_carrying_actors: Query<(&AnimState, Entity), With<WeaponSocket>>,
    mut weapon_query: Query<
        // all weapons equipped too entity
        (&Weapon, &mut Transform, &mut Velocity),
        (With<Parent>, Without<ActorType>),
    >,
) {
    if weapon_query.is_empty() || weapon_carrying_actors.is_empty() {
        return;
    }

    for (animation_state, weapon_carrying_entity) in &weapon_carrying_actors {
        for (weapon_tag, mut weapon_transform, mut weapon_velocity) in &mut weapon_query {
            if weapon_tag.holder == Some(weapon_carrying_entity) {
                // weapon_velocity.angvel =
                //     lerp(weapon_velocity.angvel, 0.0, 0.3);
                weapon_velocity.linvel = Vec2::ZERO;
                // modify weapon sprite to be below player when facing up, this
                // still looks strange but looks better than a back mounted smg
                if animation_state.animation_type == ActorAnimationType::Up {
                    // this transform is local too players transform
                    weapon_transform.translation = Vec3 {
                        x: 0.0,
                        y: 1.5,
                        z: -1.0,
                    }
                } else {
                    weapon_transform.translation = Vec3 {
                        x: 0.0,
                        y: 1.5,
                        z: 1.0,
                    }
                }
            }
        }
    }
}

/// check if the weapon is supposed to be visible
fn weapon_visibility_system(
    player_query: Query<&WeaponSocket, With<Player>>,
    mut weapon_query: Query<(&Weapon, &mut Visibility)>, // query weapons parented to entity's
) {
    if player_query.is_empty() || weapon_query.is_empty() {
        return;
    }

    let p_weapon_socket = player_query.single();

    for (weapon_tag, mut weapon_visibility) in &mut weapon_query {
        if weapon_tag.holder_slot == p_weapon_socket.drawn_slot {
            // TODO: these feels wrong, deref doesn't feel correct here
            // find a less gross solution
            *weapon_visibility = Visibility::Inherited;
        } else {
            *weapon_visibility = Visibility::Hidden;
        }
    }
}

/// removes `CurrentlyDrawnWeapon` from entity's parented to player that don't
/// match the entity in `WeaponSocket.drawn_weapon`
#[allow(clippy::type_complexity)]
fn remove_cdw_component(
    mut cmds: Commands,
    names: Query<&Name>,
    player_query: Query<&WeaponSocket, With<Player>>,

    drawn_weapon: Query<&CurrentlySelectedWeapon>,
    weapon_query: Query<
        (Entity, &Weapon),
        (With<Parent>, With<CurrentlySelectedWeapon>, Without<Player>),
    >,
) {
    if player_query.is_empty() | weapon_query.is_empty() | drawn_weapon.is_empty() {
        return;
    }

    let player_weapon_socket = player_query.single();

    for (weapon_entity, weapon_tag) in &weapon_query {
        if weapon_tag.holder_slot != player_weapon_socket.drawn_slot
            && drawn_weapon.get(weapon_entity).is_ok()
        {
            let weapon_name = names
                .get(weapon_entity)
                .expect("entity doesn't have a name");
            debug!(
                "weapon {} {:#?} should not have active component, removing",
                weapon_name, weapon_entity
            );
            cmds.entity(weapon_entity)
                .remove::<CurrentlySelectedWeapon>();
        }
    }
}

/// updates players equipped weapon based on input
#[allow(clippy::type_complexity)]
fn update_equipped_weapon(
    mut cmds: Commands,
    query_action_state: Query<&ActionState<action_maps::Gameplay>>,
    mut player_query: Query<&mut WeaponSocket, With<Player>>,
    weapon_query: Query<(Entity, &mut Weapon, &mut Transform), (With<Parent>, Without<Player>)>,
) {
    if player_query.is_empty() | weapon_query.is_empty() | query_action_state.is_empty() {
        return;
    }

    let mut player_weapon_socket = player_query.single_mut();
    let actions = query_action_state.single();

    if actions.just_pressed(action_maps::Gameplay::EquipSlot1) {
        // set whatever weapon is in slot 1 as CurrentlyDrawnWeapon and remove
        // CurrentlyDrawnWeapon from old weapon
        player_weapon_socket.drawn_slot = Some(WeaponSlots::Slot1);
        let current_weapon_slots = &mut player_weapon_socket.weapon_slots.clone();
        let current_weapon = get_current_weapon(current_weapon_slots, &player_weapon_socket);

        if let Some(ent) = current_weapon {
            cmds.entity(ent).insert(CurrentlySelectedWeapon);
            info!("equipping slot 1");
        }
    } else if actions.just_pressed(action_maps::Gameplay::EquipSlot2) {
        player_weapon_socket.drawn_slot = Some(WeaponSlots::Slot2);
        let current_weapon_slots = &mut player_weapon_socket.weapon_slots.clone();
        let new_weapon = get_current_weapon(current_weapon_slots, &player_weapon_socket);

        if let Some(ent) = new_weapon {
            cmds.entity(ent).insert(CurrentlySelectedWeapon);
            info!("equipping slot 2");
        }
    } else if actions.just_pressed(action_maps::Gameplay::EquipSlot3) {
        player_weapon_socket.drawn_slot = Some(WeaponSlots::Slot3);
        let current_weapon_slots = &mut player_weapon_socket.weapon_slots.clone();
        let new_weapon = get_current_weapon(current_weapon_slots, &player_weapon_socket);

        if let Some(ent) = new_weapon {
            cmds.entity(ent).insert(CurrentlySelectedWeapon);
            info!("equipping slot 3");
        }
    } else if actions.just_pressed(action_maps::Gameplay::EquipSlot4) {
        player_weapon_socket.drawn_slot = Some(WeaponSlots::Slot4);
        let current_weapon_slots = &mut player_weapon_socket.weapon_slots.clone();
        let new_weapon = get_current_weapon(current_weapon_slots, &player_weapon_socket);

        if let Some(ent) = new_weapon {
            cmds.entity(ent).insert(CurrentlySelectedWeapon);
            info!("equipping slot 4");
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

// TODO: refactor this system and related systems into a module for weapons, add ammo management too new module
/// receives shoot events and creates bullet at location
#[allow(clippy::type_complexity)]
pub fn receive_shoot_weapon(
    mut cmds: Commands,
    time: Res<Time>,
    assets: ResMut<ActorTextureHandles>,
    mut firing_timer: ResMut<WeaponFiringTimer>,
    mut attack_event_reader: EventReader<ShootEvent>,
    weapon_query: Query<
        // this is equivalent to if player has a weapon equipped and out
        (&mut Weapon, &WeaponStats, &Transform),
        (With<Parent>, With<CurrentlySelectedWeapon>),
    >,
) {
    firing_timer.tick(time.delta());

    if weapon_query.is_empty() {
        return;
    }

    let firing_timer = &mut firing_timer.0;
    let (_weapon_tag, weapon_stats, _) = weapon_query.single();

    firing_timer.set_mode(TimerMode::Once);
    firing_timer.set_duration(Duration::from_secs_f32(weapon_stats.attack_speed));

    for event in &mut attack_event_reader.read() {
        // info!("firing duration: {:#?}", firing_timer.duration());
        if firing_timer.finished() {
            attacks::create_bullet(&mut cmds, &assets, event, weapon_stats);
            firing_timer.reset();
            // info!("fire timer finished");
            return;
        }
    }
}

// TODO: merge both damage application systems into single system that sends an event for player deaths
// TODO: have damaged enemies use particle effect or red tint when damaged
// TODO: make this a damage queue
/// takes damaged entity's and applies damage too hit enemy
#[allow(clippy::type_complexity)]
fn deal_with_damaged(
    mut cmds: Commands,
    mut game_info: ResMut<PlayerGameInformation>,
    mut damaged_query: Query<
        (&mut ActorCombatStats, Entity, &Damage),
        (Added<Damage>, With<Enemy>),
    >,
) {
    screen_print!("{:#?}", game_info);
    for (mut enemy_stats, enemy, damage) in &mut damaged_query {
        game_info.damage_dealt += damage.0;
        enemy_stats.health -= damage.0;
        cmds.entity(enemy).remove::<Damage>();

        if enemy_stats.health <= 0.0 {
            cmds.entity(enemy).despawn_recursive();
            game_info.enemies_killed += 1;
        }
    }
}

/// deals with player deaths
fn player_death_system(
    mut cmds: Commands,
    mut game_info: ResMut<PlayerGameInformation>,
    mut player_query: Query<
        (
            &mut ActorCombatStats,
            Entity,
            &Damage,
            &mut Transform,
            &mut TextureAtlasSprite,
        ),
        With<Player>,
    >,
) {
    if player_query.is_empty() {
        return;
    }
    let (mut player_stats, player, player_damaged, mut player_loc, mut player_sprite) =
        player_query
            .get_single_mut()
            .expect("Should only ever have 1 player. Player probably didnt exist");

    game_info.damage_taken += player_damaged.0;
    if player_stats.health <= 0.0 {
        warn!("player is dead");
        player_stats.health = 150.0;
        *player_loc = Transform::from_translation(Vec3::new(-60.0, 1090.0, ACTOR_Z_INDEX));
        game_info.player_deaths += 1;
    }

    let old_color = player_sprite.color;
    player_sprite.color = Color::RED;

    player_stats.health -= player_damaged.0;
    cmds.entity(player).remove::<Damage>();
    player_sprite.color = old_color;
}
