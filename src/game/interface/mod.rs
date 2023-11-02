use belly::prelude::{eml, BodyWidgetExtension, Element, Elements, StyleSheet, Widget};
use bevy::prelude::*;

use crate::{game::AppStage, loading::assets::InitAssetHandles};

/// game menu interface data
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

/// ui plugin
pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MenuPopupEvent>()
            .add_state::<RequestedMenu>();

        app.add_systems(
            OnEnter(AppStage::Loading),
            InterfaceRoot::create_interface_root,
        )
        .add_systems(OnEnter(AppStage::PlayingGame), InterfaceRoot::hide_all)
        .add_systems(
            Update,
            (
                // update_ui_ent_with_elements.run_if(any_with_component::<Element>()),
                InterfaceRoot::show_popups,
            ),
        );
        menus::setup(app);
    }
}

/// Iterates over all UI Elements and sets entity 'Name' too element value
fn update_ui_ent_with_elements(mut cmds: Commands, mut elements: Query<(Entity, &mut Element), Changed<Element>>) {
    elements.for_each_mut(|(ent, element)| {
        let mut classes = element.classes.clone();

        // Convert Tags to strings and collect them into a Vec
        let tag_strings: Vec<String> = classes.drain().map(|tag| tag.to_string()).collect();

        if tag_strings.contains(&String::from("hidden")) {
            cmds.entity(ent).insert(Name::new("hidden"));
        } else {
            let name = tag_strings
                .last()
                .unwrap_or(&"NO CLASSES".to_string())
                .clone();
            cmds.entity(ent).insert(Name::new(name));
        }
    });
}

/// The menu root entity id
///
/// All components for the menu should be attached to this entity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, Resource)]
pub struct InterfaceRoot(pub Entity);

impl InterfaceRoot {
    /// Load the global style-sheet and create the menu root node
    fn create_interface_root(
        mut commands: Commands,
        // init_assets: Res<InitAssetHandles>,
        // mut strings: Res<Assets<StyleSheet>>,
    ) {
        // let global_style: &StyleSheet = strings
        //     .get(&init_assets.global_style_sheet.clone())
        //     .expect("global style sheet was not found in Res<Assets<String>");
        // let menu_style: &StyleSheet = strings
        //     .get(&init_assets.menu_style_sheet.clone())
        //     .expect("menu style sheet was not found in Res<Assets<String>>");
        // style sheet is already loaded and applied from InitAssetsHandles

        let entity = commands.spawn_empty().id();
        commands.insert_resource(Self(entity));

        commands.add(eml! {
            <body c:interface-root>
            </body>
        });
    }

    /// Create a popup in the lower right corner
    const fn show_popups(mut _events: EventReader<MenuPopupEvent>, mut _elements: Elements) {}

    /// A list of possible menus
    // const MENUS: [&'static str; 3] = [
    //     "div.start-menu-root",
    //     "div.settings-menu-root",
    //     "div.pause-menu-root",
    // ];

    // /// Handle the escape button
    // fn handle_escape(
    //     mut elements: Elements,
    //     input: Query<&ActionState<actions::Combat>, With<Player>>,
    //     mut ew: EventWriter<PausePlayEvent>,
    //     state: Res<State<AppStage>>,
    // ) {
    //     let menus = Self::MENUS.map(|c| {
    //         let ent = elements.select(c).entities();

    //         !elements
    //             .select(".hidden")
    //             .entities()
    //             .iter()
    //             .any(|e| ent.contains(e))
    //     });

    //     info!(
    //         "start menu: {}, settings menu: {}, pause menu: {}",
    //         menus[0], menus[1], menus[2]
    //     );

    //     if input.single().just_pressed(Combat::Pause) {
    //         match menus {
    //             [true, _, _] => {
    //                 info!("main menu should be NOT HIDDEN")
    //             }
    //             [_, true, _] => {
    //                 info!("settings menu should be NOT HIDDEN")
    //             }
    //             [_, _, true] => {
    //                 info!("pause menu should be NOT HIDDEN");
    //             }
    //             [false, false, false] => {
    //                 info!("no menus and escape pressed");
    //                 PauseMenu::show(elements);
    //                 ew.send(PausePlayEvent);
    //             }
    //         }
    //     }
    // }

    /// Hide all menus when the game starts
    fn hide_all(mut elements: Elements) {
        elements // hide start
            .select("div.start-menu-root")
            .add_class("hidden");
        // elements // hide settings
        //     .select("div.settings-menu-root")
        //     .add_class("hidden");
        // elements // hide pause
        //     .select("div.pause-menu-root")
        //     .add_class("hidden");
    }
}

/// An event that causes a popup to appear in the lower right corner
#[derive(Debug, Clone, PartialEq, Eq, Hash, Event)]
pub struct MenuPopupEvent {
    /// icon for popup
    pub icon: PopupIcon,
    /// message for popup
    pub message: String,
}

/// `IconType` for popup
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PopupIcon {
    /// Just info popup
    Info,
    /// player attention grabber
    Warning,
    /// Error Popup
    Error,
}

#[allow(dead_code)]
impl PopupIcon {
    /// creates handles from possible icons
    pub fn create_handle(self, asset_server: &AssetServer) -> Handle<Image> {
        match self {
            Self::Info => asset_server.load("interface/textures/info.png"),
            Self::Warning => asset_server.load("interface/textures/warning.png"),
            Self::Error => asset_server.load("interface/textures/error.png"),
        }
    }
}
