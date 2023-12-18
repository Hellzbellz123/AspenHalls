use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::game::actors::components::{
    EnemyColliderTag, EnemyProjectileColliderTag, EnemyProjectileTag, PlayerColliderTag,
    PlayerProjectileColliderTag, PlayerProjectileTag, ProjectileStats,
};

use super::{components::Damage, PlayerGameInformation};
// TODO: figure out what needs too be done too merge these systems

/// detects hits on enemies and applies damage
pub fn hits_on_enemy(
    mut cmds: Commands,
    mut game_info: ResMut<PlayerGameInformation>,
    mut collision_events: EventReader<CollisionEvent>,
    projectile_query: Query<&ProjectileStats, With<PlayerProjectileTag>>,
    enemy_collider_query: Query<(Entity, &Parent), With<EnemyColliderTag>>,
    player_projectile_collider_query: Query<(Entity, &Parent), With<PlayerProjectileColliderTag>>,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(a, b, _flags) = event {
            let enemy = enemy_collider_query
                .get(*b)
                .or_else(|_| enemy_collider_query.get(*a))
                .map(|(_collider, parent)| parent.get())
                .ok();

            let projectile = player_projectile_collider_query
                .get(*a)
                .or_else(|_| player_projectile_collider_query.get(*b))
                .map(|(_a, parent)| parent.get())
                .ok();

            if let Some(projectile) = projectile {
                if let Some(enemy) = enemy {
                    if let Ok(stats) = projectile_query.get(projectile) {
                        cmds.entity(projectile).despawn_recursive();
                        game_info.player_damage_sent += stats.damage;
                        cmds.entity(enemy).insert(Damage(stats.damage));
                    }
                }
            }
        }
    }
}

/// detects projectile hits on player, adds hits too Player
pub fn hits_on_player(
    mut game_info: ResMut<PlayerGameInformation>,
    mut cmds: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    player_collider_query: Query<(Entity, &Parent), With<PlayerColliderTag>>,
    bad_projectile_query: Query<&ProjectileStats, With<EnemyProjectileTag>>,
    enemy_projectile_collider_query: Query<(Entity, &Parent), With<EnemyProjectileColliderTag>>,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(a, b, _flags) = event {
            let player = player_collider_query
                .get(*b)
                .or_else(|_| player_collider_query.get(*a))
                .map(|(_collider, parent)| parent.get())
                .ok();

            let projectile = enemy_projectile_collider_query
                .get(*a)
                .or_else(|_| enemy_projectile_collider_query.get(*b))
                .map(|(_a, parent)| parent.get())
                .ok();

            if let Some(player) = player {
                if let Some(projectile) = projectile {
                    if let Ok(stats) = bad_projectile_query.get(projectile) {
                        cmds.entity(projectile).despawn_recursive();
                        game_info.enemy_damage_sent += stats.damage;
                        cmds.entity(player).insert(Damage(stats.damage));
                    }
                }
            }
        }
    }
}

// pub fn hits_on_enemy(
//     mut game_info: ResMut<PlayerGameInformation>,
//     mut cmds: Commands,
//     projectile_query: Query<&ProjectileStats, With<PlayerProjectileTag>>,
//     mut collision_events: EventReader<CollisionEvent>,
//     enemy_collider_query: Query<(Entity, &Parent), With<EnemyColliderTag>>,
//     player_projectile_collider_query: Query<(Entity, &Parent), With<PlayerProjectileColliderTag>>,
// ) {
//     for event in collision_events.iter() {
//         if let CollisionEvent::Started(a, b, _flags) = event {
//             let enemy = if enemy_collider_query.get(*b).is_ok() {
//                 let (_collider, parent) = enemy_collider_query.get(*b).unwrap();
//                 parent.get()
//             } else if enemy_collider_query.get(*a).is_ok() {
//                 let (_collider, parent) = enemy_collider_query.get(*a).unwrap();
//                 parent.get()
//             } else {
//                 return;
//             };
//             let projectile = if player_projectile_collider_query.get(*a).is_ok() {
//                 let (_a, parent) = player_projectile_collider_query.get(*a).unwrap();
//                 parent.get()
//             } else if player_projectile_collider_query.get(*b).is_ok() {
//                 let (_a, parent) = player_projectile_collider_query.get(*b).unwrap();
//                 parent.get()
//             } else {
//                 return;
//             };
//             let damage = projectile_query.get(projectile).unwrap().damage;

//             cmds.entity(projectile).despawn_recursive();
//             game_info.player_damage_sent += damage;
//             cmds.entity(enemy).insert(Damaged {
//                 amount: damage,
//                 damage_timer: Timer::from_seconds(0.1, TimerMode::Once),
//             });
//         }
//     }
// }

// pub fn hits_on_player(
//     mut game_info: ResMut<PlayerGameInformation>,
//     mut cmds: Commands,
//     mut collision_events: EventReader<CollisionEvent>,
//     player_collider_query: Query<(Entity, &Parent), With<PlayerColliderTag>>,
//     bad_projectile_query: Query<&ProjectileStats, With<EnemyProjectileTag>>,
//     enemy_projectile_collider_query: Query<(Entity, &Parent), With<EnemyProjectileColliderTag>>,
// ) {
//     for event in collision_events.iter() {
//         if let CollisionEvent::Started(a, b, _flags) = event {
//             let player = if player_collider_query.get(*b).is_ok() {
//                 let (_collider, parent) = player_collider_query.get(*b).unwrap();
//                 parent.get()
//             } else if player_collider_query.get(*a).is_ok() {
//                 let (_collider, parent) = player_collider_query.get(*a).unwrap();
//                 parent.get()
//             } else {
//                 return;
//             };
//             let projectile = if enemy_projectile_collider_query.get(*a).is_ok() {
//                 let (_a, parent) = enemy_projectile_collider_query.get(*a).unwrap();
//                 parent.get()
//             } else if enemy_projectile_collider_query.get(*b).is_ok() {
//                 let (_a, parent) = enemy_projectile_collider_query.get(*b).unwrap();
//                 parent.get()
//             } else {
//                 return;
//             };
//             let damage = bad_projectile_query.get(projectile).unwrap().damage;

//             cmds.entity(projectile).despawn_recursive();
//             game_info.enemy_damage_sent += damage;
//             cmds.entity(player).insert(Damaged {
//                 amount: damage,
//                 damage_timer: Timer::from_seconds(0.1, TimerMode::Once),
//             });
//         }
//     }
// }
