use crate::game::{
    characters::{
        ai::components::{AttackScorer, ChaseScorer},
        components::CharacterType,
        player::PlayerSelectedHero,
    },
    game_world::{
        dungeonator_v2::{
            components::{BossState, RoomBlueprint},
            GeneratorState,
        },
        RegenReason, RegenerateDungeonEvent,
    },
    progress::ProgressManager,
};
use bevy::prelude::*;
use big_brain::prelude::{HasThinker, Score};

/// update player current room
pub fn update_player_current_room(
    mut progress_manager: Query<&mut ProgressManager>,
    room_query: Query<(Entity, &GlobalTransform, &RoomBlueprint)>,
    player_query: Query<&Transform, With<PlayerSelectedHero>>,
) {
    let Ok(mut progress_manager) = progress_manager.single_mut() else {
        return;
    };
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let player_position = player_transform.translation.xy();

    let current_room = room_query
        .iter()
        .find(|f| {
            let room_xy = f.1.translation().xy();
            let size = f.2.room_space.size();

            let room_rect = Rect::from_corners(room_xy, room_xy + size.as_vec2());
            room_rect.contains(player_position)
        })
        .map(|f| f.0);

    progress_manager.current.current_room = current_room;
}

/// updates boss state based on boss ai status
pub fn update_boss_state(
    mut progress_manager: Query<&mut ProgressManager>,
    mut regen_event: EventWriter<RegenerateDungeonEvent>,
    actor_query: Query<(Entity, &Transform, &CharacterType), Without<PlayerSelectedHero>>,
    children: Query<&Children>,
    has_thinkers: Query<&HasThinker>,
    generator_state: Res<State<GeneratorState>>,
    chase_scorers: Query<&Score, With<ChaseScorer>>,
    attack_scorers: Query<&Score, With<AttackScorer>>,
) {
    let Ok(mut progress_manager) = progress_manager.single_mut() else {
        warn!("could not get progress manager");
        return;
    };

    let boss = actor_query
        .iter()
        .find(|(_, _, character_type)| **character_type == CharacterType::Boss);

    if
    // actor_query
    //     .iter()
    //     .filter(|f| {
    //         *f.2 == CharacterType::Creep
    //             || *f.2 == CharacterType::CreepElite
    //             || *f.2 == CharacterType::MiniBoss
    //     }).next()
    //     .is_none()
    //     &&
    *generator_state.get() == GeneratorState::FinishedDungeonGen
        && progress_manager.current.boss_state == BossState::Defeated
    {
        progress_manager.current.boss_state = BossState::UnSpawned;
        regen_event.write(RegenerateDungeonEvent {
            reason: RegenReason::BossDefeat,
        });
        return;
    }

    match boss {
        Some((id, _, _)) => {
            progress_manager.current.boss_id = Some(id);
            let Ok(thinker_ent) = has_thinkers.get(id) else {
                warn!("boss did not have HasThinker");
                return;
            };

            let chase_scorer = children
                .iter_descendants(thinker_ent.entity())
                .find(|f| chase_scorers.get(*f).is_ok())
                .expect("thinker entity did not have chase scorer");
            let chase_score = chase_scorers
                .get(chase_scorer)
                .expect("could not get scorer component");

            let attack_scorer = children
                .iter_descendants(thinker_ent.entity())
                .find(|f| attack_scorers.get(*f).is_ok())
                .expect("thinker did not have attack scorer");
            let attack_score = attack_scorers
                .get(attack_scorer)
                .expect("could not get scorer component");

            if attack_score.get() != 0.0
                && chase_score.get() != 0.0
                && progress_manager.current.boss_state == BossState::Idle
            {
                progress_manager.current.boss_state = BossState::Engaged;
            } else if progress_manager.current.boss_state == BossState::UnSpawned {
                progress_manager.current.boss_state = BossState::Idle;
            }
        }
        None => {
            if progress_manager.current.boss_state == BossState::Engaged {
                progress_manager.current.boss_state = BossState::Defeated;
            } else {
                progress_manager.current.boss_state = BossState::UnSpawned;
            }
        }
    };
}
