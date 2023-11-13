use belly::prelude::*;
use bevy::{app::AppExit, prelude::*};

use crate::game::{
    interface::{
        menus::{EventType, PausePlayEvent},
        InterfaceRoot,
    },
    AppStage,
};

/// Set up the main menu
pub fn setup_menu(app: &mut App) {
    app.add_systems(
        OnEnter(AppStage::StartMenu),
        (
            PauseMenu::create
                .run_if(not(any_with_component::<PauseMenu>())),
            // PauseMenu::hide.run_if(any_with_component::<PauseMenu>()),
        ),
    );
}

/// A marker component for the main menu
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct PauseMenu;

impl PauseMenu {
    /// Create the main menu
    fn create(
        root: Res<InterfaceRoot>,
        mut elements: Elements,
        mut commands: Commands,
    ) {
        commands.entity(**root).insert(Self);

        elements.select(".interface-root").add_child(eml! {
            <div c:pause-menu-root c: hidden>
                <div c:pause-menu-body>
                    <span c:pause-menu-title> "Pause Menu" </span>
                    <div c:pause-menu-button-container>
                        <button c:button on:press=|ctx| { ctx.send_event(PausePlayEvent(EventType::Resume)); }>
                        "Resume Game"
                        </button>
                        <button c:button on:press=|ctx| { Self::click_button(ctx, "div.settings-menu-root") }>
                        "Options"
                        </button>
                        <button c:button on:press=|ctx| { ctx.send_event(AppExit) }>
                        "Quit"
                        </button>
                    </div>
                </div>
            </div>
        });
    }

    /// Show the main menu
    pub fn show(mut elements: Elements) {
        elements //
            .select("div.pause-menu-root")
            .remove_class("hidden");
    }

    #[allow(dead_code)]
    /// Hide the main menu
    pub fn hide(mut elements: Elements) {
        elements //
            .select("div.pause-menu-root")
            .add_class("hidden");
    }

    /// Function to handle button clicks
    fn click_button(ctx: &mut EventContext<impl Event>, query: &str) {
        ctx.select(query).remove_class("hidden");
        // ctx.select("div.pause-menu-root").add_class("hidden");
    }
}
