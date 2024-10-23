use crate::{
    game::{
        interface::{
            random_color,
            settings_menu::SettingsMenuToggleButton,
            ui_widgets::{spawn_button, spawn_menu_title},
            InterfaceRootTag,
        },
        AppState,
    },
    loading::assets::AspenInitHandles,
};
use bevy::app::AppExit;
use bevy::prelude::*;

/// start menu module
pub struct StartMenuPlugin;

impl Plugin for StartMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(AppState::Loading), spawn_start_menu);
        app.add_systems(
            Update,
            (
                start_button_interaction,
                exit_button_interaction,
                show_start_menu,
            ),
        );
    }
}

/// Start menu marker component for querys
#[derive(Component)]
pub struct StartMenuTag;

/// marks start button for query
#[derive(Debug, Component)]
pub struct StartGameTag;

/// marks start button for query
#[derive(Debug, Component)]
pub struct ExitGameTag;

/// set start menu container too `Display::Flex` if `AppState` == `StartMenu`
fn show_start_menu(
    mut start_menu_query: Query<&mut Style, (With<Node>, With<StartMenuTag>)>,
    game_state: Res<State<AppState>>,
) {
    if game_state.is_changed() {
        let Ok(mut start_menu_style) = start_menu_query.get_single_mut() else {
            return;
        };
        if game_state.get() == &AppState::StartMenu {
            start_menu_style.display = Display::Flex;
        } else {
            start_menu_style.display = Display::None;
        }
    }
}

/// spawns start menu with buttons
fn spawn_start_menu(
    mut cmds: Commands,
    assets: Res<AspenInitHandles>,
    interface_root: Query<Entity, With<InterfaceRootTag>>,
) {
    cmds.entity(interface_root.single())
        .with_children(|children| {
            children
                .spawn((
                    Name::new("StartMenu"),
                    StartMenuTag,
                    NodeBundle {
                        style: Style {
                            display: Display::None,
                            position_type: PositionType::Absolute,
                            overflow: Overflow::clip(),
                            flex_direction: FlexDirection::Column,
                            min_height: Val::Percent(60.0),
                            min_width: Val::Vw(30.0),
                            // aspect_ratio: Some(0.8),
                            align_self: AlignSelf::Center,
                            justify_content: JustifyContent::FlexStart,
                            margin: UiRect {
                                left: Val::Vw(40.0),
                                right: Val::Px(0.0),
                                top: Val::Percent(10.0),
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
                                StartGameTag,
                            );
                            spawn_button(
                                buttons,
                                assets.font_regular.clone(),
                                "Settings",
                                SettingsMenuToggleButton,
                            );
                            spawn_button(
                                buttons,
                                assets.font_regular.clone(),
                                "Exit Game",
                                ExitGameTag,
                            );
                        });
                });
        });
}

/// updates color of all buttons with text for interactions
fn start_button_interaction(
    // mut cmds: Commands,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<StartGameTag>)>,
    mut start_menu_query: Query<&mut Style, (With<Node>, With<StartMenuTag>)>,
) {
    for interaction in &interaction_query {
        if matches!(interaction, Interaction::Pressed) {
            start_menu_query.single_mut().display = Display::None;
            // cmds.insert_resource(NextState(Some(AppState::PlayingGame)));
        }
    }
}

/// updates color of all buttons with text for interactions
fn exit_button_interaction(
    mut exit_event_writer: EventWriter<AppExit>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<ExitGameTag>)>,
) {
    for interaction in &interaction_query {
        if matches!(interaction, Interaction::Pressed) {
            exit_event_writer.send(AppExit::Success);
        }
    }
}
