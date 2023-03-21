use bevy::{app::AppExit, prelude::*};
use kayak_ui::prelude::{widgets::*, *};

use crate::{
    game::GameStage,
    loading::assets::UiTextureHandles,
    ui::{
        widgets::button::{self, MenuButton},
        MenuState,
    },
};

pub fn update_main_menu_props(
    menu_state: Res<State<MenuState>>,
    mut main_menu_props: Query<&mut MainMenuProps, Without<PreviousWidget>>,
) {
    if menu_state.is_changed() {
        for mut main_menu in main_menu_props.iter_mut() {
            main_menu.menu_state = menu_state.current().clone()
        }
    }
}

#[derive(Component, Clone, PartialEq)]
pub struct MainMenuProps {
    // pub game_state: GameStage,
    menu_state: MenuState,
}

impl Default for MainMenuProps {
    fn default() -> Self {
        Self {
            // game_state: GameStage::Menu,
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
            name: Name::new("MainMenuProps"),
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

#[must_use]
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
    images: Res<UiTextureHandles>,
    menu_state: ResMut<State<MenuState>>,
) -> bool {
    let parent_id = Some(entity);
    let state_entity = widget_context.use_state(&mut commands, entity, MenuState::default());
    let container = images.panel_brown.clone();

    let on_click_new_game = OnEvent::new(
        move |In((event_dispatcher_context, _, event, _entity)): In<(
            EventDispatcherContext,
            WidgetState,
            Event,
            Entity,
        )>,
              mut game_state: ResMut<State<GameStage>>,
              mut menu_state_r: ResMut<State<MenuState>>| {
            if let EventType::Click(..) = event.event_type {
                menu_state_r
                    .set(MenuState::HideMenu)
                    .expect("couldnt set menustate to hideMenu");
                game_state
                    .push(GameStage::PlaySubStage)
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
              mut state: Query<&mut MenuState>,
              mut menu_state_r: ResMut<State<MenuState>>| {
            if let EventType::Click(..) = event.event_type {
                event.prevent_default();
                event.stop_propagation();
                if let Ok(mut current_menu) = state.get_mut(state_entity) {
                    *current_menu = MenuState::Settings;
                    menu_state_r
                        .push(MenuState::Settings)
                        .expect("couldnt push menustate")
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

    let show_main_menu = *menu_state.current() == MenuState::Main;

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

    if show_main_menu {
        rsx! {
            <ElementBundle styles={row_styles}>
            <ElementBundle styles={middle_style}>
                    <NinePatchBundle styles={ninepatch_style} nine_patch={ NinePatch { handle: container, border:{ Edge::all(5.0)}}}>
                    <TextWidgetBundle text={TextProps { content: "Vanilla Coffee".to_string(), size: 52.0, alignment: Alignment::Middle, ..default()}}/>
                    <ElementBundle/>
                    <button::MenuButtonBundle button={ MenuButton { text: "New Game".into()}} on_event={on_click_new_game} />
                    <button::MenuButtonBundle button={ MenuButton { text: "Settings".into()}} on_event={on_click_settings} />
                    <button::MenuButtonBundle button={ MenuButton { text: "Exit".into()}} on_event={on_click_exit} />
                    </NinePatchBundle>
            </ElementBundle>
            </ElementBundle>
        };
    }

    true
}
