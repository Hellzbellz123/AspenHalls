use bevy::prelude::*;
use bevy_asepritesheet::sprite::AnimHandle;
use bevy_rapier2d::dynamics::Velocity;

use crate::{
    consts::{MIN_VELOCITY, WALK_MODIFIER},
    game::{
        animations::EventAnimationChange,
        attributes_stats::CharacterStats,
        characters::components::{CharacterMoveState, CurrentMovement},
    },
    loading::registry::RegistryIdentifier,
    utilities::vector_to_pi8,
    AppState,
};

/// character ai implementation
pub mod ai;
/// character spawn system
pub mod character_spawner;
/// character components
pub mod components;
/// creep utility functions
pub mod creeps;
/// player plugin
pub mod player;
/// utilities for charactor entities
pub mod utils;

/// character functionality for game
pub struct CharactersPlugin;

impl Plugin for CharactersPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EventSpawnCharacter>();
        app.add_plugins((player::PlayerPlugin, ai::AIPlugin));

        app.add_systems(
            Update,
            (
                (
                    update_character_move_state,
                    character_spawner::creep_spawner_system,
                )
                    .run_if(in_state(AppState::PlayingGame)),
                character_spawner::spawn_character_on_event
                    .run_if(on_event::<EventSpawnCharacter>()),
            ),
        );
    }
}

/// spawn character in world
#[derive(Debug, Reflect, Clone, Event)]
pub struct EventSpawnCharacter {
    /// id of what too spawn and how many too spawn
    pub spawn_data: (RegistryIdentifier, i32),
    /// id of who requested spawn
    pub requester: Entity,
}

/// updates actors move status component based on actors velocity and speed attribute
fn update_character_move_state(
    mut anim_events: EventWriter<EventAnimationChange>,
    mut actor_query: Query<
        (Entity, &mut CharacterMoveState, &Velocity, &CharacterStats),
        Changed<Velocity>,
    >,
) {
    for (actor, mut move_state, velocity, stats) in &mut actor_query {
        let stats = stats.attrs();
        let abs = velocity.linvel.abs();
        let nrm = velocity.linvel.normalize();

        if abs.max_element() < MIN_VELOCITY {
            if move_state.move_status.0 != CurrentMovement::None {
                anim_events.send(EventAnimationChange {
                    anim_handle: AnimHandle::from_index(0),
                    actor,
                });
                move_state.move_status.0 = CurrentMovement::None;
                return;
            }
        } else if abs.max_element() <= (stats.move_speed * WALK_MODIFIER) {
            if move_state.move_status.0 != CurrentMovement::Walk {
                move_state.move_status.0 = CurrentMovement::Walk;
                return;
            }
        } else if abs.max_element() <= stats.move_speed
            && move_state.move_status.0 != CurrentMovement::Run
        {
            move_state.move_status.0 = CurrentMovement::Run;
            return;
        }

        let dir = vector_to_pi8(nrm);
        move_state.move_status.1 = dir;
    }
}
