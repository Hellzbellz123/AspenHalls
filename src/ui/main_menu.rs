use bevy::{app::AppExit, prelude::*};
use bevy_inspector_egui::Inspectable;
use kayak_ui::prelude::{widgets::*, *};
use leafwing_input_manager::prelude::ActionState;

use crate::{
    game::GameStage,
    loading::assets::UiTextureHandles,
    ui::widgets::button::{self, MenuButton}, action_manager::actions::PlayerBindables,
};

const STARTING_GAME_STATE: GameStage = GameStage::Menu;

#[derive(Debug, Clone, PartialEq, Eq, Component, Reflect, Inspectable)]
pub enum MenuState {
    Main,
    Pause,
    Settings,
}

pub fn on_game_state_change(
    game_state: Res<State<GameStage>>,
    mut game_menu: Query<&mut GameMenuProps, Without<PreviousWidget>>,
) {
    if game_state.is_changed() {
        for mut game_menu in game_menu.iter_mut() {
            game_menu.game_state = game_state.current().clone();
        }
    }
}

impl Default for MenuState {
    fn default() -> Self {
        MenuState::Main
    }
}
#[derive(Component, Clone, PartialEq)]
pub struct GameMenuProps {
    game_state: GameStage,
}

impl Default for GameMenuProps {
    fn default() -> Self {
        Self {
            game_state: STARTING_GAME_STATE,
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
    query_action_state: Query<&ActionState<PlayerBindables>>,
) -> bool {
    let props = props.get(entity).unwrap();
    let parent_id = Some(entity);

    if !query_action_state.is_empty() {
        
    }

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
    let gameboard_spacer_styles = KStyle {
        bottom: StyleProp::Value(Units::Stretch(1.0)),
        layout_type: StyleProp::Value(LayoutType::Column),
        top: StyleProp::Value(Units::Stretch(1.0)),
        width: StyleProp::Value(Units::Pixels(600.0)),
        ..Default::default()
    };

    let row_styles = KStyle {
        layout_type: StyleProp::Value(LayoutType::Row),
        padding_top: StyleProp::Value(Units::Stretch(1.0)),
        padding_bottom: StyleProp::Value(Units::Stretch(1.0)),
        ..Default::default()
    };
    let left_styles = KStyle {
        background_color: StyleProp::Value(Color::BLUE),

        padding_left: StyleProp::Value(Units::Stretch(1.0)),
        height: StyleProp::Value(Units::Pixels(600.0)),
        border: StyleProp::Value(Edge::all(25.0)),
        ..Default::default()
    };
    let right_styles = KStyle {
        background_color: StyleProp::Value(Color::BLUE),

        height: StyleProp::Value(Units::Pixels(600.0)),
        border: StyleProp::Value(Edge::all(25.0)),
        ..Default::default()
    };

    let state_entity = widget_context.use_state(&mut commands, entity, MenuState::default());

    let menu_state = if let Ok(current_menu_state) = state.get(state_entity) {
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

    // let hide_menu = !(props.game_state == GameStage::Menu || props.game_state == GameStage::Playing && show_pause_menu);
    let hide_menu = !(props.game_state == GameStage::Menu);
    let show_main_menu = *menu_state == MenuState::Main;
    let show_settings_menu = *menu_state == MenuState::Settings;
    let show_pause_menu = *menu_state == MenuState::Pause && hide_menu;

    rsx! {
    <ElementBundle styles={row_styles}>
        <ElementBundle styles={left_styles}></ElementBundle>
        <ElementBundle styles={gameboard_spacer_styles}>
            {if !hide_menu && show_main_menu {
            constructor! { // TODO: the logic for the menu showing SEEMS to be correct. refactor these into thier own widgets so we can try to make it look nice
                <NinePatchBundle
                styles={container_styles}
                nine_patch={NinePatch {
                    handle: container,
                    border:{Edge::all(10.0)}
                }}
                >
                <button::MenuButtonBundle
                    button={MenuButton { text: "New Game".into() }}
                    on_event={on_click_new_game}
                />
                <button::MenuButtonBundle
                button={MenuButton { text: "Settings".into() }}
                on_event={on_click_settings}
                />
                <button::MenuButtonBundle
                button={MenuButton { text: "Exit".into() }}
                on_event={on_click_exit}
                />
                </NinePatchBundle>
            }
        } else if !hide_menu && show_settings_menu {
            constructor! {
                <NinePatchBundle styles={container_styles} nine_patch={NinePatch { handle: container, border:{Edge::all(10.0)}}}>
                <button::MenuButtonBundle button={ MenuButton { text: "go back".into() }} on_event={on_click_back_to_main}/>
                </NinePatchBundle>
            }
        } else if hide_menu && show_pause_menu {
            constructor! {
                <NinePatchBundle styles={container_styles} nine_patch={NinePatch { handle: container, border:{Edge::all(10.0)}}}>
                <button::MenuButtonBundle button={ MenuButton { text: "Pause Menu".into() }} />
                <button::MenuButtonBundle button={ MenuButton { text: "Back To Main Menu".into() }} on_event={on_click_back_to_main}/>
                <button::MenuButtonBundle button={ MenuButton { text: "Exit".into() }} on_event={on_click_exit} />
                </NinePatchBundle>
            }
        }
        }
        </ElementBundle>
        <ElementBundle styles={right_styles}></ElementBundle>
    </ElementBundle>
        }
    true
}
