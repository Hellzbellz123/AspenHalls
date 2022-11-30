pub mod events_handlers;

use crate::{game::GameStage, loading::assets::FontHandles, ui::events_handlers::PlayButtonEvent};
use bevy::prelude::*;

//builds menus for vanillacoffee, both ingame and main menu
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_resource::<PreloadResource>()
            .add_event::<events_handlers::PlayButtonEvent>()
            .add_plugin(KayakContextPlugin)
            .add_plugin(KayakWidgets)
            .add_system_set(
                SystemSet::on_update(GameStage::Menu)
                    .with_system(events_handlers::play_button_event), // .with_system(pass_to_play::<OnSplashScreen>),
            )
            .add_system_set(SystemSet::on_enter(GameStage::Menu).with_system(startup));
    }
}

// .add_startup_system()
// fn pass_to_play<T: Component>(
//     asset_server: ResMut<AssetServer>,

//     time: Res<Time>,
//     mut timer: ResMut<SplashTimer>,
//     to_despawn: Query<Entity, With<T>>,
//     mut commands: Commands,
//     mut game_state: ResMut<State<GameStage>>,
// ) {
//     let img: Handle<Image> = asset_server.load("images/splash/splashL.png");
//     let mut state_pushed;

//     let imgloadstate = asset_server.get_load_state(img);

//     if imgloadstate == LoadState::Loaded
//         && timer.tick(time.delta()).finished()
//         && game_state.current() == &GameStage::Menu
//     {
//         state_pushed = false;

//         // info!("splash asset loaded");
//         // game_state.set(GameStage::Menu).unwrap(); //TODO:change back too menu when updated too new kayakui version
//         info!("pushing playing state too state stack");
//         if !state_pushed {
//             game_state
//                 .push(GameStage::Playing)
//                 .expect("couldnt push state to stack");

//             state_pushed = true;

//             for entity in to_despawn.iter() {
//                 info!("despawning entity: {:#?}", entity);
//                 commands.entity(entity).despawn_recursive();
//             }
//         } else if state_pushed {
//             info!(" do nothing?")
//         }
//     }
// }

use bevy::app::AppExit;
use kayak_ui::prelude::{widgets::*, *};

#[derive(Default, Clone, PartialEq, Component)]
pub struct MenuButton {
    text: String,
}

impl Widget for MenuButton {}

#[derive(Bundle)]
pub struct MenuButtonBundle {
    button: MenuButton,
    styles: KStyle,
    on_event: OnEvent,
    widget_name: WidgetName,
}

impl Default for MenuButtonBundle {
    fn default() -> Self {
        Self {
            button: Default::default(),
            styles: KStyle {
                bottom: Units::Pixels(20.0).into(),
                cursor: KCursorIcon(CursorIcon::Hand).into(),
                ..Default::default()
            },
            on_event: OnEvent::default(),
            widget_name: MenuButton::default().get_name(),
        }
    }
}

fn menu_button_render(
    In((widget_context, entity)): In<(KayakWidgetContext, Entity)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    menu_button_query: Query<&MenuButton>,
    state_query: Query<&ButtonState>,
) -> bool {
    let state_entity =
        widget_context.use_state(&mut commands, entity, ButtonState { hovering: false });

    let button_text = menu_button_query.get(entity).unwrap().text.clone();
    let button_image = asset_server.load("ui/main_menu/button.png");
    let button_image_hover = asset_server.load("ui/main_menu/button-hover.png");

    let on_event = OnEvent::new(
        move |In((event_dispatcher_context, _, mut event, _entity)): In<(
            EventDispatcherContext,
            WidgetState,
            Event,
            Entity,
        )>,
              mut query: Query<&mut ButtonState>| {
            if let Ok(mut button) = query.get_mut(state_entity) {
                match event.event_type {
                    EventType::MouseIn(..) => {
                        event.stop_propagation();
                        button.hovering = true;
                    }
                    EventType::MouseOut(..) => {
                        button.hovering = false;
                    }
                    _ => {}
                }
            }
            (event_dispatcher_context, event)
        },
    );

    if let Ok(button_state) = state_query.get(state_entity) {
        let button_image_handle = if button_state.hovering {
            button_image_hover
        } else {
            button_image
        };

        let parent_id = Some(entity);
        rsx! {
            <NinePatchBundle
                nine_patch={NinePatch {
                    handle: button_image_handle,
                    border: Edge::all(10.0),
                }}
                styles={KStyle {
                    width: Units::Stretch(1.0).into(),
                    height: Units::Pixels(40.0).into(),
                    bottom: Units::Pixels(30.0).into(),
                    left: Units::Pixels(50.0).into(),
                    right: Units::Pixels(50.0).into(),
                    ..KStyle::default()
                }}
                on_event={on_event}
            >
                <TextWidgetBundle
                    styles={KStyle {
                        top: Units::Stretch(1.0).into(),
                        bottom: Units::Stretch(1.0).into(),
                        ..Default::default()
                    }}
                    text={TextProps {
                        alignment: Alignment::Middle,
                        content: button_text,
                        size: 28.0,
                        ..Default::default()
                    }}
                />
            </NinePatchBundle>
        }
    }
    true
}

#[derive(Default, Resource)]
pub struct PreloadResource {
    images: Vec<Handle<Image>>,
}

fn startup(
    mut commands: Commands,
    font_assets: Res<FontHandles>,
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
    mut preload_resource: ResMut<PreloadResource>,
) {
    font_mapping.set_default(font_assets.fantasque_sans_msdf.clone());

    let mut widget_context = KayakRootContext::new();
    widget_context.add_plugin(KayakWidgetsContextPlugin);
    widget_context.add_widget_data::<MenuButton, ButtonState>();
    widget_context.add_widget_system(
        MenuButton::default().get_name(),
        widget_update::<MenuButton, ButtonState>,
        menu_button_render,
    );

    let panel1_image = asset_server.load("ui/main_menu/panel1.png");
    let logo_image = asset_server.load("ui/main_menu/logo.png");
    let kayak_image = asset_server.load("ui/main_menu/kayak.png");
    let button_image = asset_server.load("ui/main_menu/button.png");
    let button_image_hover = asset_server.load("ui/main_menu/button-hover.png");

    preload_resource.images.extend(vec![
        panel1_image.clone(),
        logo_image.clone(),
        button_image,
        button_image_hover,
    ]);

    let handle_click_close = OnEvent::new(
        move |In((event_dispatcher_context, _, event, _entity)): In<(
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

    let handle_click_play = OnEvent::new(
        move |In((event_dispatcher_context, _, event, _entity)): In<(
            EventDispatcherContext,
            WidgetState,
            Event,
            Entity,
        )>,
              mut play: EventWriter<PlayButtonEvent>| {
            match event.event_type {
                EventType::Click(..) => {
                    play.send(PlayButtonEvent);
                    info!("play button clicked");
                }
                _ => {}
            }
            (event_dispatcher_context, event)
        },
    );

    let parent_id = None;
    rsx! {
        <KayakAppBundle>
            <NinePatchBundle
                nine_patch={NinePatch {
                    handle: panel1_image,
                    border: Edge::all(25.0),
                }}
                styles={KStyle {
                    width: Units::Pixels(350.0).into(),
                    height: Units::Pixels(512.0).into(),
                    left: Units::Stretch(1.0).into(),
                    right: Units::Stretch(1.0).into(),
                    top: Units::Stretch(1.0).into(),
                    bottom: Units::Stretch(1.0).into(),
                    padding: Edge::new(
                        Units::Pixels(20.0),
                        Units::Pixels(20.0),
                        Units::Pixels(50.0),
                        Units::Pixels(20.0),
                    ).into(),
                    ..KStyle::default()
                }}
            >
                <KImageBundle
                    image={KImage(kayak_image)}
                    styles={KStyle {
                        width: Units::Pixels(310.0).into(),
                        height: Units::Pixels(104.0).into(),
                        top: Units::Pixels(25.0).into(),
                        bottom: Units::Pixels(25.0).into(),
                        ..KStyle::default()
                    }}
                />
                <KImageBundle
                    image={KImage(logo_image)}
                    styles={KStyle {
                        width: Units::Pixels(310.0).into(),
                        height: Units::Pixels(78.0).into(),
                        bottom: Units::Stretch(1.0).into(),
                        ..KStyle::default()
                    }}
                />
                <MenuButtonBundle
                    button={MenuButton { text: "Play".into() }}
                    on_event={handle_click_play}
                    />
                <MenuButtonBundle button={MenuButton { text: "Options".into() }} />
                <MenuButtonBundle
                    button={MenuButton { text: "Quit".into() }}
                    on_event={handle_click_close}
                />
            </NinePatchBundle>
        </KayakAppBundle>
    }

    commands.spawn(UICameraBundle::new(widget_context));
}
