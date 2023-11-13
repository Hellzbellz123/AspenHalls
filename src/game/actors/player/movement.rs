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
        input::action_maps,
    },
    loading::splashscreen::MainCamera,
};

/// adds velocity too player based of what movement keys are pressed
pub fn player_movement_system(
    query_action_state: Query<&ActionState<action_maps::Gameplay>, With<Player>>,
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

    let (mut velocity, mut anim_state, _texture, speed_attr, ()) = player_query.single_mut();
    let action_state = query_action_state.single();
    let delta;

    if action_state.pressed(action_maps::Gameplay::Move) {
        // Virtual direction pads are one of the types which return an AxisPair
        let axis_pair = action_state.axis_pair(action_maps::Gameplay::Move).unwrap();
        delta = axis_pair.xy();

        let new_velocity = Velocity::linear(delta * speed_attr.speed);

        *velocity = new_velocity;
    } else {
        if velocity.linvel.length() <= 0.01 {
            velocity.linvel = Vec2::ZERO;
        } else {
            velocity.linvel = velocity.linvel.lerp(Vec2::ZERO, 0.2);
        }
        anim_state.animation_type = ActorAnimationType::Idle;
    }
}

/// modifies players movement speed based on sprint button
pub fn player_sprint(
    mut player_query: Query<(
        &mut AnimState,
        &mut Player,
        &ActionState<action_maps::Gameplay>,
        &mut ActorTertiaryAttributes,
    )>,
) {
    if player_query.is_empty() {
        return;
    }

    let (mut animation, mut player, action_state, mut speed_atr) = player_query.single_mut();

    if action_state.pressed(action_maps::Gameplay::Sprint) {
        animation.timer.set_duration(Duration::from_millis(100));
        player.sprint_available = true;
    }

    if action_state.released(action_maps::Gameplay::Sprint) {
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
    time: Res<Time>,
    mut main_camera_query: Query<(&mut Transform, &MainCamera)>,
    player_move_query: Query<(&Transform, &Velocity), (With<Player>, Without<MainCamera>)>,
) {
    if player_move_query.is_empty() {
        debug!("No Players too focus camera on");
        return;
    }
    if main_camera_query.is_empty() {
        debug!("No camera too move");
        return;
    }

    let (mut camera_trans, camera_data) = main_camera_query.single_mut();
    let (player_transform, player_velocity) = player_move_query.single();
    let camera_transform = camera_trans.translation.truncate();

    let camera_target = player_transform.translation.truncate()
        + (player_velocity.linvel * camera_data.look_ahead_factor);

    // Calculate the movement speed based on time.delta()
    let movement_speed: f32 =
        if player_velocity.linvel.abs().length() > camera_data.lerp_change_magnitude {
            camera_data.camera_speed * time.delta_seconds()
        } else {
            camera_data.changed_speed
        };

    // Interpolate (lerp) between the current camera position and the player's position with the adjusted speed
    camera_trans.translation = camera_transform
        .lerp(camera_target, movement_speed)
        .extend(999.0);
}
