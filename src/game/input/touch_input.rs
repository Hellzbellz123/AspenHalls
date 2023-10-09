use std::time::Duration;

use bevy::{prelude::*, utils::Instant};
use leafwing_input_manager::{
    action_state::{ActionData, Timing},
    axislike::DualAxisData,
    buttonlike::ButtonState,
    prelude::{ActionState, ActionStateDriver},
};
use virtual_joystick::*;

use crate::game::{actors::components::Player, AppStage};

use super::actions::Combat;

pub struct TouchInputPlugin;

impl Plugin for TouchInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(VirtualJoystickPlugin::<TouchJoyType>::default());
        app.add_systems(
            Update,
            (
                update_joystick.run_if(any_with_component::<VirtualJoystickKnob>()),
                spawn_touch_joysticks
                    .run_if(state_exists_and_equals(AppStage::PlayingGame).and_then(run_once())),
            ),
        );
    }
}

// TODO make these all parts of same ui node
// TODO add more buttons
// menu buttton top left corner. options menu.
// action button, pick up nearest/open closest
// swap weapon button
// fire weapon button

#[derive(Default, Reflect, Hash, Clone, PartialEq, Eq)]
pub enum TouchJoyType {
    #[default]
    MoveTouchInput,
    LookTouchInput,
}

fn spawn_touch_joysticks(
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
    player: Query<Entity, With<Player>>,
) {
    warn!("spawning joysticks");
    // Spawn Virtual Joystick at horizontal center
    cmd.spawn((
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
    // .insert(BackgroundColor(Color::ORANGE_RED.with_a(0.3)))
    .insert(VirtualJoystickInteractionArea)
    .insert(ActionStateDriver {
        action: Combat::Move,
        targets: player.single().into(),
    });

    // // Spawn Virtual Joystick at horizontal center
    // cmd.spawn((
    //     Name::new("CameraStickUI"),
    //     VirtualJoystickBundle::new(VirtualJoystickNode {
    //         border_image: asset_server.load("interface/outline_arrows.png"),
    //         knob_image: asset_server.load("interface/knob_noarrows.png"),
    //         knob_size: Vec2::new(80., 80.),
    //         dead_zone: 0.,
    //         id: TouchJoyType::LookTouchInput,
    //         axis: VirtualJoystickAxis::Both,
    //         behaviour: VirtualJoystickType::Fixed,
    //     })
    //     .set_color(TintColor(Color::WHITE.with_a(0.2)))
    //     .set_style(Style {
    //         width: Val::Px(150.),
    //         height: Val::Px(150.),
    //         position_type: PositionType::Absolute,
    //         left: Val::Percent(5.),
    //         bottom: Val::Percent(7.),
    //         // display: Display::None,
    //         ..default()
    //     }),
    // ))
    // // .insert(BackgroundColor(Color::ORANGE_RED.with_a(0.3)))
    // .insert(VirtualJoystickInteractionArea);
}

fn update_joystick(
    mut joystick: EventReader<VirtualJoystickEvent<TouchJoyType>>,
    mut joystick_color: Query<(&mut TintColor, &VirtualJoystickNode<TouchJoyType>)>,
    mut player_input: Query<&mut ActionState<Combat>, With<Player>>,
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

        match j.id() {
            TouchJoyType::LookTouchInput => {
                // we can maybe trick this by manually updating eagermouse component?
                // convert the -1..1 joystick coords too screen coords then use that too make world space coords
                // info!("player look {}", Vec2::from_array([x, y]))
            }
            TouchJoyType::MoveTouchInput => {
                let mut player_input = player_input.single_mut();
                // if axis pai value is larger than say 0.7, also set actiondata for sprint too pressed

                if x.abs() >= 0.7 || y.abs() >= 0.7 {
                    player_input.set_action_data(
                        Combat::Sprint,
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
                    )
                }

                player_input.set_action_data(
                    Combat::Move,
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
