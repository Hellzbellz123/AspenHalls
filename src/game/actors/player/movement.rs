use std::time::Duration;

use bevy::prelude::{Query, With, *};

use bevy_rapier2d::prelude::Velocity;
use leafwing_input_manager::prelude::ActionState;

use crate::{
    components::{
        actors::{
            animation::{AnimState, FacingDirection},
            general::{MovementState, Player},
        },
        MainCameraTag,
    },
    game::{input::actions, TimeInfo},
};

pub fn player_movement_system(
    timeinfo: ResMut<TimeInfo>,
    query_action_state: Query<&ActionState<actions::Combat>, With<Player>>,
    mut player_query: Query<(
        &mut Velocity,
        &mut MovementState,
        &mut TextureAtlasSprite,
        With<Player>,
    )>,
) {
    if timeinfo.game_paused {
        return;
    }

    let (mut velocity, mut player, mut texture, _) = player_query.single_mut();
    let action_state = query_action_state.single();
    let timeinfo = timeinfo.as_ref();
    let delta;

    if action_state.pressed(actions::Combat::Move) {
        // Virtual direction pads are one of the types which return an AxisPair
        let axis_pair = action_state.axis_pair(actions::Combat::Move).unwrap();

        delta = axis_pair.xy().clamp_length(-1.0, 1.0);

        let horizontal = axis_pair.x();
        let vertical = axis_pair.y();

        if horizontal < 0.0 {
            texture.flip_x = true;
            player.facing = FacingDirection::Right;
        } else if horizontal > 0.0 {
            texture.flip_x = false;
            player.facing = FacingDirection::Left;
        }

        if vertical < 0.0 {
            player.facing = FacingDirection::Down;
        } else if vertical > 0.0 {
            player.facing = FacingDirection::Up;
        }

        let new_velocity =
            Velocity::linear(delta.normalize_or_zero() * player.speed * timeinfo.time_step);

        *velocity = new_velocity;
    } else if !action_state.pressed(actions::Combat::Move) {
        velocity.linvel = velocity.linvel.lerp(Vec2::ZERO, 0.2);
        player.facing = FacingDirection::Idle;
    }
}

pub fn player_sprint(
    mut input_query: Query<&ActionState<actions::Combat>, With<MovementState>>,
    mut player_query: Query<&mut MovementState, With<Player>>,
    mut anim_query: Query<&mut AnimState, With<Player>>,
) {
    if input_query.is_empty() || player_query.is_empty() || anim_query.is_empty() {
        return;
    }

    let action_state = input_query.single_mut();
    let mut animation = anim_query.single_mut();
    let mut player = player_query.single_mut();

    if action_state.pressed(actions::Combat::Sprint) {
        animation.timer.set_duration(Duration::from_millis(100));
        player.sprint_available = true;
    }

    if action_state.released(actions::Combat::Sprint) {
        animation.timer.set_duration(Duration::from_millis(200));
        player.sprint_available = false;
    }

    if player.sprint_available {
        player.speed = 255.0;
    } else {
        player.speed = 155.0;
    }
}

pub fn camera_movement_system(
    mut camera_transform: Query<(&mut Transform, &MainCameraTag), With<Camera>>,
    player_transform: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    if player_transform.is_empty() || camera_transform.is_empty() {
        return;
    }
    let (mut camera_trans, tag) = camera_transform.single_mut();
    let playertrans = player_transform.single().translation.truncate();
    let camtrans = camera_trans.translation.truncate();

    if tag.is_active {
        camera_trans.translation = (camtrans.lerp(playertrans, 0.05)).extend(999.0);
    }
}