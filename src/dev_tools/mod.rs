use bevy_ecs_ldtk::prelude::*;
use std::time::Duration;

use bevy_ecs_ldtk::LayerMetadata;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};

use crate::{
    characters::player::{
        player_animation::{CharacterSheet, FacingDirection, FrameAnimation},
        PlayerComponent,
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
            .register_inspectable::<PlayerComponent>()
            .register_type::<TimeInfo>()
            .register_type::<FrameAnimation>()
            .register_inspectable::<CharacterSheet>()
            .register_inspectable::<FacingDirection>() // tells bevy-inspector-egui how to display the struct in the world inspector
            // LDTK debug data
            .register_type::<LayerMetadata>();
        // .register_inspectable::<LevelSet>();
        // .register_type::<LevelSet>();
    }
}
