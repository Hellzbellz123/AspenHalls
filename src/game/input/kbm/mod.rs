use bevy::{prelude::*, window::PrimaryWindow};
use leafwing_input_manager::{
    axislike::DualAxisData,
    prelude::{ActionState, ActionStateDriver},
    systems::run_if_enabled,
};

use crate::{game::actors::components::Player, loading::splashscreen::MainCameraTag};

use super::{actions::{self}, InternalInputSet};

/// holds general game utilities
/// not particularly related to gameplay
pub struct KBMPlugin;

impl Plugin for KBMPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            apply_look_driver.run_if(any_with_component::<Player>().and_then(run_once())),
        )
        .add_systems(
            PreUpdate,
            update_cursor_state_from_window
                .run_if(
                    run_if_enabled::<actions::Gameplay>
                        .and_then(any_with_component::<ActionStateDriver<actions::Gameplay>>()),
                )
                .in_set(InternalInputSet::KBMInput),
        );
    }
}

fn apply_look_driver(
    mut commands: Commands,
    window: Query<
        Entity,
        (
            With<PrimaryWindow>,
            Without<ActionStateDriver<actions::Gameplay>>,
        ),
    >,
    player_query: Query<Entity, With<Player>>,
) {
    commands.entity(window.single()).insert(ActionStateDriver {
        action: actions::Gameplay::LookLocal,
        targets: player_query.single().into(),
    });
}

fn update_cursor_state_from_window(
    window_query: Query<&Window>,
    mut action_state_query: Query<&mut ActionState<actions::Gameplay>>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCameraTag>>,
) {
    // Update each actionstate with the mouse position from the window
    // by using the referenced entities in ActionStateDriver and the stored action as
    // a key into the action data
    action_state_query.for_each_mut(|f| {
        let mut action_state = f;
        let window = window_query.single();
        let (camera, camera_global_transform) = camera.single();

        if let Some(cursor_local_pos) = window.cursor_position() {

            let cursor_world_pos = camera
                .viewport_to_world_2d(camera_global_transform, cursor_local_pos)
                .unwrap_or_else(|| {
                    warn!("Could not get cursors world position");
                    Vec2::ZERO
                });

                action_state
            .action_data_mut(actions::Gameplay::LookLocal)
            .axis_pair = Some(DualAxisData::from_xy(cursor_local_pos));

            action_state
                .action_data_mut(actions::Gameplay::LookWorld)
                .axis_pair = Some(DualAxisData::from_xy(cursor_world_pos));
        } else if action_state
            .action_data(actions::Gameplay::LookLocal)
            .axis_pair
            == None
            || action_state
                .action_data(actions::Gameplay::LookWorld)
                .axis_pair
                == None
        {
            // no cursor inside window.a
            // TODO: how does this interact with touch control
            // set point too window center
            let window_size = Vec2::from_array([window.width(), window.height()]);
            let window_center_world = camera.viewport_to_world_2d(camera_global_transform, window_size / 2.0).unwrap_or(Vec2::ZERO);
            action_state
                .action_data_mut(actions::Gameplay::LookLocal)
                .axis_pair = Some(DualAxisData::from_xy(window_size / 2.0));
            action_state
                .action_data_mut(actions::Gameplay::LookWorld)
                .axis_pair = Some(DualAxisData::from_xy(window_center_world));
        }
    });
}
