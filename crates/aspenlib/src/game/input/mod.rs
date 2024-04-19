use std::hash::Hash;

use bevy::{
    ecs::schedule::IntoSystemSetConfigs, prelude::*,
    window::PrimaryWindow,
};
use leafwing_input_manager::{
    plugin::{InputManagerPlugin, InputManagerSystem},
    prelude::{ActionState, InputMap},
};

use crate::{loading::splashscreen::MainCamera, register_types};

// / entity selection in playing stage handled with this plugin
// / TODO: implement targeting system for interactions/combat
// pub mod actor_targeting;

/// holds action maps
pub mod action_maps;
/// software cursor plugin updated with touch and kbm input settings
mod software_cursor;
/// touch input systems
mod touch_gamepad;

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
pub struct InputPlugin;

// holds default bindings for game
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        register_types!(app, [ActionState<action_maps::Gameplay>, InputMap<action_maps::Gameplay>, AspenCursorPosition]);

        // initial input plugin config
        app.add_plugins(InputManagerPlugin::<action_maps::Gameplay>::default())
            .init_resource::<ActionState<action_maps::Gameplay>>()
            .insert_resource(action_maps::Gameplay::default_input_map());

        // TODO: make this plugin only active by default if target_platform == (ANDROID || IOS) else make it a setting too enable
        app.add_plugins(touch_gamepad::TouchInputPlugin);
        // TODO: make software cursor an option in the settings, mostly only useful for debugging
        app.add_plugins(software_cursor::SoftwareCursorPlugin);
        // implement targeting system reticle that snaps too nearest interactable actor too cursor. interaction and pickup uses this system?
        // app.add_plugins(actor_targeting::ActorTargetingPlugin);

        app.insert_resource(AspenCursorPosition {
            world: Vec2::default(),
            screen: Vec2::default(),
        });

        app.add_systems(
            PreUpdate,
            (update_cursor_position_resource.before(AspenInputSystemSet::SoftwareCursor),),
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

#[derive(Debug, Resource, Reflect, Default)]
#[reflect(Resource)]
/// global cursor position resource,
/// faked for platforms with no cursor or touch only by joystick
pub struct AspenCursorPosition {
    /// cursor position in world space
    pub world: Vec2,
    /// cursor position in screen space
    pub screen: Vec2,
}

/// updates cursor position resource with joystick priority
fn update_cursor_position_resource(
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    input: Res<ActionState<action_maps::Gameplay>>,
    mut cursor_position: ResMut<AspenCursorPosition>,
    // mut last_position: Local<AspenCursorPosition>,
) {
    let window = window_query.single();
    let window_half_size = Vec2::new(window.width(), window.width()) / 2.0;
    let joy_axis = input.clamped_axis_pair(&action_maps::Gameplay::Look);

    let cursor_screen_pos: Vec2 = if joy_axis.is_some_and(|f| f.xy().abs() != Vec2::ZERO) {
        let joy_axis = joy_axis.unwrap().xy();
        Vec2::new(
            joy_axis.x.mul_add(window_half_size.x, window_half_size.x),
            (-joy_axis.y).mul_add(window_half_size.y, window_half_size.y)
        )
    } else {
        window_query
            .single()
            .cursor_position()
            .unwrap_or(window_half_size)
    };

    let Ok((camera, camera_pos)) = camera_query.get_single() else {
        return;
    };
    let cursor_world_pos = camera
        .viewport_to_world_2d(camera_pos, cursor_screen_pos)
        .unwrap_or(Vec2::ZERO);
    let new_cursor_position = AspenCursorPosition {
        world: cursor_world_pos,
        screen: cursor_screen_pos,
    };
    *cursor_position = new_cursor_position;
}
