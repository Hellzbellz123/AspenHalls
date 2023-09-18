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

/// receives spawnweapon command and sens spawn event
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
        at_player,
    })) = spawn.take()
    {
        let command_spawn_at_player = at_player.unwrap_or(true);
        let mut command_spawn_location = Vec2::new(loc_x.unwrap_or(0) as f32, loc_y.unwrap_or(0) as f32);
        let command_spawn_count = amount.unwrap_or(1);
        let command_spawn_type = WeaponType::from_str(&weapon_type);

        match command_spawn_type {
            Ok(command_spawn_type) => {
                for _ in 0..command_spawn_count {
                    if command_spawn_at_player {
                        command_spawn_location =
                            player_transform.single().translation.truncate() + vec2(offset, offset);
                    }

                    ew.send(SpawnWeaponEvent {
                        weapon_to_spawn: command_spawn_type.clone(),
                        spawn_position: command_spawn_location,
                        spawn_count: 1,
                    });
                }
                info!(
                    "console command spawnenemy() called to spawn: {:?}",
                    command_spawn_type
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

/// interprets `SpawnEnemyCommand` from console and sends `SpawnEnemyEvent`
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
        at_player,
    })) = spawn.take()
    {
        let command_spawn_at_player = at_player.unwrap_or(true);
        let command_spawn_count = amount.unwrap_or(1);
        let mut command_spawn_location = Vec2::new(loc_x.unwrap_or(0) as f32, loc_y.unwrap_or(0) as f32);
        let command_spawn_type = EnemyType::from_str(&enemy_type);

        match command_spawn_type {
            Ok(command_spawn_type) => {
                for _ in 0..command_spawn_count {
                    if command_spawn_at_player {
                        command_spawn_location =
                            player_transform.single().translation.truncate() + vec2(offset, offset);
                    }

                    ew.send(SpawnEnemyEvent {
                        enemy_to_spawn: command_spawn_type,
                        spawn_position: command_spawn_location,
                        spawn_count: 1,
                    });
                }
                info!(
                    "console command spawnenemy() called to spawn: {:?}",
                    command_spawn_type
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
pub fn teleport_player_command(
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
