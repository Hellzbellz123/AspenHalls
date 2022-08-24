use std::time::Duration;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_inspector_egui::WorldInspectorPlugin;

use crate::{characters::player::PlayerComponent, game::TimeInfo};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        let _registry = app
            .world
            .get_resource_or_insert_with(bevy_inspector_egui::InspectableRegistry::default);

        app.add_plugin(WorldInspectorPlugin::new())
            .add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(LogDiagnosticsPlugin {
                wait_duration: Duration::from_secs(20),
                ..Default::default()
            })
            .register_type::<PlayerComponent>()
            .register_type::<TimeInfo>(); // tells bevy-inspector-egui how to display the struct in the world inspector
    }
}
