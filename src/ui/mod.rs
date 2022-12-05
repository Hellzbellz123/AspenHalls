pub mod main_menu;
mod widgets;

use bevy::prelude::*;
use kayak_ui::{
    prelude::{widget_update, FontMapping, KayakRootContext, *},
    widgets::{ButtonState, KayakAppBundle, KayakWidgets, KayakWidgetsContextPlugin},
};

use crate::{
    game::GameStage,
    loading::assets::FontHandles,
    ui::{
        main_menu::{game_menu_render, GameMenuBundle, GameMenuProps, MenuState},
        widgets::button::{menu_button_render, MenuButton},
    },
};

use self::main_menu::on_game_state_change;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(KayakContextPlugin)
            .add_plugin(KayakWidgets)
            .add_system_set(
                SystemSet::on_enter(GameStage::Menu)
                    .with_system(game_ui)
                    .with_system(trace_ui),
            )
            .add_system(on_game_state_change);
    }
}

fn trace_ui() {
    info!("setting up UI");
}

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
    // if State is empty you can use the `EmptyState`
    // component!
    widget_context.add_widget_data::<GameMenuProps, MenuState>();

    // Next we need to add the systems
    widget_context.add_widget_system(
        // We are registering these systems with a specific
        // WidgetName.
        GameMenuProps::default().get_name(),
        // widget_update auto diffs props and state.
        // Optionally if you have context you can use:
        // widget_update_with_context otherwise you
        // will need to create your own widget update
        // system!
        widget_update::<GameMenuProps, MenuState>,
        // Add our render system!
        game_menu_render,
    );

    widget_context.add_widget_data::<MenuButton, ButtonState>();
    widget_context.add_widget_system(
        MenuButton::default().get_name(),
        widget_update::<MenuButton, ButtonState>,
        menu_button_render,
    );

    rsx! {
        <KayakAppBundle>
            <GameMenuBundle/>
        </KayakAppBundle>
    }
    commands.spawn((UICameraBundle::new(widget_context), Name::new("UI Camera")));
}
