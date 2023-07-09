#![allow(clippy::type_complexity)]
/// different attacks that can exist in the game
mod attacks;
/// combat related components
pub mod components;
/// hit detection for bullets
mod hit_detection;

use std::{f32::consts::FRAC_PI_2, time::Duration};

use bevy::{math::vec2, prelude::*};

use bevy_debug_text_overlay::screen_print;

use bevy_rapier2d::prelude::Velocity;
use leafwing_input_manager::prelude::ActionState;

use crate::{
    consts::ACTOR_Z_INDEX,
    game::{
        actors::{
            combat::components::{
                CurrentlySelectedWeapon, WeaponSlots, WeaponSocket, WeaponStats, WeaponTag,
            },
            player::actions::ShootEvent,
        },
        input::actions,
    },
    game::{GameStage, TimeInfo},
    loading::assets::ActorTextureHandles,
    utilities::{lerp, EagerMousePos},
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
    /// enemys killed by player
    pub enemys_killed: i32,
    /// damage enemys deal too player
    pub damage_taken: f32,
    /// times player has died
    pub player_deaths: i32,
    /// amount of damage enemys have fired that hit player and didnt get counted
    pub enemy_damage_sent: f32,
    /// amount of damage player have fired that hit enemy and didnt get counted
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
            enemys_killed: 0,
            enemy_damage_sent: 0.0,
            player_deaths: 0,
        })
        .insert_resource(WeaponFiringTimer::default())
        .add_system(remove_cdw_componenet.in_base_set(CoreSet::PreUpdate))
        .add_system(deal_with_damaged.in_base_set(CoreSet::PostUpdate))
        .add_systems(
            (
                update_equipped_weapon,
                player_death_system,
                hit_detection::hits_on_enemy,
                hit_detection::hits_on_player,
                rotate_player_weapon,
                keep_player_weapons_centered,
                weapon_visiblity_system,
                receive_shoot_weapon,
            )
                .in_set(OnUpdate(GameStage::PlayingGame)),
        );
    }
}

/// weapon firing timer
/// basically firing hammer time
#[derive(Component, Default, Reflect, Deref, DerefMut, Resource)]
#[reflect(Component)]
pub struct WeaponFiringTimer(pub Timer);

/// rotates weapon too face wherever the players mouse is
fn rotate_player_weapon(
    gametime: Res<TimeInfo>,
    eager_mouse: Res<EagerMousePos>,
    mut player_query: Query<(&AnimState, With<Player>)>,
    mut weapon_query: Query<
        // this is equivelent to if player has a weapon equipped and out
        (&WeaponTag, &GlobalTransform, &mut Transform),
        (With<Parent>, With<CurrentlySelectedWeapon>),
    >,
) {
    if gametime.game_paused || weapon_query.is_empty() {
        return;
    }
    let gmouse = eager_mouse.world;

    weapon_query.for_each_mut(|(wtag, wgtransform, mut wtransform)| {
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
            wtransform.rotation = Quat::from_euler(EulerRot::ZYX, aimangle, 0.0, 0.0);
        }
    });
}

/// keeps all weapons centered too parented entity
fn keep_player_weapons_centered(
    // actors that can equip weapons
    mut actor_query: Query<((Entity, &AnimState, &Children), With<WeaponSocket>)>,
    mut weapon_query: Query<
        // all weapons equipped too entity
        (Entity, &WeaponTag, &mut Transform, &mut Velocity),
        (With<Parent>, Without<ActorType>),
    >,
) {
    if weapon_query.is_empty() || actor_query.is_empty() {
        return;
    }

    actor_query.for_each_mut(|((_ent, animstate, childs), _)| {
        weapon_query.for_each_mut(|(went, wtag, mut wtransform, mut wvelocity)| {
            if wtag.parent.is_some() && childs.contains(&went) {
                wvelocity.angvel = lerp(wvelocity.angvel, 0.0, 0.3);
                wvelocity.linvel = Vec2::ZERO;
                // modify weapon sprite to be below player when facing up, this
                // still looks strange but looks better than a back mounted smg
                if animstate.facing != ActorAnimationType::Up {
                    // this transform is local too players transform of 8
                    wtransform.translation = Vec3 {
                        x: 0.0,
                        y: 1.5,
                        z: 1.0,
                    }
                } else {
                    // this transform is local too players transform of 8
                    wtransform.translation = Vec3 {
                        x: 0.0,
                        y: 1.5,
                        z: -1.0,
                    }
                }
            }
        });
    });
}

/// check if the weapon is supposed to be visible
fn weapon_visiblity_system(
    player_query: Query<&WeaponSocket, With<Player>>,
    mut weapon_query: Query<(&WeaponTag, &mut Visibility), With<Parent>>, // query weapons parented to entitys
) {
    if player_query.is_empty() || weapon_query.is_empty() {
        return;
    }

    let p_weaponsocket = player_query.single();
    weapon_query.for_each_mut(|(wtag, mut wvisiblity)| {
        if wtag.stored_weapon_slot == p_weaponsocket.drawn_slot {
            // TODO: these feels wrong, deref doesnt feel correct here
            // find a less gross solution
            *wvisiblity = Visibility::Inherited
        } else {
            *wvisiblity = Visibility::Hidden
        }
    });
}

/// removes `CurrentlyDrawnWeapon` from entitys parented to player that dont
/// match the entity in `Weaponsocket.drawn_weapon`
fn remove_cdw_componenet(
    mut cmds: Commands,
    names: Query<&Name>,
    player_query: Query<&WeaponSocket, With<Player>>,

    drawn_weapon: Query<&CurrentlySelectedWeapon>,
    weapon_query: Query<
        (Entity, &WeaponTag),
        (With<Parent>, With<CurrentlySelectedWeapon>, Without<Player>),
    >,
) {
    if player_query.is_empty() | weapon_query.is_empty() | drawn_weapon.is_empty() {
        return;
    }

    let playerwsocket = player_query.single();

    weapon_query.for_each(|(went, wtag)| {
        if wtag.stored_weapon_slot != playerwsocket.drawn_slot && drawn_weapon.get(went).is_ok() {
            let wname = names.get(went).expect("entity doesnt have a name");
            debug!(
                "weapon {} {:#?} shouldnt have active component, removing",
                wname, went
            );
            cmds.entity(went).remove::<CurrentlySelectedWeapon>();
        }
    });
}

/// updates players equipped weapon based on input
fn update_equipped_weapon(
    mut cmds: Commands,
    query_action_state: Query<&ActionState<actions::Combat>>,
    mut player_query: Query<&mut WeaponSocket, With<Player>>,
    weapon_query: Query<(Entity, &mut WeaponTag, &mut Transform), (With<Parent>, Without<Player>)>,
) {
    if player_query.is_empty() | weapon_query.is_empty() | query_action_state.is_empty() {
        return;
    }

    let mut wsocket = player_query.single_mut();
    let actions = query_action_state.single();

    if actions.just_pressed(actions::Combat::EquipSlot1) {
        // set whatever weapon is in slot 1 as CurrentlyDrawnWeapon and remove
        // CurrentlyDrawnWeapon from old weapon
        wsocket.drawn_slot = Some(WeaponSlots::Slot1);
        let current_weapon_slots = &mut wsocket.weapon_slots.clone();
        let current_weapon = get_current_weapon(current_weapon_slots, &wsocket);

        if let Some(ent) = current_weapon {
            cmds.entity(ent).insert(CurrentlySelectedWeapon);
            info!("equipping slot 1")
        }
    } else if actions.just_pressed(actions::Combat::EquipSlot2) {
        wsocket.drawn_slot = Some(WeaponSlots::Slot2);
        let current_weapon_slots = &mut wsocket.weapon_slots.clone();
        let newwep = get_current_weapon(current_weapon_slots, &wsocket);

        if let Some(ent) = newwep {
            cmds.entity(ent).insert(CurrentlySelectedWeapon);
            info!("equipping slot 2")
        }
    } else if actions.just_pressed(actions::Combat::EquipSlot3) {
        wsocket.drawn_slot = Some(WeaponSlots::Slot3);
        let current_weapon_slots = &mut wsocket.weapon_slots.clone();
        let newwep = get_current_weapon(current_weapon_slots, &wsocket);

        if let Some(ent) = newwep {
            cmds.entity(ent).insert(CurrentlySelectedWeapon);
            info!("equipping slot 3")
        }
    } else if actions.just_pressed(actions::Combat::EquipSlot4) {
        wsocket.drawn_slot = Some(WeaponSlots::Slot4);
        let current_weapon_slots = &mut wsocket.weapon_slots.clone();
        let newwep = get_current_weapon(current_weapon_slots, &wsocket);

        if let Some(ent) = newwep {
            cmds.entity(ent).insert(CurrentlySelectedWeapon);
            info!("equipping slot 4")
        }
    }
}

/// gets ent id of weapon in weapon slot
fn get_current_weapon(
    weaponslots: &mut bevy::utils::hashbrown::HashMap<WeaponSlots, Option<Entity>>,
    wsocket: &WeaponSocket,
) -> Option<Entity> {
    let entity_in_drawn_slot = weaponslots
        .entry(
            wsocket
                .drawn_slot
                .expect("failed to unwrap wsocket.drawnslot"),
        )
        .or_insert(None);
    let currently_equipped_from_hashmap: Option<Entity> = entity_in_drawn_slot
        .as_mut()
        .map(|current_equiped_weapon| *current_equiped_weapon);

    match currently_equipped_from_hashmap {
        Some(weapon) => Some(weapon),
        None => {
            warn!("no currently equipped weapon");
            None
        }
    }
}

// TODO: refactor this system and related systems into a module for weapons, add ammo management too new module
/// recives shoot events and creates bullet at location
pub fn receive_shoot_weapon(
    mut cmds: Commands,
    time: Res<Time>,
    assets: ResMut<ActorTextureHandles>,
    mut fireingtimer: ResMut<WeaponFiringTimer>,
    mut attackreader: EventReader<ShootEvent>,
    weapon_query: Query<
        // this is equivelent to if player has a weapon equipped and out
        (&mut WeaponTag, &WeaponStats, &Transform),
        (With<Parent>, With<CurrentlySelectedWeapon>),
    >,
) {
    fireingtimer.tick(time.delta());

    if weapon_query.is_empty() | attackreader.is_empty() {
        return;
    }

    let fireingtimer = &mut fireingtimer.0;
    let (_wtag, wstats, _wtrans) = weapon_query.single();

    fireingtimer.set_mode(TimerMode::Once);
    fireingtimer.set_duration(Duration::from_secs_f32(wstats.attack_speed));

    for event in attackreader.iter() {
        // info!("firing duration: {:#?}", fireingtimer.duration());
        if fireingtimer.finished() {
            attacks::create_bullet(&mut cmds, &assets, event, wstats);
            fireingtimer.reset();
            // info!("fire timer finished");
            return;
        }
    }
}
// TODO: merge both damage application systems into single system that sends an event for player deaths
// TODO: have damaged enemys use particle effect or red tint when damaged

// TODO: make this a damage queue
/// takes damaged entitys and applies damage too hit enemy
fn deal_with_damaged(
    mut cmds: Commands,
    mut game_info: ResMut<PlayerGameInformation>,
    mut damaged_query: Query<
        (&mut ActorCombatStats, Entity, &Damage),
        (Added<Damage>, With<Enemy>),
    >,
) {
    screen_print!("{:#?}", game_info);
    damaged_query.for_each_mut(|(mut enemy_stats, enemy, damage)| {
        game_info.damage_dealt += damage.0;
        enemy_stats.health -= damage.0;
        cmds.entity(enemy).remove::<Damage>();

        if enemy_stats.health <= 0.0 {
            cmds.entity(enemy).despawn_recursive();
            game_info.enemys_killed += 1;
        }
    });
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
        player_query.get_single_mut().unwrap();

    game_info.damage_taken += player_damaged.0;
    if player_stats.health <= 0.0 {
        warn!("player is dead");
        player_stats.health = 150.0;
        *player_loc = Transform::from_translation(Vec3::new(-60.0, 1090.0, ACTOR_Z_INDEX));
        game_info.player_deaths += 1;
    }

    let oldcolor = player_sprite.color;
    player_sprite.color = Color::RED;

    player_stats.health -= player_damaged.0;
    cmds.entity(player).remove::<Damage>();
    player_sprite.color = oldcolor;
}
