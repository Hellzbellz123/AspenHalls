use avian2d::prelude::{Collider, CollisionStarted};
use bevy::prelude::*;

use crate::{
    game::{
        attributes_stats::{CharacterStats, DamageQueue, ProjectileStats},
        characters::{ai::components::AiType, player::PlayerSelectedHero},
        components::ActorColliderType,
        game_world::{
            components::{ActorTeleportEvent, TpTriggerEffect},
            dungeonator_v2::GeneratorState,
            RegenReason, RegenerateDungeonEvent,
        },
        progress::CurrentRunInformation,
    },
    DifficultySettings,
};

// TODO: get ai differentiation component and enable/disable freindly fire here
/// detects projectile hits, adds damage too hit actors
pub fn projectile_hits(
    mut cmds: Commands,
    mut damage_queue_query: Query<&mut DamageQueue>,
    mut collision_events: EventReader<CollisionStarted>,
    actor_colliders: Query<(Entity, &ChildOf, &ActorColliderType), With<Collider>>,
    projectiles: Query<&ProjectileStats>,
    difficulty_settings: Res<DifficultySettings>,
    ai_types: Query<&AiType>,
) {
    for event in collision_events.read() {
        let CollisionStarted(a_id, b_id) = *event;

        let Some((Ok(bullet_stats), projectile)) = ({
            let mut projectile_colliders = actor_colliders
                .iter()
                .filter(|(_, _, at)| at == &&ActorColliderType::Projectile);
            projectile_colliders
                .find(|f| f.0 == a_id || f.0 == b_id)
                .map(|f| f.1.parent())
                .map(|f| (projectiles.get(f), f))
        }) else {
            continue;
        };

        let Some(hit_actor) = ({
            let mut character_colliders = actor_colliders
                .iter()
                .filter(|(_, _, at)| at == &&ActorColliderType::Character);
            character_colliders
                .find(|f| f.0 == b_id || f.0 == a_id)
                .map(|f| f.1.parent())
        }) else {
            // projectile hit something other than character,
            // only need too handle the projectile
            cmds.entity(projectile).despawn();
            continue;
        };

        // if projectile was spawned colliding with actor this happens
        // we should skip ANY handling of the bullet.
        if bullet_stats.bullet_creator == hit_actor {
            continue;
        }

        if difficulty_settings.friendly_fire_enabled {
            let Ok(bullet_owner_type) = ai_types.get(bullet_stats.bullet_creator) else {
                warn!("bullet owner did not have an AITYPE");
                continue;
            };
            let Ok(damaged_character_type) = ai_types.get(hit_actor) else {
                warn!("character damaged by bullet did not have an AITYPE");
                continue;
            };

            let mut hit_types = vec![damaged_character_type, bullet_owner_type];
            hit_types.sort();

            let mut boss_creep_hits = vec![&AiType::Stupid, &AiType::Boss];
            boss_creep_hits.sort();

            if damaged_character_type == bullet_owner_type || hit_types == boss_creep_hits {
                info!("skipping freindly fire");
                continue;
            }
        }

        info!("projectile hit detected");
        cmds.entity(projectile).despawn();

        // get hit actors damage queue
        let Ok(mut damage_queue) = damage_queue_query.get_mut(hit_actor) else {
            // actor is effectivly immune if it doesnt have a DamageQueue
            warn!("hit actor did not have a damage queue");
            continue;
        };

        // add damage too hit actors damage queueu
        damage_queue.push_damage(bullet_stats.damage);
    }
}

// TODO: have damaged characters use particle effect or red tint when damaged
/// applys
#[allow(clippy::type_complexity)]
pub fn apply_damage_system(
    mut game_info: ResMut<CurrentRunInformation>,
    mut damaged_characters: Query<
        (&mut CharacterStats, Entity, &mut DamageQueue),
        Changed<DamageQueue>,
    >,
    player_controlled: Query<&PlayerSelectedHero>,
) {
    for (mut character_stats, character, mut damage_queue) in &mut damaged_characters {
        for damage in damage_queue.iter_queue() {
            if character_stats.get_current_health() <= 0.0 {
                return;
            }
            if player_controlled.get(character).is_ok() {
                game_info.player_physical_damage_taken += damage.physical.0;
            } else {
                game_info.enemy_physical_damage_taken += damage.physical.0;
            }
            character_stats.apply_damage(*damage);
        }
        damage_queue.empty_queue();
    }
}

/// gathers entitys that have damage and despawns them if have no remaining health
#[allow(clippy::type_complexity)]
pub fn handle_death_system(
    mut game_info: ResMut<CurrentRunInformation>,
    mut cmds: Commands,
    mut damaged_query: Query<
        (Entity, &mut CharacterStats, Option<&PlayerSelectedHero>),
        Changed<CharacterStats>,
    >,
    dungeon_state: Res<State<GeneratorState>>,
    mut regen_event: EventWriter<RegenerateDungeonEvent>,
    mut tp_event: EventWriter<ActorTeleportEvent>,
) {
    for (ent, mut stats, player_control) in &mut damaged_query {
        if stats.get_current_health() <= 0.0 {
            // should probably despawn player and rebuild.
            // or auto use postion and if dead restart
            if player_control.is_some() {
                info!("player died, resetting player");
                stats.set_health(150.0);
                game_info.player_deaths += 1;

                if *dungeon_state.get() == GeneratorState::FinishedDungeonGen {
                    regen_event.write(RegenerateDungeonEvent {
                        reason: RegenReason::PlayerDeath,
                    });
                } else {
                    tp_event.write(ActorTeleportEvent {
                        tp_type: TpTriggerEffect::Event("TeleportStartLocation".to_string()),
                        target: Some(ent),
                        sender: Some(ent),
                    });
                }
                continue;
            }

            // entity that died is not player
            error!("despawning entity");
            game_info.enemies_deaths += 1;
            cmds.entity(ent).despawn();
        }
    }
}
