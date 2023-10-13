use std::time::Duration;

use bevy::{input::InputSystem, prelude::*, utils::Instant, window::PrimaryWindow};
use leafwing_input_manager::{
    action_state::{ActionData, Timing},
    axislike::DualAxisData,
    buttonlike::ButtonState,
    plugin::InputManagerSystem,
    prelude::{ActionState, ActionStateDriver},
};
use virtual_joystick::*;

use super::{
    actions::{self, Gameplay},
    InternalInputSet,
};
use crate::{
    game::{actors::components::Player, AppStage},
    loading::{assets::FontHandles, splashscreen::MainCameraTag},
};

// TODO: make this plugin only active by default if target_platform == (ANDROID || IOS) else make it a setting too enable
pub struct TouchInputPlugin;

impl Plugin for TouchInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(VirtualJoystickPlugin::<TouchJoyType>::default());

        app.add_systems(OnEnter(AppStage::PlayingGame), spawn_touch_controls_root);
        app.add_systems(
            PreUpdate,
            (update_joysticks, interaction_button_system)
                .in_set(InternalInputSet::TouchInput)
                .run_if(
                    leafwing_input_manager::systems::run_if_enabled::<actions::Gameplay>
                        .and_then(any_with_component::<ActionStateDriver<actions::Gameplay>>()),
                ),
        );
        app.add_systems(
            Update,
            (spawn_touch_joysticks, spawn_button_controls)
                .run_if(any_with_component::<TouchControlsRoot>().and_then(run_once())),
        );
    }
}

#[derive(Component)]
pub struct TouchControlsRoot;

#[derive(Component)]
pub struct InteractionButtonTag;

// TODO make these all parts of same ui node
// TODO add more buttons
// menu buttton top left corner. options menu.
// action button, pick up nearest/open closest
// swap weapon button
// fire weapon button

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

#[derive(Default, Reflect, Hash, Clone, PartialEq, Eq)]
pub enum TouchJoyType {
    #[default]
    MoveTouchInput,
    LookTouchInput,
}

fn spawn_button_controls(
    mut cmds: Commands,
    touch_root_query: Query<Entity, With<TouchControlsRoot>>,
    ui_handles: Res<FontHandles>,
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
                                font: ui_handles.main_font.clone(),
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

fn spawn_touch_joysticks(mut cmds: Commands, asset_server: Res<AssetServer>) {
    warn!("spawning joysticks");

    cmds.spawn((
        Name::new("MovementJoyStickUI"),
        VirtualJoystickBundle::new(VirtualJoystickNode {
            border_image: asset_server.load("interface/outline_noarrows.png"),
            knob_image: asset_server.load("interface/knob_arrows.png"),
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
            border_image: asset_server.load("interface/outline_arrows.png"),
            knob_image: asset_server.load("interface/knob_noarrows.png"),
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

fn update_joysticks(
    mut joystick: EventReader<VirtualJoystickEvent<TouchJoyType>>,
    mut joystick_color: Query<(&mut TintColor, &VirtualJoystickNode<TouchJoyType>)>,
    mut player_input: Query<&mut ActionState<Gameplay>, With<Player>>,
    window: Query<&Window, (With<PrimaryWindow>,)>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCameraTag>>,
) {
    for j in joystick.iter() {
        let Vec2 { x, y } = j.axis();

        match j.get_type() {
            VirtualJoystickEventType::Press | VirtualJoystickEventType::Drag => {
                for (mut color, node) in joystick_color.iter_mut() {
                    if node.id == j.id() {
                        *color = TintColor(Color::WHITE);
                    }
                }
            }
            VirtualJoystickEventType::Up => {
                for (mut color, node) in joystick_color.iter_mut() {
                    if node.id == j.id() {
                        *color = TintColor(Color::WHITE.with_a(0.2));
                    }
                }
            }
        }

        let mut player_input = player_input.single_mut();
        match j.id() {
            TouchJoyType::LookTouchInput => {
                let window = window.single();
                let (camera, camera_transform) = camera.single();
                let dead_zone = 0.006; // Adjust this threshold as needed

                // Apply dead zone to joystick input
                let normalized_input = if j.axis().length() < dead_zone {
                    Vec2::ZERO // Treat values near (0,0) as zero
                } else {
                    // Adjust input values to be centered around (0,0)
                    let centered_x = j.axis().x - dead_zone * j.axis().x.signum();
                    let centered_y = j.axis().y - dead_zone * j.axis().y.signum();

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
                    player_input.press(actions::Gameplay::Shoot)
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

            TouchJoyType::MoveTouchInput => {
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
        }
    }
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

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

        match *interaction {
            Interaction::Pressed => {
                let mut input = player_input.single_mut();
                input.press(actions::Gameplay::Interact);
            }
            _ => {}
        }
    }
}
