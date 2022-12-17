use bevy::{
    math::vec3,
    prelude::{info, Camera, EventWriter, Query, Transform, Vec3, With, Without},
};
use bevy_console::{reply, ConsoleCommand};
use rand::{thread_rng, Rng};
use std::str::FromStr;

use crate::{
    components::actors::{
        general::Player,
        spawners::{EnemyType, SpawnEnemyEvent, SpawnWeaponEvent, WeaponType},
    },
    utilities::game::ACTOR_Z_INDEX,
};

/// spawn weapon [WeaponType] x amount of times using `SpawnWeaponEvent`
#[derive(ConsoleCommand)]
#[console_command(name = "spawnweapon")]
pub struct SpawnWeaponCommand {
    /// type of w to spawn
    weapon_type: String,
    loc_x: Option<i64>,
    /// y transform
    loc_y: Option<i64>,
    /// Number of times to spawn
    amount: Option<i32>,
    /// spawn at/near player
    atplayer: Option<bool>,
}

///  spawns enemy [EnemyType] x amount of times using `SpawnEnemyEvent`
#[derive(ConsoleCommand)]
#[console_command(name = "spawnenemy")]
pub struct SpawnEnemyCommand {
    /// type of thing to spawn
    enemy_type: String,
    /// x transform
    loc_x: Option<i64>,
    /// y transform
    loc_y: Option<i64>,
    /// Number of times to spawn
    amount: Option<i32>,
    /// spawn at/near player
    atplayer: Option<bool>,
}

pub fn spawnweapon_command(
    player_transform: Query<&Transform, (With<Player>, Without<Camera>)>,
    mut spawn: ConsoleCommand<SpawnWeaponCommand>,
    mut ew: EventWriter<SpawnWeaponEvent>,
) {
    let mut rng = thread_rng();
    let offset = rng.gen_range(-70.0..70.0);

    if let Some(Ok(SpawnWeaponCommand {
        weapon_type,
        amount,
        loc_x,
        loc_y,
        atplayer,
    })) = spawn.take()
    {
        let cspawn_atplayer = atplayer.unwrap_or(true);
        let mut cspawn_location = Vec3::new(
            loc_x.unwrap_or(0) as f32,
            loc_y.unwrap_or(0) as f32,
            ACTOR_Z_INDEX,
        );
        let cspawn_count = amount.unwrap_or(1);
        let cspawn_type = WeaponType::from_str(&weapon_type);

        match cspawn_type {
            Ok(cspawn_type) => {
                for _ in 0..cspawn_count {
                    if cspawn_atplayer {
                        cspawn_location =
                            player_transform.single().translation + vec3(offset, offset, 0.0)
                    }

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
    player_transform: Query<&Transform, (With<Player>, Without<Camera>)>,
    mut spawn: ConsoleCommand<SpawnEnemyCommand>,
    mut ew: EventWriter<SpawnEnemyEvent>,
) {
    let mut rng = thread_rng();
    let offset = rng.gen_range(-70.0..=70.0);

    if let Some(Ok(SpawnEnemyCommand {
        enemy_type,
        amount,
        loc_x,
        loc_y,
        atplayer,
    })) = spawn.take()
    {
        let cspawn_atplayer = atplayer.unwrap_or(true);
        let cspawn_count = amount.unwrap_or(1);
        let mut cspawn_location = Vec3::new(
            loc_x.unwrap_or(0) as f32,
            loc_y.unwrap_or(0) as f32,
            ACTOR_Z_INDEX,
        );
        let cspawn_type = EnemyType::from_str(&enemy_type);

        match cspawn_type {
            Ok(cspawn_type) => {
                for _ in 0..cspawn_count {
                    if cspawn_atplayer {
                        cspawn_location =
                            player_transform.single().translation + vec3(offset, offset, 0.0)
                    }

                    ew.send(SpawnEnemyEvent {
                        enemy_to_spawn: cspawn_type,
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
                reply!(spawn, "invalid enemy: {}", err);
            }
        }
    }
}
