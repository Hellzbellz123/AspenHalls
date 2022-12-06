use bevy::{app::AppExit, prelude::*};
use bevy_inspector_egui::Inspectable;
use kayak_ui::prelude::{widgets::*, *};

use crate::{
    game::GameStage,
    loading::assets::UiTextureHandles,
    ui::widgets::button::{self, MenuButton},
};

#[derive(Debug, Clone, PartialEq, Eq, Component, Reflect, Inspectable, Hash)]
pub enum MenuState {
    HideMenu,
    Main,
    Pause,
    Settings,
}

pub fn on_game_state_change(
    game_state: Res<State<GameStage>>,
    mut game_menu_props: Query<&mut GameMenuProps, Without<PreviousWidget>>,
) {
    if game_state.is_changed() {
        for mut game_menu in game_menu_props.iter_mut() {
            game_menu.game_state = game_state.current().clone()
        }
    }
    // if menu_state.is_changed() {
    //     for mut props in game_menu_props.iter_mut() {
    //         props.menu_state = menu_state.current().clone()
    //     }
    // }
}

impl Default for MenuState {
    fn default() -> Self {
        MenuState::Main
    }
}
#[derive(Component, Clone, PartialEq)]
pub struct GameMenuProps {
    pub game_state: GameStage,
    // menu_state: MenuState,
}

impl Default for GameMenuProps {
    fn default() -> Self {
        Self {
            game_state: GameStage::Menu,
            // menu_state: MenuState::Main,
        }
    }
}

// In the future this will tell Kayak that these
// Props belongs to a widget. For now it's use to
// get the `WidgetName` component.
impl Widget for GameMenuProps {}

#[derive(Bundle)]
pub struct GameMenuBundle {
    pub name: Name,
    pub props: GameMenuProps,
    pub styles: KStyle,
    pub children: KChildren,
    // This allows us to hook into on click events!
    pub on_event: OnEvent,
    // Widget name is required by Kayak UI!
    pub widget_name: WidgetName,
}
impl Default for GameMenuBundle {
    fn default() -> Self {
        Self {
            name: Name::new("GameMenuProps"),
            props: GameMenuProps::default(),
            styles: KStyle::default(),
            children: KChildren::default(),
            on_event: OnEvent::default(),
            // Kayak uses this component to find out more
            // information about your widget.
            // This is done because bevy does not have the
            // ability to query traits.
            widget_name: GameMenuProps::default().get_name(),
        }
    }
}
pub fn game_menu_render(
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
    state: Query<&MenuState>,
    props: Query<&GameMenuProps>,
) -> bool {
    let props = props.get(entity).unwrap();
    let parent_id = Some(entity);

    let state_entity = widget_context.use_state(&mut commands, entity, MenuState::default());

    let propmenu_state = if let Ok(current_menu_state) = state.get(state_entity) {
        current_menu_state
    } else {
        &MenuState::Main
    };

    let container = images.panel_brown.clone();

    let on_click_new_game = OnEvent::new(
        move |In((event_dispatcher_context, _, event, _entity)): In<(
            EventDispatcherContext,
            WidgetState,
            Event,
            Entity,
        )>,
              mut game_state: ResMut<State<GameStage>>| {
            if let EventType::Click(..) = event.event_type {
                game_state
                    .push(GameStage::Playing)
                    .expect("cant push state for some reason")
            }
            (event_dispatcher_context, event)
        },
    );

    let on_click_settings = OnEvent::new(
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
                    *current_menu = MenuState::Settings;
                }
            }
            (event_dispatcher_context, event)
        },
    );

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

    let hide_menu = !(props.game_state == GameStage::Menu);
    let show_main_menu = *propmenu_state == MenuState::Main;
    let show_settings_menu = *propmenu_state == MenuState::Settings;

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
    let ninepatch_style = KStyle {
        border_radius: StyleProp::Value(Corner::all(15.0)),
        background_color: StyleProp::Value(Color::WHITE),
        bottom: StyleProp::Value(Units::Percentage(70.0)),
        top: StyleProp::Value(Units::Percentage(30.0)),
        left: StyleProp::Value(Units::Percentage(50.0)),
        right: StyleProp::Value(Units::Percentage(50.0)),
        layout_type: StyleProp::Value(LayoutType::Column),
        row_between: StyleProp::Value(Units::Pixels(50.0)),
        padding: StyleProp::Value(Edge::axis(Units::Stretch(20.0), Units::Stretch(0.0))),
        height: StyleProp::Value(Units::Pixels(500.0)),
        width: StyleProp::Value(Units::Pixels(460.0)),
        ..Default::default()
    };

    // <ElementBundle styles={left_styles = Kstyle{....}}></ElementBundle>
    // <ElementBundle styles={right_styles = Kstyle{....}}></ElementBundle>

    rsx! {
        <ElementBundle styles={row_styles}>
        <ElementBundle styles={middle_style}>
        {
        if !hide_menu {
            if show_main_menu {
                constructor! { // TODO: the logic for the menu showing SEEMS to be correct. refactor these into thier own widgets so we can try to make it look nice
                    <NinePatchBundle styles={ninepatch_style} nine_patch={ NinePatch { handle: container, border:{ Edge::all(5.0)}}}>
                    <TextWidgetBundle text={TextProps { content: "Vanilla Coffee".to_string(), size: 52.0, alignment: Alignment::Middle, ..default()}}/>
                    <ElementBundle/>
                    <button::MenuButtonBundle button={ MenuButton { text: "New Game".into(), ..default()}} on_event={on_click_new_game} />
                    <button::MenuButtonBundle button={ MenuButton { text: "Settings".into(), ..default() }} on_event={on_click_settings} />
                    <button::MenuButtonBundle button={ MenuButton { text: "Exit".into(), ..default() }} on_event={on_click_exit} />
                    </NinePatchBundle>
                }
            } else if show_settings_menu {
                constructor! {
                    <NinePatchBundle styles={ninepatch_style} nine_patch={NinePatch { handle: container, border:{Edge::all(1.0)}}}>
                    <TextWidgetBundle text={TextProps { content: "Settings Menu".to_string(), size: 32.0, alignment: Alignment::Middle, ..default()}}/>
                    <ElementBundle/>
                    <button::MenuButtonBundle button={ MenuButton { text: "go back".into(), ..default() }} on_event={on_click_back_to_main}/>
                    </NinePatchBundle>
                }
            }
        }
        }
        </ElementBundle>
        </ElementBundle>
    }

    true
}
