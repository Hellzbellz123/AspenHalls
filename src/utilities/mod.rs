use bevy::{prelude::*, window::PrimaryWindow};
use std::ops::Mul;

use crate::loading::splashscreen::MainCameraTag;

/// sets window icon and title
pub mod window;

/// holds general game utilities
/// not particularly related to gameplay
pub struct UtilitiesPlugin;

impl Plugin for UtilitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (window::set_window_icon, eager_cursor_pos))
            .insert_resource(EagerMousePos {
                world: Vec2::ZERO,
                window: Vec2::ZERO,
            });
    }
}

/// mouse position eagerly updated for latency focused stuff. probably bad, will find out
#[derive(Debug, Resource, Clone, Copy, PartialEq, Component)]
pub struct EagerMousePos {
    /// mouse pos in world space
    pub world: Vec2,
    /// mouse pos in window coords
    pub window: Vec2,
}
/// updates `EagerMousePos` resource with current mouse position and mouse pos translated to world space. no change detection, always runs
fn eager_cursor_pos(
    mut fast_mouse_pos: ResMut<EagerMousePos>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCameraTag>>,
    main_window: Query<&Window, With<PrimaryWindow>>,
) {
    if q_camera.is_empty() {
        return;
    }
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // // Games typically only have one window (the primary window).
    // // For multi-window applications, you need to use a specific window ID here.
    let wnd = main_window.single();

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width(), wnd.height());

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();

        // world_pos.y = -world_pos.y;
        // screen_pos.y = -screen_pos.y;

        fast_mouse_pos.world = world_pos;
        fast_mouse_pos.window = screen_pos;
        // info!("eager mouse update {}", world_pos);
    } else {
        fast_mouse_pos.world = Vec2::ZERO;
        fast_mouse_pos.window = Vec2::ZERO;
        // info!("eager mouse not in window");
    }
}

/// despawn any entity with T: Component
pub fn despawn_with<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    to_despawn.for_each(|entity| {
        info!("despawning entity recursively: {:#?}", entity);
        commands.entity(entity).despawn_recursive();
    });
}

/// Performs a linear interpolation between `from` and `to` based on the value `s`.
///
/// When `s` is `0.0`, the result will be equal to `self`.  When `s` is `1.0`, the result
/// will be equal to `rhs`. When `s` is outside of range `[0, 1]`, the result is linearly
/// extrapolated.
#[must_use]
pub fn lerp<T>(from: T, to: T, s: T) -> T
where
    <T as std::ops::Sub>::Output: Mul<T>,
    T: std::ops::Sub<Output = T>
        + std::ops::Add<Output = T>
        + std::ops::Mul<Output = T>
        + std::marker::Copy,
{
    from + ((to - from) * s)
}
/// simple macro that generates an add system for OnEnter(state)
#[allow(unused_macros)]
macro_rules! state_exists_and_entered {
    ($system_name:ident, $state:expr) => {
        app.add_systems(OnEnter($state), $system_name)
            .run_if(state_exists_and_equals($state))
    };
}