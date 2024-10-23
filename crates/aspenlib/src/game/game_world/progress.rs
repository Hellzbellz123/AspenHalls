// switch too hideout if player dies inside dungeon
// switch too hideout if player exits dungeon by choice
// lock doors of room player is currently in until room enemies are defeated
// if player defeats boss, regenerate dungeon and bump dungeon level

use crate::{
    game::{
        characters::{
            ai::components::{AIChaseAction, AttackScorer, ChaseScorer},
            components::CharacterType,
            player::PlayerSelectedHero,
        },
        game_world::dungeonator_v2::components::{BossState, RoomBlueprint},
    },
    register_types, AppState,
};
use bevy::prelude::*;
use big_brain::{
    prelude::{ActionSpan, HasThinker, Score, Thinker},
    scorers,
};

pub struct GameProgressPlugin;

impl Plugin for GameProgressPlugin {
    fn build(&self, app: &mut App) {
        register_types!(app, [ProgressManager]);

        app.add_systems(OnEnter(AppState::StartMenu), initialize_progress_manager);
        app.add_systems(
            FixedUpdate,
            (update_boss_state, update_player_current_room).run_if(in_state(AppState::PlayingGame)),
        );
    }
}

#[derive(Debug, Reflect, Component, Clone)]
#[reflect(Component)]
pub struct ProgressManager {
    current: CurrentLevelState,
    overall: OverallProgressState,
}

#[derive(Debug, Reflect, Component, Clone)]
pub struct CurrentLevelState {
    boss_state: BossState,
    current_room: Option<Entity>,
    boss_id: Option<Entity>,
}

#[derive(Debug, Reflect, Component, Clone)]
pub struct OverallProgressState {
    coin: i32,
    xp: i32,
    kills: i32,
}

fn initialize_progress_manager(mut cmds: Commands) {
    // load character save state here?

    cmds.spawn((
        Name::new("ProgressManager"),
        ProgressManager {
            current: CurrentLevelState {
                boss_state: BossState::UnSpawned,
                current_room: None,
                boss_id: None,
            },
            overall: OverallProgressState {
                coin: 0,
                xp: 0,
                kills: 0,
            },
        },
    ));
}

/// update player current room
fn update_player_current_room(
    mut progress_manager: Query<&mut ProgressManager>,
    room_query: Query<(Entity, &GlobalTransform, &RoomBlueprint)>,
    player_query: Query<&Transform, With<PlayerSelectedHero>>,
) {
    let mut progress_manager = progress_manager.single_mut();
    let player_position = player_query.single().translation.xy();

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

fn update_boss_state(
    mut progress_manager: Query<&mut ProgressManager>,
    boss_query: Query<(Entity, &Transform, &CharacterType)>,
    has_thinkers: Query<&HasThinker>,
    chase_scorers: Query<&Score, With<ChaseScorer>>,
    attack_scorers: Query<&Score, With<AttackScorer>>,
    children: Query<&Children>,
) {
    let Ok(mut progress_manager) = progress_manager.get_single_mut() else {
        return;
    };

    let boss = boss_query
        .iter()
        .find(|(_, _, character_type)| **character_type == CharacterType::Boss);

    let current_state = progress_manager.current.boss_state.clone();

    match boss {
        Some((id, _, _)) => {
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

            if chase_score.get() > 0.0 && current_state == BossState::Idle {
                progress_manager.current.boss_state = BossState::Engaged
            } else if attack_score.get() == 0.0 && chase_score.get() == 0.0 {
                progress_manager.current.boss_state = BossState::Idle
            }
        }
        None => {
            if current_state == BossState::Engaged {
                progress_manager.current.boss_state = BossState::Defeated
            } else {
                progress_manager.current.boss_state = BossState::UnSpawned
            }
        }
    };
}
