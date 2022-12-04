use bevy::prelude::*;
use kayak_ui::{
    prelude::{Widget, *},
    widgets::{ElementBundle, NinePatch, NinePatchBundle},
};

use crate::{
    game::GameStage,
    loading::assets::UiTextureHandles,
    ui::main_menu::{MainMenuProps, MenuState},
};

use super::menu_button::{MenuButton, MenuButtonBundle};

#[derive(Component, Clone, PartialEq, Default)]
pub struct SettingsMenuProps {
    game_state: GameStage,
    menu_state: MenuState,
}

impl Widget for SettingsMenuProps {}

#[derive(Bundle)]
pub struct SettingsMenuBundle {
    pub props: SettingsMenuProps,
    pub name: Name,
    pub styles: KStyle,
    pub children: KChildren,
    // This allows us to hook into on click events!
    pub on_event: OnEvent,
    // Widget name is required by Kayak UI!
    pub widget_name: WidgetName,
}

impl Default for SettingsMenuBundle {
    fn default() -> Self {
        Self {
            name: Name::new("SettingMenuBundle"),
            props: SettingsMenuProps::default(),
            styles: KStyle::default(),
            children: KChildren::default(),
            on_event: OnEvent::default(),
            // Kayak uses this component to find out more
            // information about your widget.
            // This is done because bevy does not have the
            // ability to query traits.
            widget_name: SettingsMenuProps::default().get_name(),
        }
    }
}

pub fn settings_menu_render(
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
    images: Res<UiTextureHandles>,
    // In this case we really only care about our buttons
    // children! Let's query for them.
    state: Query<&MenuState>,
    props: Query<&MainMenuProps>,
) -> bool {
    let props = props.get(entity).unwrap();
    let parent_id = Some(entity);

    let container_styles = KStyle {
        border_radius: StyleProp::Value(Corner::all(15.0)),
        background_color: StyleProp::Value(Color::WHITE),
        bottom: StyleProp::Value(Units::Stretch(1.0)),
        height: StyleProp::Value(Units::Pixels(500.0)),
        layout_type: StyleProp::Value(LayoutType::Column),
        left: StyleProp::Value(Units::Stretch(1.0)),
        padding: StyleProp::Value(Edge::axis(Units::Stretch(1.0), Units::Stretch(0.0))),
        right: StyleProp::Value(Units::Stretch(1.0)),
        row_between: StyleProp::Value(Units::Pixels(20.0)),
        top: StyleProp::Value(Units::Stretch(1.0)),
        width: StyleProp::Value(Units::Pixels(360.0)),
        ..Default::default()
    };

    let state_entity = widget_context.use_state(&mut commands, entity, MenuState::default());
    let container = images.panel_brown.clone();

    let menu_state = if let Ok(current_menu_state) = state.get(state_entity) {
        current_menu_state
    } else {
        &MenuState::Settings
    };

    let on_click_back_to_main = OnEvent::new(
        move |In((event_dispatcher_context, _, mut event, _entity)): In<(
            EventDispatcherContext,
            WidgetState,
            Event,
            Entity,
        )>,
              mut state: Query<&mut MenuState>| {
            if let EventType::Click(..) = event.event_type {
                info!("go back to main menu from settings clicked");
                event.prevent_default();
                event.stop_propagation();
                if let Ok(mut current_menu) = state.get_mut(entity) {
                    *current_menu = MenuState::Main;
                }
            }
            (event_dispatcher_context, event)
        },
    );

    let show_settings_menu = *menu_state == MenuState::Settings;

    info!("show settings menu is {:?}", show_settings_menu);
    if props.menu_state == MenuState::Settings {
        info!("wtf");
        rsx! {
                        <NinePatchBundle
                        styles={container_styles}
                        nine_patch={NinePatch {
                            handle: container,
                            border:{Edge::all(10.0)}
                        }}
                        >
                        <MenuButtonBundle
                            button={MenuButton { text: "back to main menu".into() }}
                            on_event={on_click_back_to_main}
                        />
                        </NinePatchBundle>
                }
    }
    true
}

//     // This is a bevy feature which allows custom
//     // parameters to be passed into a system.
//     // In this case Kayak UI gives the system a
//     // `KayakWidgetContext` and an `Entity`.
//     In((widget_context, entity)): In<(KayakWidgetContext, Entity)>,
//     // The rest of the parameters are just like those found
//     // in a bevy system! In fact you can add whatever
//     // you would like here including more queries or
//     // lookups to resources within bevy's ECS.
//     _game_state: ResMut<State<GameStage>>,
//     mut commands: Commands,
//     images: Res<UiTextureHandles>,
//     // In this case we really only care about our buttons
//     // children! Let's query for them.
//     state: Query<&MenuState>,
//     props: Query<&GameMenuProps>,
// ) -> bool {
//     let props = props.get(entity).unwrap();
//     let parent_id = Some(entity);

//     let container_styles = KStyle {
//         border_radius: StyleProp::Value(Corner::all(15.0)),
//         background_color: StyleProp::Value(Color::WHITE),
//         bottom: StyleProp::Value(Units::Stretch(1.0)),
//         height: StyleProp::Value(Units::Pixels(500.0)),
//         layout_type: StyleProp::Value(LayoutType::Column),
//         left: StyleProp::Value(Units::Stretch(1.0)),
//         padding: StyleProp::Value(Edge::axis(Units::Stretch(1.0), Units::Stretch(0.0))),
//         right: StyleProp::Value(Units::Stretch(1.0)),
//         row_between: StyleProp::Value(Units::Pixels(20.0)),
//         top: StyleProp::Value(Units::Stretch(1.0)),
//         width: StyleProp::Value(Units::Pixels(360.0)),
//         ..Default::default()
//     };
//     let gameboard_spacer_styles = KStyle {
//         bottom: StyleProp::Value(Units::Stretch(1.0)),
//         layout_type: StyleProp::Value(LayoutType::Column),
//         top: StyleProp::Value(Units::Stretch(1.0)),
//         width: StyleProp::Value(Units::Pixels(600.0)),
//         ..Default::default()
//     };

//     let row_styles = KStyle {
//         layout_type: StyleProp::Value(LayoutType::Row),
//         padding_top: StyleProp::Value(Units::Stretch(1.0)),
//         padding_bottom: StyleProp::Value(Units::Stretch(1.0)),
//         ..Default::default()
//     };
//     let left_styles = KStyle {
//         background_color: StyleProp::Value(Color::BLUE),

//         padding_left: StyleProp::Value(Units::Stretch(1.0)),
//         height: StyleProp::Value(Units::Pixels(600.0)),
//         border: StyleProp::Value(Edge::all(25.0)),
//         ..Default::default()
//     };
//     let right_styles = KStyle {
//         background_color: StyleProp::Value(Color::BLUE),

//         height: StyleProp::Value(Units::Pixels(600.0)),
//         border: StyleProp::Value(Edge::all(25.0)),
//         ..Default::default()
//     };

//     let state_entity = widget_context.use_state(&mut commands, entity, MenuState::default());
//     let container = images.panel_brown.clone();

//     let menu_state = if let Ok(current_menu_state) = state.get(state_entity) {
//         current_menu_state
//     } else {
//         &MenuState::Main
//     };

//     let on_click_back_to_main = OnEvent::new(
//         move |In((event_dispatcher_context, _, mut event, _entity)): In<(
//             EventDispatcherContext,
//             WidgetState,
//             Event,
//             Entity,
//         )>,
//               mut state: Query<&mut MenuState>| {
//             if let EventType::Click(..) = event.event_type {
//                 info!("go back to main menu from settings clicked");
//                 event.prevent_default();
//                 event.stop_propagation();
//                 if let Ok(mut current_menu) = state.get_mut(state_entity) {
//                     *current_menu = MenuState::Main;
//                 }
//             }
//             (event_dispatcher_context, event)
//         },
//     );
//     let show_settings_menu = *menu_state == MenuState::Settings;
//     // let show_settings_menu = *menu_state ==
//     // Menu::Settings;
//     info!("show settings menu is {:?}", show_settings_menu);
//     rsx! {
//     <ElementBundle styles={row_styles}>
//         <ElementBundle styles={left_styles}></ElementBundle>
//         <ElementBundle styles={gameboard_spacer_styles}>
//         {
//             if props.game_state == GameStage::Menu && show_settings_menu {
//                 constructor! {
//                     <NinePatchBundle
//                     styles={container_styles}
//                     nine_patch={NinePatch {
//                         handle: container,
//                         border:{Edge::all(10.0)}
//                     }}
//                     >
//                     <MenuButtonBundle
//                         button={MenuButton { text: "Go Back".into() }}
//                         on_event={on_click_back_to_main}
//                     />
//                     </NinePatchBundle>
//                 }
//             }
//         }
//         </ElementBundle>
//         <ElementBundle styles={right_styles}></ElementBundle>
//     </ElementBundle>
//         }
//     true
// }
