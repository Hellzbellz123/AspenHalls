use bevy::prelude::*;
use bevy_console::{AddConsoleCommand, ConsoleConfiguration, ConsolePlugin};
use tracing_subscriber::Registry;

/// holds definitions of commands
mod commands;
/// holds systems that are used by commands
mod systems;

/// adds Debug/Cheat Console functionality
/// toggled with `grave` key
pub struct QuakeConPlugin;

impl Plugin for QuakeConPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.is_plugin_added::<bevy_egui::EguiPlugin>());

        app.add_plugins(ConsolePlugin)
            .insert_resource(ConsoleConfiguration {
                keys: vec![KeyCode::Backquote],
                left_pos: 200.0,
                top_pos: 100.0,
                height: 400.0,
                width: 800.0,
                history_size: 50,
                symbol: "asha$ ".to_owned(),
                ..default()
            })
            .add_console_command::<commands::SpawnActorCommand, _>(systems::spawn_command)
            .add_console_command::<commands::TeleportCharacterCommand, _>(
                systems::teleport_command,
            );
    }
}

/// initializes log capture too ingame console unless your on a wasm target
/// wasm doesnt support std:time and somewhere in the log chain its used, causing a crash.
pub fn init_log_layers(
    app: &mut App,
) -> Option<Box<dyn tracing_subscriber::Layer<Registry> + Send + Sync>> {
    if cfg!(not(target_family = "wasm")) {
        bevy_console::make_layer(app)
    } else {
        None
    }
}
