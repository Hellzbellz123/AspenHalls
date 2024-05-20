use bevy::prelude::*;

/// settings menu toggle button
#[derive(Debug, Component)]
pub struct SettingsMenuToggleButton;

use crate::{
    game::{
        interface::{
            random_color,
            ui_widgets::{spawn_button, spawn_menu_title},
            InterfaceRootTag,
        },
        AppState,
    },
    loading::assets::AspenInitHandles,
};

// TODO: expand settings menu too include different settings

/// game configuration ui
pub struct SettingsMenuPlugin;

impl Plugin for SettingsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(AppState::Loading), spawn_settings_menu);
        app.add_systems(
            Update,
            (
                close_settings_interaction,
                apply_settings_interaction,
                toggle_settings_interactions
                    .run_if(in_state(AppState::PauseMenu).or_else(in_state(AppState::StartMenu))),
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
    cmds.entity(interface_root.single())
        .with_children(|children| {
            children
                .spawn((
                    Name::new("SettingsMenu"),
                    SettingsMenuTag,
                    NodeBundle {
                        style: Style {
                            display: Display::None,
                            position_type: PositionType::Absolute,
                            overflow: Overflow::clip(),
                            flex_direction: FlexDirection::Column,
                            min_height: Val::Percent(75.0),
                            min_width: Val::Vw(75.0),
                            // aspect_ratio: Some(0.8),
                            align_self: AlignSelf::Center,
                            justify_content: JustifyContent::FlexStart,
                            margin: UiRect {
                                left: Val::Vw(0.0),
                                right: Val::Px(0.0),
                                top: Val::Percent(10.0),
                                bottom: Val::Percent(10.0),
                            },
                            ..default()
                        },
                        z_index: ZIndex::Local(3),
                        background_color: BackgroundColor(random_color(Some(0.95))),
                        ..default()
                    },
                ))
                .with_children(|start_menu_container_childs| {
                    start_menu_container_childs
                        .spawn((
                            Name::new("TopButtonContainer"),
                            NodeBundle {
                                style: Style {
                                    position_type: PositionType::Relative,
                                    flex_direction: FlexDirection::Row,
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
                                "Apply Settings",
                                ApplySettingsTag,
                            );
                            spawn_button(
                                buttons,
                                assets.font_regular.clone(),
                                "Close Settings",
                                CloseSettingsTag,
                            );
                            spawn_menu_title(buttons, assets.font_title.clone(), "Settings Menu");
                        });
                });
        });
}

/// updates color of all buttons with text for interactions
fn close_settings_interaction(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<CloseSettingsTag>)>,
    mut settings_menu_query: Query<&mut Style, (With<Node>, With<SettingsMenuTag>)>,
) {
    for interaction in &interaction_query {
        if matches!(interaction, Interaction::Pressed) {
            settings_menu_query.single_mut().display = Display::None;
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
    mut settings_menu_query: Query<&mut Style, (With<Node>, With<SettingsMenuTag>)>,
) {
    let mut settings_menu_style = settings_menu_query.single_mut();

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
