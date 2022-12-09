use bevy::prelude::*;
use kayak_ui::prelude::{widgets::*, *};

use crate::{
    loading::assets::UiTextureHandles,
    ui::{
        widgets::button::{self, MenuButton},
        MenuState,
    },
};

pub fn update_settings_menu_props(
    menu_state: ResMut<State<MenuState>>,
    mut game_menu_props: Query<&mut SettingsMenuProps, Without<PreviousWidget>>,
) {
    if menu_state.is_changed() {
        for mut props in game_menu_props.iter_mut() {
            props.settings_menu_state = menu_state.current().clone();
        }
    }
}

#[derive(Component, Clone, PartialEq)]
pub struct SettingsMenuProps {
    settings_menu_state: MenuState,
}

impl Default for SettingsMenuProps {
    fn default() -> Self {
        Self {
            settings_menu_state: MenuState::HideMenu,
        }
    }
}

// In the future this will tell Kayak that these
// Props belongs to a widget. For now it's use to
// get the `WidgetName` component.
impl Widget for SettingsMenuProps {}

#[derive(Bundle)]
pub struct SettingsMenuBundle {
    pub name: Name,
    pub props: SettingsMenuProps,
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
            name: Name::new("SettingsMenuProps"),
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
    props: Query<&SettingsMenuProps>,
) -> bool {
    let props = props.get(entity).unwrap();
    let parent_id = Some(entity);

    let state_entity = widget_context.use_state(&mut commands, entity, MenuState::default());

    let container = images.panel_brown.clone();

    let on_click_back_to_main = OnEvent::new(
        move |In((event_dispatcher_context, _, mut event, _entity)): In<(
            EventDispatcherContext,
            WidgetState,
            Event,
            Entity,
        )>,
              mut menu_state_r: ResMut<State<MenuState>>,
              mut state: Query<&mut MenuState>| {
            if let EventType::Click(..) = event.event_type {
                event.prevent_default();
                event.stop_propagation();
                if let Ok(mut current_menu) = state.get_mut(state_entity) {
                    *current_menu = MenuState::Main;
                }
                menu_state_r
                    .pop()
                    .expect("should always be able to pop state?");
            }
            (event_dispatcher_context, event)
        },
    );

    info!(
        "re rendering settings menu widget, can show menu? {:?}",
        props.settings_menu_state == MenuState::Settings
    );

    let row_styles = KStyle {
        layout_type: StyleProp::Value(LayoutType::Row),
        padding_left: StyleProp::Value(Units::Stretch(50.0)),
        padding_right: StyleProp::Value(Units::Percentage(5.0)),
        padding_top: StyleProp::Value(Units::Stretch(50.0)),
        padding_bottom: StyleProp::Value(Units::Percentage(20.0)),
        ..Default::default()
    };

    let pause_container_style = KStyle {
        border_radius: StyleProp::Value(Corner::all(15.0)),
        top: StyleProp::Value(Units::Auto),
        bottom: StyleProp::Value(Units::Auto),
        left: StyleProp::Value(Units::Auto),
        right: StyleProp::Value(Units::Auto),
        padding: StyleProp::Value(Edge::axis(Units::Stretch(1.0), Units::Stretch(1.0))),
        layout_type: StyleProp::Value(LayoutType::Column),
        row_between: StyleProp::Value(Units::Pixels(25.0)),
        min_height: StyleProp::Value(Units::Pixels(440.0)),
        min_width: StyleProp::Value(Units::Pixels(360.0)),
        max_height: StyleProp::Value(Units::Pixels(600.0)),
        max_width: StyleProp::Value(Units::Pixels(360.0)),
        ..Default::default()
    };

    let div_style = KStyle {
        padding: StyleProp::Value(Edge::axis(Units::Stretch(1.0), Units::Stretch(1.0))),
        top: StyleProp::Value(Units::Percentage(10.0)),
        row_between: StyleProp::Value(Units::Pixels(30.0)),
        ..Default::default()
    };

    let title_style = KStyle {
        left: StyleProp::Value(Units::Stretch(1.0)),
        right: StyleProp::Value(Units::Stretch(1.0)),
        ..Default::default()
    };

    let button_style = KStyle {
        top: StyleProp::Value(Units::Auto),
        background_color: StyleProp::Value(Color::WHITE),
        border_color: StyleProp::Value(Color::RED),
        border_radius: StyleProp::Value(Corner::all(15.0)),
        border: StyleProp::Value(Edge::all(5.0)),
        color: StyleProp::Value(Color::RED),
        left: StyleProp::Value(Units::Stretch(20.0)),
        right: StyleProp::Value(Units::Stretch(20.0)),
        height: StyleProp::Value(Units::Stretch(10.0)),
        width: StyleProp::Value(Units::Stretch(10.0)),
        ..Default::default()
    };

    if props.settings_menu_state == MenuState::Settings {
        rsx! {
        <ElementBundle styles={row_styles}>
                    <NinePatchBundle styles={pause_container_style} nine_patch={NinePatch { handle: container, border:{Edge::all(10.0)}}}>
                        <TextWidgetBundle styles={title_style} text={TextProps { content: "Settings Menu".to_string(), size: 32.0, alignment: Alignment::Start, ..default()}}/>
                        <ScrollContextProviderBundle>
                            <ScrollBoxBundle>
                            <button::MenuButtonBundle style={button_style.clone()} button={ MenuButton { text: "one".into()}}/>
                            <button::MenuButtonBundle style={button_style.clone()} button={ MenuButton { text: "two".into()}}/>
                            <button::MenuButtonBundle style={button_style.clone()} button={ MenuButton { text: "three".into()}}/>
                            </ScrollBoxBundle>
                        </ScrollContextProviderBundle>
                        <ElementBundle styles={div_style}>
                            <button::MenuButtonBundle style={button_style} button={ MenuButton { text: "Exit Settings".into()}} on_event={on_click_back_to_main}/>
                        </ElementBundle>
                    </NinePatchBundle>
        </ElementBundle>
        };
    }
    true
}

// <ScrollContextProviderBundle>
//     <ScrollBoxBundle>
//         <TextWidgetBundle
//             text={TextProps {
//                 content: lorem_ipsum,
//                 size: 14.0,
//                 ..Default::default()
//             }}
//         />
//     </ScrollBoxBundle>
// </ScrollContextProviderBundle>

//                     let lorem_ipsum = r#"
// Lorem ipsum dolor sit amet, consectetur adipiscing elit. Cras sed tellus neque. Proin tempus ligula a mi molestie aliquam. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Nullam venenatis consequat ultricies. Sed ac orci purus. Nullam velit nisl, dapibus vel mauris id, dignissim elementum sapien. Vestibulum faucibus sapien ut erat bibendum, id lobortis nisi luctus. Mauris feugiat at lectus at pretium. Pellentesque vitae finibus ante. Nulla non ex neque. Cras varius, lorem facilisis consequat blandit, lorem mauris mollis massa, eget consectetur magna sem vel enim. Nam aliquam risus pulvinar, volutpat leo eget, eleifend urna. Suspendisse in magna sed ligula vehicula volutpat non vitae augue. Phasellus aliquam viverra consequat. Nam rhoncus molestie purus, sed laoreet neque imperdiet eget. Sed egestas metus eget sodales congue.

// Sed vel ante placerat, posuere lacus sit amet, tempus enim. Cras ullamcorper ex vitae metus consequat, a blandit leo semper. Nunc lacinia porta massa, a tempus leo laoreet nec. Sed vel metus tincidunt, scelerisque ex sit amet, lacinia dui. In sollicitudin pulvinar odio vitae hendrerit. Maecenas mollis tempor egestas. Nulla facilisi. Praesent nisi turpis, accumsan eu lobortis vestibulum, ultrices id nibh. Suspendisse sed dui porta, mollis elit sed, ornare sem. Cras molestie est libero, quis faucibus leo semper at.

// Nulla vel nisl rutrum, fringilla elit non, mollis odio. Donec convallis arcu neque, eget venenatis sem mattis nec. Nulla facilisi. Phasellus risus elit, vehicula sit amet risus et, sodales ultrices est. Quisque vulputate felis orci, non tristique leo faucibus in. Duis quis velit urna. Sed rhoncus dolor vel commodo aliquet. In sed tempor quam. Nunc non tempus ipsum. Praesent mi lacus, vehicula eu dolor eu, condimentum venenatis diam. In tristique ligula a ligula dictum, eu dictum lacus consectetur. Proin elementum egestas pharetra. Nunc suscipit dui ac nisl maximus, id congue velit volutpat. Etiam condimentum, mauris ac sodales tristique, est augue accumsan elit, ut luctus est mi ut urna. Mauris commodo, tortor eget gravida lacinia, leo est imperdiet arcu, a ullamcorper dui sapien eget erat.

// Vivamus pulvinar dui et elit volutpat hendrerit. Praesent luctus dolor ut rutrum finibus. Fusce ut odio ultrices, laoreet est at, condimentum turpis. Morbi at ultricies nibh. Mauris tempus imperdiet porta. Proin sit amet tincidunt eros. Quisque rutrum lacus ac est vehicula dictum. Pellentesque nec augue mi.

// Vestibulum rutrum imperdiet nisl, et consequat massa porttitor vel. Ut velit justo, vehicula a nulla eu, auctor eleifend metus. Ut egestas malesuada metus, sit amet pretium nunc commodo ac. Pellentesque gravida, nisl in faucibus volutpat, libero turpis mattis orci, vitae tincidunt ligula ligula ut tortor. Maecenas vehicula lobortis odio in molestie. Curabitur dictum elit sed arcu dictum, ut semper nunc cursus. Donec semper felis non nisl tincidunt elementum.
//     "#.to_string();
