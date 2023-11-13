use crate::ahp::{
    engine::{
        bevy, info, warn, Assets, DespawnRecursiveExt, Entity, Image, NonSend, Query, Res, Window,
        With,
    },
    game::InitAssetHandles,
};
// use bevy::{
//     ecs::system::NonSend,
//     prelude::{
//         info, warn, DespawnRecursiveExt, Entity, Query, With,
//     },
//     window::Window,
// };
use std::ops::Mul;
use winit::window::Icon;

/// # Panics
/// will panic if it cant find a window to attach icons, or the icon is not present
pub fn set_window_icon(
    // we have to use `NonSend` here
    window_query: Query<Entity, &Window>,
    init_assets: Res<InitAssetHandles>,
    image_assets: Res<Assets<Image>>,
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
            let favicon = image_assets
                .get(&init_assets.img_favicon)
                .expect("if this system is running this exists");
            let image = favicon.clone().try_into_dynamic().unwrap().into_rgba8();
            //  match image::open("assets/favicon.png") {
            //     Ok(img) => img.into_rgba8(),
            //     Err(e) => {
            //         warn!("couldnt load window icon: {}", e);
            //         return;
            //     }
            // };
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

/// takes array of types and runs `app.register_type::<T>()` on each
#[allow(unused_macros)]
macro_rules! register_types {
    ($app:expr, [ $($t:ty),* ]) => {
        $(
            $app.register_type::<$t>();
        )*
    };
}

use bevy::ecs::query::{ReadOnlyWorldQuery, WorldQuery};

/// get either mutably util function
pub trait GetEitherMut<'world, Element, Filter = ()>
where
    Element: WorldQuery,
    Filter: ReadOnlyWorldQuery,
{
    /// mutable get either entity
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
        if self.get(this).is_ok() {
            to_query = this;
        } else if self.get(otherwise).is_ok() {
            to_query = otherwise;
        } else {
            return None;
        };

        self.get_mut(to_query).ok()
    }
}

/// trait allowing get either for a readonly world query
pub trait GetEither<'world, Element, Filter = ()>
where
    Element: ReadOnlyWorldQuery,
{
    /// returns one of two elements from world query
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
        if self.get(this).is_ok() {
            to_query = this;
        } else if self.get(otherwise).is_ok() {
            to_query = otherwise;
        } else {
            return None;
        };

        self.get(to_query).ok()
    }
}
