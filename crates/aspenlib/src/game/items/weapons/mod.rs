use bevy::prelude::*;
use bevy_rapier2d::{pipeline::CollisionEvent, prelude::Velocity};

use crate::{
    game::{
        attributes_stats::EquipmentStats,
        items::weapons::{
            components::{
                AttackDamage, CurrentAmmo, CurrentlyDrawnWeapon, WeaponCarrier, WeaponDescriptor,
                WeaponHolder, WeaponTimers,
            },
            forms::GunShootEvent,
            hit_detection::projectile_hits,
        },
        AppState,
    },
    loading::registry::RegistryIdentifier,
};

/// combat related components
pub mod components;
/// different weapon attack types
pub mod forms;
/// projectiles from weapon hit detection
/// applies damage too hit entity if its a character else despawns
mod hit_detection;
/// holds different utils, currently weapon spawn, to be split into gun spawn,...,etc...
pub mod utils;

/// plugin for all actor weapon functionality
pub struct WeaponItemPlugin;

impl Plugin for WeaponItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(forms::GunWeaponsPlugin);

        app.register_type::<AttackDamage>()
            .register_type::<EquipmentStats>();

        app.add_event::<EventAttackWeapon>()
            .add_systems(
                PreUpdate,
                update_selected_weapon.run_if(state_exists_and_equals(AppState::PlayingGame)),
            )
            .add_systems(PreUpdate, prepare_weapons)
            .add_systems(
                Update,
                (
                    projectile_hits.run_if(on_event::<CollisionEvent>()),
                    handle_weapon_attacks.run_if(on_event::<EventAttackWeapon>()),
                    flip_weapon_sprites,
                    equipped_weapon_positioning,
                    weapon_visibility_system,
                )
                    .run_if(state_exists_and_equals(AppState::PlayingGame)),
            );
    }
}

/// adds needed components too weapons that are missing
fn prepare_weapons(
    mut cmds: Commands,
    weapon_query: Query<
        (
            Entity,
            &WeaponDescriptor,
            &WeaponHolder,
            &RegistryIdentifier,
        ),
        Added<WeaponDescriptor>,
    >,
) {
    for (entity, weapon_descriptor, _, _) in &weapon_query {
        match weapon_descriptor {
            WeaponDescriptor::Gun(cfg) => {
                cmds.entity(entity).insert((
                    WeaponTimers {
                        attack: Timer::from_seconds(cfg.fire_rate, TimerMode::Once),
                        refill: Timer::from_seconds(cfg.reload_time, TimerMode::Once),
                        duration: Timer::from_seconds(cfg.fire_rate, TimerMode::Once),
                    },
                    CurrentAmmo {
                        count: cfg.max_ammo,
                        max: cfg.max_ammo,
                    },
                ));
                warn!("gun weapons are unfinished");
            } // WeaponDescriptor::Flail { .. } => {
              //     warn!("flail weapons are unimplemented");
              // }
              // WeaponDescriptor::Blade { .. } => {
              //     warn!("blade weapons are unimplemented");
              // }
        }
    }
}

/// gets weapon attack requests and sends attack event based on descriptor
fn handle_weapon_attacks(
    mut gunshoot_events: EventWriter<GunShootEvent>,
    mut weapon_attack_events: EventReader<EventAttackWeapon>,
    weapon_query: Query<(&WeaponDescriptor, &WeaponHolder), With<Parent>>,
) {
    for weapon_attack in weapon_attack_events.read() {
        let Ok((weapon_descriptor, _)) = weapon_query.get(weapon_attack.weapon) else {
            warn!("invalid weapon");
            continue;
        };

        match weapon_descriptor {
            WeaponDescriptor::Gun(cfg) => gunshoot_events.send(GunShootEvent {
                gun: weapon_attack.weapon,
                settings: *cfg,
            }),
            // WeaponDescriptor::Flail { .. } => {}
            // WeaponDescriptor::Blade { .. } => {}
        }
    }
}

/// event sent when character attacks with weapon
#[derive(Debug, Event)]
pub struct EventAttackWeapon {
    /// who attacked with this weapon
    pub requester: Entity,
    /// weapon used for attack
    pub weapon: Entity,
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

            // if weapon sprite angle is not between -90..90, mirror y axis of sprite
            sprite.flip_y = !(-90.0..=90.0).contains(&degrees);
        }
    }
}

/// keeps all weapons centered too parented entity
#[allow(clippy::type_complexity)]
fn equipped_weapon_positioning(
    children: Query<&Children>,
    // actors that can equip weapons
    characters: Query<Entity, With<WeaponCarrier>>,
    mut weapon_query: Query<
        // all weapons equipped too entity
        (&mut Transform, &mut Velocity),
        (With<WeaponHolder>, Without<WeaponCarrier>),
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
    carrier_query: Query<&WeaponCarrier>,
    mut weapon_query: Query<(&WeaponHolder, &mut Visibility, Option<&Parent>)>,
) {
    for (weapon_holder, mut weapon_visibility, parent) in &mut weapon_query {
        if let Some((weapon_slot, weapon_holder)) = weapon_holder.0
            && let Some(parent) = parent
        {
            let parent = parent.get();
            if parent != weapon_holder {
                warn!("weapon is parented incorrectly");
            }
            let Ok(weapon_socket) = carrier_query.get(parent) else {
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
    weapon_carry_actors: Query<(Entity, &WeaponCarrier), Changed<WeaponCarrier>>,
    selected_weapon: Query<&CurrentlyDrawnWeapon>,
) {
    for (_actor, socket) in &weapon_carry_actors {
        if let Some(drawn_slot) = socket.drawn_slot {
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
        } else {
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
        };
    }
}
