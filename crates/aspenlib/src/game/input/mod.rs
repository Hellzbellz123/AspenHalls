use bevy::{
    ecs::schedule::IntoSystemSetConfigs, prelude::*, utils::Instant, window::PrimaryWindow,
};
use leafwing_input_manager::{
    action_state::{ActionData, Timing},
    axislike::DualAxisData,
    buttonlike::ButtonState,
    prelude::ActionState,
};
use std::{hash::Hash, time::Duration};

use crate::{
    ahp::{
        engine::{App, InputManagerPlugin, InputManagerSystem, Plugin, PreUpdate, SystemSet},
        game::MainCamera,
    },
    game::{action_maps::Gameplay, actors::components::Player},
    AppState,
};

/// holds action maps
pub mod action_maps;
/// keyboard input systems
mod kbm;
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
        // updates LookWorld and LookLocal based off mouse position inside window
        app.add_plugins(kbm::KBMPlugin);
        // TODO: make software cursor an option in the settings, mostly only useful for debugging
        app.add_plugins(software_cursor::SoftwareCursorPlugin);

        app.add_systems(
            PreUpdate,
            fake_mouse_input
                .after(AspenInputSystemSet::TouchInput)
                .run_if(state_exists_and_equals(AppState::PlayingGame)),
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
fn fake_mouse_input(
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut player_input: Query<&mut ActionState<Gameplay>, With<Player>>,
) {
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
