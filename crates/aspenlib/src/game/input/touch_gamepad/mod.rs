pub mod shunts;

use bevy::prelude::*;
use bevy_touch_stick::{
    TouchStick, TouchStickGamepadMapping, TouchStickPlugin, TouchStickType, TouchStickUiBundle,
    TouchStickUiKnob, TouchStickUiOutline,
};
use leafwing_input_manager::prelude::ActionState;
use std::hash::Hash;

use crate::{
    game::{
        input::{action_maps, AspenInputSystemSet},
        interface::InterfaceRoot,
        AppState,
    },
    loading::assets::{AspenInitHandles, AspenTouchHandles},
};

// TODO: handle controllers on mobile properly
/// adds touch input functionality too the app
/// also spawns joysticks and buttons for touching
pub struct TouchInputPlugin;

impl Plugin for TouchInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TouchStickPlugin::<TouchStickBinding>::default());
        // TODO: handle menus properly. despawn touch controls when exiting PlayingGame
        app.add_systems(OnEnter(AppState::PlayingGame), spawn_touch_gamepad);
        app.add_systems(
            PreUpdate,
            (
                update_button_colors,
                (
                    shunts::touch_trigger_sprint,
                    shunts::touch_trigger_shoot,
                    shunts::touch_interaction_button,
                )
                    .run_if(state_exists_and_equals(AppState::PlayingGame)),
            )
                .in_set(AspenInputSystemSet::TouchInput)
                .run_if(resource_exists::<ActionState<action_maps::Gameplay>>()),
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
fn spawn_touch_gamepad(
    mut cmds: Commands,
    touch_root_query: Query<Entity, With<InterfaceRoot>>,
    init_handles: Res<AspenInitHandles>,
    touch_assets: Res<AspenTouchHandles>,
) {
    cmds.entity(touch_root_query.single())
        .with_children(|ui_root_children| {
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
                        &touch_assets.move_knob,
                        &touch_assets.move_outline,
                        "MoveTouchStick".to_string(),
                        UiRect {
                            left: Val::Percent(10.0),
                            bottom: (Val::Percent(5.0)),
                            right: Val::Auto,
                            top: Val::Auto,
                        },
                        (Val::Px(100.0), Val::Px(100.0)),
                        TouchStickGamepadMapping::LEFT_STICK,
                        TouchStickBinding::MoveTouchInput,
                    );

                    spawn_touchstick(
                        touch_controls_root_children,
                        &touch_assets.look_knob,
                        &touch_assets.look_outline,
                        "LookTouchStick".to_string(),
                        UiRect {
                            right: Val::Percent(10.0),
                            bottom: (Val::Percent(5.0)),
                            left: Val::Auto,
                            top: Val::Auto,
                        },
                        (Val::Px(100.0), Val::Px(100.0)),
                        TouchStickGamepadMapping::RIGHT_STICK,
                        TouchStickBinding::LookTouchInput,
                    );
                });
        });
}

/// spawns button with <S> marker component
/// takes button size, button name, button position and button id (just a component for querying)
fn spawn_controlsbutton<S: Component>(
    touch_controls_builder: &mut ChildBuilder<'_, '_, '_>,
    init_handles: Res<'_, AspenInitHandles>,
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
    S: Hash + Sync + Send + Clone + Default + Reflect + FromReflect + TypePath + 'static,
>(
    touch_controls_builder: &mut ChildBuilder<'_, '_, '_>,
    knob: &Handle<Image>,
    outline: &Handle<Image>,
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
                    image: knob.clone().into(),
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
                    image: outline.clone().into(),
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
fn update_button_colors(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, mut border_color, _) in &mut interaction_query {
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
    }
}
