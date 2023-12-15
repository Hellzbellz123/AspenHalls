use bevy::{prelude::*, window::PrimaryWindow};
use leafwing_input_manager::{
    action_state::ActionStateDriverTarget,
    axislike::DualAxisData,
    prelude::{ActionState, ActionStateDriver},
};

use crate::{loading::splashscreen::MainCamera, ahp::game::Player};

use super::{
    action_maps::{self},
    AspenInputSystemSet,
};

/// adds mouse input functionality too app
pub struct KBMPlugin;

impl Plugin for KBMPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            apply_look_driver.run_if(
                // any_with_component::<Player>()
                // .and_then(
                run_once(), // ),
            ),
        )
        .add_systems(
            PreUpdate,
            update_cursor_state_from_window
                .run_if(
                    any_with_component::<ActionState<action_maps::Gameplay>>()
                        .and_then(any_with_component::<ActionStateDriver<action_maps::Gameplay>>()),
                )
                .in_set(AspenInputSystemSet::KBMInput),
        );
    }
}

/// adds look driver too window for updating `Gameplay::Look`
fn apply_look_driver(
    mut commands: Commands,
    window_query: Query<
        Entity,
        (
            With<PrimaryWindow>,
            Without<ActionStateDriver<action_maps::Gameplay>>,
        ),
    >,
    // player_query: Query<Entity, With<Player>>,
) {
    commands
        .entity(window_query.single())
        .insert(ActionStateDriver {
            action: action_maps::Gameplay::CursorScreen,
            targets: ActionStateDriverTarget::None, // player_query.single().into(),
        });
}

/// updates cursor position in look action with winit window cursor position
fn update_cursor_state_from_window(
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut action_state_query: Query<&mut ActionState<action_maps::Gameplay>>,
) {
    let window = window_query.single();
    let mut action_state = action_state_query.single_mut();
    let (camera, camera_global_transform) = camera_query.single();

    let mut new_cursor_local: Vec2 = Vec2 {
        x: window.width() / 2.0,
        y: window.height() / 2.0,
    };
    let mut new_cursor_world = camera
        .viewport_to_world_2d(camera_global_transform, new_cursor_local)
        .unwrap_or_default();

    if let Some(cursor_local_pos) = window.cursor_position() {
        let cursor_world_pos = camera
            .viewport_to_world_2d(camera_global_transform, cursor_local_pos)
            .unwrap_or_else(|| {
                warn!("Could not get cursors world position");
                new_cursor_world
            });

        new_cursor_local = cursor_local_pos;
        new_cursor_world = cursor_world_pos;
    }

    action_state
        .action_data_mut(action_maps::Gameplay::CursorScreen)
        .axis_pair = Some(DualAxisData::from_xy(new_cursor_local));
    action_state
        .action_data_mut(action_maps::Gameplay::CursorWorld)
        .axis_pair = Some(DualAxisData::from_xy(new_cursor_world));
}

/// updates cursor position in look action with winit window cursor position
fn update_cursor_state_from_window_old(
    window_query: Query<&Window>,
    mut action_state_query: Query<&mut ActionState<action_maps::Gameplay>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    // Update action state with the mouse position from the window
    let window = window_query.single();
    let mut action_state = action_state_query.single_mut();
    let (camera, camera_global_transform) = camera_query.single();

    if let Some(cursor_local_pos) = window.cursor_position() {
        let cursor_world_pos = camera
            .viewport_to_world_2d(camera_global_transform, cursor_local_pos)
            .unwrap_or_else(|| {
                warn!("Could not get cursors world position");
                Vec2::ZERO
            });

        action_state
            .action_data_mut(action_maps::Gameplay::CursorScreen)
            .axis_pair = Some(DualAxisData::from_xy(cursor_local_pos));

        action_state
            .action_data_mut(action_maps::Gameplay::CursorWorld)
            .axis_pair = Some(DualAxisData::from_xy(cursor_world_pos));
    } else {
        let window_size = Vec2::from_array([window.width(), window.height()]);
        let window_center_world = camera
            .viewport_to_world_2d(camera_global_transform, window_size / 2.0)
            .unwrap_or(Vec2::ZERO);
        action_state
            .action_data_mut(action_maps::Gameplay::CursorScreen)
            .axis_pair = Some(DualAxisData::from_xy(window_size / 2.0));
        action_state
            .action_data_mut(action_maps::Gameplay::CursorWorld)
            .axis_pair = Some(DualAxisData::from_xy(window_center_world));
    }
}
