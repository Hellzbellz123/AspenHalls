use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::game::actors::components::{
    EnemyColliderTag, EnemyProjectileColliderTag, EnemyProjectileTag, PlayerColliderTag,
    PlayerProjectileColliderTag, PlayerProjectileTag, ProjectileStats,
};

use super::{components::Damage, PlayerGameInformation};

/// detects hits on enemys and applies damage
pub fn hits_on_enemy(
    mut game_info: ResMut<PlayerGameInformation>,
    mut cmds: Commands,
    projectile_query: Query<&ProjectileStats, With<PlayerProjectileTag>>,
    mut collision_events: EventReader<CollisionEvent>,
    enemycollider_query: Query<(Entity, &Parent), With<EnemyColliderTag>>,
    playerprojectilecollider_query: Query<(Entity, &Parent), With<PlayerProjectileColliderTag>>,
) {
    collision_events.iter().for_each(|event| {
        if let CollisionEvent::Started(a, b, _flags) = event {
            let enemy = enemycollider_query
                .get(*b)
                .or_else(|_| enemycollider_query.get(*a))
                .map(|(_collider, parent)| parent.get())
                .ok();

            let projectile = playerprojectilecollider_query
                .get(*a)
                .or_else(|_| playerprojectilecollider_query.get(*b))
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
    });
}

// pub fn hits_on_enemy(
//     mut game_info: ResMut<PlayerGameInformation>,
//     mut cmds: Commands,
//     projectile_query: Query<&ProjectileStats, With<PlayerProjectileTag>>,
//     mut collision_events: EventReader<CollisionEvent>,
//     enemycollider_query: Query<(Entity, &Parent), With<EnemyColliderTag>>,
//     playerprojectilecollider_query: Query<(Entity, &Parent), With<PlayerProjectileColliderTag>>,
// ) {
//     for event in collision_events.iter() {
//         if let CollisionEvent::Started(a, b, _flags) = event {
//             let enemy = if enemycollider_query.get(*b).is_ok() {
//                 let (_collider, parent) = enemycollider_query.get(*b).unwrap();
//                 parent.get()
//             } else if enemycollider_query.get(*a).is_ok() {
//                 let (_collider, parent) = enemycollider_query.get(*a).unwrap();
//                 parent.get()
//             } else {
//                 return;
//             };
//             let projectile = if playerprojectilecollider_query.get(*a).is_ok() {
//                 let (_a, parent) = playerprojectilecollider_query.get(*a).unwrap();
//                 parent.get()
//             } else if playerprojectilecollider_query.get(*b).is_ok() {
//                 let (_a, parent) = playerprojectilecollider_query.get(*b).unwrap();
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
//     // collision_events.clear();
// }

// pub fn hits_on_player(
//     mut game_info: ResMut<PlayerGameInformation>,
//     mut cmds: Commands,
//     mut collision_events: EventReader<CollisionEvent>,
//     playercollider_query: Query<(Entity, &Parent), With<PlayerColliderTag>>,
//     bad_projectile_query: Query<&ProjectileStats, With<EnemyProjectileTag>>,
//     enemyprojectilecollider_query: Query<(Entity, &Parent), With<EnemyProjectileColliderTag>>,
// ) {
//     for event in collision_events.iter() {
//         if let CollisionEvent::Started(a, b, _flags) = event {
//             let player = if playercollider_query.get(*b).is_ok() {
//                 let (_collider, parent) = playercollider_query.get(*b).unwrap();
//                 parent.get()
//             } else if playercollider_query.get(*a).is_ok() {
//                 let (_collider, parent) = playercollider_query.get(*a).unwrap();
//                 parent.get()
//             } else {
//                 return;
//             };
//             let projectile = if enemyprojectilecollider_query.get(*a).is_ok() {
//                 let (_a, parent) = enemyprojectilecollider_query.get(*a).unwrap();
//                 parent.get()
//             } else if enemyprojectilecollider_query.get(*b).is_ok() {
//                 let (_a, parent) = enemyprojectilecollider_query.get(*b).unwrap();
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
//     // collision_events.clear();
// }

/// detects projectile hits on player, adds hits too Player
pub fn hits_on_player(
    mut game_info: ResMut<PlayerGameInformation>,
    mut cmds: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    playercollider_query: Query<(Entity, &Parent), With<PlayerColliderTag>>,
    bad_projectile_query: Query<&ProjectileStats, With<EnemyProjectileTag>>,
    enemyprojectilecollider_query: Query<(Entity, &Parent), With<EnemyProjectileColliderTag>>,
) {
    collision_events.iter().for_each(|event| {
        if let CollisionEvent::Started(a, b, _flags) = event {
            let player = playercollider_query
                .get(*b)
                .or_else(|_| playercollider_query.get(*a))
                .map(|(_collider, parent)| parent.get())
                .ok();

            let projectile = enemyprojectilecollider_query
                .get(*a)
                .or_else(|_| enemyprojectilecollider_query.get(*b))
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
    });
}
