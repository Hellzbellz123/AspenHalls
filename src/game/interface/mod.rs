use belly::prelude::{eml, BodyWidgetExtension, Elements, StyleSheet, Widget};
use bevy::prelude::*;

use crate::game::{
    interface::menus::{start_menu::{self, MainMenu}, settings_menu},
    AppStage,
};

mod menus;

/// currently active menu
#[derive(Debug, Default, States, Hash, PartialEq, Eq, Clone, Copy, Reflect)]
pub enum RequestedMenu {
    /// no menu spawned
    #[default]
    None,
    /// start menu
    Start,
    /// pause menu
    Pause,
    /// settings menu
    Settings,
}

/// updates menu state based on game stage
fn control_menu_state(mut cmds: Commands, game_state: Res<State<AppStage>>) {
    if game_state.is_changed() {
        match game_state.get() {
            AppStage::StartMenu => cmds.insert_resource(NextState(Some(RequestedMenu::Start))),
            AppStage::PauseMenu => cmds.insert_resource(NextState(Some(RequestedMenu::Pause))),
            AppStage::PlayingGame => cmds.insert_resource(NextState(Some(RequestedMenu::None))),
            _ => {}
        }
    }
}

/// ui plugin
pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MenuPopupEvent>();

        app.add_systems(OnExit(AppStage::Loading), MenuRoot::create);
        app.add_systems(Update, (MenuRoot::show_popups, MenuRoot::handle_escape));
        app.add_systems(OnEnter(AppStage::PlayingGame), MenuRoot::hide_all);

        // TODO: Add menu systems
        start_menu::setup_menu(app);
        settings_menu::setup_menu(app);

        app.add_state::<RequestedMenu>()
            .add_systems(Update, (control_menu_state,));
    }
}

/// The menu root entity id
///
/// All components for the menu should be attached to this entity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, Resource)]
pub struct MenuRoot(pub Entity);

impl MenuRoot {
    /// Load the global stylesheet and create the menu root node
    fn create(mut commands: Commands) {
        commands.add(StyleSheet::load("interface/style/global.ess"));
        commands.add(StyleSheet::load("interface/style/menu.ess"));

        let entity = commands.spawn_empty().id();
        commands.insert_resource(MenuRoot(entity));

        commands.add(eml! {
            <body c:root>
            </body>
        });
    }

    /// Create a popup in the lower right corner
    fn show_popups(mut _events: EventReader<MenuPopupEvent>, mut _elements: Elements) {}

    // A list of possible menus
    const MENUS: [&'static str; 2] = ["div.options-menu", "div.main-menu-root"];

    /// Handle the escape button
    fn handle_escape(mut elements: Elements, input: Res<Input<KeyCode>>) {
        if !input.just_pressed(KeyCode::Escape) {
            return;
        }

        let menus = Self::MENUS.map(|c| {
            let ent = elements.select(c).entities();

            !elements
                .select(".hidden")
                .entities()
                .iter()
                .any(|e| ent.contains(e))
        });

        match menus {
            [false, true] => {
                // On main menu, do nothing
            }
            [false, false] => {
                warn!("Escape pressed and no menus are open!");
                MainMenu::show(elements);
            }
            _ => {
                warn!("Escape pressed, but no menu found to close!");
            }
        }
    }

    /// Hide all menus when the game starts
    fn hide_all(mut elements: Elements) {
        elements.select("body.root").add_class("hidden");
    }
}

/// An event that causes a popup to appear in the lower right corner
#[derive(Debug, Clone, PartialEq, Eq, Hash, Event)]
pub struct MenuPopupEvent {
    pub icon: PopupIcon,
    pub message: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PopupIcon {
    Info,
    Warning,
    Error,
}

#[allow(dead_code)]
impl PopupIcon {
    pub fn create_handle(&self, asset_server: &AssetServer) -> Handle<Image> {
        match self {
            Self::Info => asset_server.load("interface/images/info.png"),
            Self::Warning => asset_server.load("interface/images/warning.png"),
            Self::Error => asset_server.load("interface/images/error.png"),
        }
    }
}
