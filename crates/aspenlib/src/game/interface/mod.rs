use crate::{game::AppState, loading::assets::InitAssetHandles};
use bevy::{app::AppExit, prelude::*};
use rand::Rng;

/// currently active menu
#[derive(Debug, Default, States, Hash, PartialEq, Eq, Clone, Copy, Reflect)]
pub enum RequestedMenu {
    /// no menu spawned
    #[default]
    None,
    /// start menu
    Start,
    /// pause menu
    Pause,
    /// settings menu
    Settings,
}

/// ui plugin
pub struct InterfacePlugin;

/// simple marker component
#[derive(Debug, Component)]
pub struct InterfaceRoot;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::BootingApp), (spawn_interface_root,));
        app.add_systems(OnEnter(AppState::StartMenu), spawn_start_menu);
        app.add_systems(
            Update,
            (
                update_button_color,
                start_button_interaction,
                exit_button_interaction,
            ),
        );
    }
}

/// spawns entity that all UI is parented under
fn spawn_interface_root(mut cmds: Commands) {
    cmds.spawn((
        Name::new("InterfaceRoot"),
        InterfaceRoot,
        NodeBundle {
            style: Style {
                position_type: PositionType::Relative,
                display: Display::Flex,
                direction: Direction::LeftToRight,
                min_width: Val::Vw(100.0),
                min_height: Val::Vh(100.0),
                justify_content: JustifyContent::FlexStart,
                flex_direction: FlexDirection::Column,
                flex_wrap: FlexWrap::NoWrap,
                ..default()
            },
            ..default()
        },
    ));
}

/// Start menu marker component for querys
#[derive(Component)]
pub struct StartMenu;

/// spawns start menu with buttons
fn spawn_start_menu(
    mut cmds: Commands,
    assets: Res<InitAssetHandles>,
    interface_root: Query<Entity, With<InterfaceRoot>>,
) {
    cmds.entity(interface_root.single())
        .with_children(|children| {
            children
                .spawn((
                    Name::new("StartMenu"),
                    StartMenu,
                    NodeBundle {
                        style: Style {
                            display: Display::Flex,
                            position_type: PositionType::Absolute,
                            overflow: Overflow::clip(),
                            flex_direction: FlexDirection::Column,
                            min_height: Val::Vh(60.0),
                            min_width: Val::Vw(30.0),
                            // aspect_ratio: Some(0.8),
                            align_self: AlignSelf::Center,
                            justify_content: JustifyContent::FlexStart,
                            margin: UiRect {
                                left: Val::Vw(40.0),
                                right: Val::Px(0.0),
                                top: Val::Vh(10.0),
                                bottom: Val::Percent(10.0),
                            },
                            ..default()
                        },
                        background_color: BackgroundColor(random_color(Some(0.8))),
                        ..default()
                    },
                ))
                .with_children(|start_menu_container_childs| {
                    spawn_menu_title(
                        start_menu_container_childs,
                        assets.font_title.clone(),
                        "Aspen Halls",
                    );
                    start_menu_container_childs
                        .spawn((
                            Name::new("ButtonContainer"),
                            NodeBundle {
                                style: Style {
                                    position_type: PositionType::Relative,
                                    flex_direction: FlexDirection::Column,
                                    justify_content: JustifyContent::SpaceEvenly,
                                    align_items: AlignItems::Center,
                                    width: Val::Percent(70.0),
                                    height: Val::Percent(70.0),
                                    // min_height: Val::Percent(20.0),
                                    // max_height: Val::Percent(85.0),
                                    margin: UiRect {
                                        left: Val::Auto,
                                        right: Val::Auto,
                                        top: Val::Px(5.0),
                                        bottom: Val::Px(15.0),
                                    },
                                    ..default()
                                },
                                border_color: BorderColor(random_color(None)),
                                ..default()
                            },
                        ))
                        .with_children(|buttons| {
                            spawn_button(
                                buttons,
                                assets.font_regular.clone(),
                                "Start Game",
                                StartButton,
                            );
                            spawn_button(
                                buttons,
                                assets.font_regular.clone(),
                                "Exit Game",
                                ExitButton,
                            );
                        });
                });
        });
}

/// spawns styled menu button
fn spawn_button<T: Component>(
    buttons: &mut ChildBuilder<'_, '_, '_>,
    font: Handle<Font>,
    text: &str,
    component: T,
) {
    buttons
        .spawn((
            component,
            ButtonBundle {
                style: Style {
                    width: Val::Px(100.0),
                    height: Val::Px(60.0),
                    ..default()
                },
                background_color: BackgroundColor(Color::PURPLE),
                border_color: BorderColor(Color::PINK),
                ..default()
            },
        ))
        .with_children(|button_text| {
            button_text.spawn((
                Name::new("ButtonText"),
                TextBundle::from_section(
                    text,
                    TextStyle {
                        font,
                        font_size: 14.0,
                        color: Color::WHITE,
                    },
                ),
            ));
        });
}

/// spawns a text bundle with alignment center
/// styling for this component makes
/// it a good title for menu like interfaces
fn spawn_menu_title(child_builder: &mut ChildBuilder<'_, '_, '_>, font: Handle<Font>, text: &str) {
    child_builder.spawn((
        Name::new("Title"),
        TextBundle::from_section(
            text,
            TextStyle {
                font,
                color: Color::WHITE,
                font_size: 48.0,
            },
        )
        .with_background_color(random_color(Some(0.6)))
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            aspect_ratio: None,
            display: Display::Flex,
            position_type: PositionType::Relative,
            align_self: AlignSelf::Center,
            align_content: AlignContent::Center,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            margin: UiRect {
                left: Val::Percent(50.0),
                right: Val::Percent(50.0),
                top: Val::Percent(5.0),
                bottom: Val::Percent(5.0),
            },
            width: Val::Percent(65.0),
            height: Val::Px(75.0),
            ..default()
        }),
    ));
}

/// generated random Rgba color with alpha between 0.8-1.0
pub fn random_color(alpha: Option<f32>) -> Color {
    let mut rng = crate::ahp::rand::thread_rng();
    Color::Rgba {
        red: rng.gen(),
        green: rng.gen(),
        blue: rng.gen(),
        alpha: { alpha.map_or_else(|| rng.gen_range(0.8..=1.0), |alpha| alpha) },
    }
}

/// marks start button for query
#[derive(Debug, Component)]
pub struct StartButton;

/// marks start button for query
#[derive(Debug, Component)]
pub struct ExitButton;

/// unpressed unhovered color
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
/// cursor goes over or finger drags over color
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
/// cursor click or finger lift color
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

/// updates color of all buttons with text for interactions
#[allow(clippy::type_complexity)]
fn update_button_color(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
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

/// updates color of all buttons with text for interactions
fn start_button_interaction(
    mut cmds: Commands,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<StartButton>)>,
    mut start_menu_query: Query<&mut Style, (With<Node>, With<StartMenu>)>,
) {
    for interaction in &mut interaction_query {
        if matches!(interaction, Interaction::Pressed) {
            cmds.insert_resource(NextState(Some(AppState::PlayingGame)));
            start_menu_query.single_mut().display = Display::None;
        }
    }
}

/// updates color of all buttons with text for interactions
fn exit_button_interaction(
    mut exit_event_writer: EventWriter<AppExit>,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<ExitButton>)>,
) {
    for interaction in &mut interaction_query {
        if matches!(interaction, Interaction::Pressed) {
            exit_event_writer.send(AppExit);
        }
    }
}
