//! Easy plugins for showing UI panels.
//! **Pros:** no manual code required
//! **Cons:** not configurable
//! When you want something more custom, you can use these plugins as a starting point.
use bevy::{
    app::Plugin,
    asset::Asset,
    ecs::{
        hierarchy::ChildOf,
        observer::ObserverState,
        prelude::{
            Condition, IntoScheduleConfigs, IntoSystem, ReadOnlySystem, Resource, With, World,
        },
        query::{QueryFilter, Without},
        schedule::{BoxedCondition, common_conditions::resource_exists},
        system::Res,
    },
    reflect::Reflect,
    state::state::FreelyMutableState,
};
use bevy_egui::{EguiContext, EguiPlugin, EguiPrimaryContextPass, PrimaryEguiContext, egui};
use bevy_inspector_egui::bevy_inspector::Filter;
use big_brain::prelude::ActionState;
use pretty_type_name::pretty_type_name;
use std::{marker::PhantomData, sync::Mutex};

use bevy_inspector_egui::{DefaultInspectorConfigPlugin, bevy_inspector};

use crate::{
    AppStage, GameStage, dev_tools::DebugConfig, game::game_world::dungeonator_v2::GeneratorState,
};

pub struct EguiToolsPlugin;

impl Plugin for EguiToolsPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        assert!(app.is_plugin_added::<bevy_egui::EguiPlugin>()); // each plugin also checks this?
        if !app.is_plugin_added::<DefaultInspectorConfigPlugin>() {
            app.add_plugins(DefaultInspectorConfigPlugin);
        }

        app.add_plugins((
            ResourceInspectorPlugin::<DebugConfig>::default()
                .run_if(resource_exists::<DebugConfig>.and(|res: Res<DebugConfig>| res.enabled)),
            StateInspectorPlugin::<AppStage>::default().run_if(
                resource_exists::<DebugConfig>
                    .and(|res: Res<DebugConfig>| res.enabled && res.show_appstate),
            ),
            StateInspectorPlugin::<GeneratorState>::default().run_if(
                resource_exists::<DebugConfig>
                    .and(|res: Res<DebugConfig>| res.enabled && res.show_generatorstate),
            ),
            StateInspectorPlugin::<GameStage>::default().run_if(
                resource_exists::<DebugConfig>
                    .and(|res: Res<DebugConfig>| res.enabled && res.show_gamestate),
            ),
            WorldInspectorPlugin::default().run_if(
                resource_exists::<DebugConfig>
                    .and(|res: Res<DebugConfig>| res.enabled && res.show_world_inspector),
            ),
        ));
    }
}

const DEFAULT_SIZE: (f32, f32) = (320., 160.);

/// Plugin displaying a egui window with an entity list, resources and assets
#[derive(Default)]
pub struct WorldInspectorPlugin {
    condition: Mutex<Option<BoxedCondition>>,
}

impl WorldInspectorPlugin {
    /// Only show the UI of the specified condition is active
    pub fn run_if<M>(mut self, condition: impl Condition<M>) -> Self {
        let condition_system = IntoSystem::into_system(condition);
        self.condition = Mutex::new(Some(Box::new(condition_system) as BoxedCondition));
        self
    }
}

impl Plugin for WorldInspectorPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        let condition = self.condition.lock().unwrap().take();
        let mut system = world_inspector_ui.into_configs();
        if let Some(condition) = condition {
            system.run_if_dyn(condition);
        }
        app.add_systems(EguiPrimaryContextPass, system);
    }
}

fn world_inspector_ui(world: &mut World) {
    let egui_context = world
        .query_filtered::<&mut EguiContext, With<PrimaryEguiContext>>()
        .single(world);

    let Ok(egui_context) = egui_context else {
        return;
    };
    let mut egui_context = egui_context.clone();

    egui::Window::new("World Inspector")
        .default_size(DEFAULT_SIZE)
        .show(egui_context.get_mut(), |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                ui_for_world(world, ui);
                ui.allocate_space(ui.available_size());
            });
        });
}

/// Display `Entities`, `Resources` and `Assets` using their respective functions inside headers
pub fn ui_for_world(world: &mut World, ui: &mut egui::Ui) {
    egui::CollapsingHeader::new("Entities")
        .default_open(true)
        .show(ui, |ui| {
            let filter = Filter::<(
                Without<ActionState>,
                Without<ObserverState>,
                Without<ChildOf>,
            )>::all();
            bevy_inspector::ui_for_entities_filtered(world, ui, true, &filter);
        });
    egui::CollapsingHeader::new("Resources").show(ui, |ui| {
        bevy_inspector::ui_for_resources(world, ui);
    });
    egui::CollapsingHeader::new("Assets").show(ui, |ui| {
        bevy_inspector::ui_for_all_assets(world, ui);
    });
}

/// Plugin displaying an egui window for a single resource.
pub struct ResourceInspectorPlugin<T> {
    condition: Mutex<Option<BoxedCondition>>,
    marker: PhantomData<fn() -> T>,
}

impl<T> Default for ResourceInspectorPlugin<T> {
    fn default() -> Self {
        Self {
            marker: PhantomData,
            condition: Mutex::new(None),
        }
    }
}

impl<T> ResourceInspectorPlugin<T> {
    /// Only show the UI of the specified condition is active
    pub fn run_if<M>(mut self, condition: impl Condition<M>) -> Self {
        let condition_system = IntoSystem::into_system(condition);
        self.condition = Mutex::new(Some(Box::new(condition_system) as BoxedCondition));
        self
    }
}

impl<T: Resource + Reflect> Plugin for ResourceInspectorPlugin<T> {
    fn build(&self, app: &mut bevy::app::App) {
        let condition = self.condition.lock().unwrap().take();
        let mut system = inspector_ui::<T>.into_configs();
        if let Some(condition) = condition {
            system.run_if_dyn(condition);
        }
        app.add_systems(EguiPrimaryContextPass, system);
    }
}

fn inspector_ui<T: Resource + Reflect>(world: &mut World) {
    let egui_context = world
        .query_filtered::<&mut EguiContext, With<PrimaryEguiContext>>()
        .single(world);

    let Ok(egui_context) = egui_context else {
        return;
    };
    let mut egui_context = egui_context.clone();

    egui::Window::new(pretty_type_name::<T>())
        .default_size((0., 0.))
        .show(egui_context.get_mut(), |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                bevy_inspector::ui_for_resource::<T>(world, ui);

                ui.allocate_space(ui.available_size());
            });
        });
}

/// Plugin displaying an egui window for an app state.
/// Remember to call [`App::add_state`](bevy_app::App::init_state).
pub struct StateInspectorPlugin<T> {
    condition: Mutex<Option<BoxedCondition>>,
    marker: PhantomData<fn() -> T>,
}

impl<T> Default for StateInspectorPlugin<T> {
    fn default() -> Self {
        StateInspectorPlugin {
            condition: Mutex::new(None),
            marker: PhantomData,
        }
    }
}
impl<T> StateInspectorPlugin<T> {
    /// Only show the UI of the specified condition is active
    pub fn run_if<M>(mut self, condition: impl Condition<M>) -> Self {
        let condition_system = IntoSystem::into_system(condition);
        self.condition = Mutex::new(Some(Box::new(condition_system) as BoxedCondition));
        self
    }
}

impl<T: FreelyMutableState + Reflect> Plugin for StateInspectorPlugin<T> {
    fn build(&self, app: &mut bevy::app::App) {
        let condition = self.condition.lock().unwrap().take();
        let mut system = state_ui::<T>.into_configs();
        if let Some(condition) = condition {
            system.run_if_dyn(condition);
        }
        app.add_systems(EguiPrimaryContextPass, system);
    }
}

fn state_ui<T: FreelyMutableState + Reflect>(world: &mut World) {
    let egui_context = world
        .query_filtered::<&mut EguiContext, With<PrimaryEguiContext>>()
        .single(world);

    let Ok(egui_context) = egui_context else {
        return;
    };
    let mut egui_context = egui_context.clone();

    egui::Window::new(std::any::type_name::<T>())
        .resizable(false)
        .title_bar(false)
        .show(egui_context.get_mut(), |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                ui.heading(pretty_type_name::<T>());
                bevy_inspector::ui_for_state::<T>(world, ui);
            });
        });
}

/// Plugin displaying an egui window for all assets of type `A`.
/// Remember to call [`App::register_asset_reflect`](bevy_asset::AssetApp::register_asset_reflect).
pub struct AssetInspectorPlugin<A> {
    condition: Mutex<Option<BoxedCondition>>,
    marker: PhantomData<fn() -> A>,
}

impl<A> Default for AssetInspectorPlugin<A> {
    fn default() -> Self {
        Self {
            condition: Mutex::new(None),
            marker: PhantomData,
        }
    }
}
impl<A> AssetInspectorPlugin<A> {
    /// Only show the UI of the specified condition is active
    pub fn run_if<M>(mut self, condition: impl Condition<M>) -> Self {
        let condition_system = IntoSystem::into_system(condition);
        self.condition = Mutex::new(Some(Box::new(condition_system) as BoxedCondition));
        self
    }
}

impl<A: Asset + Reflect> Plugin for AssetInspectorPlugin<A> {
    fn build(&self, app: &mut bevy::app::App) {
        let condition = self.condition.lock().unwrap().take();
        let mut system = asset_inspector_ui::<A>.into_configs();
        if let Some(condition) = condition {
            system.run_if_dyn(condition);
        }
        app.add_systems(EguiPrimaryContextPass, system);
    }
}

fn asset_inspector_ui<A: Asset + Reflect>(world: &mut World) {
    let egui_context = world
        .query_filtered::<&mut EguiContext, With<PrimaryEguiContext>>()
        .single(world);

    let Ok(egui_context) = egui_context else {
        return;
    };
    let mut egui_context = egui_context.clone();

    egui::Window::new(pretty_type_name::<A>())
        .default_size(DEFAULT_SIZE)
        .show(egui_context.get_mut(), |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                bevy_inspector::ui_for_assets::<A>(world, ui);

                ui.allocate_space(ui.available_size());
            });
        });
}

/// Plugin displaying an egui window for all entities matching the filter `F`.
pub struct FilterQueryInspectorPlugin<F> {
    condition: Mutex<Option<BoxedCondition>>,
    marker: PhantomData<fn() -> F>,
}

impl<F> Default for FilterQueryInspectorPlugin<F> {
    fn default() -> Self {
        Self {
            condition: Mutex::new(None),
            marker: PhantomData,
        }
    }
}
impl<A> FilterQueryInspectorPlugin<A> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Only show the UI of the specified condition is active
    pub fn run_if<M>(mut self, condition: impl Condition<M>) -> Self {
        let condition_system = IntoSystem::into_system(condition);
        self.condition = Mutex::new(Some(Box::new(condition_system) as BoxedCondition));
        self
    }
}

impl<F: 'static> Plugin for FilterQueryInspectorPlugin<F>
where
    F: QueryFilter,
{
    fn build(&self, app: &mut bevy::app::App) {
        let condition: Option<Box<dyn ReadOnlySystem<In = (), Out = bool>>> =
            self.condition.lock().unwrap().take();
        let mut system = entity_query_ui::<F>.into_configs();
        if let Some(condition) = condition {
            system.run_if_dyn(condition);
        }
        app.add_systems(EguiPrimaryContextPass, system);
    }
}

fn entity_query_ui<F: QueryFilter>(world: &mut World) {
    let egui_context = world
        .query_filtered::<&mut EguiContext, With<PrimaryEguiContext>>()
        .single(world);

    let Ok(egui_context) = egui_context else {
        return;
    };
    let mut egui_context = egui_context.clone();

    egui::Window::new(pretty_type_name::<F>())
        .default_size(DEFAULT_SIZE)
        .show(egui_context.get_mut(), |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                bevy_inspector::ui_for_entities_filtered(world, ui, false, &Filter::<F>::all());
                ui.allocate_space(ui.available_size());
            });
        });
}
