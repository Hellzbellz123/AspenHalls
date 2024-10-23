use bevy::prelude::*;
use bevy_asepritesheet::sprite::AnimHandle;
use bevy_rapier2d::dynamics::Velocity;
use rand::prelude::{thread_rng, Rng};

use crate::{
    consts::{MIN_VELOCITY, WALK_MODIFIER},
    game::{
        animations::{CharacterAnimations, EventAnimationChange},
        attributes_stats::CharacterStats,
        characters::{
            boss::EventSpawnBoss,
            components::{CharacterInventory, CharacterMoveState, CharacterType, CurrentMovement},
            creeps::EventSpawnCreep,
        },
        game_world::components::CharacterSpawner,
    },
    loading::registry::{ActorRegistry, RegistryIdentifier},
    register_types,
    utilities::vector_to_pi8,
    AppState,
};

/// character ai implementation
pub mod ai;
/// boss util functions
pub mod boss;
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
        register_types!(app, [CharacterType, CharacterMoveState, CharacterInventory]);

        app.add_event::<EventSpawnCharacter>();
        app.add_plugins((
            player::PlayerPlugin,
            ai::AIPlugin,
            boss::BossPlugin,
            creeps::CreepPlugin,
        ));

        app.add_systems(
            Update,
            (
                (update_character_move_state,).run_if(in_state(AppState::PlayingGame)),
                spawn_character_on_event.run_if(on_event::<EventSpawnCharacter>()),
            ),
        );
    }
}

// TODO: should this be an enum?
/// spawn character in world
#[derive(Debug, Reflect, Clone, Event)]
pub struct EventSpawnCharacter {
    /// id of what too spawn and how many too spawn
    pub identifier: RegistryIdentifier,
    /// id of who requested spawn
    pub requester: Entity,
}

impl Default for EventSpawnCharacter {
    fn default() -> Self {
        Self {
            identifier: RegistryIdentifier::default(),
            requester: Entity::PLACEHOLDER,
        }
    }
}

/// takes character spawn events and gets position and passes event along
/// delegate character spawns
pub fn spawn_character_on_event(
    spawners: Query<&CharacterSpawner>,
    global_transforms: Query<&GlobalTransform>,
    registry: Res<ActorRegistry>,
    mut character_requests: EventReader<EventSpawnCharacter>,
    mut creep_events: EventWriter<EventSpawnCreep>,
    mut boss_events: EventWriter<EventSpawnBoss>,
) {
    for event in character_requests.read() {
        handle_character_spawn(
            &global_transforms,
            event,
            &registry,
            &spawners,
            &mut creep_events,
            &mut boss_events,
        );
    }
}

/// delegates requested charcter spawns based on character type
fn handle_character_spawn(
    global_transforms: &Query<'_, '_, &GlobalTransform>,
    event: &EventSpawnCharacter,
    registry: &Res<'_, ActorRegistry>,
    spawners: &Query<'_, '_, &CharacterSpawner>,
    creep_events: &mut EventWriter<'_, EventSpawnCreep>,
    boss_events: &mut EventWriter<'_, EventSpawnBoss>,
) {
    let Ok(requester_transform) = global_transforms.get(event.requester) else {
        error!("entity requesting teleport does not have a transform");
        return;
    };
    let mut rng = thread_rng();
    let spawn_pos = requester_transform.translation().truncate();

    let Some(character_type) = registry.characters.get_character_type(&event.identifier) else {
        error!(
            "requested item did not exist in character registry: {:?}",
            event.identifier
        );
        return;
    };

    let mut random_radius = |x: f32| rng.gen_range(-(x * 0.45)..(x * 0.45));

    match character_type {
        CharacterType::Creep => {
            let spawn_pos = spawners
                .get(event.requester)
                .map_or(spawn_pos, |spawner| Vec2 {
                    x: spawn_pos.x + random_radius(spawner.spawn_radius),
                    y: spawn_pos.y + random_radius(spawner.spawn_radius),
                });

            creep_events.send(EventSpawnCreep {
                actor_id: event.identifier.clone(),
                spawner: event.requester,
                position: spawn_pos,
            });
        }
        CharacterType::Boss => {
            info!("got boss character type");
            let spawn_pos = spawners
                .get(event.requester)
                .map_or(spawn_pos, |spawner| Vec2 {
                    x: spawn_pos.x + random_radius(spawner.spawn_radius),
                    y: spawn_pos.y + random_radius(spawner.spawn_radius),
                });

            boss_events.send(EventSpawnBoss {
                actor_id: event.identifier.clone(),
                spawner: event.requester,
                position: spawn_pos,
            });
        }
        CharacterType::Hero
        | CharacterType::CreepElite
        | CharacterType::MiniBoss
        | CharacterType::Critter
        | CharacterType::HeroPet
        | CharacterType::Shopkeep => {
            info!("character type unimplemented");
        }
    }
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
        let total_velocity = velocity.linvel.abs();
        let velocity = velocity.linvel;

        let walk_speed = stats.base_speed * WALK_MODIFIER;

        let dir = vector_to_pi8(velocity);
        move_state.move_status.1 = dir;

        if (MIN_VELOCITY..=walk_speed).contains(&total_velocity.length()) {
            // walking
            if move_state.move_status.0 != CurrentMovement::Walk {
                move_state.move_status.0 = CurrentMovement::Walk;
                continue;
            }
        } else if total_velocity.length() > walk_speed {
            // running
            if move_state.move_status.0 != CurrentMovement::Run {
                move_state.move_status.0 = CurrentMovement::Run;
                continue;
            }
        } else {
            // not moving
            if move_state.move_status.0 != CurrentMovement::None {
                move_state.move_status.0 = CurrentMovement::None;
                anim_events.send(EventAnimationChange {
                    anim_handle: AnimHandle::from_index(CharacterAnimations::IDLE),
                    actor,
                });
                continue;
            }
        }
    }
}
