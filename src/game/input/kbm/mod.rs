use bevy::{prelude::*, window::PrimaryWindow};
use leafwing_input_manager::{
    axislike::DualAxisData,
    prelude::{ActionState, ActionStateDriver},
    systems::run_if_enabled,
};

use crate::{game::actors::components::Player, loading::splashscreen::MainCameraTag};

use super::{
    action_maps::{self},
    AspenInputSystemSet,
};

/// adds mouse input functionality too app
pub struct KBMPlugin;

impl Plugin for KBMPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(
            Update,
            apply_look_driver.run_if(any_with_component::<Player>().and_then(run_once())),
        )
        .add_systems(
            PreUpdate,
            update_cursor_state_from_window
                .run_if(
                    run_if_enabled::<action_maps::Gameplay>
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
    player_query: Query<Entity, With<Player>>,
) {
    commands.entity(window_query.single()).insert(ActionStateDriver {
        action: action_maps::Gameplay::LookLocal,
        targets: player_query.single().into(),
    });
}

/// updates cursor position in look action with winit window cursor position
fn update_cursor_state_from_window(
    window_query: Query<&Window>,
    mut action_state_query: Query<&mut ActionState<action_maps::Gameplay>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCameraTag>>,
) {
    // Update each action state with the mouse position from the window
    // by using the referenced entities in ActionStateDriver and the stored action as
    // a key into the action data
    action_state_query.for_each_mut(|f| {
        let mut action_state = f;
        let window = window_query.single();
        let (camera, camera_global_transform) = camera_query.single();

        if let Some(cursor_local_pos) = window.cursor_position() {
            let cursor_world_pos = camera
                .viewport_to_world_2d(camera_global_transform, cursor_local_pos)
                .unwrap_or_else(|| {
                    warn!("Could not get cursors world position");
                    Vec2::ZERO
                });

            action_state
                .action_data_mut(action_maps::Gameplay::LookLocal)
                .axis_pair = Some(DualAxisData::from_xy(cursor_local_pos));

            action_state
                .action_data_mut(action_maps::Gameplay::LookWorld)
                .axis_pair = Some(DualAxisData::from_xy(cursor_world_pos));
        } else if action_state
            .action_data(action_maps::Gameplay::LookLocal)
            .axis_pair.is_none()
            || action_state
                .action_data(action_maps::Gameplay::LookWorld)
                .axis_pair.is_none()
        {
            // no cursor inside window.a
            // TODO: how does this interact with touch control
            // set point too window center
            let window_size = Vec2::from_array([window.width(), window.height()]);
            let window_center_world = camera
                .viewport_to_world_2d(camera_global_transform, window_size / 2.0)
                .unwrap_or(Vec2::ZERO);
            action_state
                .action_data_mut(action_maps::Gameplay::LookLocal)
                .axis_pair = Some(DualAxisData::from_xy(window_size / 2.0));
            action_state
                .action_data_mut(action_maps::Gameplay::LookWorld)
                .axis_pair = Some(DualAxisData::from_xy(window_center_world));
        }
    });
}
