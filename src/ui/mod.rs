pub mod events_handlers;
mod widgets;
pub mod zfailed_load_menu;
pub mod zmain_menu;
mod zpause_menu;
pub mod zsettings_menu;

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use kayak_ui::{
    prelude::{widget_update, FontMapping, KayakRootContext, *},
    widgets::{ButtonState, KayakAppBundle, KayakWidgets, KayakWidgetsContextPlugin},
    CameraUIKayak,
};
use leafwing_input_manager::prelude::ActionState;

use crate::{
    action_manager::actions::PlayerBindables,
    components::OnSplashScreen,
    game::{GameStage, TimeInfo},
    loading::assets::FontHandles,
    ui::{
        widgets::button::{menu_button_render, MenuButton},
        zmain_menu::{main_menu_render, MainMenuBundle, MainMenuProps},
        zpause_menu::{pause_menu_render, PauseMenuBundle, PauseMenuProps},
        zsettings_menu::{settings_menu_render, SettingsMenuBundle, SettingsMenuProps},
    },
    utilities::despawn_with,
};

use self::{
    events_handlers::PlayButtonEvent, zfailed_load_menu::failed_load_ui,
    zmain_menu::update_main_menu_props, zpause_menu::update_pause_menu_props,
    zsettings_menu::update_settings_menu_props,
};

#[derive(Debug, Clone, PartialEq, Eq, Component, Reflect, Inspectable, Hash)]
pub enum MenuState {
    HideMenu,
    Main,
    Pause,
    Settings,
}

impl Default for MenuState {
    fn default() -> Self {
        MenuState::Main
    }
}

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayButtonEvent>()
            .add_state(MenuState::Main)
            .add_plugin(KayakContextPlugin)
            .add_plugin(KayakWidgets)
            .add_system_set(
                SystemSet::on_exit(GameStage::Loading)
                    .with_system(despawn_with::<OnSplashScreen>)
                    .with_system(game_ui),
            )
            .add_system_set(
                SystemSet::on_enter(GameStage::FailedLoading).with_system(failed_load_ui),
            )
            .add_system_set(SystemSet::on_update(GameStage::Playing).with_system(toggle_pause_menu))
            .add_system(update_main_menu_props)
            .add_system(update_pause_menu_props)
            .add_system(update_settings_menu_props);
    }
}

pub fn toggle_pause_menu(
    mut timeinfo: ResMut<TimeInfo>,
    query_action_state: Query<&ActionState<PlayerBindables>>,
    mut menu_state: ResMut<State<MenuState>>,
    event_reader: EventReader<PlayButtonEvent>,
) {
    if !query_action_state.is_empty()
        && (query_action_state
            .get_single()
            .expect("should always only ever be one")
            .just_pressed(PlayerBindables::Pause)
            || !event_reader.is_empty())
    {
        let mut timeinfo = timeinfo.as_mut();

        if menu_state.current() == &MenuState::Pause {
            //if calling this function and MenuState is pause we are already paused and want too unpause
            timeinfo.pause_menu = false;
            timeinfo.game_paused = false;
            timeinfo.time_step = 1.0;
            menu_state
                .set(MenuState::HideMenu)
                .expect("couldnt push hidemenu state");
            info!("no pause menu should be shown and game should be playing");
            event_reader.clear();
        } else if menu_state.current() == &MenuState::HideMenu {
            //if calling this function and MenuState is HideMenu we want too set menustate too pause and freeze time. kayak will listen for the menustate.
            timeinfo.pause_menu = true;
            timeinfo.game_paused = true;
            timeinfo.time_step = 0.;
            menu_state
                .set(MenuState::Pause)
                .expect("couldnt push pausemenu state");
            event_reader.clear();
            info!("pause menu should be shown and game should be paused")
        }
    }
}

pub fn despawn_ui(
    mut commands: Commands,
    to_despawn: Query<Entity, With<CameraUIKayak>>,
    widts: Query<Entity, With<KStyle>>,
) {
    for entity in to_despawn.iter() {
        info!("despawning kayak_root_context: {:#?}", entity);
        commands.entity(entity).despawn_recursive();
        for widget in widts.iter() {
            commands.entity(widget).despawn_recursive();
        }
    }
}

// THIS ONLY RUNS ONCE. VERY IMPORTANT FACT.
pub fn game_ui(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    fonts: Res<FontHandles>,
) {
    info!("setting up UI");
    font_mapping.set_default(fonts.fantasque_sans_msdf.clone());

    let mut widget_context = KayakRootContext::new();
    widget_context.add_plugin(KayakWidgetsContextPlugin);

    let parent_id = None;

    // We need to register the prop and state types.
    // if State is empty you can use the `EmptyState`
    // component!
    widget_context.add_widget_data::<MainMenuProps, MenuState>();
    widget_context.add_widget_data::<PauseMenuProps, MenuState>();
    widget_context.add_widget_data::<SettingsMenuProps, MenuState>();

    // Next we need to add the systems
    widget_context.add_widget_system(
        // We are registering these systems with a specific WidgetName.
        MainMenuProps::default().get_name(),
        // widget_update auto diffs props and state. Optionally if you have context you can use:
        // widget_update_with_context otherwise you will need to create your own widget update system!
        widget_update::<MainMenuProps, MenuState>,
        // Add our render system!
        main_menu_render,
    );

    widget_context.add_widget_system(
        PauseMenuProps::default().get_name(),
        widget_update::<PauseMenuProps, EmptyState>,
        pause_menu_render,
    );

    widget_context.add_widget_system(
        SettingsMenuProps::default().get_name(),
        widget_update::<SettingsMenuProps, EmptyState>,
        settings_menu_render,
    );

    widget_context.add_widget_data::<MenuButton, ButtonState>();
    widget_context.add_widget_system(
        MenuButton::default().get_name(),
        widget_update::<MenuButton, ButtonState>,
        menu_button_render,
    );

    rsx! {
        <KayakAppBundle>
            <MainMenuBundle/>
            <PauseMenuBundle/>
            <SettingsMenuBundle/>
        </KayakAppBundle>
    };
    commands.spawn((UICameraBundle::new(widget_context), Name::new("UICamera")));
}
