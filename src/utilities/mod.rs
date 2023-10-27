use bevy::{
    ecs::system::NonSend,
    prelude::{
        info, warn, Assets, Commands, DespawnRecursiveExt, Entity, Query, Res, ResMut, With, Name,
    },
    window::Window, render::view::NoFrustumCulling,
};
use bevy_tiling_background::{BackgroundImageBundle, BackgroundMaterial, SetImageRepeatingExt};
use std::ops::Mul;
use winit::window::Icon;

/// # Panics
/// will panic if it cant find a window to attach icons, or the icon is not present
pub fn set_window_icon(
    // we have to use `NonSend` here
    window_query: Query<Entity, &Window>,
    windows: NonSend<bevy::winit::WinitWindows>,
) {
    if let Ok(main_window) = window_query.get_single() {
        let Some(winit_window) = windows.get_window(main_window) else {
            warn!("NO WINDOW TOO SET ICON");
            return;
        };

        // here we use the `image` crate to load our icon data from a png file
        // this is not a very bevy-native solution, but it will do
        let (icon_rgba, icon_width, icon_height) = {
            let image = match image::open("assets/favicon.png") {
                Ok(img) => img.into_rgba8(),
                Err(e) => {
                    warn!("couldnt load window icon: {}", e);
                    return;
                }
            };
            let (width, height) = image.dimensions();
            let rgba = image.into_raw();
            (rgba, width, height)
        };

        let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height)
            .expect("the icon is not the correct size");

        winit_window.set_window_icon(Some(icon));
    }
}

/// despawn any entity with T: Component
pub fn despawn_with<T: bevy::prelude::Component>(
    to_despawn: Query<Entity, With<T>>,
    mut commands: bevy::prelude::Commands,
) {
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


use bevy::{
    ecs::query::{ReadOnlyWorldQuery, WorldQuery},
    prelude::*,
};

pub trait GetEitherMut<'world, Element, Filter = ()>
where
    Element: WorldQuery,
    Filter: ReadOnlyWorldQuery,
{
    fn get_either_mut(&mut self, this: Entity, otherwise: Entity) -> Option<Element::Item<'_>>;
}

impl<'world, 'state, Element, Filter> GetEitherMut<'world, Element, Filter>
    for Query<'world, 'state, Element, Filter>
where
    Element: WorldQuery,
    Filter: ReadOnlyWorldQuery,
{
    fn get_either_mut(&mut self, this: Entity, otherwise: Entity) -> Option<Element::Item<'_>> {
        let to_query: Entity;
        if let Ok(_) = self.get(this) {
            to_query = this;
        } else if let Ok(_) = self.get(otherwise) {
            to_query = otherwise;
        } else {
            return None;
        };

        self.get_mut(to_query).ok()
    }
}

pub trait GetEither<'world, Element, Filter = ()>
where
    Element: ReadOnlyWorldQuery,
{
    fn get_either(&self, this: Entity, otherwise: Entity) -> Option<Element::Item<'_>>;
}

impl<'world, 'state, Element, Filter> GetEither<'world, Element, Filter>
    for Query<'world, 'state, Element, Filter>
where
    Element: ReadOnlyWorldQuery,
    Filter: ReadOnlyWorldQuery,
{
    fn get_either(&self, this: Entity, otherwise: Entity) -> Option<Element::Item<'_>> {
        let to_query: Entity;
        if let Ok(_) = self.get(this) {
            to_query = this;
        } else if let Ok(_) = self.get(otherwise) {
            to_query = otherwise;
        } else {
            return None;
        };

        self.get(to_query).ok()
    }
}