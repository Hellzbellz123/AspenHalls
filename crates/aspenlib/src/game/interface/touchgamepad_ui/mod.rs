/// ui button interaction systems
pub mod shunts;

use bevy::prelude::*;
use bevy_touch_stick::{
    TouchStick, TouchStickGamepadMapping, TouchStickPlugin, TouchStickType,
    TouchStickUiKnob, TouchStickUiOutline,
};
use leafwing_input_manager::prelude::ActionState;
use std::hash::Hash;

use crate::{
    game::{
        input::{action_maps, AspenInputSystemSet},
        interface::InterfaceRootTag,
    },
    loading::assets::AspenTouchHandles,
    AppStage, GameStage, GeneralSettings,
};

// TODO: handle controllers on mobile properly
/// adds touch input functionality too the app
/// also spawns joysticks and buttons for touching
pub struct TouchInputUIPlugin;

impl Plugin for TouchInputUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TouchStickPlugin::<TouchStickBinding>::default());
        // TODO: handle menus properly. despawn touch controls when exiting PlayingGame
        app.add_systems(OnEnter(AppStage::Starting), spawn_touch_gamepad);
        app.add_systems(
            Update,
            handle_touch_controls_visibility.run_if(any_with_component::<TouchControlsRoot>),
        );
        app.add_systems(
            Update,
            (
                update_button_colors,
                shunts::touch_pause_game,
                (
                    shunts::touch_trigger_sprint,
                    shunts::touch_trigger_attack,
                    shunts::touch_cycle_weapon,
                    shunts::touch_interaction_button,
                    shunts::touch_heal,
                    shunts::touch_zoom_out,
                    shunts::touch_zoom_in,
                )
                    .run_if(in_state(AppStage::Running)),
            )
                .in_set(AspenInputSystemSet::TouchInput)
                .run_if(
                    resource_exists::<ActionState<action_maps::Gameplay>>
                        .and(|res: Res<GeneralSettings>| res.enable_touch_controls),
                ),
        );
    }
}

/// tag for touch controls root
#[derive(Component)]
pub struct TouchControlsRoot;

/// tag too query interact button
#[derive(Component)]
pub struct InteractionTag;

/// tag too query weapon swap/hide button
#[derive(Component)]
pub struct SwapWeaponTag;

/// tag too query game pause button
#[derive(Component)]
pub struct PauseTag;

/// tag too query heal button
#[derive(Component)]
pub struct HealTag;

/// tag too query action button 1
#[derive(Component)]
pub struct Action1Tag;

/// tag too query action button 2
#[derive(Component)]
pub struct Action2Tag;

/// tag too query action button 3
#[derive(Component)]
pub struct Action3Tag;

/// tag too query action button 4
#[derive(Component)]
pub struct Action4Tag;

/// tag too query action button 5
#[derive(Component)]
pub struct Action5Tag;

/// tag too query zoom out button
#[derive(Component)]
pub struct ZoomOutTag;

/// tag too query zoom in button
#[derive(Component)]
pub struct ZoomInTag;

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

/// hides touch controls if menus with buttons should be shown
fn handle_touch_controls_visibility(
    game_state: Option<Res<State<GameStage>>>,
    cfg: Res<GeneralSettings>,
    mut touch_root_query: Query<&mut Node, With<TouchControlsRoot>>,
) {
    let Ok(mut touch_root_style) = touch_root_query.single_mut() else {
        info!("no touch controls");
        return;
    };

    if let Some(gamestate) = game_state {
        if cfg.enable_touch_controls {
            match gamestate.get() {
                GameStage::PlayingGame => {
                    touch_root_style.display = Display::Flex;
                }
                _ => {
                    touch_root_style.display = Display::None;
                }
            }
        } else {
            touch_root_style.display = Display::None;
        }
    } else {
        touch_root_style.display = Display::None;
    }
}

/// spawn controls for touch screen players
fn spawn_touch_gamepad(
    mut cmds: Commands,
    interface_root: Query<Entity, With<InterfaceRootTag>>,
    // init_handles: Res<AspenInitHandles>,
    touch_assets: Res<AspenTouchHandles>,
) {
    let Ok(interface_root) = interface_root.single() else {
        return;
    };

    cmds.entity(interface_root)
        .with_children(|ui_root_children| {
            ui_root_children
                .spawn((
                    TouchControlsRoot,
                    Name::new("TouchControls"),
                    Node {
                        margin: UiRect::all(Val::Auto),
                        border: UiRect::all(Val::Px(2.0)),
                        display: Display::None,
                        position_type: PositionType::Absolute,
                        // flex_direction: FlexDirection::Column,
                        // align_items: AlignItems::Center,
                        // align_self: AlignSelf::Center,
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                ))
                .with_children(|touch_controls_root_children| {
                    touch_controls_root_children
                        .spawn((
                            Name::new("LeftControlsPod"),
                            Node {
                                position_type: PositionType::Absolute,
                                width: Val::Percent(20.0),
                                height: Val::Percent(40.0),
                                margin: UiRect {
                                    left: Val::Px(25.0),
                                    right: Val::Auto,
                                    top: Val::Auto,
                                    bottom: Val::Px(25.0),
                                },
                                ..default()
                            },
                        ))
                        .with_children(|left_pod_parts| {
                            spawn_touchstick(
                                left_pod_parts,
                                (&touch_assets.move_knob, &touch_assets.move_outline),
                                "MoveTouchStick".to_string(),
                                UiRect {
                                    left: Val::Percent(5.0),
                                    bottom: Val::Percent(5.0),
                                    right: Val::Auto,
                                    top: Val::Auto,
                                },
                                (Val::Px(145.0), Val::Px(145.0)),
                                (
                                    TouchStickBinding::MoveTouchInput,
                                    TouchStickGamepadMapping::LEFT_STICK,
                                ),
                            );
                        });

                    touch_controls_root_children
                        .spawn((
                            Name::new("RightControlsPod"),
                            Node {
                                position_type: PositionType::Absolute,
                                flex_direction: FlexDirection::Column,
                                width: Val::Px(230.0),
                                height: Val::Percent(95.0),
                                margin: UiRect {
                                    left: Val::Auto,
                                    right: Val::Px(25.0),
                                    top: Val::Auto,
                                    bottom: Val::Px(25.0),
                                },
                                ..default()
                            },
                        ))
                        .with_children(|right_pod_parts| {
                            create_button_rows(right_pod_parts, &touch_assets);
                            spawn_touchstick(
                                right_pod_parts,
                                (&touch_assets.look_knob, &touch_assets.look_outline),
                                "LookTouchStick".to_string(),
                                UiRect {
                                    right: Val::Percent(5.0),
                                    bottom: Val::Percent(5.0),
                                    left: Val::Auto,
                                    top: Val::Auto,
                                },
                                (Val::Px(150.0), Val::Px(150.0)),
                                (
                                    TouchStickBinding::LookTouchInput,
                                    TouchStickGamepadMapping::RIGHT_STICK,
                                ),
                            );
                        });
                });
        });
}

/// fills right pod with button rows
fn create_button_rows(right_pod_parts: &mut ChildSpawnerCommands, touch_assets: &Res<AspenTouchHandles>) {
    right_pod_parts
        .spawn((
            Name::new("RightPodButtons"),
            Node {
                flex_direction: FlexDirection::Column,
                position_type: PositionType::Relative,
                width: Val::Percent(100.0),
                height: Val::Auto,
                flex_grow: 0.95,
                ..default()
            },
        ))
        .with_children(|button_rows| {
            button_rows
                .spawn((
                    Name::new("TopButtonContainer"),
                    Node {
                        width: Val::Px(250.0),
                        height: Val::Px(100.0),
                        margin: UiRect {
                            left: Val::Auto,
                            right: Val::Px(0.0),
                            top: Val::Px(0.0),
                            bottom: Val::Auto,
                        },
                        ..default()
                    },
                ))
                .with_children(|top_buttons| {
                    spawn_top_buttons(top_buttons, touch_assets);
                });

            button_rows
                .spawn((
                    Name::new("MiddleButtonContainer"),
                    Node {
                        width: Val::Px(250.0),
                        height: Val::Px(100.0),
                        margin: UiRect {
                            left: Val::Auto,
                            right: Val::Px(0.0),
                            top: Val::Auto,
                            bottom: Val::Auto,
                        },
                        ..default()
                    },
                ))
                .with_children(|middle_buttons| {
                    spawn_middle_buttons(middle_buttons, touch_assets);
                });
        });
}

/// spawn game utility buttons
fn spawn_top_buttons(top_buttons: &mut ChildSpawnerCommands, touch_assets: &Res<AspenTouchHandles>) {
    spawn_controlsbutton(
        top_buttons,
        Some(touch_assets.menu_button.clone()),
        "Pause Game".to_string(),
        UiRect {
            right: Val::Percent(5.0),
            top: (Val::Percent(5.0)),
            bottom: Val::Auto,
            left: Val::Auto,
        },
        (Val::Px(75.0), Val::Px(75.0)),
        PauseTag,
    );
    spawn_controlsbutton(
        top_buttons,
        Some(touch_assets.zoom_minus_button.clone()),
        "Zoom Out".to_string(),
        UiRect {
            right: Val::Percent(45.0),
            bottom: Val::Auto,
            left: Val::Auto,
            top: Val::Percent(5.0),
        },
        (Val::Px(50.0), Val::Px(50.0)),
        ZoomOutTag,
    );
    spawn_controlsbutton(
        top_buttons,
        Some(touch_assets.zoom_plus_button.clone()),
        "Zoom In".to_string(),
        UiRect {
            right: Val::Percent(70.0),
            top: Val::Percent(5.0),
            left: Val::Auto,
            bottom: Val::Auto,
        },
        (Val::Px(50.0), Val::Px(50.0)),
        ZoomInTag,
    );
}

/// spawn player functionality buttons
fn spawn_middle_buttons(middle_buttons: &mut ChildSpawnerCommands, touch_assets: &Res<AspenTouchHandles>) {
    // lower buttons
    spawn_controlsbutton(
        middle_buttons,
        None,
        "Interact".to_string(),
        UiRect {
            right: Val::Percent(2.0),
            bottom: (Val::Percent(5.0)),
            left: Val::Auto,
            top: Val::Auto,
        },
        (Val::Px(100.0), Val::Px(60.0)),
        InteractionTag,
    );
    spawn_controlsbutton(
        middle_buttons,
        Some(touch_assets.swap_button.clone()),
        "Cycle Weapon".to_string(),
        UiRect {
            right: Val::Percent(45.0),
            bottom: (Val::Percent(5.0)),
            left: Val::Auto,
            top: Val::Auto,
        },
        (Val::Px(50.0), Val::Px(50.0)),
        SwapWeaponTag,
    );
    spawn_controlsbutton(
        middle_buttons,
        Some(touch_assets.heal_button.clone()),
        "Heal".to_string(),
        UiRect {
            right: Val::Percent(70.0),
            bottom: (Val::Percent(5.0)),
            left: Val::Auto,
            top: Val::Auto,
        },
        (Val::Px(50.0), Val::Px(50.0)),
        HealTag,
    );
}

/// spawns button with <S> marker component
/// takes button size, button name, button position and button id (just a component for querying)
fn spawn_controlsbutton<S: Component>(
    touch_controls_builder: &mut ChildSpawnerCommands,
    image: Option<Handle<Image>>,
    name: String,
    position: UiRect,
    size: (Val, Val),
    id: S,
) {
    let debug_name = name.trim().to_string();

    // let image = image.map_or_else(ImageNode::default, ImageNode::new);

    touch_controls_builder
        .spawn((
            Name::new(debug_name),
            id,
            Button,
            Node {
                display: Display::Flex,
                position_type: PositionType::Absolute,
                margin: position,
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
            ZIndex(1),
        ))
        .with_children(|text| {
            text.spawn((
                Text::new(name),
                TextColor(Color::WHITE),
                TextFont::from_font_size(14.0),
                Node {
                    display: Display::Flex,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ZIndex(2),
            ));
        });
}

/// spawns configured touchstick
/// takes name, position, size, mapping, and id
/// touch stick is fixed position
#[allow(clippy::too_many_arguments)]
fn spawn_touchstick<
    S: Hash
        + Sync
        + Send
        + Clone
        + Default
        + Reflect
        + FromReflect
        + TypePath
        + bevy::reflect::GetTypeRegistration
        + 'static,
>(
    touch_controls_builder: &mut ChildSpawnerCommands,
    images: (&Handle<Image>, &Handle<Image>),
    name: String,
    position: UiRect,
    size: (Val, Val),
    mapping: (S, TouchStickGamepadMapping),
) {
    touch_controls_builder
        .spawn((
            Name::new(name),
            mapping.1,
            // BackgroundColor(crate::colors::ANTIQUE_WHITE.with_alpha(0.25).into()),
            Node {
                width: size.0,
                height: size.1,
                border: UiRect::all(Val::Px(2.0)),
                position_type: PositionType::Relative,
                margin: position,
                padding: UiRect::all(Val::Auto),
                ..default()
            },
            TouchStick {
                id: mapping.0,
                stick_type: TouchStickType::Fixed,
                dead_zone: 0.001,
                // base_position: (),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("TouchStickKnob"),
                TouchStickUiKnob,
                ImageNode {
                    image: images.0.clone(),
                    color: crate::colors::ORANGE.with_alpha(0.3).into(),
                    ..default()
                },
                Node {
                    // (Val::Px(100.0), Val::Px(100.0)),
                    width: size.0 / 2.0,
                    height: size.1 / 2.0,
                    position_type: PositionType::Absolute,
                    margin: UiRect::all(Val::Auto),
                    padding: UiRect::all(Val::Auto),
                    ..default()
                },
            ));
            parent.spawn((
                Name::new("TouchStickOutline"),
                TouchStickUiOutline,
                ImageNode {
                    image: images.1.clone(),
                    color: crate::colors::ORANGE.with_alpha(0.3).into(),
                    ..default()
                },
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Px(150.),
                    height: Val::Px(150.),
                    margin: UiRect::all(Val::Auto),
                    padding: UiRect::all(Val::Auto),
                    ..default()
                },
            ));
        });
}

/// color for button with no interactions
const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
/// color for hovered button
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
/// color for pressed buttons
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

/// updates interacted button colors
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
                *color = PRESSED_BUTTON.into();
                border_color.0 = crate::colors::RED.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = crate::colors::WHITE.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = crate::colors::BLACK.into();
            }
        }
    }
}
