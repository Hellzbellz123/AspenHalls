use std::time::Duration;

use bevy::prelude::{Query, With, *};
use kayak_ui::bevy::CameraUiKayak;
use leafwing_input_manager::prelude::ActionState;

use crate::{
    action_manager::actions::PlayerBindables, characters::player::PDataComponent, game::TimeInfo,
    loading::assets::RexTextureHandles,
};

use super::animation::{FacingDirection, TargetAnimation};

pub fn player_movement_system(
    _player_animations: Res<RexTextureHandles>,
    timeinfo: ResMut<TimeInfo>,
    query_action_state: Query<&ActionState<PlayerBindables>>,
    time: Res<Time>,
    mut player_query: Query<(
        &mut Transform,
        &mut PDataComponent,
        &mut Handle<TextureAtlas>,
        &mut TextureAtlasSprite,
    )>,
) {
    // let movement_dir = Vec3::ZERO;
    let (mut player_transform, mut player, _texture_atlas_handle, mut texture) =
        player_query.single_mut();
    let timeinfo = timeinfo.as_ref();

    if player.sprint_available {
        player.speed = 255.0;
    }

    if !player.sprint_available {
        player.speed = 100.0;
    }

    let action_state = query_action_state.single();
    if action_state.pressed(PlayerBindables::Move) {
        // Virtual direction pads are one of the types which return an AxisPair. The values will be
        // represented as `-1.0`, `0.0`, or `1.0` depending on the combination of buttons pressed.
        let axis_pair = action_state.axis_pair(PlayerBindables::Move).unwrap();

        let horizontal = axis_pair.x();
        let vertical = axis_pair.y();
        let mut velocity = Vec3::ZERO;

        if horizontal <= -0.1 && !timeinfo.game_paused {
            texture.flip_x = true;
            velocity.x += horizontal
                * player.speed
                * time.delta_seconds()
                * timeinfo.time_step.clamp(-1.0, 1.0);
            player.facing = FacingDirection::Left;
        }

        if horizontal >= 0.1 && !timeinfo.game_paused {
            texture.flip_x = false;
            velocity.x += horizontal
                * player.speed
                * time.delta_seconds()
                * timeinfo.time_step.clamp(-1.0, 1.0);
            player.facing = FacingDirection::Right;
        }

        if vertical <= -0.1 && !timeinfo.game_paused {
            velocity.y += vertical * player.speed * time.delta_seconds() * timeinfo.time_step;
            player.facing = FacingDirection::Down;
        }

        if vertical >= 0.1 && !timeinfo.game_paused {
            player_transform.translation.y +=
                vertical * player.speed * time.delta_seconds() * timeinfo.time_step;
            player.facing = FacingDirection::Up;
        }
        player_transform.translation += velocity;
    } else if action_state.released(PlayerBindables::Move) {
        player.facing = FacingDirection::Idle;
    }
}

pub fn player_sprint(
    mut input_query: Query<&ActionState<PlayerBindables>, With<PDataComponent>>,
    mut player_query: Query<&mut PDataComponent>,
    mut anim_query: Query<&mut TargetAnimation, With<PDataComponent>>,
) {
    // let (mut player_transform, mut player, _texture_atlas_handle, mut texture) =
    //     player_query.single_mut();

    let action_state = input_query.single_mut();
    let mut animation = anim_query.single_mut();
    let mut player = player_query.single_mut();

    if action_state.pressed(PlayerBindables::Dash) {
        animation.timer.set_duration(Duration::from_millis(100));
        // animation.timer= Timer::from_seconds(0.1, true);
        player.sprint_available = true;
    }

    if action_state.released(PlayerBindables::Dash) {
        animation.timer.set_duration(Duration::from_millis(200));
        // animation.timer= Timer::from_seconds(0.1, true);
        player.sprint_available = false;
    }
}

pub fn camera_movement_system(
    // trunk-ignore(clippy/type_complexity)
    mut querymany: ParamSet<(
        Query<(&mut Transform, &Camera), Without<CameraUiKayak>>,
        Query<&mut Transform, With<PDataComponent>>,
    )>, // mut camera_query: Query<(&mut Transform, &Camera)>
) {
    let camera_pos = querymany.p0().single_mut().0.translation;
    let player_pos = querymany.p1().single_mut().translation;
    // let mut camera_transform = camera_query.single();

    // camera_transform.0.translation = player_transform.translation;

    querymany.p0().single_mut().0.translation = querymany.p1().single_mut().translation;

    info!("moving camera using {}, and {}", player_pos, camera_pos)
}
