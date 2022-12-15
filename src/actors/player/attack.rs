use bevy::prelude::*;
use bevy_mouse_tracking_plugin::MousePos;

use leafwing_input_manager::prelude::*;

use crate::{action_manager::actions::PlayerActions, components::actors::general::MovementState};

pub enum AttackEventType {
    Melee,
    Ranged,
}

pub struct PlayerShootEvent {}

pub struct PlayerMeleeEvent {}

/// send shoot request to gun control system.
pub fn player_attack_sender(
    mut input_query: Query<&ActionState<PlayerActions>, With<MovementState>>,
    mut shootwriter: EventWriter<PlayerShootEvent>,
    mut meleewriter: EventWriter<PlayerMeleeEvent>,
    mouse: Res<MousePos>,
) {
    let action_state = input_query.single_mut();

    if action_state.pressed(PlayerActions::Shoot) {
        info!("shooting towards: {:?}", mouse);
        shootwriter.send(PlayerShootEvent {})
    }
    if action_state.pressed(PlayerActions::Melee) {
        info!("meleeing towards: {:?}", mouse);
        meleewriter.send(PlayerMeleeEvent {})
    }
}
