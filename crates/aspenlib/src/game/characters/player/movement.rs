use bevy::prelude::{Query, With, *};

use bevy_rapier2d::prelude::Velocity;
use leafwing_input_manager::prelude::ActionState;

use crate::{
    consts::{MIN_VELOCITY, SPRINT_MODIFIER, WALK_MODIFIER},
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
        (&mut Velocity, &CharacterMoveState, &CharacterStats),
        With<PlayerSelectedHero>,
    >,
) {
    let (mut velocity, move_state, player_stats) = match player_query.get_single_mut() {
        Ok(query) => query,
        Err(e) => {
            warn!("unable too update player velocity: {}", e);
            return;
        }
    };

    let Some(delta) = actions.clamped_axis_pair(&action_maps::Gameplay::Move) else {return;};

    let speed = if actions.pressed(&action_maps::Gameplay::Sprint)
        && move_state.move_perms == AllowedMovement::Run
    {
        player_stats.attrs().move_speed * SPRINT_MODIFIER
    } else {
        player_stats.attrs().move_speed * WALK_MODIFIER
    };

    let new_velocity = Velocity::linear(delta.xy().clamp_length_max(1.0) * speed);

    *velocity = new_velocity;
}

/// keeps camera centered on player
#[allow(clippy::type_complexity)]
pub fn camera_movement_system(
    time: Res<Time>,
    mut main_camera_query: Query<(&mut Transform, &MainCamera)>,
    player_move_query: Query<
        (&Transform, &Velocity),
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
