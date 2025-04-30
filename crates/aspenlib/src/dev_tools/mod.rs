use std::time::Duration;

use avian2d::{debug_render::PhysicsDebugPlugin, prelude::PhysicsGizmos};
use bevy::{
    diagnostic::{
        EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin,
    },
    prelude::*,
    render::view::RenderLayers,
};
use bevy_ecs_ldtk::{prelude::LdtkProject, GridCoords, IntGridCell, LayerMetadata};
use big_brain::{
    choices::Choice,
    prelude::{Actor, HasThinker, Score, Scorer, Thinker},
};
use leafwing_input_manager::prelude::ActionState;

#[cfg(not(feature="develop"))]
use bevy::diagnostic::SystemInformationDiagnosticsPlugin;

use crate::{
    dev_tools::debug_visuals::{DebugDraw, DrawnMap},
    game::input::action_maps::Gameplay,
    register_types, GeneralSettings,
};

#[cfg(all(
    feature = "develop",
    not(any(target_os = "ios", target_os = "android", target_family = "wasm"))
))]
pub mod debug_dirs;

#[cfg(all(
    feature = "develop",
    not(any(target_os = "ios", target_os = "android", target_family = "wasm"))
))]
pub mod dump_schedules;

pub mod console;
pub mod debug_visuals;
pub mod egui_tools;
pub mod picking_debug;

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Resource, Reflect, Clone)]
#[reflect(Resource)]
pub struct DebugConfig {
    enabled: bool,
    pub drawmap: DrawnMap,
    pub show_world_inspector: bool,
    pub disable_animation: bool,
    pub physics_draw: bool,
    pub aabb_draw: bool,
    pub show_appstate: bool,
    pub show_gamestate: bool,
    pub show_generatorstate: bool,
}

impl Default for DebugConfig {
    fn default() -> Self {
        Self {
            drawmap: DrawnMap::default(),
            enabled: cfg!(feature = "develop"),
            show_world_inspector: cfg!(feature = "develop"),
            physics_draw: cfg!(feature = "develop"),
            disable_animation: false,
            aabb_draw: false,
            show_appstate: false,
            show_gamestate: false,
            show_generatorstate: false,
        }
    }
}

pub struct AspenDevToolsPlugin;

impl Plugin for AspenDevToolsPlugin {
    fn build(&self, app: &mut App) {
        register_types!(app, [PhysicsGizmos]);
        // debug tools unregistered types
        register_types!(app, [DrawnMap, DebugDraw, DebugConfig]);
        // ldtk unregistered types
        register_types!(
            app,
            [
            LdtkProject,
            LayerMetadata,
            IntGridCell,
            GridCoords,
            Handle<LdtkProject>
            ]
        );
        // BigBrain unregistered types
        register_types!(
            app,
            [
                Actor,
                big_brain::prelude::Action,
                Scorer,
                Score,
                Choice,
                Thinker,
                HasThinker
            ]
        );

        app.init_resource::<DebugConfig>();

        app.add_plugins((
            // internal tools
            console::QuakeConPlugin,
            debug_visuals::DebugVisualsPlugin,
            egui_tools::EguiToolsPlugin,
            // external tools
            PhysicsDebugPlugin::new(FixedPostUpdate),
            FrameTimeDiagnosticsPlugin::default() ,
            EntityCountDiagnosticsPlugin,
            #[cfg(not(feature="develop"))]
            SystemInformationDiagnosticsPlugin,
            picking_debug::DebugPickingPlugin,
            LogDiagnosticsPlugin {
                debug: false,
                wait_duration: Duration::from_secs(5),
                filter: None,
            },
        ));

        app.insert_gizmo_config(
            PhysicsGizmos::all().with_mesh_visibility(true),
            GizmoConfig {
                enabled: false,
                depth_bias: 0.0,
                render_layers: RenderLayers::default(),
                line: GizmoLineConfig { width: 2.0, ..default() }
            },
        );

        app.add_systems(
            Update,
            (
                toggle_debug_systems,
                toggle_physics_visualizations,
                bridge_debug_cfg_setting.run_if(resource_changed::<GeneralSettings>),
            ),
        );

        #[cfg(all(
            feature = "develop",
            not(any(target_os = "ios", target_os = "android", target_family = "wasm"))
        ))]
        {
            debug_dirs::dump_launch_directory();
            dump_schedules::debug_dump_graphs(app);
        }
    }
}

fn bridge_debug_cfg_setting(
    mut debug_ctrl: ResMut<DebugConfig>,
    general_settings: ResMut<GeneralSettings>,
) {
    if general_settings.is_changed() || debug_ctrl.enabled != general_settings.enable_debug {
        debug_ctrl.enabled = general_settings.enable_debug;
    }
}

fn toggle_debug_systems(
    mut cfg: ResMut<GeneralSettings>,
    mut debug_ctrl: ResMut<DebugConfig>,
    input: Res<ActionState<Gameplay>>,
) {
    if input.just_pressed(&Gameplay::DebugF3) {
        if cfg.enable_debug {
            debug_ctrl.enabled = false;
            cfg.enable_debug = false;
        } else {
            debug_ctrl.enabled = true;
            cfg.enable_debug = true;
        }
    }
}

fn toggle_physics_visualizations(
    debug_ctrl: Res<DebugConfig>,
    mut store: ResMut<GizmoConfigStore>,
) {
    let (config, _gizmos) = store.config_mut::<PhysicsGizmos>();
    if config.enabled != debug_ctrl.physics_draw {
        config.enabled = debug_ctrl.physics_draw;
    }
}
