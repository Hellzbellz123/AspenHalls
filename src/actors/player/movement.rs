use std::time::Duration;

use bevy::prelude::{Query, With, *};
use heron::Velocity;
use kayak_ui::bevy::CameraUiKayak;
use leafwing_input_manager::prelude::ActionState;

use crate::{
    action_manager::actions::PlayerBindables,
    actors::player::animation::{AnimState, FacingDirection},
    actors::player::PlayerState,
    game::TimeInfo,
};

pub fn player_movement_system(
    timeinfo: ResMut<TimeInfo>,
    query_action_state: Query<&ActionState<PlayerBindables>>,
    mut player_query: Query<(
        &mut Velocity,
        &mut PlayerState,
        &mut TextureAtlasSprite,
        With<Transform>,
    )>,
) {
    let (mut velocity, mut player, mut texture, _) = player_query.single_mut();
    let action_state = query_action_state.single();
    let timeinfo = timeinfo.as_ref();
    let delta;

    if action_state.pressed(PlayerBindables::Move) {
        // Virtual direction pads are one of the types which return an AxisPair
        let axis_pair = action_state.axis_pair(PlayerBindables::Move).unwrap();

        delta = axis_pair.xy();

        let horizontal = axis_pair.x();
        let vertical = axis_pair.y();

        if horizontal < 0.0 && !timeinfo.game_paused {
            texture.flip_x = true;
            player.facing = FacingDirection::Right;
        } else if horizontal > 0.0 && !timeinfo.game_paused {
            texture.flip_x = false;
            player.facing = FacingDirection::Left;
        }
        if vertical < 0.0 && !timeinfo.game_paused {
            player.facing = FacingDirection::Down;
        } else if vertical > 0.0 && !timeinfo.game_paused {
            player.facing = FacingDirection::Up;
        }

        let new_velocity = Velocity::from_linear(
            delta.extend(0.0).normalize_or_zero() * player.speed * timeinfo.time_step,
        );

        *velocity = new_velocity;
    } else if !action_state.pressed(PlayerBindables::Move) {
        player.facing = FacingDirection::Idle;
        let new_velocity = Velocity::from_linear(Vec3::ZERO);
        *velocity = new_velocity;
    }
}

pub fn player_sprint(
    mut input_query: Query<&ActionState<PlayerBindables>, With<PlayerState>>,
    mut player_query: Query<&mut PlayerState>,
    mut anim_query: Query<&mut AnimState, With<PlayerState>>,
) {
    let action_state = input_query.single_mut();
    let mut animation = anim_query.single_mut();
    let mut player = player_query.single_mut();

    if action_state.pressed(PlayerBindables::Dash) {
        animation.timer.set_duration(Duration::from_millis(100));
        player.sprint_available = true;
    }

    if action_state.released(PlayerBindables::Dash) {
        animation.timer.set_duration(Duration::from_millis(200));
        player.sprint_available = false;
    }

    if player.sprint_available {
        player.speed = 255.0;
    } else {
        player.speed = 150.0;
    }
}

pub fn camera_movement_system(
    mut camera_transform: Query<&mut Transform, (Without<CameraUiKayak>, With<Camera>)>,
    player_transform: Query<&Transform, (With<PlayerState>, Without<Camera>)>,
) {
    let mut camera_trans = camera_transform.single_mut();
    let player_trans = player_transform.single();

    camera_trans.translation = camera_trans
        .translation
        .lerp(player_trans.translation, 0.05);
}