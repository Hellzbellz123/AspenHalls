//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]  no terminal on windows in release?
use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    // Window setup
    app.insert_resource(WindowDescriptor {
        title: "Mine Sweeper!".to_string(),
        width: 700.,
        height: 800.,
        ..Default::default()
    });
    // Bevy default plugins
    app.add_plugins(DefaultPlugins);
    app.run();
}
