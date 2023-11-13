use std::time::Duration;

use bevy::{prelude::*, utils::Instant, window::PrimaryWindow};
use leafwing_input_manager::{
    action_state::{ActionData, Timing},
    axislike::DualAxisData,
    buttonlike::ButtonState,
    prelude::{ActionState, ActionStateDriver},
};
use virtual_joystick::{
    TintColor, VirtualJoystickAxis, VirtualJoystickBundle, VirtualJoystickEvent,
    VirtualJoystickEventType, VirtualJoystickInteractionArea, VirtualJoystickNode,
    VirtualJoystickPlugin, VirtualJoystickType,
};

use super::{
    action_maps::{self, Gameplay},
    AspenInputSystemSet,
};
use crate::{
    game::{actors::components::Player, AppStage},
    loading::{
        assets::{InitAssetHandles, TouchControlAssetHandles},
        splashscreen::MainCamera,
    },
};

// TODO: handle controllers on mobile properly
/// adds touch input functionality too the app
/// also spawns joysticks and buttons for touching
pub struct TouchInputPlugin;

impl Plugin for TouchInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(VirtualJoystickPlugin::<TouchJoyType>::default());
        app.add_systems(OnEnter(AppStage::BootingApp), spawn_touch_controls_root);
        app.add_systems(
            OnEnter(AppStage::PlayingGame),
            (spawn_touch_joysticks, spawn_button_controls),
        );
        app.add_systems(
            PreUpdate,
            (update_joysticks, interaction_button_system)
                .in_set(AspenInputSystemSet::TouchInput)
                .run_if(
                    leafwing_input_manager::systems::run_if_enabled::<action_maps::Gameplay>
                        .and_then(any_with_component::<ActionStateDriver<action_maps::Gameplay>>()),
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

// TODO make these all parts of same ui node
// TODO add more buttons
// menu buttton top left corner. options menu.
// action button, pick up nearest/open closest
// swap weapon button
// fire weapon button
/// root ui entity that holds all touch input buttons
fn spawn_touch_controls_root(mut cmds: Commands) {
    cmds.spawn((
        Name::new("TouchControlsRoot"),
        TouchControlsRoot,
        NodeBundle {
            z_index: ZIndex::Global(10),
            style: Style {
                position_type: PositionType::Absolute,
                display: Display::Flex,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ..default()
        },
    ));
}

/// type of joystick, cursor input or move input
#[derive(Default, Reflect, Hash, Clone, PartialEq, Eq)]
pub enum TouchJoyType {
    #[default]
    /// joystick controls movement input
    MoveTouchInput,
    /// joystick controls look/cursor input
    LookTouchInput,
}

/// spawns buttons related too `GamePlay::*`
fn spawn_button_controls(
    mut cmds: Commands,
    touch_root_query: Query<Entity, With<TouchControlsRoot>>,
    init_handles: Res<InitAssetHandles>,
) {
    cmds.entity(touch_root_query.single())
        .with_children(|touch_controls| {
            touch_controls
                .spawn((
                    Name::new("InteractionButton"),
                    InteractionButtonTag,
                    ButtonBundle {
                        button: Button,
                        style: Style {
                            position_type: PositionType::Relative,
                            left: Val::Percent(45.0),
                            top: Val::Percent(15.0),
                            width: Val::Percent(8.0),
                            height: Val::Percent(8.0),
                            margin: UiRect::all(Val::Auto),
                            padding: UiRect::all(Val::Auto),
                            border: UiRect::all(Val::Auto),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
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
                            "Interact Button",
                            TextStyle {
                                font: init_handles.font_regular.clone(),
                                font_size: 25.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        )
                        .with_style(Style {
                            display: Display::Flex,
                            position_type: PositionType::Relative,
                            ..default()
                        }),
                    );
                });
        });
}

// TODO: convert too init pack
/// system too spawn joysticks
fn spawn_touch_joysticks(
    mut cmds: Commands,
    touch_assets: Res<TouchControlAssetHandles>,
    asset_server: Res<AssetServer>,
) {
    warn!("spawning joysticks");

    cmds.spawn((
        Name::new("MovementJoyStickUI"),
        VirtualJoystickBundle::new(VirtualJoystickNode {
            border_image: touch_assets.outline_no_arrows.clone(),
            knob_image: touch_assets.knob_arrows.clone(),
            knob_size: Vec2::new(80., 80.),
            dead_zone: 0.01,
            id: TouchJoyType::MoveTouchInput,
            axis: VirtualJoystickAxis::Both,
            behaviour: VirtualJoystickType::Fixed,
        })
        .set_color(TintColor(Color::WHITE.with_a(0.2)))
        .set_style(Style {
            width: Val::Px(150.),
            height: Val::Px(150.),
            position_type: PositionType::Absolute,
            right: Val::Percent(5.),
            bottom: Val::Percent(7.),
            // display: Display::None,
            ..default()
        }),
    ))
    .insert(VirtualJoystickInteractionArea);

    cmds.spawn((
        Name::new("CameraStickUI"),
        VirtualJoystickBundle::new(VirtualJoystickNode {
            border_image: touch_assets.outline_arrows.clone(),
            knob_image: touch_assets.knob_no_arrows.clone(),
            knob_size: Vec2::new(80., 80.),
            dead_zone: 0.,
            id: TouchJoyType::LookTouchInput,
            axis: VirtualJoystickAxis::Both,
            behaviour: VirtualJoystickType::Fixed,
        })
        .set_color(TintColor(Color::WHITE.with_a(0.2)))
        .set_style(Style {
            width: Val::Px(150.),
            height: Val::Px(150.),
            position_type: PositionType::Absolute,
            left: Val::Percent(5.),
            bottom: Val::Percent(7.),
            // display: Display::None,
            ..default()
        }),
    ))
    .insert(VirtualJoystickInteractionArea);
}

/// updates `actions::GamePlay::Move|Look` using touch joystick events
fn update_joysticks(
    mut joystick_events: EventReader<VirtualJoystickEvent<TouchJoyType>>,
    mut joystick_color_query: Query<(&mut TintColor, &VirtualJoystickNode<TouchJoyType>)>,
    mut player_input_query: Query<&mut ActionState<Gameplay>, With<Player>>,
    window_query: Query<&Window, (With<PrimaryWindow>,)>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    for joystick_event in &mut joystick_events {
        let Vec2 { x: joy_x, y: joy_y } = joystick_event.axis();

        handle_joystick_color(joystick_event, &mut joystick_color_query);

        let mut player_input = player_input_query.single_mut();
        match joystick_event.id() {
            TouchJoyType::LookTouchInput => {
                update_mouse_input(
                    &window_query,
                    &camera_query,
                    joystick_event,
                    joy_x,
                    joy_y,
                    &mut player_input,
                );
            }

            TouchJoyType::MoveTouchInput => {
                update_move_input(joy_x, joy_y, player_input);
            }
        }
    }
}

/// changes joystick color based on event type
fn handle_joystick_color(joystick_event: &VirtualJoystickEvent<TouchJoyType>, joystick_color_query: &mut Query<'_, '_, (&mut TintColor, &VirtualJoystickNode<TouchJoyType>)>) {
    match joystick_event.get_type() {
        VirtualJoystickEventType::Press | VirtualJoystickEventType::Drag => {
            for (mut color, node) in joystick_color_query {
                if node.id == joystick_event.id() {
                    *color = TintColor(Color::WHITE);
                }
            }
        }
        VirtualJoystickEventType::Up => {
            for (mut color, node) in joystick_color_query {
                if node.id == joystick_event.id() {
                    *color = TintColor(Color::WHITE.with_a(0.2));
                }
            }
        }
    }
}

/// updates player move input with given x/y value
///
/// used by joystick handling code
fn update_move_input(x: f32, y: f32, mut player_input: Mut<'_, ActionState<Gameplay>>) {
    if x.abs() >= 0.7 || y.abs() >= 0.7 {
        player_input.set_action_data(
            Gameplay::Sprint,
            ActionData {
                axis_pair: None,
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

    player_input.set_action_data(
        Gameplay::Move,
        ActionData {
            axis_pair: Some(DualAxisData::from_xy(
                Vec2 { x, y }.clamp(Vec2::splat(-1.0), Vec2::splat(1.0)),
            )),
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

/// updates mouse look for player action state
fn update_mouse_input(
    window_query: &Query<'_, '_, &Window, (With<PrimaryWindow>,)>,
    camera_query: &Query<'_, '_, (&Camera, &GlobalTransform), With<MainCamera>>,
    joystick_event: &VirtualJoystickEvent<TouchJoyType>,
    x: f32,
    y: f32,
    player_input: &mut Mut<'_, ActionState<Gameplay>>,
) {
    let window = window_query.single();
    let (camera, camera_transform) = camera_query.single();
    let dead_zone = 0.006;
    // Adjust this threshold as needed

    // Apply dead zone to joystick input
    let normalized_input = if joystick_event.axis().length() < dead_zone {
        Vec2::ZERO // Treat values near (0,0) as zero
    } else {
        // Adjust input values to be centered around (0,0)
        let centered_x =
            dead_zone.mul_add(-joystick_event.axis().x.signum(), joystick_event.axis().x); //j.axis().x - dead_zone * j.axis().x.signum();
        let centered_y =
            dead_zone.mul_add(-joystick_event.axis().y.signum(), joystick_event.axis().y); // j.axis().y - dead_zone * j.axis().y.signum();

        // Normalize input values to [-1.0, 1.0]
        let normalized_x = centered_x / (1.0 - dead_zone);
        let normalized_y = centered_y / (1.0 - dead_zone);

        Vec2::new(normalized_x, normalized_y)
    };

    let fake_cursor_position = Vec2::new(
        (normalized_input.x * window.width() / 2.0) + (window.width() / 2.0),
        (-normalized_input.y * window.height() / 2.0) + (window.height() / 2.0),
    );

    let fake_world_position = camera
        .viewport_to_world_2d(camera_transform, fake_cursor_position)
        .unwrap_or_else(|| {
            warn!("no cursor");
            Vec2::ZERO
        });

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

/// color for button with no interactions
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
/// color for hovered button
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
/// color for pressed buttons
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

/// links UI interact button too `Gameplay::Interact` action
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
    for (interaction, mut color, mut border_color, _children) in &mut interaction_query {
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
