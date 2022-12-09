use bevy::{app::AppExit, prelude::*};
use kayak_ui::prelude::{widgets::*, *};

use crate::{
    loading::assets::UiTextureHandles,
    ui::{
        events_handlers::PlayButtonEvent,
        widgets::button::{self, MenuButton},
        MenuState
    },
};

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

    let _state_entity = widget_context.use_state(&mut commands, entity, MenuState::default());

    let container = images.panel_brown.clone();

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

    if props.pause_menu_state == MenuState::Pause {
        rsx! {
        <ElementBundle styles={row_styles}>
                    <NinePatchBundle styles={pause_container_style} nine_patch={NinePatch { handle: container, border:{Edge::all(10.0)}}}>
                        <TextWidgetBundle styles={title_style} text={TextProps { content: "Pause Menu".to_string(), size: 32.0, alignment: Alignment::Start, ..default()}}/>
                        <ElementBundle styles={div_style}>
                            <button::MenuButtonBundle style={button_style.clone()} button={ MenuButton { text: "Resume Game".into()}} on_event={on_click_resume}/>
                            <button::MenuButtonBundle style={button_style} button={ MenuButton { text: "Exit".into()}} on_event={on_click_exit} />
                        </ElementBundle>
                    </NinePatchBundle>
        </ElementBundle>
        };
    }
    true
}
