use bevy::{
    app::Update,
    ecs::{query::Changed, system::Query},
};
use bevy_rapier2d::dynamics::Velocity;

use crate::{
    ahp::engine::{App, Plugin},
    consts::{MIN_VELOCITY, WALK_MODIFIER},
    game::actors::components::{ActorMoveState, CurrentMovement},
};

/// all functionality for artificial intelligence on actors is stored here
pub mod ai;
/// holds animation functionality for actors plugin
pub mod animation;
/// holds player stat functionality
pub mod attributes_stats;
/// game combat functionality
pub mod combat;
/// shared actor components
pub mod components;
/// holds enemy functionality
pub mod enemies;
/// holds player information and functions
pub mod player;
/// holds spawner info
pub mod spawners;

/// all Characters in the game, along with spawners for spawn able characters
// TODO: make actors "configurable". load actor types from $PACK/definitions/$ACTORTYPE/ and add them too a database.
// use this database for "available actors" when spawning
pub struct ActorPlugin;

impl Plugin for ActorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            spawners::SpawnerPlugin,
            animation::AnimationPlugin,
            player::PlayerPlugin,
            combat::ActorWeaponPlugin,
            enemies::EnemyPlugin,
            ai::AIPlugin,
        ))
        .add_systems(Update, update_actor_move_status);
    }
}

/// updates actors move status component based on actors velocity and speed attribute
fn update_actor_move_status(
    mut actor_query: Query<
        (
            &mut ActorMoveState,
            &Velocity,
            &components::ActorTertiaryAttributes,
        ),
        Changed<Velocity>,
    >,
) {
    for (mut move_state, velocity, tert_attrs) in &mut actor_query {
        if velocity.linvel.abs().max_element() < MIN_VELOCITY {
            if move_state.move_status != CurrentMovement::None {
                move_state.move_status = CurrentMovement::None;
                return;
            }
        } else if velocity.linvel.abs().max_element() <= (tert_attrs.speed * WALK_MODIFIER) {
            if move_state.move_status != CurrentMovement::Walk {
                move_state.move_status = CurrentMovement::Walk;
                return;
            }
        } else if velocity.linvel.abs().max_element() <= tert_attrs.speed
            && move_state.move_status != CurrentMovement::Run
        {
            move_state.move_status = CurrentMovement::Run;
            return;
        }
    }
}
