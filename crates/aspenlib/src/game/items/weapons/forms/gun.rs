use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    bundles::{ActorColliderBundle, ProjectileBundle, RigidBodyBundle},
    consts::{AspenCollisionLayer, ACTOR_PHYSICS_Z_INDEX},
    game::{
        animations::{EventAnimationChange, GunAnimations},
        attributes_stats::{Damage, ProjectileStats},
        components::{ActorColliderType, TimeToLive},
        items::weapons::components::{
            AttackDamage, CurrentAmmo, CurrentlyDrawnWeapon, GunCfg, WeaponHolder, WeaponTimers,
        },
    },
    loading::assets::AspenInitHandles,
    utilities::EntityCreator,
};

/// holds gun item functionality
pub struct GunWeaponsPlugin;

impl Plugin for GunWeaponsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GunShootEvent>().add_systems(
            Update,
            (
                receive_gun_shots.run_if(on_event::<GunShootEvent>()),
                update_gun_timers,
            ),
        );
    }
}

/// gun should create a bullet
#[derive(Debug, Event)]
pub struct GunShootEvent {
    /// what gun should shoot
    pub gun: Entity,
    /// data used too create bullet
    pub settings: GunCfg,
}

#[allow(clippy::type_complexity)]
/// updates weapon timers
fn update_gun_timers(
    time: Res<Time>,
    mut anim_events: EventWriter<EventAnimationChange>,
    mut weapon_query: Query<
        (Entity, &mut CurrentAmmo, &mut WeaponTimers),
        (With<Parent>, With<CurrentlyDrawnWeapon>),
    >,
) {
    for (weapon, mut current_ammo, mut firing_timers) in &mut weapon_query {
        if current_ammo.current == 0 {
            if firing_timers.refill.remaining_secs() < 0.7 {
                anim_events.send(EventAnimationChange {
                    anim_handle: AnimHandle::from_index(GunAnimations::RELOAD),
                    actor: weapon,
                });
            }
            firing_timers.refill.tick(time.delta());

            if firing_timers.refill.finished() {
                // warn!("finished reloading");
                firing_timers.refill.reset();
                current_ammo.current = current_ammo.max;
            }
        } else {
            firing_timers.attack.tick(time.delta());
        }
    }
}

/// receives shoot events and handles gun reload, then shoots
#[allow(clippy::type_complexity)]
pub fn receive_gun_shots(
    mut cmds: Commands,
    assets: Res<AspenInitHandles>,
    mut gun_shoot_events: EventReader<GunShootEvent>,
    mut anim_events: EventWriter<EventAnimationChange>,
    mut weapon_query: Query<
        (
            Entity,
            &GlobalTransform,
            &mut CurrentAmmo,
            &mut WeaponTimers,
            &WeaponHolder,
            &AttackDamage,
        ),
        (With<Parent>, With<CurrentlyDrawnWeapon>),
    >,
) {
    for event in &mut gun_shoot_events.read() {
        let Ok((weapon, global_transform, mut current_ammo, mut timers, holder, attack)) =
            weapon_query.get_mut(event.gun)
        else {
            error!("invalid gun");
            continue;
        };
        let cfg = event.settings;

        if current_ammo.current == 0 {
            if timers.refill.remaining_secs() < 0.5 {
                anim_events.send(EventAnimationChange {
                    anim_handle: AnimHandle::from_index(GunAnimations::RELOAD),
                    actor: weapon,
                });
            }
            // warn!("reloading");
            continue;
        } else if timers.attack.finished() || current_ammo.current == cfg.max_ammo {
            // info!("bang!");
            anim_events.send(EventAnimationChange {
                anim_handle: AnimHandle::from_index(GunAnimations::FIRE),
                actor: weapon,
            });

            let requester = holder.0.unwrap().1;
            let (_, rotation, translation) = global_transform.to_scale_rotation_translation();
            let offset = rotation * cfg.barrel_end.extend(0.0);
            let transform =
                Transform::from_translation(translation + offset).with_rotation(rotation);

            timers.attack.reset();
            current_ammo.current -= 1;
            create_bullet(
                requester,
                &mut cmds,
                &assets,
                attack,
                transform,
                (cfg.projectile_speed, cfg.projectile_size),
            );
        }
    }
}

/// creates player bullet
pub fn create_bullet(
    entity: Entity,
    cmds: &mut Commands,
    assets: &Res<AspenInitHandles>,
    weapon_damage: &AttackDamage,
    location: Transform,
    speed_size: (f32, f32),
) {
    let projectile_speed = speed_size.0;
    let projectile_size = speed_size.1;

    let (axis, angle) = location.rotation.to_axis_angle();
    let velocity_direction = if axis.z >= 0.0 {
        Vec2::new(angle.cos(), angle.sin())
    } else {
        Vec2::new(angle.cos(), -angle.sin())
    };

    cmds.spawn((
        ProjectileBundle {
            name: Name::new("GunProjectile"),
            projectile_stats: ProjectileStats {
                damage: Damage {
                    physical: weapon_damage.physical,
                    elemental: weapon_damage.elemental,
                },
                entity_that_shot: entity,
            },
            ttl: TimeToLive(Timer::from_seconds(3.5, TimerMode::Repeating)),
            sprite_bundle: SpriteBundle {
                texture: assets.img_favicon.clone(),
                transform: location,
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(projectile_size)),
                    ..default()
                },
                ..default()
            },
            rigidbody_bundle: RigidBodyBundle {
                velocity: Velocity::linear(velocity_direction * projectile_speed),
                rigidbody: RigidBody::Dynamic,
                friction: Friction::coefficient(0.2),
                how_bouncy: Restitution::coefficient(0.8),
                mass_prop: ColliderMassProperties::Density(2.1),
                rotation_locks: LockedAxes::ROTATION_LOCKED,
                damping_prop: Damping {
                    linear_damping: 0.1,
                    angular_damping: 0.1,
                },
            },
        },
        Sensor,
    ))
    .with_children(|child| {
        child.spawn((
            EntityCreator(entity),
            ActorColliderBundle {
                name: Name::new("GunProjectileCollider"),
                transform_bundle: TransformBundle {
                    local: (Transform {
                        translation: Vec2::ZERO.extend(ACTOR_PHYSICS_Z_INDEX),
                        ..default()
                    }),
                    ..default()
                },
                collider: Collider::ball(3.0),
                collision_groups: CollisionGroups::new(
                    AspenCollisionLayer::PROJECTILE,
                    AspenCollisionLayer::WORLD | AspenCollisionLayer::ACTOR,
                ),
                tag: ActorColliderType::Projectile,
            },
            ActiveEvents::COLLISION_EVENTS,
            ActiveHooks::FILTER_CONTACT_PAIRS,
        ));
    });
}

use bevy_asepritesheet::{
    prelude::Spritesheet,
    sprite::{AnimEndAction, AnimHandle},
};

/// format gun animations with proper speed and transitions
pub fn format_gun_animations(sheet: &mut Spritesheet) {
    let handle_idle = sheet.get_anim_handle("idle");
    let handle_wiggle = sheet.get_anim_handle("wiggle");
    let handle_fire = sheet.get_anim_handle("fire");
    let handle_reload = sheet.get_anim_handle("reload");

    if let Ok(anim_idle) = sheet.get_anim_mut(&handle_idle) {
        anim_idle.end_action = AnimEndAction::Loop;
    }
    if let Ok(anim_wiggle) = sheet.get_anim_mut(&handle_wiggle) {
        anim_wiggle.time_scale = 1.0;
        anim_wiggle.end_action = AnimEndAction::Next(handle_idle);
    }
    if let Ok(anim_fire) = sheet.get_anim_mut(&handle_fire) {
        anim_fire.time_scale = 2.0;
        anim_fire.end_action = AnimEndAction::Next(handle_idle);
    }
    if let Ok(anim_reload) = sheet.get_anim_mut(&handle_reload) {
        anim_reload.time_scale = 0.5;
        anim_reload.end_action = AnimEndAction::Next(handle_idle);
    }
}

// use bevy::{
//     ecs::{query::Without, schedule::IntoSystemConfigs},
//     log::info,
//     prelude::{
//         default, in_state, App, BuildChildren, Commands, Deref, DerefMut, Handle,
//         Image, Name, Plugin, Query, Res, Resource, Sprite, SpriteBundle, Time, Timer, TimerMode,
//         Transform, TransformBundle, Update, Vec2, Vec3, With,
//     },
// };
// use bevy_rapier2d::prelude::{
//     ActiveEvents, Collider, ColliderMassProperties, CollisionGroups, Damping, Friction, LockedAxes,
//     Restitution, RigidBody, Sensor, Velocity,
// };

// use crate::{
//     bundles::{ItemColliderBundle, ProjectileBundle, RigidBodyBundle},
//     consts::{AspenCollisionLayer, ACTOR_PHYSICS_Z_INDEX, ACTOR_Z_INDEX, BULLET_SPEED_MODIFIER},
//     game::actors::{
//         ai::components::AIShootConfig,
//         attributes_stats::{Damage, ElementalEffect, PhysicalDamage, ProjectileStats},
//         components::TimeToLive,
//     },
//     loading::assets::AspenInitHandles,
//     prelude::{engine, game::action_maps},
//     AppState,
// };

// //TODO: on startup, load all ron files in assets/packs/asha/actors
// //create hashmap of `(ActorBundle, ActorColliderBundle)` with key `String` as app resource
// // spawn functions should pull from this resource

// /// shooting and graphics for enemies
// pub struct EnemyPlugin;
// impl Plugin for EnemyPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_systems(
//             Update,
//             enemy_can_shoot_check.run_if(in_state(AppState::PlayingGame)),
//         );
//     }
// }

// /// timer for shooting
// #[derive(Resource, Deref, DerefMut)]
// pub struct ShootTimer(pub Timer);

// /// checks if enemy can shoot and shoots if check is true
// pub fn enemy_can_shoot_check(
//     mut cmds: Commands,
//     time: Res<Time>,
//     assets: Res<AspenInitHandles>,
//     player_query: Query<&Transform, With<engine::ActionState<action_maps::Gameplay>>>,
//     mut enemy_query: Query<
//         (&Transform, &mut AIShootConfig),
//         Without<engine::ActionState<action_maps::Gameplay>>,
//     >,
// ) {
//     let player_transform = player_query.single();

//     for (enemy_transform, mut ai_attack_state) in &mut enemy_query {
//         let enemy_loc = enemy_transform.translation.truncate();
//         let player_loc = player_transform.translation.truncate();
//         let direction: Vec2 = (player_loc - enemy_loc).normalize_or_zero();

//         // Make sure that the projectiles spawn outside of the body so that it doesn't collide
//         let beyond_body_diff: Vec2 = direction * 36.;
//         let modified_spawn_location: Vec2 = enemy_loc + beyond_body_diff;

//         if ai_attack_state.should_shoot {
//             info!("should shoot");
//             if ai_attack_state.timer.tick(time.delta()).finished() {
//                 spawn_enemy_projectile(
//                     &mut cmds,
//                     assets.img_favicon.clone(),
//                     direction,
//                     modified_spawn_location,
//                 );
//                 ai_attack_state.timer.reset();
//             }
//         }
//     }
// }

// //TODO: make this an event
// /// spawns enemy projectile
// pub fn spawn_enemy_projectile(
//     cmds: &mut Commands,
//     projectile_texture: Handle<Image>,
//     direction: Vec2,
//     location: Vec2,
// ) {
//     cmds.spawn((
//         ProjectileBundle {
//             name: Name::new("EnemyProjectile"),
//             projectile_stats: ProjectileStats {
//                 damage: Damage {
//                     physical: PhysicalDamage(10.0),
//                     elemental: ElementalEffect::default(),
//                 },
//                 is_player_projectile: false,
//             },
//             ttl: TimeToLive(Timer::from_seconds(2.0, TimerMode::Repeating)),
//             sprite_bundle: SpriteBundle {
//                 texture: projectile_texture,
//                 transform: Transform::from_translation(location.extend(ACTOR_Z_INDEX)),
//                 sprite: Sprite {
//                     custom_size: Some(Vec2::splat(5.0)),
//                     ..default()
//                 },
//                 ..default()
//             },
//             rigidbody_bundle: RigidBodyBundle {
//                 velocity: Velocity::linear(direction * (BULLET_SPEED_MODIFIER * 5.0)),
//                 rigidbody: RigidBody::Dynamic,
//                 friction: Friction::coefficient(0.2),
//                 how_bouncy: Restitution::coefficient(0.8),
//                 mass_prop: ColliderMassProperties::Density(2.1),
//                 rotation_locks: LockedAxes::ROTATION_LOCKED,
//                 damping_prop: Damping {
//                     linear_damping: 0.1,
//                     angular_damping: 0.1,
//                 },
//             },
//         },
//         Sensor,
//     ))
//     .with_children(|child| {
//         child.spawn((
//             ItemColliderBundle {
//                 name: Name::new("EnemyProjectileCollider"),
//                 transform_bundle: TransformBundle {
//                     local: (Transform {
//                         translation: (Vec3 {
//                             x: 0.,
//                             y: 0.,
//                             z: ACTOR_PHYSICS_Z_INDEX,
//                         }),
//                         ..default()
//                     }),
//                     ..default()
//                 },
//                 collider: Collider::ball(3.0),
//                 collision_groups: CollisionGroups::new(
//                     AspenCollisionLayer::PROJECTILE,
//                     AspenCollisionLayer::EVERYTHING,
//                 ),
//                 tag: super::components::ActorColliderType::Projectile,
//             },
//             ActiveEvents::COLLISION_EVENTS,
//         ));
//     });
// }
