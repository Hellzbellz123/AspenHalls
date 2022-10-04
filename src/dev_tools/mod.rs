use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_ecs_ldtk::{GridCoords, IntGridCell, LayerMetadata};
use bevy_inspector_egui::{InspectorPlugin, RegisterInspectable, WorldInspectorPlugin};
use std::time::Duration;

use crate::{
    action_manager::actions::PlayerBindables,
    actors::{
        animation::{AnimState, AnimationSheet, FacingDirection},
        ActorState,
    },
    game::TimeInfo,
    game_world::world_components::Collides,
    AppSettings,
};

mod debug_dirs;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        debug_dirs::debugdir();
        let _registry = app
            .world
            .get_resource_or_insert_with(bevy_inspector_egui::InspectableRegistry::default);

        app.add_plugin(InspectorPlugin::<AppSettings>::new())
            .add_plugin(WorldInspectorPlugin::new())
            .add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(LogDiagnosticsPlugin {
                wait_duration: Duration::from_secs(20),
                ..Default::default()
            })
            //custom inspectables not from plugins
            .register_inspectable::<ActorState>()
            .register_type::<TimeInfo>()
            .register_type::<AnimState>()
            .register_inspectable::<AnimationSheet>()
            .register_inspectable::<FacingDirection>() // tells bevy-inspector-egui how to display the struct in the world inspector
            .register_inspectable::<Collides>()
            .register_type::<PlayerBindables>()
            // LDTK debug data
            .register_type::<LayerMetadata>()
            .register_type::<IntGridCell>()
            .register_type::<GridCoords>();
    }
}
