use std::default;

use bevy::prelude::*;
use kayak_ui::{prelude::*, widgets::ElementBundle};

use crate::{
    game::GameStage,
    ui::{
        widgets::{settings_menu::SettingsMenuBundle, start_menu::StartMenuBundle},
        STARTING_GAME_STATE,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Component, Default)]
pub enum MenuState {
    #[default]
    Main,
    Settings,
}

pub fn on_game_state_change(
    game_state: Res<State<GameStage>>,
    mut game_menu: Query<&mut MainMenuProps, Without<PreviousWidget>>,
) {
    if game_state.is_changed() {
        for mut game_menu in game_menu.iter_mut() {
            game_menu.game_state = game_state.current().clone();
        }
    }
}

#[derive(Component, Clone, PartialEq)]
pub struct MainMenuProps {
    pub game_state: GameStage,
    pub menu_state: MenuState,
}

impl Default for MainMenuProps {
    fn default() -> Self {
        Self {
            game_state: STARTING_GAME_STATE,
            menu_state: MenuState::Main,
        }
    }
}

// In the future this will tell Kayak that these
// Props belongs to a widget. For now it's use to
// get the `WidgetName` component.
impl Widget for MainMenuProps {}

#[derive(Bundle)]
pub struct MainMenuBundle {
    pub name: Name,
    pub props: MainMenuProps,
    pub styles: KStyle,
    pub children: KChildren,
    // This allows us to hook into on click events!
    pub on_event: OnEvent,
    // Widget name is required by Kayak UI!
    pub widget_name: WidgetName,
}

impl Default for MainMenuBundle {
    fn default() -> Self {
        Self {
            name: Name::new("MainMenuBundle"),
            props: MainMenuProps::default(),
            styles: KStyle::default(),
            children: KChildren::default(),
            on_event: OnEvent::default(),
            // Kayak uses this component to find out more
            // information about your widget.
            // This is done because bevy does not have the
            // ability to query traits.
            widget_name: MainMenuProps::default().get_name(),
        }
    }
}

pub fn main_menu_render(
    // This is a bevy feature which allows custom
    // parameters to be passed into a system.
    // In this case Kayak UI gives the system a
    // `KayakWidgetContext` and an `Entity`.
    In((widget_context, entity)): In<(KayakWidgetContext, Entity)>,
    // The rest of the parameters are just like those found
    // in a bevy system! In fact you can add whatever
    // you would like here including more queries or
    // lookups to resources within bevy's ECS.
    mut commands: Commands,
    // In this case we really only care about our buttons
    // children! Let's query for them.
    props: Query<&MainMenuProps>,
) -> bool {
    let props = props.get(entity).unwrap();
    let parent_id = None;

    let row_styles = KStyle {
        layout_type: StyleProp::Value(LayoutType::Row),
        padding_top: StyleProp::Value(Units::Stretch(1.0)),
        padding_bottom: StyleProp::Value(Units::Stretch(1.0)),
        ..Default::default()
    };

    info!("updating main menu widget");
        if props.game_state == GameStage::Menu && props.menu_state == MenuState::Main {
        rsx!(
                <StartMenuBundle/>
            );
        } else if props.menu_state == MenuState::Settings {
        rsx!(
                <SettingsMenuBundle/>
            );
        }
    true
}

// let on_click_new_game = OnEvent::new(
//     move |In((event_dispatcher_context, _, event, _entity)): In<(
//         EventDispatcherContext,
//         WidgetState,
//         Event,
//         Entity,
//     )>,
//           mut game_state: ResMut<State<GameStage>>| {
//         if let EventType::Click(..) = event.event_type {
//             game_state
//                 .push(GameStage::Playing)
//                 .expect("cant push state for some reason")
//         }
//         (event_dispatcher_context, event)
//     },
// );

// let on_click_settings = OnEvent::new(
//     move |In((event_dispatcher_context, _, mut event, _entity)): In<(
//         EventDispatcherContext,
//         WidgetState,
//         Event,
//         Entity,
//     )>,
//           mut state: Query<&mut MenuState>| {
//         if let EventType::Click(..) = event.event_type {
//             event.prevent_default();
//             event.stop_propagation();
//             if let Ok(mut current_menu) = state.get_mut(state_entity) {
//                 *current_menu = MenuState::Settings;
//             }
//         }
//         (event_dispatcher_context, event)
//     },
// );

// let on_click_exit = OnEvent::new(
//     move |In((event_dispatcher_context, _, event, _entity)): In<(
//         EventDispatcherContext,
//         WidgetState,
//         Event,
//         Entity,
//     )>,
//           mut exit: EventWriter<AppExit>| {
//         if let EventType::Click(..) = event.event_type {
//             exit.send(AppExit);
//         }
//         (event_dispatcher_context, event)
//     },
// );
