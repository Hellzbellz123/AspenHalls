use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    bundles::{Aspen2dPhysicsBundle, AspenColliderBundle, NeedsCollider, ProjectileBundle},
    consts::{AspenCollisionLayer, ACTOR_PHYSICS_Z_INDEX},
    game::{
        animations::{EventAnimationChange, GunAnimations},
        attributes_stats::{Damage, ProjectileStats},
        audio::{EventPlaySpatialSound, S_GUNSHOT},
        components::{ActorColliderType, TimeToLive},
        items::weapons::components::{
            AttackDamage, CurrentlyDrawnWeapon, GunCfg, WeaponAmmoCount, WeaponHolder, WeaponTimers,
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
                receive_gun_shots.run_if(on_event::<GunShootEvent>),
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
    mut weapon_query: Query<
        (&mut WeaponAmmoCount, &mut WeaponTimers),
        (With<ChildOf>, With<CurrentlyDrawnWeapon>),
    >,
) {
    for (mut current_ammo, mut firing_timers) in &mut weapon_query {
        if current_ammo.reloading {
            firing_timers.refill.tick(time.delta());

            if firing_timers.refill.finished() {
                current_ammo.current = current_ammo.max;
                current_ammo.reloading = false;
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
    mut sound_events: EventWriter<EventPlaySpatialSound>,
    mut weapon_query: Query<
        (
            Entity,
            &GlobalTransform,
            &mut WeaponAmmoCount,
            &mut WeaponTimers,
            &WeaponHolder,
            &AttackDamage,
        ),
        (With<ChildOf>, With<CurrentlyDrawnWeapon>),
    >,
) {
    for event in &mut gun_shoot_events.read() {
        let Ok((weapon, global_transform, mut ammo_counter, mut timers, holder, attack)) =
            weapon_query.get_mut(event.gun)
        else {
            error!("invalid gun");
            continue;
        };
        let cfg = event.settings;

        if ammo_counter.current == 0 && !ammo_counter.reloading {
            timers.refill.reset();
            ammo_counter.reloading = true;
            anim_events.write(EventAnimationChange {
                anim_handle: vec![GunAnimations::RELOAD, GunAnimations::IDLE],
                actor: weapon,
            });
            continue;
        } else if timers.attack.finished() && ammo_counter.current != 0 {
            sound_events.write(EventPlaySpatialSound {
                emitter_id: weapon,
                sound_id: S_GUNSHOT,
            });
            anim_events.write(EventAnimationChange {
                anim_handle: vec![GunAnimations::FIRE, GunAnimations::IDLE],
                actor: weapon,
            });

            let requester = holder.0.unwrap().1;
            let (_, rotation, translation) = global_transform.to_scale_rotation_translation();
            let offset = rotation * cfg.barrel_end.extend(0.0);
            let transform =
                Transform::from_translation(translation + (offset + offset)).with_rotation(rotation);

            ammo_counter.current -= 1;
            create_bullet(
                requester,
                &mut cmds,
                &assets,
                attack,
                transform,
                (cfg.projectile_speed, cfg.projectile_size),
            );
            timers.attack.reset();
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
        // TODO: load weapon projectile as seperate sprite resource?
        ProjectileBundle {
            name: Name::new("GunProjectile"),
            projectile_stats: ProjectileStats {
                damage: Damage {
                    physical: weapon_damage.physical,
                    elemental: weapon_damage.elemental,
                },
                bullet_creator: entity,
            },
            ttl: TimeToLive(Timer::from_seconds(3.5, TimerMode::Repeating)),
            rigidbody_bundle: Aspen2dPhysicsBundle::new_projectile(
                velocity_direction * projectile_speed,
            ),
        },
        Sprite {
            image: assets.img_favicon.clone(),
            custom_size: Some(Vec2::splat(projectile_size)),
            ..default()
        },
        location,
        Sensor,
    ))
    .with_children(|child| {
        child.spawn((
            EntityCreator(entity),
            CollisionEventsEnabled,
            AspenColliderBundle {
                name: Name::new("GunProjectileCollider"),
                transform: Transform {
                    translation: Vec2::ZERO.extend(ACTOR_PHYSICS_Z_INDEX),
                    ..default()
                },
                collider: NeedsCollider::Aabb, //Collider::ball(3.0),
                collision_groups: AspenCollisionLayer::projectile_actor(),
                tag: ActorColliderType::Projectile,
            },
            // ActiveEvents::COLLISION_EVENTS,
            // ActiveHooks::FILTER_CONTACT_PAIRS,
        ));
    });
}

// /// format gun animations with proper speed and transitions
// pub fn format_gun_animations(sheet: &mut Spritesheet) {
//     let handle_idle = sheet.get_anim_handle("idle");
//     let handle_wiggle = sheet.get_anim_handle("wiggle");
//     let handle_fire = sheet.get_anim_handle("fire");
//     let handle_reload = sheet.get_anim_handle("reload");

//     if let Ok(anim_idle) = sheet.get_anim_mut(&handle_idle) {
//         anim_idle.end_action = AnimEndAction::Loop;
//     }
//     if let Ok(anim_wiggle) = sheet.get_anim_mut(&handle_wiggle) {
//         anim_wiggle.time_scale = 1.0;
//         anim_wiggle.end_action = AnimEndAction::Next(handle_idle);
//     }
//     if let Ok(anim_fire) = sheet.get_anim_mut(&handle_fire) {
//         anim_fire.time_scale = 2.0;
//         anim_fire.end_action = AnimEndAction::Next(handle_idle);
//     }
//     if let Ok(anim_reload) = sheet.get_anim_mut(&handle_reload) {
//         anim_reload.time_scale = 0.5;
//         anim_reload.end_action = AnimEndAction::Next(handle_idle);
//     }
// }

// use bevy::{
//     ecs::{query::Without, schedule::IntoSystemConfigs},
//     log::info,
//     prelude::{
//         default, in_state, App, BuildChildren, Commands, Deref, DerefMut, Handle,
//         Image, Name, Plugin, Query, Res, Resource, Sprite, SpriteBundle, Time, Timer, TimerMode,
//         Transform, TransformBundle, Update, Vec2, Vec3, With,
//     },
// };
//  use avian2d::prelude::{
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
