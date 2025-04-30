use crate::{colors, AppStage, GameStage};
use bevy::prelude::*;

/// settings menu toggle button
#[derive(Debug, Component)]
pub struct SettingsMenuToggleButton;

use crate::{
    game::interface::{
        random_color,
        ui_widgets::{spawn_button, spawn_menu_title},
        InterfaceRootTag,
    },
    loading::assets::AspenInitHandles,
};

// TODO: expand settings menu too include different settings

/// game configuration ui
pub struct SettingsMenuPlugin;

impl Plugin for SettingsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppStage::Starting), spawn_settings_menu);
        app.add_systems(
            Update,
            (
                settings_menu_visibility,
                close_settings_interaction,
                apply_settings_interaction,
                toggle_settings_interactions.run_if(
                    in_state(GameStage::PausedGame).or(in_state(GameStage::StartMenu)),
                ),
            ),
        );
    }
}

/// Start menu marker component for querys
#[derive(Component)]
pub struct SettingsMenuTag;

/// marks 'go back to main menu button' for query
#[derive(Debug, Component)]
pub struct ApplySettingsTag;

/// marks start button for query
#[derive(Debug, Component)]
pub struct CloseSettingsTag;

/// spawns start menu with buttons
fn spawn_settings_menu(
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
                    Name::new("SettingsMenu"),
                    SettingsMenuTag,
                    Node {
                        display: Display::None,
                        position_type: PositionType::Absolute,
                        flex_direction: FlexDirection::Column,
                        height: Val::Percent(90.0),
                        width: Val::Percent(85.0),
                        margin: UiRect::all(Val::Auto)
                            .with_top(Val::Px(50.0))
                            .with_bottom(Val::Px(50.0)),
                        ..default()
                    },
                    bevy::ui::FocusPolicy::Block,
                    ZIndex(3),
                    BackgroundColor(random_color(Some(0.95))),
                ))
                .with_children(|start_menu_container_childs| {
                    start_menu_container_childs
                        .spawn((
                            Name::new("SettingsTopBar"),
                            BackgroundColor(colors::REBECCA_PURPLE.into()),
                            BorderRadius::all(Val::Px(15.0)),
                            Node {
                                position_type: PositionType::Relative,
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                height: Val::Percent(15.0),
                                // justify_content: JustifyContent::SpaceEvenly,
                                // width: Val::Percent(70.0),
                                // height: Val::Percent(70.0),
                                // // min_height: Val::Percent(20.0),
                                // // max_height: Val::Percent(85.0),
                                padding: UiRect::left(Val::Px(30.0)),
                                margin: UiRect::all(Val::Px(10.0)),
                                border: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            BorderColor(random_color(None)),
                        ))
                        .with_children(|buttons| {
                            buttons
                                .spawn((
                                    Name::new("TopBarButtonsContainer"),
                                    Node {
                                        column_gap: Val::Px(15.0),
                                        ..default()
                                    },
                                ))
                                .with_children(|buttons| {
                                    spawn_button(
                                        buttons,
                                        assets.font_regular.clone(),
                                        "Apply Settings",
                                        ApplySettingsTag,
                                    );
                                    spawn_button(
                                        buttons,
                                        assets.font_regular.clone(),
                                        "Close Settings",
                                        CloseSettingsTag,
                                    );
                                });
                            spawn_menu_title(
                                buttons,
                                assets.font_title.clone(),
                                "Settings Menu",
                                38.0,
                            );
                        });
                });
        });
}

fn settings_menu_visibility(
    game_state: Option<Res<State<GameStage>>>,
    mut settings_menu_query: Query<&mut Node, With<SettingsMenuTag>>,
) {
    let Some(state) = game_state else {
        return;
    };
    match state.get() {
        GameStage::PlayingGame | GameStage::SelectCharacter => {
            let Ok(mut s_menu_node) = settings_menu_query.single_mut() else {
                return;
            };
            
            s_menu_node.display = Display::None;
        }
        _ => {}
    }
}

/// updates color of all buttons with text for interactions
fn close_settings_interaction(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<CloseSettingsTag>)>,
    mut settings_menu_query: Query<&mut Node, With<SettingsMenuTag>>,
) {
    for interaction in &interaction_query {
        if matches!(interaction, Interaction::Pressed) {
            let Ok(mut s_menu_node) = settings_menu_query.single_mut() else {
                return;
            };
            s_menu_node.display = Display::None;
        }
    }
}

/// updates color of all buttons with text for interactions
fn apply_settings_interaction(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<ApplySettingsTag>)>,
) {
    for interaction in &interaction_query {
        if matches!(interaction, Interaction::Pressed) {
            //TODO: apply game settings?
            // should probably be an event
            info!("applying game settings");
        }
    }
}

/// toggles display of settings menu
fn toggle_settings_interactions(
    // mut cmds: Commands,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<SettingsMenuToggleButton>)>,
    mut settings_menu_query: Query<&mut Node, With<SettingsMenuTag>>,
) {
    let Ok(mut settings_menu_style) = settings_menu_query.single_mut() else {
        return
    };

    for interaction in &interaction_query {
        if matches!(interaction, Interaction::Pressed) {
            if settings_menu_style.display == Display::None {
                settings_menu_style.display = Display::Flex;
            } else {
                settings_menu_style.display = Display::None;
            }
        }
    }
}
