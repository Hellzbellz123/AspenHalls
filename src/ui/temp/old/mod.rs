use std::time::Duration;

use bevy::{app::AppExit, prelude::*};
use iyes_loopless::state::{CurrentState, NextState};
use kayak_ui::prelude::{widgets::*, *};

use crate::{
    assets::ImageAssets,
    scoring::{HighScore, Score, Timer},
    ui::button::{
        block_breaker_menu_button_render,
        BlockBreakerMenuButton,
    },
    // settings::GameSettings,
    GameState,
    STARTING_GAME_STATE,
};

mod button;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(KayakContextPlugin)
            .add_plugin(KayakWidgets)
            .add_startup_system(game_ui)
            .add_system(on_game_state_change);
    }
}

// THIS ONLY RUNS ONCE. VERY IMPORTANT FACT.
fn game_ui(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(UICameraBundle::new());
    font_mapping.set_default(
        asset_server.load("roboto.kayak_font"),
    );

    let mut widget_context = KayakRootContext::new();

    let parent_id = None;

    // We need to register the prop and state types.
    // State is empty so you can use the `EmptyState`
    // component!
    widget_context.add_widget_data::<GameMenuProps, Menu>();

    // Next we need to add the systems
    widget_context.add_widget_system(
        // We are registering these systems with a specific
        // WidgetName.
        GameMenuProps::default().get_name(),
        // widget_update auto diffs props and state.
        // Optionally if you have context you can use:
        // widget_update_with_context otherwise you
        // will need to create your own widget update
        // system!
        widget_update::<GameMenuProps, Menu>,
        // Add our render system!
        game_menu_render,
    );

    widget_context
        .add_widget_data::<BlockBreakerMenuButton, ButtonState>();
    widget_context.add_widget_system(
        BlockBreakerMenuButton::default().get_name(),
        widget_update::<BlockBreakerMenuButton, ButtonState>,
        block_breaker_menu_button_render,
    );

    rsx! {
        <KayakAppBundle>
            <GameMenuBundle/>
        </KayakAppBundle>
    }

    commands.insert_resource(widget_context);
}

#[derive(Debug, Clone, PartialEq, Eq, Component)]
pub enum Menu {
    Main,
    Settings,
}

fn on_game_state_change(
    game_state: Res<CurrentState<GameState>>,
    mut game_menu: Query<
        &mut GameMenuProps,
        Without<PreviousWidget>,
    >,
) {
    if game_state.is_changed() {
        for mut game_menu in game_menu.iter_mut() {
            game_menu.game_state = game_state.0;
        }
    }
}

impl Default for Menu {
    fn default() -> Self {
        Menu::Main
    }
}
#[derive(Component, Clone, PartialEq)]
pub struct GameMenuProps {
    game_state: GameState,
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
            props: GameMenuProps::default(),
            styles: KStyle::default(),
            children: KChildren::default(),
            on_event: OnEvent::default(),
            // Kayak uses this component to find out more
            // information about your widget.
            // This is done because bevy does not have the
            // ability to query traits.
            widget_name: GameMenuProps::default()
                .get_name(),
        }
    }
}
pub fn game_menu_render(
    // This is a bevy feature which allows custom
    // parameters to be passed into a system.
    // In this case Kayak UI gives the system a
    // `KayakWidgetContext` and an `Entity`.
    In((widget_context, entity)): In<(
        KayakWidgetContext,
        Entity,
    )>,
    // The rest of the parameters are just like those found
    // in a bevy system! In fact you can add whatever
    // you would like here including more queries or
    // lookups to resources within bevy's ECS.
    mut commands: Commands,
    images: Res<ImageAssets>,
    // In this case we really only care about our buttons
    // children! Let's query for them.
    state: Query<&Menu>,
    props: Query<&GameMenuProps>,
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
        padding: StyleProp::Value(Edge::axis(
            Units::Stretch(1.0),
            Units::Stretch(0.0),
        )),
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
        padding_bottom: StyleProp::Value(Units::Stretch(
            1.0,
        )),
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

    let state_entity = widget_context.use_state(
        &mut commands,
        entity,
        Menu::default(),
    );

    let menu_state = if let Ok(current_menu_state) =
        state.get(state_entity)
    {
        current_menu_state
    } else {
        &Menu::Main
    };

    let container = images.panel.clone();

    let on_click_new_game = OnEvent::new(
        move |In((
            event_dispatcher_context,
            _,
            event,
            _entity,
        )): In<(
            EventDispatcherContext,
            WidgetState,
            Event,
            Entity,
        )>,
              mut commands: Commands| {
            match event.event_type {
                EventType::Click(..) => {
                    commands.insert_resource(NextState(
                        GameState::Playing,
                    ));
                }
                _ => {}
            }
            (event_dispatcher_context, event)
        },
    );

    let on_click_settings = OnEvent::new(
        move |In((
            event_dispatcher_context,
            _,
            mut event,
            _entity,
        )): In<(
            EventDispatcherContext,
            WidgetState,
            Event,
            Entity,
        )>,
              mut state: Query<&mut Menu>| {
            match event.event_type {
                EventType::Click(..) => {
                    event.prevent_default();
                    event.stop_propagation();
                    if let Ok(mut current_menu) =
                        state.get_mut(state_entity)
                    {
                        *current_menu = Menu::Settings;
                    }
                }
                _ => {}
            }
            (event_dispatcher_context, event)
        },
    );

    let on_click_exit = OnEvent::new(
        move |In((
            event_dispatcher_context,
            _,
            event,
            _entity,
        )): In<(
            EventDispatcherContext,
            WidgetState,
            Event,
            Entity,
        )>,
              mut exit: EventWriter<AppExit>| {
            match event.event_type {
                EventType::Click(..) => {
                    exit.send(AppExit);
                }
                _ => {}
            }
            (event_dispatcher_context, event)
        },
    );

    let show_main_menu = *menu_state == Menu::Main;
    // let show_settings_menu = *menu_state ==
    // Menu::Settings;

    rsx! {
    <ElementBundle styles={row_styles}>
      <ElementBundle styles={left_styles}></ElementBundle>
      <ElementBundle styles={gameboard_spacer_styles}>
        {if props.game_state == GameState::Menu && show_main_menu {
          constructor! {
            <NinePatchBundle
              styles={container_styles}
              nine_patch={NinePatch {
                  handle: container,
                  border:{Edge::all(10.0)}
              }}
              >
              <button::BlockBreakerMenuButtonBundle
                button={BlockBreakerMenuButton { text: "New Game".into() }}
                on_event={on_click_new_game}
              />
              <button::BlockBreakerMenuButtonBundle
              button={BlockBreakerMenuButton { text: "Settings".into() }}
              on_event={on_click_settings}
            />
            <button::BlockBreakerMenuButtonBundle
            button={BlockBreakerMenuButton { text: "Exit".into() }}
            on_event={on_click_exit}
          />
            </NinePatchBundle>
          }}}
      </ElementBundle>
      <ElementBundle styles={right_styles}></ElementBundle>
    </ElementBundle>
        }
    true
}
