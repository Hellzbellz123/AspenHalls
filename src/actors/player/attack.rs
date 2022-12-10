use bevy::prelude::*;
use bevy_mouse_tracking_plugin::MousePos;

use leafwing_input_manager::prelude::*;

use crate::{
    action_manager::actions::PlayerBindables,
    components::actors::general::{MovementState, Player},
};

pub enum AttackEventType {
    Melee,
    Ranged,
}

pub struct PlayerShootEvent {
    pub eventtype: AttackEventType,
}

pub struct PlayerMeleeEvent {}

/// send shoot request to gun control system.
pub fn player_shoot_sender(
    mut input_query: Query<&ActionState<PlayerBindables>, With<MovementState>>,
    mut attackewriter: EventWriter<PlayerShootEvent>,
) {
    let action_state = input_query.single_mut();

    if action_state.pressed(PlayerBindables::Shoot) {
        attackewriter.send(PlayerShootEvent {
            eventtype: AttackEventType::Ranged,
        })
    }
    if action_state.pressed(PlayerBindables::Melee) {
        attackewriter.send(PlayerShootEvent {
            eventtype: AttackEventType::Melee,
        })
    }
}

pub fn player_melee(
    mouse: Res<MousePos>,
    attackreader: EventReader<PlayerShootEvent>,
    _player: Query<(&mut Player, &Transform), With<MovementState>>,
) {
    if !attackreader.is_empty() {
        info!("meleeing towards: {:?}", mouse);
    }
}
