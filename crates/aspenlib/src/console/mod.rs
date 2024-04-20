use bevy::prelude::*;
use bevy_console::{AddConsoleCommand, ConsoleConfiguration, ConsolePlugin};

/// holds definitions of commands
mod commands;
/// holds systems that are used by commands
mod systems;

/// adds Debug/Cheat Console functionality
/// toggled with `grave` key
pub struct QuakeConPlugin;

impl Plugin for QuakeConPlugin {
    fn build(&self, app: &mut App) {
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

// TODO make a global subscriber that takes this line and prints it
// too a tab in the console page, 3 tabs, a repl like tab, cmds tab, and a log tab
// fn write_to_console(mut console_line: EventWriter<PrintConsoleLine>) {
//     console_line.send(PrintConsoleLine::new("Hello".to_string()));
// }
