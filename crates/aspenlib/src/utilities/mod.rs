#![allow(unused)]

use std::{cmp::Ordering, ops::Mul};
use winit::window::Icon;

use bevy::{
    ecs::{query::WorldQuery, system::EntityCommands},
    log::warn,
    prelude::*,
    window::CursorGrabMode,
};
use bevy_rapier2d::{pipeline::CollisionEvent, rapier::geometry::CollisionEventFlags};

use crate::{
    consts::TILE_SIZE, game::characters::components::MoveDirection,
    loading::assets::AspenInitHandles,
};

/// hold reference too entity that spawned entity with this component
#[derive(Debug, Component, Deref, DerefMut, Reflect)]
#[reflect(Component)]
pub struct EntityCreator(pub Entity);

/// takes array of types and runs `app.register_type::<T>()` on each
#[allow(unused_macros)]
#[macro_export]
macro_rules! register_types {
    ($app:expr, [ $($t:ty),* ]) => {
        $(
            $app.register_type::<$t>();
        )*
    };
}

/// simple macro that generates an add system for OnEnter(state)
#[allow(unused_macros)]
macro_rules! on_enter {
    ($system_name:ident, $state:expr) => {
        app.add_systems(OnEnter($state), $system_name)
            .run_if(in_state($state))
    };
}

/// # Panics
/// will panic if it cant find a window to attach icons, or the icon is not present
pub fn set_window_icon(
    window_query: Query<(Entity, &Window)>,
    init_assets: Res<AspenInitHandles>,
    image_assets: Res<Assets<Image>>,
    // we have to use `NonSend` here
    windows: NonSend<bevy::winit::WinitWindows>,
) {
    // TODO: use bevy internal window icon utilties

    if let Ok((main_window, bevy_window)) = window_query.get_single() {
        let Some(winit_window) = windows.get_window(main_window) else {
            warn!("NO WINDOW TOO SET ICON");
            return;
        };

        // here we use the `image` crate to load our icon data from a png file
        // this is not a very bevy-native solution, but it will do
        let (icon_rgba, icon_width, icon_height) = {
            let favicon = image_assets
                .get(&init_assets.img_favicon)
                .expect("if this system is running this exists");
            let image = favicon.clone().try_into_dynamic().unwrap().into_rgba8();
            let (width, height) = image.dimensions();
            let rgba = image.into_raw();
            (rgba, width, height)
        };

        let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height)
            .expect("the icon is not the correct size");

        winit_window.set_window_icon(Some(icon));
    }
}

/// scales a `Vec2` so its largest value is smaller than x/y of final size
pub fn scale_to_fit(current: Vec2, final_size: Vec2) -> Vec2 {
    // Calculate scaling factors for both dimensions
    let min_scale = (final_size.x / current.x).min(final_size.y / current.y);

    // Scale the Vec2
    Vec2 {
        x: current.x * min_scale,
        y: current.y * min_scale,
    }
}

/// converts tile amount too f32 value
pub fn tiles_to_f32(distance: i32) -> f32 {
    distance as f32 * TILE_SIZE
}

/// converts velocity too move direction in 8 cardinals
pub fn vector_to_pi8(vec: Vec2) -> MoveDirection {
    let angle = get_velocity_angle(vec);
    let index = ((angle + 22.5) / 45.0) as usize % 8;

    match index {
        0 => MoveDirection::East,
        1 => MoveDirection::NorthEast,
        2 => MoveDirection::North,
        3 => MoveDirection::NorthWest,
        4 => MoveDirection::West,
        5 => MoveDirection::SouthWest,
        6 => MoveDirection::South,
        _ => MoveDirection::SouthEast,
    }
}

/// converts velocity too move direction in 4 cardinals
pub fn vector_to_cardinal_direction(linear_velocity: Vec2) -> MoveDirection {
    if linear_velocity.x.abs() > linear_velocity.y.abs() {
        if linear_velocity.x >= 0.0 {
            MoveDirection::East
        } else {
            MoveDirection::West
        }
    } else if linear_velocity.y >= 0.0 {
        MoveDirection::North
    } else {
        MoveDirection::South
    }
}

// does this actually work the way im expecting?
/// returns normal direction of given velocity
fn get_velocity_angle(vec: Vec2) -> f32 {
    let mut angle = vec.y.atan2(vec.x).to_degrees();
    if angle < 0.0 {
        angle += 360.0;
    } else if angle >= 360.0 {
        angle -= 360.0;
    }

    angle
}

/// turns a collision event into its parts
pub const fn collision_to_data(
    event: &CollisionEvent,
) -> (Entity, Entity, &CollisionEventFlags, bool) {
    match event {
        CollisionEvent::Started(a, b, flags) => (*a, *b, flags, true),
        CollisionEvent::Stopped(a, b, flags) => (*a, *b, flags, false),
    }
}

/// despawn any entity with T: Component
pub fn despawn_with<T: Component>(
    to_despawn: Query<Entity, With<T>>,
    mut commands: bevy::prelude::Commands,
) {
    for entity in &to_despawn {
        info!("despawning entity recursively: {:#?}", entity);
        commands.entity(entity).despawn_recursive();
    }
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

/// `RunCondition` that checks if state was entered since last frame
/// resets if the state is left
///
/// returns false if the state does not exist in the world
pub fn state_exists_and_entered<S: States>(state: S) -> impl Condition<()> {
    IntoSystem::into_system(
        move |mut entered: Local<bool>, mut exists: Local<bool>, state_q: Option<Res<State<S>>>| {
            if state_q.is_some() {
                let current_state = state_q.unwrap();
                if (!*exists || !*entered) && *current_state.get() == state {
                    // debug!("state exists and wanted state value entered");
                    *entered = true;
                    *exists = true;
                    true
                } else if (*exists || *entered) && *current_state.get() != state {
                    // debug!("state was entered, but its not enterered anymore. resetting state machine");
                    *entered = false;
                    *exists = false;
                    false
                } else {
                    // debug!("state exists but its not the one we want");
                    false
                }
            } else {
                // debug!("state does not exist yet");
                *exists = false;
                false
            }
        },
    )
}

/// Generates a [`Condition`](bevy::ecs::Condition)-satisfying closure that returns `true`
/// if there are more `T` than the last run.
///
/// `T` must be a `Component`
///
/// # Note
/// this function works, but components must be added with the correct value for systems that use them
/// - things like `GlobalTransform` wont be accurate till 1 run later
pub fn on_component_added<T: Component>(
) -> impl FnMut((Local<usize>, Local<bool>, Query<(), With<T>>)) -> bool + Clone {
    move |(mut local_count, mut loop_once, query)| {
        if *loop_once {
            *loop_once = false;
            false
        } else {
            let last_count: usize = *local_count;
            let current_count = query.into_iter().len();

            // info!("run condition count: {:?}", *local);

            *loop_once = true;
            match current_count.cmp(&last_count) {
                // no change in component count
                Ordering::Equal => {
                    // info!("Condition(on_component_added): no change in component count ");
                    false
                }
                // entity with component was despawned or component was removed
                Ordering::Less => {
                    *local_count = current_count;
                    // info!("Condition(on_component_added): Component was removed from world");
                    false
                }
                // component was inserted on entity OR entity was spawned with component
                Ordering::Greater => {
                    *local_count = current_count;
                    // info!("Condition(on_component_added): Component was added too world");
                    true
                }
            }
        }
    }
}
