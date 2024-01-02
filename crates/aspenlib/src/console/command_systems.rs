use bevy::{
    ecs::{entity::Entity, system::Res},
    log::error,
};

use crate::{
    console::{
        commands::{SpawnActorCommand, TeleportCharacterCommand},
        CommandPosition, CommandSpawnType, CommandTarget,
    },
    game::{
        actors::{ai::components::ActorType, components::ActorMoveState},
        game_world::components::{ActorTeleportEvent, TpTriggerEffect},
    },
    loading::registry::ActorRegistry,
    prelude::{
        engine::{
            bevy_console::ConsoleCommand, ActionState, EventWriter, Query, Transform, Vec2, With,
            Without,
        },
        game::{action_maps, SpawnActorEvent},
    },
};

/// interprets `SpawnCommand` from console and sends `SpawnActorEvent`
///
/// # Panics
/// will panic of there is NO player OR more than ONE
pub fn spawn_command(
    player_query: Query<&Transform, With<ActionState<action_maps::Gameplay>>>,
    mut spawn: ConsoleCommand<SpawnActorCommand>,
    mut ew: EventWriter<SpawnActorEvent>,
    registry: Res<ActorRegistry>,
) {
    let spawn_clone = spawn.take();
    let Some(Ok(SpawnActorCommand {
        actor_type,
        identifier,
        position,
        amount,
        where_spawn,
    })) = spawn_clone
    else {
        // no spawn command issued
        return;
    };

    let spawn_count = amount.unwrap_or(1);

    let spawn_position: Vec2 =
        if where_spawn.is_some_and(|f| f == CommandTarget::Player) || position.is_none() {
            let player_pos: Vec2 = player_query.get_single().map_or_else(
                |f| {
                    error!("could not et player pos: {f}");
                    Vec2::ZERO
                },
                |f| f.translation.truncate(),
            );
            player_pos
        } else {
            position.unwrap_or(CommandPosition(0.0, 0.0)).into()
        };

    match actor_type {
        CommandSpawnType::Item => {
            let item_reg = &registry.items;
            if item_reg.weapons.contains_key(&identifier) {
                spawn.reply("got weapon item");
            } else {
                spawn.reply_failed("item did not exist in registry");
                return;
            }

            spawn.reply_ok("Spawning item");
            ew.send(SpawnActorEvent {
                what_to_spawn: identifier,
                who_spawned: None,
                spawn_position,
                actor_type: ActorType::Weapon,
                spawn_count,
            });
        }
        CommandSpawnType::Npc => {
            let char_reg = &registry.characters;
            let bundle = if char_reg.creeps.contains_key(&identifier) {
                registry.characters.creeps.get(&identifier).unwrap()
            } else if char_reg.bosses.contains_key(&identifier) {
                registry.characters.bosses.get(&identifier).unwrap()
            } else if char_reg.heroes.contains_key(&identifier) {
                registry.characters.heroes.get(&identifier).unwrap()
            } else {
                spawn.reply_failed("Npc did not exist in registry");
                return;
            };

            spawn.reply_ok("Spawning Npc");
            ew.send(SpawnActorEvent {
                what_to_spawn: identifier,
                who_spawned: None,
                spawn_position,
                actor_type: bundle.actor_type,
                spawn_count,
            });
        }
    };
}

/// receives tp command and teleports actor too location
#[allow(clippy::type_complexity)]
pub fn teleport_command(
    player_query: Query<(Entity, &Transform), With<ActionState<action_maps::Gameplay>>>,
    other_actors: Query<
        (Entity, &Transform),
        (
            With<ActorMoveState>,
            Without<ActionState<action_maps::Gameplay>>,
        ),
    >,
    mut spawn: ConsoleCommand<TeleportCharacterCommand>,
    mut ew: EventWriter<ActorTeleportEvent>,
) {
    if let Some(Ok(TeleportCharacterCommand { who, pos })) = spawn.take() {
        let who = who.unwrap_or(super::CommandTarget::Player);
        let player = player_query.get_single();

        match who {
            super::CommandTarget::Player => {
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
            super::CommandTarget::Nearest => {
                let Ok((player, player_pos)) = player else {
                    spawn.reply_failed("No Player too teleport");
                    return;
                };
                let Some(closest) = other_actors.iter().min_by(|lhs, rhs| {
                    let distance_a = lhs.1.translation.distance_squared(player_pos.translation);
                    let distance_b = rhs.1.translation.distance_squared(player_pos.translation);
                    distance_a
                        .partial_cmp(&distance_b)
                        .unwrap_or(std::cmp::Ordering::Less)
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
            super::CommandTarget::Everyone => {
                let mut too_teleport: Vec<Entity> = Vec::new();
                let Ok((player, _)) = player else {
                    error!("no player too teleport");
                    return;
                };
                too_teleport.push(player);
                other_actors.iter().for_each(|f| {
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
