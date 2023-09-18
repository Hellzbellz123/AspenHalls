use std::{fs::File, sync::Arc};
use tracing_subscriber::{prelude::*, EnvFilter, Registry};

pub mod prelude {
    //! The Bevy Log Prelude.
    #[doc(hidden)]
    pub use bevy::utils::tracing::{
        debug, debug_span, error, error_span, info, info_span, trace, trace_span, warn, warn_span,
    };
}

pub use bevy::utils::tracing::{
    debug, debug_span, error, error_span, info, info_span, trace, trace_span, warn, warn_span,
    Level,
};

use bevy::app::{App, Plugin};
use tracing_log::LogTracer;

/// Adds logging to Apps, logs too text file
/// * Using [`android_log-sys`](https://crates.io/crates/android_log-sys) on Android,
/// logging to Android logs.
/// You can configure this plugin.
/// ```no_run
/// # use bevy_app::{App, NoopPluginGroup as DefaultPlugins, PluginGroup};
/// # use bevy_log::LogPlugin;
/// # use bevy_utils::tracing::Level;
/// fn main() {
///     App::new()
///         .add_plugins(DefaultPlugins.set(LogPlugin {
///             level: Level::DEBUG,
///             filter: "wgpu=error,bevy_render=info,bevy_ecs=trace".to_string(),
///         }))
///         .run();
/// }
/// ```
/// Log level can also be changed using the `RUST_LOG` environment variable.
/// For example, using `RUST_LOG=wgpu=error,bevy_render=info,bevy_ecs=trace cargo run ..`
/// It has the same syntax as the field [`LogPlugin::filter`], see [`EnvFilter`].
/// If you define the `RUST_LOG` environment variable, the [`LogPlugin`] settings
/// will be ignored. \
/// # Panics
/// This plugin should not be added multiple times in the same process. This plugin
/// sets up global logging configuration for **all** Apps in a given process, and
/// rerunning the same initialization multiple times will lead to a panic.
pub struct VCLogPlugin {
    /// Filters logs using the [`EnvFilter`] format
    pub filter: String,

    /// Filters out logs that are "less than" the given level.
    /// This can be further filtered using the `filter` setting.
    pub level: Level,
}

impl Default for VCLogPlugin {
    fn default() -> Self {
        Self {
            filter: "wgpu=error".to_string(),
            level: Level::TRACE,
        }
    }
}

impl Plugin for VCLogPlugin {
    #[cfg_attr(not(feature = "tracing-chrome"), allow(unused_variables))]
    fn build(&self, app: &mut App) {
        LogTracer::init().unwrap();
        let default_filter = { format!("{},{}", self.level, self.filter) };

        let filter_layer = EnvFilter::try_from_default_env()
            .or_else(|_| EnvFilter::try_new(&default_filter))
            .unwrap();

        let subscriber = Registry::default()
            .with(filter_layer)
            .with(tracing_error::ErrorLayer::default());

        #[cfg(all(not(target_arch = "wasm32"), not(target_os = "android")))]
        {
            let stdout_log = tracing_subscriber::fmt::layer();
            // A layer that logs events to a file.
            let file = File::create("debug.log");
            let file = match file {
                Ok(file) => file,
                Err(error) => panic!("Error: {error:?}"),
            };
            let debug_log = tracing_subscriber::fmt::layer()
                .pretty()
                .with_ansi(false)
                .with_writer(Arc::new(file));

            let subscriber = subscriber.with(
                stdout_log
                    // Combine the filtered `stdout_log` layer with the
                    // `debug_log` layer, producing a new `Layered` layer.
                    .and_then(debug_log),
            );

            bevy::utils::tracing::subscriber::set_global_default(subscriber)
                .expect("Could not set global default tracing subscriber. If you've already set up a tracing subscriber, please disable LogPlugin from Bevy's DefaultPlugins");
        }

        #[cfg(target_os = "android")]
        {
            use crate::utilities::android_tracing::AndroidLayer;
            let subscriber = subscriber.with(AndroidLayer::default());
            bevy::utils::tracing::subscriber::set_global_default(subscriber)
                .expect("Could not set global default tracing subscriber. If you've already set up a tracing subscriber, please disable LogPlugin from Bevy's DefaultPlugins");
        }
    }
}
