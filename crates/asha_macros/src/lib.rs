//! macros for ashacrop activities

/// takes array of types and runs `app.register_type::<T>()` on each
#[allow(unused_macros)]
#[macro_export]
macro_rules! register_types {
    ($app:expr_2021, [ $($t:ty),* ]) => {
        $(
            $app.register_type::<$t>();
        )*
    };
}

/// simple macro that generates an add system for OnEnter(state)
#[allow(unused_macros)]
#[macro_export]
macro_rules! on_enter {
    ($system_name:ident, $state:expr_2021) => {
        app.add_systems(OnEnter($state), $system_name)
            .run_if(in_state($state))
    };
}
