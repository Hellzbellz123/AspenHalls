use std::time::Duration;

use bevy::prelude::{Query, With, *};
use leafwing_input_manager::prelude::ActionState;

use crate::{
    action_manager::actions::GameActions, characters::player::PlayerComponent, game::TimeInfo,
    loading::assets::RexTextureAssets,
};

use super::player_animation::{FacingDirection, FrameAnimation};

pub fn player_movement_system(
    _player_animations: Res<RexTextureAssets>,
    timeinfo: ResMut<TimeInfo>,
    query_action_state: Query<&ActionState<GameActions>>,
    time: Res<Time>,
    mut player_query: Query<(
        &mut Transform,
        &mut PlayerComponent,
        &mut Handle<TextureAtlas>,
        &mut TextureAtlasSprite,
    )>,
) {
    // let movement_dir = Vec3::ZERO;
    let (mut player_transform, mut player, _texture_atlas_handle, mut texture) =
        player_query.single_mut();
    let timeinfo = timeinfo.as_ref();

    if player.sprint_available {
        player.speed = 255.0
    }

    if !player.sprint_available {
        player.speed = 100.0
    }

    let action_state = query_action_state.single();
    if action_state.pressed(GameActions::Move) {
        // Virtual direction pads are one of the types which return an AxisPair. The values will be
        // represented as `-1.0`, `0.0`, or `1.0` depending on the combination of buttons pressed.
        let axis_pair = action_state.axis_pair(GameActions::Move).unwrap();

        let horizontal = axis_pair.x();
        let vertical = axis_pair.y();
        let mut velocity = Vec3::ZERO;

        if horizontal <= -0.1 && !timeinfo.game_paused {
            texture.flip_x = true;
            velocity.x += horizontal
                * player.speed
                * time.delta_seconds()
                * timeinfo.time_step.clamp(-1.0, 1.0);
            player.facing = FacingDirection::Left
        }

        if horizontal >= 0.1 && !timeinfo.game_paused {
            texture.flip_x = false;
            velocity.x += horizontal
                * player.speed
                * time.delta_seconds()
                * timeinfo.time_step.clamp(-1.0, 1.0);
            player.facing = FacingDirection::Right
        }

        if vertical <= -0.1 && !timeinfo.game_paused {
            velocity.y += vertical * player.speed * time.delta_seconds() * timeinfo.time_step;
            player.facing = FacingDirection::Down
        }

        if vertical >= 0.1 && !timeinfo.game_paused {
            player_transform.translation.y +=
                vertical * player.speed * time.delta_seconds() * timeinfo.time_step;
            player.facing = FacingDirection::Up
        }
        player_transform.translation += velocity;
    } else if action_state.released(GameActions::Move) {
        player.facing = FacingDirection::Idle
    }
}

pub fn player_sprint(
    mut input_query: Query<&ActionState<GameActions> , With<PlayerComponent>>,
    mut player_query: Query<&mut PlayerComponent>,
    mut anim_query: Query<&mut FrameAnimation, With<PlayerComponent>>
) {
    // let (mut player_transform, mut player, _texture_atlas_handle, mut texture) =
    //     player_query.single_mut();


    let action_state = input_query.single_mut();
    let mut animation = anim_query.single_mut();
    let mut player = player_query.single_mut();

    if action_state.pressed(GameActions::Dash) {
        animation.timer.set_duration(Duration::from_millis(100));
        // animation.timer= Timer::from_seconds(0.1, true);
        player.sprint_available = true;
    }

    if action_state.released(GameActions::Dash) {
        animation.timer.set_duration(Duration::from_millis(200));
        // animation.timer= Timer::from_seconds(0.1, true);
        player.sprint_available = false;
    }
}
