use bevy::prelude::*;
use big_brain::prelude::{
    ActionBuilder, ActionSpan, ActionState, Actor, HasThinker, Score, Thinker,
};
use big_brain::{thinker::ThinkerBuilder, BigBrainSet};
use std::collections::VecDeque;

use crate::game::attributes_stats::{Damage, ElementalEffect, PhysicalDamage};
use crate::game::characters::ai::components::ChaseScorer;
use crate::game::items::weapons::components::AttackDamage;
use crate::game::items::weapons::forms::create_bullet;
use crate::game::{
    characters::ai::components::{AICombatAggroConfig, AIWanderConfig},
    AppState,
};
use crate::loading::assets::AspenInitHandles;
use crate::register_types;
use crate::utilities::EntityCreator;

use super::components::AIAutoShootConfig;

/// stupid ai systems and functions
pub struct SkillusingAiPlugin;

impl Plugin for SkillusingAiPlugin {
    fn build(&self, app: &mut App) {
        register_types!(
            app,
            [
                AIShootPatternsConfig,
                AIPatternEnergy,
                ShootPatternSpawner,
                EntityCreator
            ]
        );

        app.add_systems(
            PreUpdate,
            (ai_patterns_use_system)
                .run_if(in_state(AppState::PlayingGame))
                .in_set(BigBrainSet::Scorers),
        )
        .add_systems(
            Update,
            (
                ai_patternskill_action.in_set(BigBrainSet::Actions),
                shootpatternspawner_system,
            )
                .run_if(in_state(AppState::PlayingGame)),
        );
    }
}

// a rethink might be warranted on ai layout, currently the mess of configs is difficult, it might be more useful
// too have a list of behaviors that 'actors' can use and are required components are added if needed
/// All Components needed for `stupid_ai` functionality
#[derive(Bundle)]
pub struct SkillusingAIBundle {
    /// ai shooting patterns config
    pub shootpattern: AIShootPatternsConfig,
    /// auto shoot action
    pub shoot_config: AIAutoShootConfig,
    /// ai chase/attack config
    pub combat_config: AICombatAggroConfig,
    /// stupid wander action
    pub wander_config: AIWanderConfig,
    /// chooses action
    pub thinker: ThinkerBuilder,
}

/// scorer tag
#[derive(Component, Default, Clone, Debug, Reflect, ActionBuilder)]
#[reflect(Component)]
pub struct AIShootPatterns;

/// shhoot pattern config component
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct AIShootPatternsConfig {
    /// list of possible patterns that can be used
    pub patterns: VecDeque<(i32, ShootPattern)>,
    /// cooldown time between patterns
    pub time_between_patterns: Timer,
}

/// resource for creating `ShootPatterns`
#[derive(Component, Default, Clone, Debug, Reflect, ActionBuilder)]
#[reflect(Component)]
pub struct AIPatternEnergy {
    /// energy regen per second
    pub per_second: f32,
    /// current energy character possess
    pub current: f32,
}

/// different ways groups bullets can be placed in world
#[derive(Reflect, Debug, Clone)]
pub enum ShootPattern {
    /// pillars of bullets between arcs
    BulletsOverArc {
        /// how many time should this pattern be duplicated
        waves: i32,
        /// 0* - 360*, clamped if outside
        arc: i32,
        /// projectile pillar amount. divieded by degrees
        amount: i32,
        /// rotation applied too spawn area per wave
        rotation_per_wave: i32,
        /// focus casters enemy
        focus: bool,
    },
    /// singular beams divided between arc
    BeamedArc {
        /// how many beams too place evenly inside arc
        beams: i32,
        /// individual beam width in pixels
        beam_width: f32,
        /// arc too spawn beams inside
        arc: i32,
    },
}

/// maximum pattern energy characters are allowed too possess
pub const MAX_PATTERN_ENERGY: f32 = 100.0;

/// pattern spawner state
#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub struct ShootPatternSpawner {
    /// what pattern this spawner should be creating
    shootpattern: ShootPattern,
    /// how long between pattern repeats
    pattern_timer: Timer,
    /// how many times has this patternspawner created a pattern
    runs: i32,
}

// query skill users energy, if skill user has available energy and is in combat state then set scorer too 1.0
// else scorer should be 0.0
/// queues `ShootPatternAction` if actor has enough energy for any pattern and timer between patterns has finished
fn ai_patterns_use_system(
    time: Res<Time>,
    mut pattern_energy: Query<(Entity, &mut AIPatternEnergy, &mut AIShootPatternsConfig)>,
    // We need to get to the Thinker. That takes a couple of steps.
    has_thinkers: Query<&HasThinker>,
    mut thinkers: Query<(&mut Thinker, &ActionSpan)>,
    scorers: Query<&Score, With<ChaseScorer>>,
    children: Query<&Children>,
) {
    for (actor, mut pattern_energy, mut patterns_cfg) in &mut pattern_energy {
        let updated_energy = pattern_energy
            .per_second
            .mul_add(time.delta().as_secs_f32(), pattern_energy.current);

        patterns_cfg.time_between_patterns.tick(time.delta());
        pattern_energy.current = updated_energy.clamp(0.0, MAX_PATTERN_ENERGY);

        if patterns_cfg.time_between_patterns.finished()
            && patterns_cfg
                .patterns
                .iter()
                .any(|(cost, _)| (*cost as f32) < pattern_energy.current)
        {
            patterns_cfg.time_between_patterns.reset();

            let thinker_ent = has_thinkers.get(actor).unwrap().entity();

            let chase_scorer = children
                .iter_descendants(thinker_ent)
                .find(|f| scorers.get(*f).is_ok())
                .expect("pattern users should have chase scorer");
            let chase_score = scorers
                .get(chase_scorer)
                .expect("chase scorer should exist");

            if chase_score.get() < 0.5 {
                continue;
            }

            let (mut thinker, span) = thinkers.get_mut(thinker_ent).unwrap();
            let _guard = span.span().enter();

            info!("Scheduling one-off shoot pattern action");
            thinker.schedule_action(AIShootPatterns);
        }
    }
}

/// spawns `ShootPatternSpawner` for selected shootpattern when ai actor has required energy
fn ai_patternskill_action(
    mut cmds: Commands,
    mut enemy_query: Query<(
        Entity,
        &Transform,
        &mut AIShootPatternsConfig,
        &mut AIPatternEnergy,
    )>,
    mut action_query: Query<(&Actor, &mut ActionState, &ActionSpan), With<AIShootPatterns>>,
) {
    for (actor, mut action_state, _span) in &mut action_query {
        match *action_state {
            ActionState::Requested => {}
            ActionState::Cancelled => {
                debug!("One-off action was cancelled. Considering this a failure.");
                *action_state = ActionState::Failure;
                continue;
            }
            _ => {
                continue;
            }
        }

        let Ok((entity, enemy_pos, mut enemy_patterns, mut pattern_energy)) =
            enemy_query.get_mut(actor.0)
        else {
            error!("Could not get entitiy for 'AiShootPatterns' action");
            *action_state = ActionState::Failure;
            continue;
        };

        // technically we should do this the other way around but rust borrowing rules
        // dont allow it easily AND effeciently, i wonder if it optimizes out
        // another option is store the index of pattern inside AiShootPatternsConfig and update/use that for current pattern.
        enemy_patterns.patterns.rotate_left(1);
        if enemy_patterns
            .patterns
            .iter()
            .all(|(cost, _)| (*cost as f32) > pattern_energy.current)
        {
            // pattern energy is empty and we should skip until its recharged more
            continue;
        }

        let Some((cost, pattern)) = enemy_patterns
            .patterns
            .iter()
            .find(|(cost, _)| (*cost as f32) < pattern_energy.current)
        else {
            error!("Ai actor did not have a 'ShootPattern' inside AiShootPatternsConfig");
            continue;
        };

        info!("creating shoot pattern spawner");
        pattern_energy.current -= *cost as f32;
        *action_state = ActionState::Success;
        cmds.spawn((
            Name::new("ShootPatternSpawner"),
            EntityCreator(entity),
            ShootPatternSpawner {
                shootpattern: pattern.clone(),
                pattern_timer: Timer::from_seconds(PATTERN_DUPLICATE_TIME, TimerMode::Once),
                runs: 0,
            },
            SpatialBundle::from_transform(*enemy_pos),
        ));
    }
}

/// how many seconds between `ShootPatternSpawner` iterations
const PATTERN_DUPLICATE_TIME: f32 = 0.3;

/// creates entity too replicate shoot patterns
pub fn shootpatternspawner_system(
    time: Res<Time>,
    init_handles: Res<AspenInitHandles>,
    mut cmds: Commands,
    mut pattern_spawners: Query<(Entity, &mut ShootPatternSpawner, &Transform, &EntityCreator)>,
) {
    for (spawner_ent, mut pattern_spawner, spawner_pos, spawner_creator) in &mut pattern_spawners {
        pattern_spawner.pattern_timer.tick(time.delta());

        if pattern_spawner.pattern_timer.finished() {
            pattern_spawner.pattern_timer.reset();
            pattern_spawner.runs += 1;

            let mut bullet_spawn = *spawner_pos;

            match pattern_spawner.shootpattern {
                ShootPattern::BulletsOverArc {
                    waves,
                    arc: _,
                    amount,
                    rotation_per_wave,
                    focus: _,
                } => {
                    info!("creating BulletsOverArc pattern");

                    for _ in 1..=amount {
                        info!("spawning bullet for shoot pattern");

                        bullet_spawn.rotate_local_x(
                            ((rotation_per_wave + (pattern_spawner.runs * 10)) as f32).to_radians(),
                        );

                        // mostly works, must offset each one by some amount
                        create_bullet(
                            spawner_creator.0,
                            &mut cmds,
                            &init_handles,
                            &AttackDamage(Damage {
                                physical: PhysicalDamage(120.0),
                                elemental: ElementalEffect::Fire(20.0),
                            }),
                            bullet_spawn,
                            (100.0, 15.0),
                        );
                    }

                    if pattern_spawner.runs == waves {
                        info!("pattern spawner has finished its pattern, despawning");
                        cmds.entity(spawner_ent).despawn_recursive();
                    }
                }
                ShootPattern::BeamedArc {
                    beams: _,
                    beam_width: _,
                    arc: _,
                } => {
                    warn!("unhandled shoot pattern, despawning");
                    cmds.entity(spawner_ent).despawn_recursive();
                }
            }
        }
    }
}
