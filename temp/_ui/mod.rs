mod main_menu;
mod widgets;

use bevy::prelude::*;
use kayak_ui::prelude::KChildren;
use kayak_ui::{
    prelude::{rsx, widget_update, FontMapping, KayakContextPlugin, KayakRootContext, Widget},
    widgets::{ButtonState, KayakAppBundle, KayakWidgets, KayakWidgetsContextPlugin},
    UICameraBundle,
};

use crate::ui::main_menu::{main_menu_render, MainMenuProps};
use crate::ui::widgets::settings_menu::{settings_menu_render, SettingsMenuProps};
use crate::ui::widgets::start_menu::{start_menu_render, StartMenuProps};
use crate::{
    game::GameStage,
    loading::assets::FontHandles,
    ui::{
        main_menu::{MainMenuBundle, MenuState},
        widgets::menu_button::{menu_button_render, MenuButton},
    },
};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(KayakContextPlugin)
            .add_plugin(KayakWidgets)
            .add_system_set(SystemSet::on_enter(GameStage::Menu).with_system(game_ui))
            .add_system(main_menu::on_game_state_change);
    }
}

const STARTING_GAME_STATE: GameStage = GameStage::Menu;

// THIS ONLY RUNS ONCE. VERY IMPORTANT FACT.
pub fn game_ui(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    fonts: Res<FontHandles>,
) {
    font_mapping.set_default(fonts.fantasque_sans_msdf.clone());

    let mut widget_context = KayakRootContext::new();
    widget_context.add_plugin(KayakWidgetsContextPlugin);

    let parent_id = None;

    // We need to register the prop and state types.
    // State is empty so you can use the `EmptyState`
    // component!
    widget_context.add_widget_data::<MainMenuProps, MenuState>();
    widget_context.add_widget_data::<StartMenuProps, MenuState>();
    widget_context.add_widget_data::<SettingsMenuProps, MenuState>();
    widget_context.add_widget_data::<MenuButton, ButtonState>();

    // Next we need to add the systems

    widget_context.add_widget_system(
        StartMenuProps::default().get_name(),
        widget_update::<StartMenuProps, MenuState>,
        start_menu_render,
    );

    widget_context.add_widget_system(
        SettingsMenuProps::default().get_name(),
        widget_update::<SettingsMenuProps, MenuState>,
        settings_menu_render,
    );

    widget_context.add_widget_system(
        // We are registering these systems with a specific
        // WidgetName.
        MainMenuProps::default().get_name(),
        // widget_update auto diffs props and state.
        // Optionally if you have context you can use:
        // widget_update_with_context otherwise you
        // will need to create your own widget update
        // system!
        widget_update::<MainMenuProps, MenuState>,
        // Add our render system!
        main_menu_render,
    );

    widget_context.add_widget_system(
        MenuButton::default().get_name(),
        widget_update::<MenuButton, ButtonState>,
        menu_button_render,
    );

    rsx! {
        <KayakAppBundle>
            <MainMenuBundle/>
        </KayakAppBundle>
    }
    commands.spawn((UICameraBundle::new(widget_context), Name::new("UI Camera")));
}
