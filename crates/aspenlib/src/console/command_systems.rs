use bevy::ecs::entity::Entity;
use std::str::FromStr;
use strum::VariantNames;

use crate::{
    ahp::{
        engine::{
            bevy_console::{reply, ConsoleCommand},
            info, Camera, EventWriter, Query, Transform, Vec2, With, Without,
        },
        game::{ActorType, EnemyType, Faction, Player, SpawnActorEvent, WeaponType},
        rand::{thread_rng, Rng},
    },
    console::commands::{SpawnEnemyCommand, SpawnWeaponCommand, TeleportPlayerCommand},
    game::game_world::components::{ActorTeleportEvent, TpTriggerEffect},
};

/// receives spawnweapon command and sens spawn event
///
/// # Panics
/// will panic of there is NO player OR more than ONE
pub fn spawnweapon_command(
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
    mut spawn: ConsoleCommand<SpawnWeaponCommand>,
    mut ew: EventWriter<SpawnActorEvent>,
) {
    let mut rng = thread_rng();
    let _offset = rng.gen_range(-70.0..70.0);

    if let Some(Ok(SpawnWeaponCommand {
        weapon_type,
        amount,
        loc_x,
        loc_y,
        at_player,
    })) = spawn.take()
    {
        let command_spawn_at_player = at_player.unwrap_or(true);
        let mut command_spawn_location = Vec2::new(loc_x.unwrap_or(0.0), loc_y.unwrap_or(0.0));
        let command_spawn_count = amount.unwrap_or(1);
        let command_spawn_type = WeaponType::from_str(&weapon_type);

        match command_spawn_type {
            Ok(command_spawn_type) => {
                for _ in 0..command_spawn_count {
                    if command_spawn_at_player {
                        let player_transform = player_query
                            .get_single()
                            .expect("should only ever be one player.");
                        command_spawn_location = player_transform.translation.truncate();
                    }
                    ew.send(SpawnActorEvent {
                        spawner: None,
                        actor_type: ActorType::Item,
                        what_to_spawn: weapon_type.clone(),
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
///
/// # Panics
/// will panic of there is NO player OR more than ONE
pub fn spawnenemy_command(
    player_query: Query<&Transform, With<Player>>,
    mut spawn: ConsoleCommand<SpawnEnemyCommand>,
    mut ew: EventWriter<SpawnActorEvent>,
) {
    let mut rng = thread_rng();
    let _offset = rng.gen_range(-70.0..=70.0);

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
        let mut command_spawn_location = Vec2::new(loc_x.unwrap_or(0.0), loc_y.unwrap_or(0.0));
        let command_spawn_type = EnemyType::from_str(&enemy_type);

        match command_spawn_type {
            Ok(command_spawn_type) => {
                if command_spawn_at_player {
                    let player_transform = player_query
                        .get_single()
                        .expect("should only ever be one player.");
                    command_spawn_location = player_transform.translation.truncate();
                }

                ew.send(SpawnActorEvent {
                    actor_type: ActorType::Npc(Faction::Enemy),
                    what_to_spawn: enemy_type,
                    spawner: None,
                    spawn_position: command_spawn_location,
                    spawn_count: command_spawn_count,
                });
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
///
/// # Panics
/// will panic of there is NO player OR more than ONE
pub fn teleport_player_command(
    player_query: Query<Entity, With<Player>>,
    mut spawn: ConsoleCommand<TeleportPlayerCommand>,
    mut ew: EventWriter<ActorTeleportEvent>,
) {
    if let Some(Ok(TeleportPlayerCommand { loc_x, loc_y })) = spawn.take() {
        let player = player_query
            .get_single()
            .expect("should only ever be one player.");
        ew.send(ActorTeleportEvent {
            tp_type: TpTriggerEffect::Global(Vec2 { x: loc_x, y: loc_y }),
            target: Some(player),
            sender: None,
        });
    }
}
