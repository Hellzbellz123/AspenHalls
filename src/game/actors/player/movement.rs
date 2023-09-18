use std::time::Duration;

use bevy::prelude::{Query, With, *};

use bevy_rapier2d::prelude::Velocity;
use leafwing_input_manager::prelude::ActionState;

use crate::{
    game::{
        actors::{
            animation::components::{ActorAnimationType, AnimState},
            components::{ActorTertiaryAttributes, Player},
        },
        input::actions,
    },
    loading::splashscreen::MainCameraTag,
};

/// adds velocity too player based of what movement keys are pressed
pub fn player_movement_system(
    query_action_state: Query<&ActionState<actions::Combat>, With<Player>>,
    mut player_query: Query<(
        &mut Velocity,
        &mut AnimState,
        &mut TextureAtlasSprite,
        &ActorTertiaryAttributes,
        With<Player>,
    )>,
) {
    if player_query.is_empty() {
        return;
    }

    let (mut velocity, mut anim_state, mut texture, speed_attr, ()) = player_query.single_mut();
    let action_state = query_action_state.single();
    let delta;

    if action_state.pressed(actions::Combat::Move) {
        // Virtual direction pads are one of the types which return an AxisPair
        let axis_pair = action_state.axis_pair(actions::Combat::Move).unwrap();

        delta = axis_pair.xy().clamp_length(-1.0, 1.0);

        let horizontal = axis_pair.x();
        let vertical = axis_pair.y();

        if horizontal < 0.0 {
            texture.flip_x = true;
            anim_state.facing = ActorAnimationType::Right;
        } else if horizontal > 0.0 {
            texture.flip_x = false;
            anim_state.facing = ActorAnimationType::Left;
        }

        if vertical < 0.0 {
            anim_state.facing = ActorAnimationType::Down;
        } else if vertical > 0.0 {
            anim_state.facing = ActorAnimationType::Up;
        }

        let new_velocity = Velocity::linear(delta.normalize_or_zero() * speed_attr.speed);

        *velocity = new_velocity;
    } else if !action_state.pressed(actions::Combat::Move) {
        velocity.linvel = velocity.linvel.lerp(Vec2::ZERO, 0.2);
        anim_state.facing = ActorAnimationType::Idle;
    }
}

/// modifies players movement speed based on sprint button
pub fn player_sprint(
    mut player_query: Query<(
        &mut AnimState,
        &mut Player,
        &ActionState<actions::Combat>,
        &mut ActorTertiaryAttributes,
    )>,
) {
    if player_query.is_empty() {
        return;
    }

    let (mut animation, mut player, action_state, mut speed_atr) = player_query.single_mut();

    if action_state.pressed(actions::Combat::Sprint) {
        animation.timer.set_duration(Duration::from_millis(100));
        player.sprint_available = true;
    }

    if action_state.released(actions::Combat::Sprint) {
        animation.timer.set_duration(Duration::from_millis(200));
        player.sprint_available = false;
    }

    if player.sprint_available {
        speed_atr.speed = 255.0;
    } else {
        speed_atr.speed = 155.0;
    }
}

/// keeps camera centered on player
pub fn camera_movement_system(
    mut camera_transform: Query<(&mut Transform, &MainCameraTag), With<Camera>>,
    player_transform: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    if player_transform.is_empty() || camera_transform.is_empty() {
        return;
    }
    let (mut camera_trans, tag) = camera_transform.single_mut();
    let player_transform = player_transform.single().translation.truncate();
    let camera_transform = camera_trans.translation.truncate();

    if tag.is_active {
        camera_trans.translation = (camera_transform.lerp(player_transform, 0.05)).extend(999.0);
    }
}
