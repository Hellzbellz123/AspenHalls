use belly::prelude::*;
use bevy::{app::AppExit, prelude::*};
use rand::seq::IteratorRandom;

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
            StartMenu::create
                .run_if(not(any_with_component::<StartMenu>())),
            apply_deferred,
            StartMenu::show.run_if(any_with_component::<StartMenu>()),
        )
            .chain(),
    );
}

/// A marker component for the main menu
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct StartMenu;

impl StartMenu {
    /// Create the main menu
    fn create(
        root: Res<InterfaceRoot>,
        mut elements: Elements,
        mut commands: Commands,
    ) {
        commands.entity(**root).insert(Self);

        elements.select(".interface-root").add_child(eml! {
            <div c:start-menu-root c:hidden>
                <div c:start-menu-buttons>
                    <div c:start-menu-title>
                        <img c:start-menu-logo src="init/favicon.png"/>
                        <div c:start-menu-subtitle><span>{ Self::get_subtitle() }</span></div>
                    </div>
                    <button c:button on:press=|ctx| { ctx.send_event(PausePlayEvent(EventType::Play)) }>
                        "Play"
                    </button>
                    <button c:button on:press=|ctx| { Self::click_button(ctx, "div.settings-menu-root") }>
                        "Options"
                    </button>
                    <button c:button on:press=|ctx| { ctx.send_event(AppExit) }>
                        "Quit"
                    </button>
                </div>
                <div c:menu-version>
                    "VanillaCoffee v"
                    { env!("CARGO_PKG_VERSION") }
                </div>
                <div c:menu-disclaimer>
                    "ALPHA SOFTWARE - USE AT YOUR OWN RISK"
                </div>
            </div>
        });
    }
    /// Show the main menu
    pub fn show(mut elements: Elements) {
        elements
            .select("div.start-menu-root")
            .remove_class("hidden");
    }

    #[allow(dead_code)]
    /// Hide the main menu
    pub fn hide(mut elements: Elements) {
        elements.select("div.start-menu-root").add_class("hidden");
    }

    /// Function to handle button clicks
    fn click_button(ctx: &mut EventContext<impl Event>, query: &str) {
        ctx.select(query).toggle_class("hidden");
        ctx.select("div.start-menu-root").add_class("hidden");
    }

    /// The list of possible subtitles
    const SUBTITLES: &'static str =
        include_str!("../../../../assets/init/language/menu_subtitle.txt");

    /// Get a random subtitle from the list
    fn get_subtitle() -> &'static str {
        let mut rng = rand::thread_rng();

        Self::SUBTITLES.lines().choose(&mut rng).unwrap_or_else(|| {
            Self::SUBTITLES.lines().next().expect("No subtitles found")
        })
        // .unwrap_or(Self::SUBTITLES.lines().next().expect("No subtitles found"))
    }
}
