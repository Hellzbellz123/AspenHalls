use bevy::prelude::{Query, With, *};

use avian2d::prelude::LinearVelocity;
use leafwing_input_manager::prelude::ActionState;

use crate::{
    consts::{SPRINT_MODIFIER, WALK_MODIFIER},
    game::{
        attributes_stats::CharacterStats,
        characters::{
            components::{AllowedMovement, CharacterMoveState},
            player::PlayerSelectedHero,
        },
        input::action_maps,
    },
    loading::splashscreen::MainCamera,
};

/// adds velocity too player based off what movement keys are pressed
pub fn update_player_velocity(
    actions: Res<ActionState<action_maps::Gameplay>>,
    // TODO: use global settings resource
    mut player_query: Query<
        (&mut LinearVelocity, &CharacterMoveState, &CharacterStats),
        With<PlayerSelectedHero>,
    >,
) {
    let (mut velocity, move_state, player_stats) = match player_query.single_mut() {
        Ok(query) => query,
        Err(e) => {
            warn!("unable too update player velocity: {}", e);
            return;
        }
    };

    let delta = actions.clamped_axis_pair(&action_maps::Gameplay::Move);

    let speed = if actions.pressed(&action_maps::Gameplay::Sprint)
        && move_state.move_perms == AllowedMovement::Run
    {
        player_stats.attrs().base_speed * SPRINT_MODIFIER
    } else {
        player_stats.attrs().base_speed * WALK_MODIFIER
    };

    let new_velocity = LinearVelocity(delta.xy() * speed);

    *velocity = new_velocity;
}

/// keeps camera centered on player
#[allow(clippy::type_complexity)]
pub fn camera_movement_system(
    time: Res<Time>,
    mut main_camera_query: Query<(&mut Transform, &MainCamera)>,
    player_move_query: Query<
        (&Transform, &LinearVelocity),
        (With<PlayerSelectedHero>, Without<MainCamera>),
    >,
) {
    if player_move_query.is_empty() {
        debug!("No Players too focus camera on");
        return;
    }
    if main_camera_query.is_empty() {
        debug!("No camera too move");
        return;
    }

    let Ok((mut camera_trans, camera_data)) = main_camera_query.single_mut() else {return;};
    let Ok((player_transform, player_velocity)) = player_move_query.single() else {return;};
    let camera_transform = camera_trans.translation.truncate();

    let scaled_player_velocity = Vec2 {
        x: player_velocity.x * camera_data.movement_scales.x,
        y: player_velocity.y * camera_data.movement_scales.y,
    };

    let camera_target = player_transform.translation.truncate()
        + (scaled_player_velocity * camera_data.look_ahead_factor);

    // Calculate the movement speed based on time.delta()
    let movement_speed: f32 = if player_velocity.abs().length() > camera_data.lerp_change_magnitude
    {
        camera_data.recenter_speed * time.delta_secs()
    } else {
        camera_data.player_still_recenter_speed
    };

    if camera_transform.is_finite() {
        let distance = camera_transform.distance(player_transform.translation.truncate());

        if distance > 500.0 {
            camera_trans.translation = player_transform.translation.truncate().extend(999.0);
        } else {
            // Interpolate (lerp) between the current camera position and the player's position with the adjusted speed
            camera_trans.translation = camera_transform
                .lerp(camera_target, movement_speed)
                .extend(999.0);
        }
    } else {
        camera_trans.translation = player_transform.translation.truncate().extend(999.0);
    }
}
