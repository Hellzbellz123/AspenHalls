pub mod components;

use std::{f32::consts::FRAC_PI_2, time::Duration};

use bevy::{math::vec2, prelude::*};

use bevy_debug_text_overlay::screen_print;
use bevy_rapier2d::prelude::{
    ActiveEvents, Collider, ColliderMassProperties, CollisionEvent, CollisionGroups, Damping,
    Friction, Group, LockedAxes, Restitution, RigidBody, Sensor, Velocity,
};
use leafwing_input_manager::prelude::ActionState;

use crate::{
    action_manager::actions::PlayerActions,
    actors::{
        player::actions::PlayerShootEvent,
        weapons::components::{
            CurrentlySelectedWeapon, WeaponSlots, WeaponSocket, WeaponStats, WeaponTag,
        },
    },
    components::actors::{
        ai::AIEnemy,
        animation::FacingDirection,
        bundles::{
            EnemyColliderTag, PlayerProjectileBundle, PlayerProjectileColliderBundle,
            PlayerProjectileColliderTag, PlayerProjectileTag, RigidBodyBundle,
        },
        general::{DefenseStats, MovementState, Player, ProjectileStats, TimeToLive},
    },
    game::{GameStage, TimeInfo},
    loading::assets::ActorTextureHandles,
    utilities::{
        game::{
            SystemLabels, ACTOR_PHYSICS_Z_INDEX, ACTOR_Z_INDEX, BULLET_SPEED_MODIFIER,
            PLAYER_PROJECTILE_LAYER,
        },
        lerp, EagerMousePos,
    },
};

#[derive(Debug, Clone, Copy, Resource)]
pub struct EnemyCounterInformation {
    pub enemys_killed: i32,
    pub damage_dealt: f32,
}

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemyCounterInformation {
            enemys_killed: 0,
            damage_dealt: 0.0,
        })
            .insert_resource(WeaponFiringTimer::default())
            .add_system_to_stage(CoreStage::PreUpdate, remove_cdw_componenet)
            .add_system_set(
                SystemSet::on_update(GameStage::Playing)
                    .with_system(rotate_player_weapon)
                    .with_system(weapon_visiblity_system)
                    .with_system(update_equipped_weapon)
                    .with_system(keep_player_weapons_centered)
                    .with_system(shoot_weapon)
                    .with_system(detect_bullet_hits_on_enemy)
                    .with_system(remove_dead_enemys)
                    .after(SystemLabels::Spawn),
            );
    }
}

#[derive(Component, Default, Reflect, Deref, DerefMut, Resource)]
#[reflect(Component)]
pub struct WeaponFiringTimer(pub Timer);

fn rotate_player_weapon(
    gametime: Res<TimeInfo>,
    eager_mouse: Res<EagerMousePos>,
    mut player_query: Query<(&mut MovementState, With<Player>)>,

    #[allow(clippy::type_complexity)]
    // trunk-ignore(clippy/type_complexity)
    mut weapon_query: Query<
        // this is equivelent to if player has a weapon equipped and out
        (&mut WeaponTag, &GlobalTransform, &mut Transform),
        (With<Parent>, With<CurrentlySelectedWeapon>, Without<Player>),
    >,
) {
    if gametime.game_paused || weapon_query.is_empty() {
        return;
    }
    let gmouse = eager_mouse.world;

    for (wtag, wgtransform, mut wtransform) in weapon_query.iter_mut() {
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
    }
}

fn keep_player_weapons_centered(
    mut player_query: Query<(&mut MovementState, With<Player>)>,

    #[allow(clippy::type_complexity)]
    // trunk-ignore(clippy/type_complexity)
    mut weapon_query: Query<
        // this is equivelent to if player has a weapon equipped and out
        (&mut WeaponTag, &mut Transform, &mut Velocity),
        (With<Parent>, Without<Player>),
    >,
) {
    if weapon_query.is_empty() {
        return;
    }

    for (wtag, mut wtransform, mut wvelocity) in weapon_query.iter_mut() {
        if wtag.parent.is_some() {
            let (playerstate, _) = player_query.single_mut();
            // modify weapon sprite to be below player when facing up, this still looks strange but looks better than a back mounted smg
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
    }
}

// check if the weapon is supposed to be visible
fn weapon_visiblity_system(
    player_query: Query<&WeaponSocket, With<Player>>,
    mut weapon_query: Query<(&WeaponTag, &mut Visibility), With<Parent>>, // query weapons parented to entitys
) {
    let p_weaponsocket = player_query.single();
    for (wtag, mut wvisiblity) in weapon_query.iter_mut() {
        if wtag.stored_weapon_slot == Some(p_weaponsocket.drawn_slot) {
            wvisiblity.is_visible = true;
        } else {
            wvisiblity.is_visible = false;
        }
    }
}

/// removes CurrentlyDrawnWeapon from entitys parented to player that dont match the entity in Weaponsocket.drawn_weapon
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

    for (went, wtag) in weapon_query.iter() {
        if wtag.stored_weapon_slot != Some(wsocket.drawn_slot) && drawn_weapon.get(went).is_ok() {
            let wname = names.get(went).expect("entity doesnt have a name");
            debug!(
                "weapon {} {:#?} shouldnt have active component, removing",
                wname, went
            );
            cmds.entity(went).remove::<CurrentlySelectedWeapon>();
        }
    }
}

fn update_equipped_weapon(
    mut cmds: Commands,
    query_action_state: Query<&ActionState<PlayerActions>>,
    mut player_query: Query<(Entity, &mut WeaponSocket, &mut Transform), With<Player>>,

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

    let (_ent, mut wsocket, _transform) = player_query.single_mut();
    let actions = query_action_state.single();

    // TODO: this mostly works, but we need to have a system that checks if the current equippped weapon has a CurrentlyDrawnWeapon and adds it if it doesnt
    // a default is basically what we need, so a weapon is always out if available i guess
    if actions.just_pressed(PlayerActions::EquipSlot1) {
        // set whatever weapon is in slot 1 as CurrentlyDrawnWeapon and remove CurrentlyDrawnWeapon from old weapon
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
    mut attackreader: EventReader<PlayerShootEvent>,
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
            create_bullet(&mut cmds, &assets, event, wstats);
            fireingtimer.reset();
            // info!("fire timer finished");
            return;
        } else {
            // info!("fire timer not finished");
        }
    }
}

fn create_bullet(
    cmds: &mut Commands,
    assets: &ResMut<ActorTextureHandles>,
    event: &PlayerShootEvent,
    wstats: &WeaponStats,
) {
    cmds.spawn((
        PlayerProjectileBundle {
            name: Name::new("PlayerProjectile"),
            tag: PlayerProjectileTag,
            projectile_stats: ProjectileStats {
                damage: wstats.damage,
                speed: wstats.bullet_speed,
                size: wstats.projectile_size,
            },
            ttl: TimeToLive(Timer::from_seconds(2.0, TimerMode::Repeating)),
            sprite_bundle: SpriteBundle {
                texture: assets.bevy_icon.clone(),
                transform: Transform::from_translation(
                    event.bullet_spawn_loc, //- Vec3 { x: 0.0, y: -5.0, z: 0.0 },
                ),
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(wstats.projectile_size)),
                    ..default()
                },
                ..default()
            },
            rigidbody_bundle: RigidBodyBundle {
                velocity: Velocity::linear(
                    event.travel_dir * (wstats.bullet_speed * BULLET_SPEED_MODIFIER),
                ),
                rigidbody: RigidBody::Dynamic,
                friction: Friction::coefficient(0.7),
                howbouncy: Restitution::coefficient(0.3),
                massprop: ColliderMassProperties::Density(0.3),
                rotationlocks: LockedAxes::ROTATION_LOCKED,
                dampingprop: Damping {
                    linear_damping: 1.0,
                    angular_damping: 1.0,
                },
            },
        },
        Sensor,
    ))
    .with_children(|child| {
        child.spawn((
            PlayerProjectileColliderBundle {
                name: Name::new("PlayerProjectileCollider"),
                transformbundle: TransformBundle {
                    local: (Transform {
                        translation: (Vec3 {
                            x: 0.,
                            y: 0.,
                            z: ACTOR_PHYSICS_Z_INDEX,
                        }),
                        ..default()
                    }),
                    ..default()
                },
                collider: Collider::ball(3.0),
                tag: crate::components::actors::bundles::PlayerProjectileColliderTag,
                collisiongroups: CollisionGroups::new(
                    PLAYER_PROJECTILE_LAYER,
                    Group::from_bits_truncate(0b00101),
                ),
                ttl: TimeToLive(Timer::from_seconds(2.0, TimerMode::Repeating)),
            },
            ActiveEvents::COLLISION_EVENTS,
        ));
    });
}

pub fn detect_bullet_hits_on_enemy(
    mut cmds: Commands,
    name_query: Query<&Name>,
    mut collision_events: EventReader<CollisionEvent>,
    enemycollider_query: Query<(Entity, &Parent), With<EnemyColliderTag>>,
    playerprojectilecollider_query: Query<(Entity, &Parent), With<PlayerProjectileColliderTag>>,
    mut enemy_query: Query<(&mut DefenseStats, Entity), With<AIEnemy>>,
    playerprojectile_query: Query<&ProjectileStats, With<PlayerProjectileTag>>,
) {
    for event in collision_events.iter() {
        if let CollisionEvent::Started(a, b, _flags) = event {
            let noname = Name::new("no name on ent a");
            let aname = name_query.get(*a).unwrap_or(&noname);
            let bname = name_query.get(*b).unwrap_or(&noname);
            info!("{}{:?} started colliding with {}{:?}", bname, b, aname, a,);

            if playerprojectilecollider_query.get(*a).is_ok() | enemycollider_query.get(*b).is_ok()
            {
                let projcollider = playerprojectilecollider_query.get(*a);
                let enemycollider = enemycollider_query.get(*b);

                if let Ok((_proj, parent)) = projcollider {
                    let projparent = parent.get();
                    if let Ok((_ent, enemy)) = enemycollider {
                        let hitenemy = enemy_query.get_mut(enemy.get());
                        let proj = playerprojectile_query.get(parent.get());

                        if let Ok((mut hitenemystats, _hitent)) = hitenemy {
                            if let Ok(stats) = proj {
                                hitenemystats.health -= stats.damage;
                                cmds.entity(projparent).despawn_recursive();
                                info!(
                                    "hit enemy and took {} from hp: {}",
                                    stats.damage, hitenemystats.health
                                )
                            }
                        }
                    }
                }
            }
            if enemycollider_query.get(*a).is_ok() | playerprojectilecollider_query.get(*b).is_ok()
            {
                let projcollider = playerprojectilecollider_query.get(*b);
                let enemycollider = enemycollider_query.get(*a);

                if let Ok((_proj, parent)) = projcollider {
                    let projparent = parent.get();
                    if let Ok((_ent, enemy)) = enemycollider {
                        let hitenemy = enemy_query.get_mut(enemy.get());
                        let proj = playerprojectile_query.get(parent.get());

                        if let Ok((mut hitenemystats, _hitent)) = hitenemy {
                            if let Ok(stats) = proj {
                                hitenemystats.health -= stats.damage;
                                cmds.entity(projparent).despawn_recursive();
                                info!(
                                    "hit enemy and took {} from hp: {}",
                                    stats.damage, hitenemystats.health
                                )
                            }
                        }
                    }
                }
            };
        }
    }
    collision_events.clear();
}

pub fn remove_dead_enemys(
    mut enemy_stats: ResMut<EnemyCounterInformation>,
    enemy_query: Query<(&mut DefenseStats, Entity), With<AIEnemy>>,
    mut cmds: Commands,
) {
    screen_print!("{:#?}", enemy_stats);

    if enemy_query.is_empty() {
        return;
    }

    for (enemystats, ent) in enemy_query.iter() {
        if enemystats.health <= 0.0 {
            cmds.entity(ent).despawn_recursive();
            enemy_stats.enemys_killed += 1
        }
    }
}

// TODO: add system that find enemies with no health and despawns them,
