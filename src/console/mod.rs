use crate::ahp::engine::{
    bevy_console::{
        AddConsoleCommand, ConsoleConfiguration, ConsolePlugin,
        ToggleConsoleKey,
    },
    default, App, KeyCode, Plugin,
};
/// holds systems that are used by commands
pub mod command_systems;
/// holds definitions of commands
pub mod commands;

pub use command_systems::*;
pub use commands::*;

/// adds Debug/Cheat Console functionality
/// toggled with `grave` key
pub struct QuakeConPlugin;

impl Plugin for QuakeConPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ConsolePlugin)
            .insert_resource(ConsoleConfiguration {
                keys: vec![ToggleConsoleKey::KeyCode(KeyCode::Grave)],
                left_pos: 200.0,
                top_pos: 100.0,
                height: 400.0,
                width: 800.0,
                history_size: 50,
                symbol: "asha$ ".to_owned(),
                ..default()
            })
            .add_console_command::<SpawnEnemyCommand, _>(
                spawnenemy_command,
            )
            .add_console_command::<SpawnWeaponCommand, _>(
                spawnweapon_command,
            )
            .add_console_command::<TeleportPlayerCommand, _>(
                teleport_player_command,
            );
    }
}

// TODO: make this a macro or just a simple one shot, is tricky to do events.
// TODO better idea, make a global subscriber that takes this line and prints it
// too a tab in the console page, 3 tabs, a repl like tab, cmds tab, and a log tab
// fn write_to_console(mut console_line: EventWriter<PrintConsoleLine>) {
//     console_line.send(PrintConsoleLine::new("Hello".to_string()));
// }
