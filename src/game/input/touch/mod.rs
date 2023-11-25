use bevy::{prelude::*, utils::Instant, window::PrimaryWindow};
use bevy_touch_stick::{
    TouchStick, TouchStickGamepadMapping, TouchStickPlugin,
    TouchStickType, TouchStickUiBundle, TouchStickUiKnob,
    TouchStickUiOutline,
};
use leafwing_input_manager::{
    action_state::{ActionData, Timing},
    axislike::DualAxisData,
    buttonlike::ButtonState,
    prelude::{ActionState, ActionStateDriver},
};
use std::{hash::Hash, time::Duration};

use super::{
    action_maps::{self, Gameplay},
    AspenInputSystemSet,
};
use crate::{
    ahp::game::MainCamera,
    game::{
        actors::components::Player, interface::InterfaceRoot, AppState,
    },
    loading::assets::{InitAssetHandles, TouchControlAssetHandles},
};

// TODO: handle controllers on mobile properly
/// adds touch input functionality too the app
/// also spawns joysticks and buttons for touching
pub struct TouchInputPlugin;

impl Plugin for TouchInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TouchStickPlugin::<TouchStickBinding>::default());
        app.add_systems(
            OnEnter(AppState::PlayingGame),
            spawn_touch_controls,
        );
        app.add_systems(
            PreUpdate,
            (interaction_button_system, fake_mouse_input)
                .in_set(AspenInputSystemSet::TouchInput)
                .run_if(
                    leafwing_input_manager::systems::run_if_enabled::<
                        action_maps::Gameplay,
                    >
                        .and_then(
                            any_with_component::<
                                ActionStateDriver<action_maps::Gameplay>,
                            >(),
                        ),
                ),
        );
    }
}

/// tag for touch controls root
#[derive(Component)]
pub struct TouchControlsRoot;

/// tag too query interact button
#[derive(Component)]
pub struct InteractionButtonTag;

// TODO add more buttons
// menu buttton top left corner. options menu.
// action button, pick up nearest/open closest
// swap weapon button
// fire weapon button

// type of joystick, cursor input or move input
// #[derive(Default, Reflect, Hash, Clone, PartialEq, Eq)]
// pub struct TouchStickMove;

/// type of joystick, cursor input or move input
#[derive(Default, Reflect, Hash, Clone, PartialEq, Eq)]
pub enum TouchStickBinding {
    #[default]
    /// joystick controls movement input
    MoveTouchInput,
    /// joystick controls look/cursor input
    LookTouchInput,
}

/// spa
fn spawn_touch_controls(
    mut cmds: Commands,
    touch_root_query: Query<Entity, With<InterfaceRoot>>,
    init_handles: Res<InitAssetHandles>,
    touch_assets: Res<TouchControlAssetHandles>,
) {
    cmds.entity(touch_root_query.single()).with_children(
        |ui_root_children| {
            ui_root_children
                .spawn((
                    Name::new("TouchControls"),
                    NodeBundle {
                        style: Style {
                            margin: UiRect::all(Val::Auto),
                            border: UiRect::all(Val::Px(2.0)),
                            display: Display::Flex,
                            position_type: PositionType::Relative,
                            // flex_direction: FlexDirection::Column,
                            // align_items: AlignItems::Center,
                            // align_self: AlignSelf::Center,
                            width: Val::Vw(98.0),
                            height: Val::Vh(98.0),
                            ..default()
                        },
                        ..default()
                    },
                ))
                .with_children(|touch_controls_root_children| {
                    info!("Spawning Touch Button Controls");
                    spawn_controlsbutton(
                        touch_controls_root_children,
                        init_handles,
                        "Interaction Button".to_string(),
                        UiRect {
                            right: Val::Percent(4.0),
                            bottom: (Val::Percent(12.0)),
                            left: Val::Auto,
                            top: Val::Auto,
                        },
                        (Val::Px(100.0), Val::Px(50.0)),
                        InteractionButtonTag,
                    );

                    info!("Spawning Touch Axis Controls");
                    spawn_touchstick(
                        touch_controls_root_children,
                        &touch_assets,
                        "MoveTouchStick".to_string(),
                        UiRect {
                            right: Val::Percent(10.0),
                            bottom: (Val::Percent(5.0)),
                            left: Val::Auto,
                            top: Val::Auto,
                        },
                        (Val::Px(100.0), Val::Px(100.0)),
                        TouchStickGamepadMapping::LEFT_STICK,
                        TouchStickBinding::MoveTouchInput,
                    );

                    spawn_touchstick(
                        touch_controls_root_children,
                        &touch_assets,
                        "LookTouchStick".to_string(),
                        UiRect {
                            left: Val::Percent(10.0),
                            bottom: (Val::Percent(5.0)),
                            right: Val::Auto,
                            top: Val::Auto,
                        },
                        (Val::Px(100.0), Val::Px(100.0)),
                        TouchStickGamepadMapping::RIGHT_STICK,
                        TouchStickBinding::LookTouchInput,
                    );
                });
        },
    );
}

/// spawns button with <S> marker component
/// takes button size, button name, button position and button id (just a component for querying)
fn spawn_controlsbutton<S: Component>(
    touch_controls_builder: &mut ChildBuilder<'_, '_, '_>,
    init_handles: Res<'_, InitAssetHandles>,
    name: String,
    position: UiRect,
    size: (Val, Val),
    id: S,
) {
    let debug_name = name.trim().to_string();
    touch_controls_builder
        .spawn((
            Name::new(debug_name),
            id,
            ButtonBundle {
                button: Button,
                style: Style {
                    display: Display::Flex,
                    position_type: PositionType::Absolute,
                    margin: UiRect {
                        left: position.left,
                        right: position.right,
                        top: position.top,
                        bottom: position.bottom,
                    },
                    // padding: UiRect::all(Val::Auto),
                    border: UiRect::all(Val::Px(2.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    // left: position.left,
                    // right: position.right,
                    // top: position.top,
                    // bottom: position.right,
                    width: size.0,
                    height: size.1,
                    ..default()
                },
                background_color: BackgroundColor(Color::SEA_GREEN),
                border_color: BorderColor(Color::RED),
                z_index: ZIndex::Local(1),
                // TODO: change from button with text too fancier image
                // image: todo!(),
                ..default()
            },
        ))
        .with_children(|text| {
            text.spawn(
                TextBundle::from_section(
                    name,
                    TextStyle {
                        font: init_handles.font_regular.clone(),
                        font_size: 25.0,
                        color: Color::WHITE,
                    },
                )
                .with_style(Style {
                    display: Display::Flex,
                    position_type: PositionType::Absolute,
                    ..default()
                }),
            );
        });
}

/// spawns configured touchstick
/// takes name, position, size, mapping, and id
/// touch stick is fixed position
fn spawn_touchstick<
    S: Hash
        + Sync
        + Send
        + Clone
        + Default
        + Reflect
        + FromReflect
        + TypePath
        + 'static,
>(
    touch_controls_builder: &mut ChildBuilder<'_, '_, '_>,
    touch_assets: &Res<'_, TouchControlAssetHandles>,
    name: String,
    position: UiRect,
    size: (Val, Val),
    mapping: TouchStickGamepadMapping,
    id: S,
) {
    touch_controls_builder
        .spawn((
            Name::new(name),
            mapping,
            TouchStickUiBundle {
                stick: TouchStick {
                    id,
                    stick_type: TouchStickType::Fixed,
                    dead_zone: 0.001,
                    // base_position: (),
                    ..default()
                },
                // configure the interactable area through bevy_ui
                style: Style {
                    width: size.0,
                    height: size.1,
                    border: UiRect::all(Val::Px(2.0)),
                    position_type: PositionType::Absolute,
                    margin: UiRect {
                        left: position.left,
                        right: position.right,
                        top: position.top,
                        bottom: position.bottom,
                    },
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("TouchStickKnob"),
                TouchStickUiKnob,
                ImageBundle {
                    image: touch_assets.knob_no_arrows.clone().into(),
                    style: Style {
                        // (Val::Px(100.0), Val::Px(100.0)),
                        width: size.0 / 2.0,
                        height: size.1 / 2.0,
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    background_color: Color::ORANGE.with_a(0.3).into(),
                    ..default()
                },
            ));
            parent.spawn((
                Name::new("TouchStickOutline"),
                TouchStickUiOutline,
                ImageBundle {
                    image: touch_assets.outline_arrows.clone().into(),
                    style: Style {
                        position_type: PositionType::Absolute,
                        width: Val::Px(150.),
                        height: Val::Px(150.),
                        margin: UiRect::all(Val::Px(50.0)),
                        ..default()
                    },
                    background_color: Color::ORANGE.with_a(0.3).into(),
                    ..default()
                },
            ));
        });
}

/// color for button with no interactions
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
/// color for hovered button
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
/// color for pressed buttons
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

/// links UI interact button too `Gameplay::Interact` action
#[allow(clippy::type_complexity)]
fn interaction_button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (
            Changed<Interaction>,
            With<Button>,
            With<InteractionButtonTag>,
        ),
    >,
    mut player_input: Query<&mut ActionState<Gameplay>, With<Player>>,
) {
    for (interaction, mut color, mut border_color, _children) in
        &mut interaction_query
    {
        match *interaction {
            Interaction::Pressed => {
                info!("interaction button pressed, send thing");
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }

        if *interaction == Interaction::Pressed {
            let mut input = player_input.single_mut();
            input.press(action_maps::Gameplay::Interact);
        }
    }
}

/// creates fake mouse input using a joystick x/y value and the window x/y value
fn fake_mouse_input(
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut player_input: Query<&mut ActionState<Gameplay>, With<Player>>, //Mut<'_, ActionState<Gameplay>>,
) {
    let mut player_input = player_input.single_mut();
    let joy_axis = player_input
        .action_data(Gameplay::JoystickDelta)
        .axis_pair
        .unwrap()
        .xy();
    let window = window_query.single();
    if joy_axis == Vec2::ZERO {
        return;
    }

    let Vec2 { x, y } = joy_axis;

    let (camera, camera_transform) = camera_query.single();
    let dead_zone: f32 = 0.000_5;
    // Adjust this threshold as needed

    // Apply dead zone to joystick input
    let normalized_input = {
        // let normalized_x = x;
        // let normalized_y = y;

        // Adjust input values to be centered around (0,0)
        let centered_x =
            dead_zone.mul_add(-joy_axis.x.signum(), joy_axis.x); //j.axis().x - dead_zone * j.axis().x.signum();
        let centered_y =
            dead_zone.mul_add(-joy_axis.y.signum(), joy_axis.y); // j.axis().y - dead_zone * j.axis().y.signum();

        // Normalize input values to [-1.0, 1.0]
        let normalized_x = centered_x / (1.0 - dead_zone);
        let normalized_y = centered_y / (1.0 - dead_zone);
        Vec2::new(normalized_x, normalized_y)
    };

    let fake_cursor_position = Vec2::new(
        (normalized_input.x * window.width() / 2.0)
            + (window.width() / 2.0),
        (-normalized_input.y * window.height() / 2.0)
            + (window.height() / 2.0),
    );

    let fake_world_position = camera
        .viewport_to_world_2d(camera_transform, fake_cursor_position)
        .unwrap_or_else(|| {
            warn!("no cursor");
            Vec2::ZERO
        });

    trace!("fake_world_position: {:?}", fake_world_position);
    trace!("fake_local_position: {:?}", fake_cursor_position);

    // set the action data
    if x.abs() >= 0.3 || y.abs() >= 0.3 {
        player_input.press(action_maps::Gameplay::Shoot);
    }

    player_input.set_action_data(
        Gameplay::LookLocal,
        ActionData {
            axis_pair: Some(DualAxisData::from_xy(fake_cursor_position)),
            consumed: false,
            state: ButtonState::JustPressed,
            value: 1.0,
            timing: Timing {
                instant_started: Some(Instant::now()),
                current_duration: Duration::from_secs(1),
                previous_duration: Duration::from_secs(0),
            },
        },
    );

    player_input.set_action_data(
        Gameplay::LookWorld,
        ActionData {
            axis_pair: Some(DualAxisData::from_xy(fake_world_position)),
            consumed: false,
            state: ButtonState::JustPressed,
            value: 1.0,
            timing: Timing {
                instant_started: Some(Instant::now()),
                current_duration: Duration::from_secs(1),
                previous_duration: Duration::from_secs(0),
            },
        },
    );
}

// // TODO: convert too init pack
// /// system too spawn joysticks
// fn spawn_touch_joysticks(
//     mut cmds: Commands,
//     touch_assets: Res<TouchControlAssetHandles>,
// ) {
//     warn!("spawning joysticks");

//     let background_size = Vec2::splat(150.0);
//     let knob_size = Vec2::splat(80.0);
//     let style = Style {
//         width: Val::Px(150.),
//         height: Val::Px(150.),
//         position_type: PositionType::Absolute,
//         right: Val::Percent(5.),
//         bottom: Val::Percent(7.),
//         // display: Display::None,
//         ..default()
//     };

//     let mut spawn =
//         cmds.spawn((VirtualJoystickBundle::new(VirtualJoystickNode {
//             id: TouchJoyType::LookTouchInput,
//             dead_zone: 0.01,
//             axis: VirtualJoystickAxis::Both,
//             behaviour: VirtualJoystickType::Fixed,
//         })
//         .set_style(style),));
//     spawn
//         .insert(VirtualJoystickInteractionArea)
//         .with_children(|parent| {
//             parent.spawn((
//                 VirtualJoystickUIKnob,
//                 ImageBundle {
//                     image: touch_assets.knob_no_arrows.clone().into(),
//                     style: Style {
//                         width: Val::Px(knob_size.x),
//                         height: Val::Px(knob_size.y),
//                         ..default()
//                     },
//                     background_color: Color::WHITE.with_a(0.2).into(),
//                     ..default()
//                 },
//             ));
//             parent.spawn((
//                 VirtualJoystickUIBackground,
//                 ImageBundle {
//                     image: touch_assets.outline_arrows.clone().into(),
//                     style: Style {
//                         width: Val::Px(background_size.x),
//                         height: Val::Px(background_size.y),
//                         ..default()
//                     },
//                     background_color: Color::WHITE.with_a(0.2).into(),
//                     ..default()
//                 },
//             ));
//         });
// }

// /// updates `actions::GamePlay::Move|Look` using touch joystick events
// fn update_joysticks(
//     mut joystick_events: EventReader<VirtualJoystickEvent<TouchJoyType>>,
//     mut player_input_query: Query<
//         &mut ActionState<Gameplay>,
//         With<Player>,
//     >,
//     window_query: Query<&Window, (With<PrimaryWindow>,)>,
//     camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
// ) {
//     for joystick_event in joystick_events.read() {
//         let mut player_input = player_input_query.single_mut();
//         let Vec2 { x: joy_x, y: joy_y } = joystick_event.axis();

//         match joystick_event.id() {
//             TouchJoyType::LookTouchInput => {
//                 fake_mouse_input(
//                     &window_query,
//                     &camera_query,
//                     joy_x,
//                     joy_y,
//                     &mut player_input,
//                 );
//             }

//             TouchJoyType::MoveTouchInput => {
//                 fake_directional_input(joy_x, joy_y, player_input);
//             }
//         }
//     }
// }

// /// updates player move input with given x/y value
// /// used by joystick handling code
// fn fake_directional_input(
//     x: f32,
//     y: f32,
//     mut player_input: Mut<'_, ActionState<Gameplay>>,
// ) {
//     if x.abs() >= 0.7 || y.abs() >= 0.7 {
//         player_input.set_action_data(
//             Gameplay::Sprint,
//             ActionData {
//                 axis_pair: None,
//                 consumed: false,
//                 state: ButtonState::JustPressed,
//                 value: 1.0,
//                 timing: Timing {
//                     instant_started: Some(Instant::now()),
//                     current_duration: Duration::from_secs(1),
//                     previous_duration: Duration::from_secs(0),
//                 },
//             },
//         );
//     }

//     player_input.set_action_data(
//         Gameplay::Move,
//         ActionData {
//             axis_pair: Some(DualAxisData::from_xy(
//                 Vec2 { x, y }.clamp(Vec2::splat(-1.0), Vec2::splat(1.0)),
//             )),
//             consumed: false,
//             state: ButtonState::JustPressed,
//             value: 1.0,
//             timing: Timing {
//                 instant_started: Some(Instant::now()),
//                 current_duration: Duration::from_secs(1),
//                 previous_duration: Duration::from_secs(0),
//             },
//         },
//     );
// }
