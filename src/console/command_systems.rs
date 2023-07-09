use rand::{thread_rng, Rng};
use std::str::FromStr;
use strum::VariantNames;

use bevy::{
    math::vec2,
    prelude::{info, Camera, EventWriter, Query, Transform, Vec2, Vec3, With, Without},
};

use bevy_console::{reply, ConsoleCommand};

use crate::{
    consts::ACTOR_Z_INDEX,
    game::actors::{
        components::Player,
        spawners::components::{EnemyType, SpawnEnemyEvent, SpawnWeaponEvent, WeaponType},
    },
};

use super::commands::{SpawnEnemyCommand, SpawnWeaponCommand, TeleportPlayerCommand};

/// recieves spawnweapon command and sens spawn event
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
        let mut cspawn_location = Vec2::new(loc_x.unwrap_or(0) as f32, loc_y.unwrap_or(0) as f32);
        let cspawn_count = amount.unwrap_or(1);
        let cspawn_type = WeaponType::from_str(&weapon_type);

        match cspawn_type {
            Ok(cspawn_type) => {
                for _ in 0..cspawn_count {
                    if cspawn_atplayer {
                        cspawn_location =
                            player_transform.single().translation.truncate() + vec2(offset, offset)
                    }

                    ew.send(SpawnWeaponEvent {
                        weapon_to_spawn: cspawn_type.clone(),
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
                reply!(
                    spawn,
                    "{}, possible values are {:?}",
                    err,
                    WeaponType::VARIANTS
                );
            }
        }
    }
}

/// recieves spawnenemy command and send spawnevent
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
        let mut cspawn_location = Vec2::new(loc_x.unwrap_or(0) as f32, loc_y.unwrap_or(0) as f32);
        let cspawn_type = EnemyType::from_str(&enemy_type);

        match cspawn_type {
            Ok(cspawn_type) => {
                for _ in 0..cspawn_count {
                    if cspawn_atplayer {
                        cspawn_location =
                            player_transform.single().translation.truncate() + vec2(offset, offset)
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
                reply!(
                    spawn,
                    "{}, possible values are {:?}",
                    err,
                    EnemyType::VARIANTS
                );
            }
        }
    }
}

/// receives tp command and teleports player too location
pub fn teleportplayer_command(
    mut player_transform: Query<&mut Transform, (With<Player>, Without<Camera>)>,
    mut spawn: ConsoleCommand<TeleportPlayerCommand>,
) {
    if let Some(Ok(TeleportPlayerCommand { loc_x, loc_y })) = spawn.take() {
        player_transform.single_mut().translation = Vec3 {
            x: loc_x as f32,
            y: loc_y as f32,
            z: ACTOR_Z_INDEX,
        }
    }
}
