use bevy::prelude::*;
use bevy_console::ConsoleCommand;

use crate::{
    console::{
        commands::{CommandSpawnType, CommandTarget},
        commands::{SpawnActorCommand, TeleportCharacterCommand},
    },
    game::{
        characters::{
            components::CharacterMoveState, player::PlayerSelectedHero, EventSpawnCharacter,
        },
        game_world::components::{ActorTeleportEvent, TpTriggerEffect},
        items::EventSpawnItem,
    },
    loading::registry::ActorRegistry,
};

/// interprets `SpawnCommand` from console and sends `SpawnActorEvent`
///
/// # Panics
/// will panic of there is NO player OR more than ONE
pub fn spawn_command(
    player_query: Query<Entity, With<PlayerSelectedHero>>,
    mut spawn: ConsoleCommand<SpawnActorCommand>,
    mut ew_character: EventWriter<EventSpawnCharacter>,
    mut ew_item: EventWriter<EventSpawnItem>,
    registry: Res<ActorRegistry>,
) {
    let spawn_clone = spawn.take();
    let Some(Ok(SpawnActorCommand {
        actor_type: spawn_type,
        identifier,
        position: _,
        amount,
        where_spawn,
    })) = spawn_clone
    else {
        // no spawn command issued
        return;
    };

    let spawn_count = amount.unwrap_or(1);
    let target_entity = match where_spawn.unwrap_or(CommandTarget::Player) {
        CommandTarget::Player => player_query.single(),
        CommandTarget::Nearest => {
            todo!("sort by closest too player, send closest")
        }
        CommandTarget::Everyone => {
            spawn.reply_failed("Targeting Everyone for spawn is unsupported");
            return;
        }
    };

    match spawn_type {
        CommandSpawnType::Item => {
            let item_reg = &registry.items;
            if item_reg.weapons.contains_key(&identifier) {
                spawn.reply("got weapon item");
            } else {
                spawn.reply_failed("item did not exist in registry");
                return;
            }

            spawn.reply_ok("Spawning item");
            ew_item.send(EventSpawnItem {
                spawn_data: (identifier, spawn_count),
                // TODO: this is a shortcut, fix
                requester: target_entity,
            });
        }
        CommandSpawnType::Npc => {
            let char_reg = &registry.characters;
            if char_reg.creeps.contains_key(&identifier) {
                spawn.reply("got creep");
            } else if char_reg.bosses.contains_key(&identifier) {
                spawn.reply("got boss");
            } else if char_reg.heroes.contains_key(&identifier) {
                spawn.reply("got hero");
            } else {
                spawn.reply_failed("character did not exist in registry");
                return;
            };

            spawn.reply_ok("Spawning character");
            ew_character.send(EventSpawnCharacter {
                identifier,
                // TODO: this is a shortcut, fix
                requester: player_query.single(),
            });
        }
    };
}

/// receives tp command and teleports actor too location
#[allow(clippy::type_complexity)]
pub fn teleport_command(
    player_query: Query<(Entity, &Transform), (With<CharacterMoveState>, With<PlayerSelectedHero>)>,
    characters: Query<
        (Entity, &Transform),
        (With<CharacterMoveState>, Without<PlayerSelectedHero>),
    >,
    mut spawn: ConsoleCommand<TeleportCharacterCommand>,
    mut ew: EventWriter<ActorTeleportEvent>,
) {
    if let Some(Ok(TeleportCharacterCommand { who, pos })) = spawn.take() {
        let who = who.unwrap_or(CommandTarget::Player);
        let player = player_query.get_single();

        match who {
            CommandTarget::Player => {
                let Ok((player, _)) = player else {
                    spawn.reply_failed("No Player too teleport");
                    return;
                };

                spawn.reply_ok("Teleporting Player");
                ew.send(ActorTeleportEvent {
                    tp_type: TpTriggerEffect::Global(pos.into()),
                    target: Some(player),
                    sender: Some(player),
                });
            }
            CommandTarget::Nearest => {
                let Ok((player, player_pos)) = player else {
                    spawn.reply_failed("No Player too teleport");
                    return;
                };
                let Some(closest) = characters.iter().min_by(|lhs, rhs| {
                    // Not saving distance, doesnt matter if we square faster
                    let distance_a = lhs
                        .1
                        .translation
                        .as_ivec3()
                        .distance_squared(player_pos.translation.as_ivec3());
                    let distance_b = rhs
                        .1
                        .translation
                        .as_ivec3()
                        .distance_squared(player_pos.translation.as_ivec3());
                    distance_a.cmp(&distance_b)
                }) else {
                    spawn.reply_failed("Closest Enemy error");
                    return;
                };

                spawn.reply_ok("Teleporting nearest character");
                ew.send(ActorTeleportEvent {
                    tp_type: TpTriggerEffect::Global(pos.into()),
                    target: Some(closest.0),
                    sender: Some(player),
                });
            }
            CommandTarget::Everyone => {
                let mut too_teleport: Vec<Entity> = Vec::new();
                let Ok((player, _)) = player else {
                    error!("no player too teleport");
                    return;
                };
                too_teleport.push(player);
                characters.iter().for_each(|f| {
                    too_teleport.push(f.0);
                });

                spawn.reply_ok("Teleporting all characters");
                for f in &too_teleport {
                    ew.send(ActorTeleportEvent {
                        tp_type: TpTriggerEffect::Global(pos.into()),
                        target: Some(*f),
                        sender: Some(player),
                    });
                }
            }
        }
    }
}
