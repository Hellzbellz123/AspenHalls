pub mod debug_dirs;

use std::time::Duration;

use bevy_ecs_ldtk::{LayerMetadata, IntGridCell, GridCoords, LevelSet};

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};

use crate::{
    characters::player::{
        animation::{AnimState, CharacterSheet, FacingDirection},
        PlayerState,
    },
    game::TimeInfo,
};

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
            //custom inspectables not from plugins
            .register_inspectable::<PlayerState>()
            .register_type::<TimeInfo>()
            .register_type::<AnimState>()
            .register_inspectable::<CharacterSheet>()
            .register_inspectable::<FacingDirection>() // tells bevy-inspector-egui how to display the struct in the world inspector
            // LDTK debug data
            .register_type::<LayerMetadata>()
            .register_type::<IntGridCell>()
            .register_type::<GridCoords>() ;
    }
}
