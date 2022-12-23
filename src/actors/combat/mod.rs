mod attacks;
pub mod components;

use std::{f32::consts::FRAC_PI_2, time::Duration};

use bevy::{math::vec2, prelude::*};

use bevy_debug_text_overlay::screen_print;
use bevy_rapier2d::prelude::{CollisionEvent, Velocity};
use leafwing_input_manager::prelude::ActionState;

use crate::{
    actions::PlayerActions,
    actors::{
        combat::components::{
            CurrentlySelectedWeapon, WeaponSlots, WeaponSocket, WeaponStats, WeaponTag,
        },
        player::actions::ShootEvent,
    },
    components::actors::{
        ai::AIEnemy,
        animation::FacingDirection,
        bundles::{
            EnemyColliderTag, EnemyProjectileColliderTag, EnemyProjectileTag, PlayerColliderTag,
            PlayerProjectileColliderTag, PlayerProjectileTag,
        },
        general::{DefenseStats, MovementState, Player, ProjectileStats},
    },
    game::{GameStage, TimeInfo},
    loading::assets::ActorTextureHandles,
    utilities::{game::ACTOR_Z_INDEX, lerp, EagerMousePos},
};

use self::components::Damaged;

#[derive(Debug, Clone, Copy, Resource)]
pub struct PlayerGameInformation {
    pub damage_dealt: f32,
    pub enemys_killed: i32,
    pub damage_taken: f32,
    pub player_deaths: i32,
    pub enemy_damage_sent: f32,
    pub player_damage_sent: f32,
}

#[derive(SystemLabel)]
pub enum CombatSystemOrders {
    Sysone,
    Systwo,
    Systhree,
}

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
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
        .add_system_to_stage(CoreStage::PreUpdate, remove_cdw_componenet)
        .add_system_to_stage(CoreStage::PreUpdate, deal_with_damaged)
        .add_system_set(
            SystemSet::on_update(GameStage::Playing)
                .with_system(detect_bullet_hits_on_enemy)
                .with_system(detect_bullet_hits_on_player)
                .with_system(rotate_player_weapon)
                .with_system(keep_player_weapons_centered)
                .with_system(weapon_visiblity_system)
                .with_system(update_equipped_weapon)
                .with_system(shoot_weapon)
                .with_system(player_death_system),
        );
    }
}

#[derive(Component, Default, Reflect, Deref, DerefMut, Resource)]
#[reflect(Component)]
pub struct WeaponFiringTimer(pub Timer);

fn rotate_player_weapon(
    gametime: Res<TimeInfo>,
    eager_mouse: Res<EagerMousePos>,
    mut player_query: Query<(&MovementState, With<Player>)>,

    #[allow(clippy::type_complexity)]
    // trunk-ignore(clippy/type_complexity)
    mut weapon_query: Query<
        // this is equivelent to if player has a weapon equipped and out
        (&WeaponTag, &GlobalTransform, &mut Transform),
        (With<Parent>, With<CurrentlySelectedWeapon>, Without<Player>),
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
            *wtransform.rotation = *(Quat::from_euler(EulerRot::ZYX, aimangle, 0.0, 0.0));
        }
    });
}

fn keep_player_weapons_centered(
    mut player_query: Query<(&MovementState, With<Player>)>,

    #[allow(clippy::type_complexity)]
    // trunk-ignore(clippy/type_complexity)
    mut weapon_query: Query<
        // this is equivelent to if player has a weapon equipped and out
        (&WeaponTag, &mut Transform, &mut Velocity),
        (With<Parent>, Without<Player>),
    >,
) {
    if weapon_query.is_empty() {
        return;
    }

    weapon_query.for_each_mut(|(wtag, mut wtransform, mut wvelocity)| {
        if wtag.parent.is_some() {
            let (playerstate, _) = player_query.single_mut();
            // modify weapon sprite to be below player when facing up, this
            // still looks strange but looks better than a back mounted smg
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
                    z: ACTOR_Z_INDEX,
                }
            }
            wvelocity.angvel = lerp(wvelocity.angvel, 0.0, 0.3);
        }
    });
}

// check if the weapon is supposed to be visible
fn weapon_visiblity_system(
    player_query: Query<&WeaponSocket, With<Player>>,
    mut weapon_query: Query<(&WeaponTag, &mut Visibility), With<Parent>>, // query weapons parented to entitys
) {
    let p_weaponsocket = player_query.single();
    weapon_query.for_each_mut(|(wtag, mut wvisiblity)| {
        if wtag.stored_weapon_slot == Some(p_weaponsocket.drawn_slot) {
            wvisiblity.is_visible = true;
        } else {
            wvisiblity.is_visible = false;
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
    #[allow(clippy::type_complexity)]
    // trunk-ignore(clippy/type_complexity)
    weapon_query: Query<
        (Entity, &WeaponTag),
        (With<Parent>, With<CurrentlySelectedWeapon>, Without<Player>),
    >,
) {
    if player_query.is_empty() | weapon_query.is_empty() | drawn_weapon.is_empty() {
        return;
    }

    let wsocket = player_query.single();

    weapon_query.for_each(|(went, wtag)| {
        if wtag.stored_weapon_slot != Some(wsocket.drawn_slot) && drawn_weapon.get(went).is_ok() {
            let wname = names.get(went).expect("entity doesnt have a name");
            debug!(
                "weapon {} {:#?} shouldnt have active component, removing",
                wname, went
            );
            cmds.entity(went).remove::<CurrentlySelectedWeapon>();
        }
    });
}

fn update_equipped_weapon(
    mut cmds: Commands,
    query_action_state: Query<&ActionState<PlayerActions>>,
    mut player_query: Query<&mut WeaponSocket, With<Player>>,

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

    let mut wsocket = player_query.single_mut();
    let actions = query_action_state.single();

    // TODO: this mostly works, but we need to have a system that checks if the
    // current equippped weapon has a CurrentlyDrawnWeapon and adds it if it
    // doesnt a default is basically what we need, so a weapon is always out if
    // available i guess
    if actions.just_pressed(PlayerActions::EquipSlot1) {
        // set whatever weapon is in slot 1 as CurrentlyDrawnWeapon and remove
        // CurrentlyDrawnWeapon from old weapon
        wsocket.drawn_slot = WeaponSlots::Slot1;
        let current_weapon_slots = &mut wsocket.weapon_slots.clone();
        let current_weapon = get_current_weapon(current_weapon_slots, &wsocket);

        if let Some(ent) = current_weapon {
            cmds.entity(ent).insert(CurrentlySelectedWeapon);
            info!("equipping slot 1")
        }
    } else if actions.just_pressed(PlayerActions::EquipSlot2) {
        wsocket.drawn_slot = WeaponSlots::Slot2;
        let current_weapon_slots = &mut wsocket.weapon_slots.clone();
        let newwep = get_current_weapon(current_weapon_slots, &wsocket);

        if let Some(ent) = newwep {
            cmds.entity(ent).insert(CurrentlySelectedWeapon);
            info!("equipping slot 2")
        }
    } else if actions.just_pressed(PlayerActions::EquipSlot3) {
        wsocket.drawn_slot = WeaponSlots::Slot3;
        let current_weapon_slots = &mut wsocket.weapon_slots.clone();
        let newwep = get_current_weapon(current_weapon_slots, &wsocket);

        if let Some(ent) = newwep {
            cmds.entity(ent).insert(CurrentlySelectedWeapon);
            info!("equipping slot 3")
        }
    } else if actions.just_pressed(PlayerActions::EquipSlot4) {
        wsocket.drawn_slot = WeaponSlots::Slot4;
        let current_weapon_slots = &mut wsocket.weapon_slots.clone();
        let newwep = get_current_weapon(current_weapon_slots, &wsocket);

        if let Some(ent) = newwep {
            cmds.entity(ent).insert(CurrentlySelectedWeapon);
            info!("equipping slot 4")
        }
    }
}

fn get_current_weapon(
    weaponslots: &mut bevy::utils::hashbrown::HashMap<WeaponSlots, Option<Entity>>,
    wsocket: &WeaponSocket,
) -> Option<Entity> {
    let entity_in_drawn_slot = weaponslots.entry(wsocket.drawn_slot).or_insert(None);
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

pub fn shoot_weapon(
    mut cmds: Commands,
    time: Res<Time>,
    assets: ResMut<ActorTextureHandles>,
    mut fireingtimer: ResMut<WeaponFiringTimer>,
    mut attackreader: EventReader<ShootEvent>,
    #[allow(clippy::type_complexity)]
    // trunk-ignore(clippy/type_complexity)
    weapon_query: Query<
        // this is equivelent to if player has a weapon equipped and out
        (&mut WeaponTag, &WeaponStats, &Transform),
        (With<Parent>, With<CurrentlySelectedWeapon>, Without<Player>),
    >,
) {
    fireingtimer.tick(time.delta());

    if weapon_query.is_empty() | attackreader.is_empty() {
        return;
    }

    let fireingtimer = &mut fireingtimer.0;
    let (_wtag, wstats, _wtrans) = weapon_query.single();

    fireingtimer.set_mode(TimerMode::Once);
    fireingtimer.set_duration(Duration::from_secs_f32(wstats.firing_speed));

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

// TODO: Make damage dealt an event or component that is inserted onto the
// enemy. have system that takes entities that get that componenet and applies
// damage to them, if the enemys health goes below 0 in that system it should
// add a dead componenet
pub fn detect_bullet_hits_on_enemy(
    mut game_info: ResMut<PlayerGameInformation>,
    mut cmds: Commands,
    projectile_query: Query<&ProjectileStats, With<PlayerProjectileTag>>,
    mut collision_events: EventReader<CollisionEvent>,
    enemycollider_query: Query<(Entity, &Parent), With<EnemyColliderTag>>,
    playerprojectilecollider_query: Query<(Entity, &Parent), With<PlayerProjectileColliderTag>>,
) {
    for event in collision_events.iter() {
        if let CollisionEvent::Started(a, b, _flags) = event {
            let enemy = if enemycollider_query.get(*b).is_ok() {
                let (_collider, parent) = enemycollider_query.get(*b).unwrap();
                parent.get()
            } else if enemycollider_query.get(*a).is_ok() {
                let (_collider, parent) = enemycollider_query.get(*a).unwrap();
                parent.get()
            } else {
                return;
            };
            let projectile = if playerprojectilecollider_query.get(*a).is_ok() {
                let (_a, parent) = playerprojectilecollider_query.get(*a).unwrap();
                parent.get()
            } else if playerprojectilecollider_query.get(*b).is_ok() {
                let (_a, parent) = playerprojectilecollider_query.get(*b).unwrap();
                parent.get()
            } else {
                return;
            };
            let damage = projectile_query.get(projectile).unwrap().damage;

            cmds.entity(projectile).despawn_recursive();
            game_info.player_damage_sent += damage;
            cmds.entity(enemy).insert(Damaged { amount: damage });
        }
    }
    // collision_events.clear();
}

pub fn detect_bullet_hits_on_player(
    mut game_info: ResMut<PlayerGameInformation>,
    mut cmds: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    playercollider_query: Query<(Entity, &Parent), With<PlayerColliderTag>>,
    bad_projectile_query: Query<&ProjectileStats, With<EnemyProjectileTag>>,
    enemyprojectilecollider_query: Query<(Entity, &Parent), With<EnemyProjectileColliderTag>>,
) {
    for event in collision_events.iter() {
        if let CollisionEvent::Started(a, b, _flags) = event {
            let player = if playercollider_query.get(*b).is_ok() {
                let (_collider, parent) = playercollider_query.get(*b).unwrap();
                parent.get()
            } else if playercollider_query.get(*a).is_ok() {
                let (_collider, parent) = playercollider_query.get(*a).unwrap();
                parent.get()
            } else {
                return;
            };
            let projectile = if enemyprojectilecollider_query.get(*a).is_ok() {
                let (_a, parent) = enemyprojectilecollider_query.get(*a).unwrap();
                parent.get()
            } else if enemyprojectilecollider_query.get(*b).is_ok() {
                let (_a, parent) = enemyprojectilecollider_query.get(*b).unwrap();
                parent.get()
            } else {
                return;
            };
            let damage = bad_projectile_query.get(projectile).unwrap().damage;

            cmds.entity(projectile).despawn_recursive();
            game_info.enemy_damage_sent += damage;
            cmds.entity(player).insert(Damaged { amount: damage });
        }
    }
    // collision_events.clear();
}

fn deal_with_damaged(
    mut cmds: Commands,
    mut game_info: ResMut<PlayerGameInformation>,
    mut enemy_query: Query<(&mut DefenseStats, Entity, &Damaged), (Added<Damaged>, With<AIEnemy>)>,
) {
    screen_print!("{:#?}", game_info);
    enemy_query.for_each_mut(|(mut enemy_stats, enemy, damage)| {
        game_info.damage_dealt += damage.amount;
        enemy_stats.health -= damage.amount;
        cmds.entity(enemy).remove::<Damaged>();

        if enemy_stats.health <= 0.0 {
            cmds.entity(enemy).despawn_recursive();
            game_info.enemys_killed += 1;
        }
    });
}

fn player_death_system(
    mut cmds: Commands,
    mut game_info: ResMut<PlayerGameInformation>,
    mut player_query: Query<(&mut DefenseStats, Entity, &Damaged, &mut Transform), With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }
    let (mut player_stats, player, player_damaged, mut player_loc) =
        player_query.get_single_mut().unwrap();

    game_info.damage_taken += player_damaged.amount;
    if player_stats.health <= 0.0 {
        warn!("player is dead");
        player_stats.health = 150.0;
        *player_loc = Transform::from_translation(Vec3::new(-60.0, 1090.0, ACTOR_Z_INDEX));
        game_info.player_deaths += 1;
    }

    player_stats.health -= player_damaged.amount;
    cmds.entity(player).remove::<Damaged>();
}
