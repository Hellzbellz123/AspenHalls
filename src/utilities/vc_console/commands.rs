use std::str::FromStr;

use bevy::prelude::{info, EventWriter, Vec3};
use bevy_console::{reply, ConsoleCommand};

use crate::{
    components::actors::spawners::{EnemyType, SpawnEnemyEvent, SpawnWeaponEvent, WeaponType},
    utilities::game::ACTOR_LAYER,
};

/// spawn weapon [WeaponType] x amount of times using `SpawnWeaponEvent`
#[derive(ConsoleCommand)]
#[console_command(name = "spawnweapon")]
pub struct SpawnWeaponCommand {
    /// type of w to spawn
    weapon_type: String,
    /// x transform
    loc_x: i64,
    /// y transform
    loc_y: i64,
    /// Number of times to spawn
    amount: Option<i32>,
}

///  spawns enemy [EnemyType] x amount of times using `SpawnEnemyEvent`
#[derive(ConsoleCommand)]
#[console_command(name = "spawnenemy")]
pub struct SpawnEnemyCommand {
    /// type of thing to spawn
    enemy_type: String,
    /// x transform
    loc_x: i64,
    /// y transform
    loc_y: i64,
    /// Number of times to spawn
    amount: Option<i32>,
}

pub fn spawnweapon_command(
    mut spawn: ConsoleCommand<SpawnWeaponCommand>,
    mut ew: EventWriter<SpawnWeaponEvent>,
) {
    if let Some(Ok(SpawnWeaponCommand {
        weapon_type,
        amount,
        loc_x,
        loc_y,
    })) = spawn.take()
    {
        let cspawn_count = amount.unwrap_or(1);
        let cspawn_location = Vec3::new(loc_x as f32, loc_y as f32, ACTOR_LAYER);
        let cspawn_type = WeaponType::from_str(&weapon_type);
        match cspawn_type {
            Ok(cspawn_type) => {
                for _ in 0..cspawn_count {
                    ew.send(SpawnWeaponEvent {
                        weapon_to_spawn: cspawn_type,
                        spawn_position: cspawn_location,
                        spawn_count: 1,
                    })
                }
                info!(
                    "console command spawnenemy() called to spawn: {:?}",
                    cspawn_type
                );
                spawn.ok();
            }
            Err(err) => {
                reply!(spawn, "invalid weapon: {}", err);
            }
        }
    }
}

pub fn spawnenemy_command(
    mut spawn: ConsoleCommand<SpawnEnemyCommand>,
    mut ew: EventWriter<SpawnEnemyEvent>,
) {
    if let Some(Ok(SpawnEnemyCommand {
        enemy_type,
        amount,
        loc_x,
        loc_y,
    })) = spawn.take()
    {
        let cspawn_count = amount.unwrap_or(1);
        let cspawn_location = Vec3::new(loc_x as f32, loc_y as f32, ACTOR_LAYER);
        let cspawn_type = EnemyType::from_str(&enemy_type);
        match cspawn_type {
            Ok(cspawn_type) => {
                ew.send(SpawnEnemyEvent {
                    enemy_to_spawn: cspawn_type,
                    spawn_position: cspawn_location,
                    spawn_count: cspawn_count,
                });
                info!("spawning: {:?}", cspawn_type);
                spawn.ok();
            }
            Err(err) => {
                reply!(spawn, "invalid enemy: {}", err);
            }
        }
    }
}
