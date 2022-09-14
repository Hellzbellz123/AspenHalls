use std::time::Duration;

use bevy::prelude::{Query, With, *};
use heron::Velocity;
use kayak_ui::bevy::CameraUiKayak;
use leafwing_input_manager::prelude::ActionState;

use crate::{
    action_manager::actions::PlayerBindables,
    characters::player::animation::{AnimState, FacingDirection},
    characters::player::PlayerState,
    game::TimeInfo,
    game_world::level::components::Collides,
    loading::assets::RexTextureHandles,
};

pub fn player_movement_system(
    _player_animations: Res<RexTextureHandles>,
    timeinfo: ResMut<TimeInfo>,
    query_action_state: Query<&ActionState<PlayerBindables>>,
    _time: Res<Time>,
    _wall_collider_query: Query<(&Transform, &Collides, Without<PlayerState>)>,
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
    let mut delta = Vec2::ZERO;

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
    } else if action_state.released(PlayerBindables::Move) {
        delta = Vec2::ZERO;
        player.facing = FacingDirection::Idle;
        let new_velocity = Velocity::from_linear(Vec3::ZERO);
        *velocity = new_velocity;
    }
    // info!("player velocity: {:?}", velocity);
    // info!("player dpad delta{:?}", delta);
}

// mut camera_query: Query<(&mut Transform, &Camera)>
// let mut camera_transform = camera_query.single();
// camera_transform.0.translation = player_transform.translation;
// info!("moving camera using {}, and {}", player_pos, camera_pos);

// if horizontal <= -0.1 && !timeinfo.game_paused {
//     player.facing = FacingDirection::Left;
// }
// if horizontal >= 0.2 && !timeinfo.game_paused {
//     player.facing = FacingDirection::Right;
// }
// if vertical <= -0.2 && !timeinfo.game_paused {
//     player.facing = FacingDirection::Down
// }
// if vertical >= 0.1 && !timeinfo.game_paused {
//     player.facing = FacingDirection::Up
// }

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
    mut querymany: ParamSet<(
        Query<(&mut Transform, &Camera), Without<CameraUiKayak>>,
        Query<&mut Transform, With<PlayerState>>,
    )>,
) {
    let camera_trans = querymany.p0().single_mut().0.translation;
    let player_trans = querymany.p1().single_mut().translation;

    querymany.p0().single_mut().0.translation = camera_trans.lerp(player_trans, 0.05);
}
