use std::{hash::Hash, time::Duration};

use bevy::{
    ecs::schedule::IntoSystemSetConfigs, prelude::*, utils::Instant, window::PrimaryWindow,
};
use leafwing_input_manager::{
    action_state::{ActionData, ActionStateDriverTarget, Timing},
    axislike::DualAxisData,
    buttonlike::ButtonState,
    prelude::{ActionState, ActionStateDriver},
};

use crate::{
    game::action_maps::Gameplay,
    prelude::{
        engine::{App, InputManagerPlugin, InputManagerSystem, Plugin, PreUpdate, SystemSet},
        game::MainCamera,
    },
    AppState,
};

/// holds action maps
pub mod action_maps;
/// software cursor plugin updated with touch and kbm input settings
mod software_cursor;
/// touch input systems
mod touch;

/// system set for ordering input related systems
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AspenInputSystemSet {
    /// KBM input is collected first
    KBMInput,
    /// Then touch input is collected, overwriting KBM input if touches present
    TouchInput,
    /// software cursor is updated after mouse/touch input is added
    SoftwareCursor,
}

/// player input plugin
pub struct ActionsPlugin;

// holds default bindings for game
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<action_maps::Gameplay>::default());
        // TODO: make this plugin only active by default if target_platform == (ANDROID || IOS) else make it a setting too enable
        app.add_plugins(touch::TouchInputPlugin);
        // TODO: make software cursor an option in the settings, mostly only useful for debugging
        app.add_plugins(software_cursor::SoftwareCursorPlugin);

        app.add_systems(Update, apply_look_driver.run_if(run_once()))
            .add_systems(
                PreUpdate,
                fake_mouse_input_from_joystick
                    .after(AspenInputSystemSet::TouchInput)
                    .run_if(state_exists_and_equals(AppState::PlayingGame).and_then(
                        |q: Query<&ActionState<Gameplay>>| {
                            q.single()
                                .action_data(Gameplay::JoystickDelta)
                                .axis_pair
                                .is_some_and(|f| f.xy().max_element().abs() > 0.0)
                        },
                    )),
            )
            .add_systems(
                PreUpdate,
                update_cursor_state_from_window
                    .run_if(
                        any_with_component::<ActionState<action_maps::Gameplay>>().and_then(
                            any_with_component::<ActionStateDriver<action_maps::Gameplay>>(),
                        ),
                    )
                    .in_set(AspenInputSystemSet::KBMInput),
            );

        app.configure_sets(
            PreUpdate,
            (
                AspenInputSystemSet::KBMInput,
                AspenInputSystemSet::TouchInput,
                AspenInputSystemSet::SoftwareCursor,
            )
                .chain()
                .in_set(InputManagerSystem::ManualControl),
        );
    }
}

/// creates fake mouse input using a joystick x/y value and the window x/y value
fn fake_mouse_input_from_joystick(
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut player_input: Query<&mut ActionState<Gameplay>>,
) {
    debug!("updating fake mouselook");

    let mut player_input = player_input.single_mut();
    let joy_axis = player_input
        .action_data(Gameplay::JoystickDelta)
        .axis_pair
        .unwrap()
        .xy();
    let window = window_query.single();
    if joy_axis == Vec2::ZERO {
        return;
    }

    let (camera, camera_transform) = camera_query.single();
    let dead_zone: f32 = 0.000_5;
    // Adjust this threshold as needed

    // Apply dead zone to joystick input
    let normalized_input = {
        // Adjust input values to be centered around (0,0)
        let centered_x = dead_zone.mul_add(-joy_axis.x.signum(), joy_axis.x);
        let centered_y = dead_zone.mul_add(-joy_axis.y.signum(), joy_axis.y);

        // Normalize input values to [-1.0, 1.0]
        let normalized_x = centered_x / (1.0 - dead_zone);
        let normalized_y = centered_y / (1.0 - dead_zone);

        Vec2::new(normalized_x, normalized_y)
    };

    let fake_cursor_position = Vec2::new(
        (normalized_input.x * window.width() / 2.0) + (window.width() / 2.0),
        (-normalized_input.y * window.height() / 2.0) + (window.height() / 2.0),
    );

    let fake_world_position = camera
        .viewport_to_world_2d(camera_transform, fake_cursor_position)
        .unwrap_or_else(|| {
            warn!("no cursor");
            Vec2::ZERO
        });

    trace!("fake_world_position: {:?}", fake_world_position);
    trace!("fake_local_position: {:?}", fake_cursor_position);

    player_input.set_action_data(
        Gameplay::CursorScreen,
        ActionData {
            axis_pair: Some(DualAxisData::from_xy(fake_cursor_position)),
            consumed: false,
            state: ButtonState::JustPressed,
            value: 1.0,
            timing: Timing {
                instant_started: Some(Instant::now()),
                current_duration: Duration::from_secs(1),
                previous_duration: Duration::from_secs(0),
            },
        },
    );

    player_input.set_action_data(
        Gameplay::CursorWorld,
        ActionData {
            axis_pair: Some(DualAxisData::from_xy(fake_world_position)),
            consumed: false,
            state: ButtonState::JustPressed,
            value: 1.0,
            timing: Timing {
                instant_started: Some(Instant::now()),
                current_duration: Duration::from_secs(1),
                previous_duration: Duration::from_secs(0),
            },
        },
    );
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
) {
    commands
        .entity(window_query.single())
        .insert(ActionStateDriver {
            action: action_maps::Gameplay::CursorScreen,
            targets: ActionStateDriverTarget::None,
        });
}

/// updates cursor position in look action with winit window cursor position
fn update_cursor_state_from_window(
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut action_state_query: Query<&mut ActionState<action_maps::Gameplay>>,
) {
    let window = window_query.single();
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

    let mut action_state = action_state_query.single_mut();

    action_state
        .action_data_mut(action_maps::Gameplay::CursorScreen)
        .axis_pair = Some(DualAxisData::from_xy(new_cursor_local));
    action_state
        .action_data_mut(action_maps::Gameplay::CursorWorld)
        .axis_pair = Some(DualAxisData::from_xy(new_cursor_world));
}
