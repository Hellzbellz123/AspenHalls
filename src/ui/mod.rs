pub mod events_handlers;
pub mod main_menu;
mod pause_menu;
mod widgets;

use bevy::{app::AppExit, prelude::*};
use kayak_ui::{
    prelude::{widget_update, FontMapping, KayakRootContext, *},
    widgets::{
        ButtonState, ElementBundle, KayakAppBundle, KayakWidgets, KayakWidgetsContextPlugin,
        NinePatch, NinePatchBundle, TextProps, TextWidgetBundle,
    },
};
use leafwing_input_manager::prelude::ActionState;

use crate::{
    action_manager::actions::PlayerBindables,
    components::OnSplashScreen,
    game::{GameStage, TimeInfo},
    loading::assets::FontHandles,
    ui::{
        main_menu::{game_menu_render, GameMenuBundle, GameMenuProps, MenuState},
        pause_menu::{pause_menu_render, PauseMenuBundle, PauseMenuProps},
        widgets::button::{self, menu_button_render, MenuButton},
    },
};

use self::{
    events_handlers::PlayButtonEvent, main_menu::on_game_state_change,
    pause_menu::update_pause_menu_props,
};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<PlayButtonEvent>()
            .add_state(MenuState::HideMenu)
            .add_plugin(KayakContextPlugin)
            .add_plugin(KayakWidgets)
            .add_system_set(
                SystemSet::on_enter(GameStage::Menu)
                    .with_system(despawn_screen::<OnSplashScreen>)
                    .with_system(game_ui)
                    .with_system(trace_ui),
            )
            .add_system_set(
                SystemSet::on_enter(GameStage::FailedLoading).with_system(failed_load_ui),
            )
            .add_system_set(SystemSet::on_update(GameStage::Playing))
            .add_system(show_pause_menu)
            .add_system(on_game_state_change)
            .add_system(update_pause_menu_props);
    }
}

pub fn show_pause_menu(
    mut timeinfo: ResMut<TimeInfo>,
    query_action_state: Query<&ActionState<PlayerBindables>>,
    mut menu_state: ResMut<State<MenuState>>,
    event_reader: EventReader<PlayButtonEvent>,
) {
    if !query_action_state.is_empty()
        && (query_action_state
            .get_single()
            .expect("should always only ever be one")
            .just_pressed(PlayerBindables::Pause)
            || !event_reader.is_empty())
    {
        let mut timeinfo = timeinfo.as_mut();

        if menu_state.current() == &MenuState::Pause {
            //if calling this function and MenuState is pause we are already paused and want too unpause
            timeinfo.pause_menu = false;
            timeinfo.game_paused = false;
            timeinfo.time_step = 1.0;
            menu_state
                .set(MenuState::HideMenu)
                .expect("couldnt push hidemenu state");
            event_reader.clear();
        } else if menu_state.current() == &MenuState::HideMenu {
            //if calling this function and MenuState is HideMenu we want too set menustate too pause and freeze time. kayak will listen for the menustate.
            timeinfo.pause_menu = true;
            timeinfo.game_paused = true;
            timeinfo.time_step = 0.;
            menu_state
                .set(MenuState::Pause)
                .expect("couldnt push pausemenu state");
            event_reader.clear();
        }

        if timeinfo.pause_menu {
            info!("pause menu should be shown and game should be paused")
        } else {
            info!("no pause menu should be shown and game should be playing")
        }
    }
}

fn trace_ui() {
    info!("setting up UI");
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in to_despawn.iter() {
        info!("despawning entity: {:#?}", entity);
        commands.entity(entity).despawn_recursive();
    }
}

// THIS ONLY RUNS ONCE. VERY IMPORTANT FACT.
pub fn game_ui(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    fonts: Res<FontHandles>,
) {
    font_mapping.set_default(fonts.fantasque_sans_msdf.clone());

    let mut widget_context = KayakRootContext::new();
    widget_context.add_plugin(KayakWidgetsContextPlugin);

    let parent_id = None;

    // We need to register the prop and state types.
    // if State is empty you can use the `EmptyState`
    // component!
    widget_context.add_widget_data::<GameMenuProps, MenuState>();
    widget_context.add_widget_data::<PauseMenuProps, MenuState>();

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
        widget_update::<GameMenuProps, MenuState>,
        // Add our render system!
        game_menu_render,
    );

    widget_context.add_widget_system(
        // We are registering these systems with a specific
        // WidgetName.
        PauseMenuProps::default().get_name(),
        // widget_update auto diffs props and state.
        // Optionally if you have context you can use:
        // widget_update_with_context otherwise you
        // will need to create your own widget update
        // system!
        widget_update::<PauseMenuProps, EmptyState>,
        // Add our render system!
        pause_menu_render,
    );

    widget_context.add_widget_data::<MenuButton, ButtonState>();
    widget_context.add_widget_system(
        MenuButton::default().get_name(),
        widget_update::<MenuButton, ButtonState>,
        menu_button_render,
    );

    rsx! {
        <KayakAppBundle>
            <GameMenuBundle/>
            <PauseMenuBundle/>
        </KayakAppBundle>
    }
    commands.spawn((UICameraBundle::new(widget_context), Name::new("UI Camera")));
}

pub fn failed_load_ui(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    fonts: Res<FontHandles>,
) {
    font_mapping.set_default(fonts.fantasque_sans_msdf.clone());

    let mut widget_context = KayakRootContext::new();
    widget_context.add_plugin(KayakWidgetsContextPlugin);

    let parent_id = None;

    // We need to register the prop and state types.
    // if State is empty you can use the `EmptyState`
    // component!
    widget_context.add_widget_data::<GameMenuProps, MenuState>();
    widget_context.add_widget_data::<PauseMenuProps, MenuState>();

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
        widget_update::<GameMenuProps, MenuState>,
        // Add our render system!
        game_menu_render,
    );

    widget_context.add_widget_system(
        // We are registering these systems with a specific
        // WidgetName.
        PauseMenuProps::default().get_name(),
        // widget_update auto diffs props and state.
        // Optionally if you have context you can use:
        // widget_update_with_context otherwise you
        // will need to create your own widget update
        // system!
        widget_update::<PauseMenuProps, EmptyState>,
        // Add our render system!
        pause_menu_render,
    );

    widget_context.add_widget_data::<MenuButton, ButtonState>();
    widget_context.add_widget_system(
        MenuButton::default().get_name(),
        widget_update::<MenuButton, ButtonState>,
        menu_button_render,
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

    rsx! {
        <KayakAppBundle>
                    <NinePatchBundle styles={ninepatch_style} nine_patch={NinePatch {border:{Edge::all(1.0)}, ..default()}}>
                    <TextWidgetBundle text={TextProps { content: "loading game failed. there was missing assets".to_string(), size: 32.0, alignment: Alignment::Middle, ..default()}}/>
                    <ElementBundle/>
                    <button::MenuButtonBundle button={ MenuButton { text: "exit game".into(), ..default() }} on_event={on_click_exit}/>
                    </NinePatchBundle>
        </KayakAppBundle>
    }
    commands.spawn((UICameraBundle::new(widget_context), Name::new("UI Camera")));
}
