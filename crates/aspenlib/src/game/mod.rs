use avian2d::prelude::Collider;
use bevy::{prelude::*, render::primitives::Aabb};

use crate::{
    bundles::NeedsCollider,
    game::{
        components::{ActorColliderType, TimeToLive},
        game_world::dungeonator_v2::GeneratorState,
    },
    playing_game, register_types,
    utilities::EntityCreator, GameStage,
};

/// animation functionality
pub mod animations;
/// character/item stats functionality
pub mod attributes_stats;
/// audio data for game
pub mod audio;
/// game characters spawning and functionality
pub mod characters;
/// combat functionality plugin
pub mod combat;
/// shared components for game
pub mod components;
/// sanctuary and dungeon generator
pub mod game_world;
/// input from player
pub mod input;
/// Game `UserInterface` Module, contains interface plugin
pub mod interface;
/// game item spawning and functionality
pub mod items;
/// player progression module
pub mod progress;

/// are we in dungeon yet?
#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Resource, Default, Reflect)]
pub enum GameProgress {
    /// homeroom
    #[default]
    Sanctuary,
    /// in dungeon now
    Dungeon,
}

/// each dungeon run has 4 stages that get progressivly larger/harder
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum DungeonFloor {
    /// first floor of dungeon, easiest enemys, start here
    #[default]
    One,
    /// slighlty deeper, bit larger, more creeps
    Two,
    /// has more enemys/minibosses
    Three,
    /// final level of the dungeon
    Four,
}

/// plugin that holds all game functionality as plugin modules
pub struct AspenHallsPlugin;

impl Plugin for AspenHallsPlugin {
    fn build(&self, app: &mut App) {
        register_types!(app, [ActorColliderType, EntityCreator]);

        app
            // actual game plugin
            .add_plugins((
                progress::GameProgressPlugin,
                audio::AudioPlugin,
                combat::CombatPlugin,
                characters::CharactersPlugin,
                items::ItemsPlugin,
                input::InputPlugin,
                game_world::GameWorldPlugin,
                interface::InterfacePlugin,
                animations::AnimationsPlugin,
            ))
            .add_systems(Update, time_to_live.run_if(playing_game()))
            .add_systems(
                PostUpdate,
                add_aabb_based_colliders
                    .run_if(
                        any_with_component::<NeedsCollider>.and(
                            in_state(GameStage::SelectCharacter)
                                .or(in_state(GeneratorState::FinishedDungeonGen)),
                        ),
                    )
                    .after(TransformSystem::TransformPropagate),
            );
    }
}

/// despawn any entity with `TimeToLive` timer thats finished
fn time_to_live(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut TimeToLive)>,
) {
    for (entity, mut timer) in &mut query {
        if timer.tick(time.delta()).finished() {
            let Ok(mut a) = commands.get_entity(entity) else {
                continue;
            };
            a.try_despawn();
        }
    }
}

/// adds colliders too entitys that should have colliders using the aabb pf the entity
fn add_aabb_based_colliders(
    mut cmds: Commands,
    aabbs_q: Query<&Aabb>,
    needscollider_q: Query<(Entity, &ChildOf, &NeedsCollider), Without<Collider>>,
) {
    for (needs_collider, parent, collider_type) in &needscollider_q {
        let collider = match collider_type {
            NeedsCollider::Aabb => {
                let Ok(aabb) = aabbs_q.get(parent.parent()) else {
                    continue;
                };
                let start = Vec2::ZERO
                    + Vec2 {
                        x: 0.0,
                        y: aabb.half_extents.x / 2.0,
                    };
                let end = Vec2::ZERO
                    + Vec2 {
                        x: 0.0,
                        y: aabb.half_extents.y,
                    };

                Collider::capsule_endpoints(aabb.half_extents.x / 2.0, start, end)
            }
            NeedsCollider::Rectangle { x, y } => Collider::rectangle(*x, *y),
        };

        cmds.entity(needs_collider)
            .remove::<NeedsCollider>()
            .insert((collider, GeneratedCollider));
    }
}

#[derive(Component)]
pub struct GeneratedCollider;
