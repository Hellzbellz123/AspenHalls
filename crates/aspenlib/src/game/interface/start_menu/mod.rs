use crate::{
    game::interface::{
        random_color,
        settings_menu::SettingsMenuToggleButton,
        ui_widgets::{spawn_button, spawn_menu_title},
        InterfaceRootTag,
    },
    loading::assets::AspenInitHandles,
    AppStage, GameStage,
};
use bevy::app::AppExit;
use bevy::prelude::*;

/// start menu module
pub struct StartMenuPlugin;

impl Plugin for StartMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppStage::Starting), spawn_start_menu);

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

// TODO: maybe use this comment structure elsewhere?
// button tags
//---------------------------------------------------------//
/// marks start button for query
#[derive(Debug, Component)]
pub struct StartGameTag;

/// marks start button for query
#[derive(Debug, Component)]
pub struct ExitGameTag;

// /// marks load game button for query
// #[derive(Debug, Component)]
// pub struct LoadGameTag;

// /// marks save game button for query
// #[derive(Debug, Component)]
// pub struct SaveGameTag;

//------------------------------------------------------------//

/// set start menu container too `Display::Flex` if `AppState` == `StartMenu`
fn show_start_menu(
    mut start_menu_query: Query<&mut Node, With<StartMenuTag>>,
    game_state: Option<Res<State<GameStage>>>,
) {
    let Ok(mut start_menu_style) = start_menu_query.single_mut() else {
        return;
    };
    let Some(state) = game_state else {
        start_menu_style.display = Display::None;
        return;
    };
    if state.get() == &GameStage::StartMenu {
        start_menu_style.display = Display::Flex;
    } else {
        start_menu_style.display = Display::None;
    }
}

/// spawns start menu with buttons
fn spawn_start_menu(
    mut cmds: Commands,
    assets: Res<AspenInitHandles>,
    interface_root: Query<Entity, With<InterfaceRootTag>>,
) {
    let Ok(interface_root) = interface_root.single() else {
        return;
    };

    cmds.entity(interface_root)
        .with_children(|children| {
            children
                .spawn((
                    Name::new("StartMenu"),
                    StartMenuTag,
                    BackgroundColor(random_color(Some(0.8))),
                    Node {
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
                        padding: UiRect::all(Val::Px(0.0)).with_top(Val::Px(5.0)),
                        ..default()
                    },
                ))
                .with_children(|start_menu_container_childs| {
                    spawn_menu_title(
                        start_menu_container_childs,
                        assets.font_title.clone(),
                        "Aspen Halls",
                        48.0,
                    );
                    start_menu_container_childs
                        .spawn((
                            Name::new("ButtonContainer"),
                            BorderColor(random_color(None)),
                            Node {
                                flex_direction: FlexDirection::Column,
                                position_type: PositionType::Relative,
                                align_items: AlignItems::Center,
                                row_gap: Val::Px(15.0),
                                margin: UiRect {
                                    left: Val::Auto,
                                    right: Val::Auto,
                                    top: Val::Px(15.0),
                                    bottom: Val::Px(15.0),
                                },
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
                            #[cfg(not(target_family = "wasm"))]
                            spawn_button(
                                buttons,
                                assets.font_regular.clone(),
                                "Quit Game",
                                ExitGameTag,
                            );
                        });
                });
        });
}

/// updates color of all buttons with text for interactions
fn start_button_interaction(
    mut cmds: Commands,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<StartGameTag>)>,
    mut start_menu_query: Query<&mut Node, With<StartMenuTag>>,
) {
    for interaction in &interaction_query {
        if matches!(interaction, Interaction::Pressed) {
            let Ok(mut start_menu_node) = start_menu_query.single_mut() else {
                return
            };

            start_menu_node.display = Display::None;
            cmds.insert_resource(NextState::Pending(GameStage::SelectCharacter));
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
            exit_event_writer.write(AppExit::Success);
        }
    }
}
