use bevy::{app::AppExit, prelude::*};

use kayak_ui::prelude::{widgets::*, *};

use crate::{
    loading::assets::UiTextureHandles,
    ui::{
        events_handlers::PlayButtonEvent,
        widgets::button::{self, MenuButton},
    },
};

use super::main_menu::MenuState;

pub fn update_pause_menu_props(
    menu_state: ResMut<State<MenuState>>,
    mut game_menu_props: Query<&mut PauseMenuProps, Without<PreviousWidget>>,
) {
    if menu_state.is_changed() {
        for mut props in game_menu_props.iter_mut() {
            props.pause_menu_state = menu_state.current().clone();
        }
    }
}

#[derive(Component, Clone, PartialEq)]
pub struct PauseMenuProps {
    pause_menu_state: MenuState,
}

impl Default for PauseMenuProps {
    fn default() -> Self {
        Self {
            pause_menu_state: MenuState::HideMenu,
        }
    }
}

// In the future this will tell Kayak that these
// Props belongs to a widget. For now it's use to
// get the `WidgetName` component.
impl Widget for PauseMenuProps {}

#[derive(Bundle)]
pub struct PauseMenuBundle {
    pub name: Name,
    pub props: PauseMenuProps,
    pub styles: KStyle,
    pub children: KChildren,
    // This allows us to hook into on click events!
    pub on_event: OnEvent,
    // Widget name is required by Kayak UI!
    pub widget_name: WidgetName,
}
impl Default for PauseMenuBundle {
    fn default() -> Self {
        Self {
            name: Name::new("PauseMenuProps"),
            props: PauseMenuProps::default(),
            styles: KStyle::default(),
            children: KChildren::default(),
            on_event: OnEvent::default(),
            // Kayak uses this component to find out more
            // information about your widget.
            // This is done because bevy does not have the
            // ability to query traits.
            widget_name: PauseMenuProps::default().get_name(),
        }
    }
}
pub fn pause_menu_render(
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
    props: Query<&PauseMenuProps>,
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
              mut state: Query<&mut MenuState>| {
            if let EventType::Click(..) = event.event_type {
                event.prevent_default();
                event.stop_propagation();
                if let Ok(mut current_menu) = state.get_mut(state_entity) {
                    *current_menu = MenuState::Main;
                }
            }
            (event_dispatcher_context, event)
        },
    );

    let on_click_exit = OnEvent::new(
        move |In((event_dispatcher_context, _, event, _entity)): In<(
            EventDispatcherContext,
            WidgetState,
            Event,
            Entity,
        )>,
              mut exit: EventWriter<AppExit>| {
            if let EventType::Click(..) = event.event_type {
                exit.send(AppExit);
            }
            (event_dispatcher_context, event)
        },
    );

    let on_click_resume = OnEvent::new(
        move |In((event_dispatcher_context, _, event, _entity)): In<(
            EventDispatcherContext,
            WidgetState,
            Event,
            Entity,
        )>,
              mut exit: EventWriter<PlayButtonEvent>| {
            if let EventType::Click(..) = event.event_type {
                exit.send(PlayButtonEvent);
            }
            (event_dispatcher_context, event)
        },
    );

    info!(
        "re rendering pause menu widget, can show menu? {:?}",
        props.pause_menu_state == MenuState::Pause
    );

    
    let button_style = KStyle {
        ..Default::default()
    };
    
    let bg_style = KStyle {
        // background_color: StyleProp::Value(Color::BLUE),
        // height: StyleProp::Value(Units::Percentage(100.0)),
        // width: StyleProp::Value(Units::Percentage(100.0)),
        // left: StyleProp::Value(Units::Pixels(0.0)),
        // right: StyleProp::Value(Units::Pixels(0.0)),
        // top: StyleProp::Value(Units::Pixels(0.0)),
        // bottom: StyleProp::Value(Units::Percentage(0.0)),
        // padding_bottom: StyleProp::Value(Units::Percentage(30.0)),
        // padding_left: StyleProp::Value(Units::Percentage(60.0)),
        // padding_right: StyleProp::Value(Units::Percentage(10.0)),
        // padding_top: StyleProp::Value(Units::Percentage(10.0)),
        ..Default::default()
    };

    
    let row_style = KStyle {
        layout_type: StyleProp::Value(LayoutType::Row),
        padding_top: StyleProp::Value(Units::Percentage(20.0)),
        padding_bottom: StyleProp::Value(Units::Percentage(30.0)),
        padding_left: StyleProp::Value(Units::Percentage(45.0)),
        padding_right: StyleProp::Value(Units::Percentage(5.0)),
        ..Default::default()
    };
    
    let button_style = KStyle {
        height: StyleProp::Value(Units::Pixels(20.0)),
        width: StyleProp::Value(Units::Pixels(90.0)),
        // left: StyleProp::Value(Units::Percentage(20.0)),
        // right: StyleProp::Value(Units::Percentage(20.0)),
        // top: StyleProp::Value(Units::Percentage(20.0)),
        // bottom: StyleProp::Value(Units::Percentage(20.0)),
        ..Default::default()
    };

    let pause_container_style = KStyle {
        border_radius: StyleProp::Value(Corner::all(15.0)),
        top: StyleProp::Value(Units::Percentage(50.0)),
        bottom: StyleProp::Value(Units::Percentage(50.0)),
        left: StyleProp::Value(Units::Percentage(50.0)),
        right: StyleProp::Value(Units::Percentage(50.0)),
        height: StyleProp::Value(Units::Pixels(500.0)),
        width: StyleProp::Value(Units::Pixels(460.0)),
        layout_type: StyleProp::Value(LayoutType::Column),
        row_between: StyleProp::Value(Units::Pixels(20.0)),
        ..Default::default()
    };
    let row_styles = KStyle {
        layout_type: StyleProp::Value(LayoutType::Row),
        padding_top: StyleProp::Value(Units::Stretch(20.0)),
        padding_bottom: StyleProp::Value(Units::Stretch(20.0)),
        ..Default::default()
    };
    let middle_style = KStyle {
        // width: StyleProp::Value(Units::Pixels(600.0)),
        layout_type: StyleProp::Value(LayoutType::Column),
        bottom: StyleProp::Value(Units::Auto),
        top: StyleProp::Value(Units::Auto),
        ..Default::default()
    };

    if props.pause_menu_state == MenuState::Pause {
        rsx! {
            <ElementBundle styles={row_styles}>
                <ElementBundle styles={middle_style}>
                    <NinePatchBundle styles={pause_container_style} nine_patch={NinePatch { handle: container, border:{Edge::all(10.0)}}}>
                        <TextWidgetBundle text={TextProps { content: "Pause Menu".to_string(), size: 32.0, alignment: Alignment::Middle, ..default()}}/>
                        <ElementBundle/>
                        <button::MenuButtonBundle button={ MenuButton { text: "Resume Game".into()}} on_event={on_click_resume} />
                        <button::MenuButtonBundle button={ MenuButton { text: "Back To Main Menu".into()}} on_event={on_click_back_to_main}/>
                        <button::MenuButtonBundle button={ MenuButton { text: "Exit".into()}} on_event={on_click_exit} />
                    </NinePatchBundle>
                </ElementBundle>
            </ElementBundle>
        }
    }
    true
}
