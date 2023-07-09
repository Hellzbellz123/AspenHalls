// allow type complexity for whole file because i cant allow it for individual fn arguments
#![allow(clippy::type_complexity)]
use bevy::app::AppExit;
use bevy::prelude::{
    default, AlignContent, AlignItems, BackgroundColor, BuildChildren, ButtonBundle, Changed,
    Children, Color, Commands, Entity, EventWriter, FlexDirection, Interaction, JustifyContent,
    Label, Name, NextState, NodeBundle, Query, Res, ResMut, Size, Style, Text, TextBundle,
    TextStyle, UiRect, Val, With, Without, ZIndex,
};

use crate::{game::GameStage, loading::assets::FontHandles};

use super::{
    components::{ContinueButton, ExitButton, PauseMenuRoot, SettingsButton, UiRoot},
    HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON,
};

/// creates menu as child of ui_root_node
pub fn build(
    mut commands: Commands,
    fonts: Res<FontHandles>,
    root_node: Query<(Entity, With<UiRoot>)>,
) {
    let root_node = root_node.single().0;

    commands.entity(root_node).with_children(|parent| {
        // Start Menu
        parent
            .spawn((
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::Center,
                        size: Size::width(Val::Percent(20.0)),
                        margin: UiRect {
                            top: Val::Percent(10.0),
                            bottom: Val::Percent(5.0),
                            ..default()
                        },
                        position: UiRect {
                            left: Val::Percent(70.0),
                            ..default()
                        },
                        border: UiRect::all(Val::Px(4.0)),
                        ..default()
                    },
                    background_color: Color::rgba(0.22, 0.22, 0.43, 0.22).into(),
                    z_index: ZIndex::Local(1),
                    ..default()
                },
                Name::new("Start Menu"),
                PauseMenuRoot,
            ))
            .with_children(|parent| {
                // Title
                parent.spawn((
                    TextBundle::from_section(
                        "Game Paused",
                        TextStyle {
                            font: fonts.title_font.clone(),
                            font_size: 40.,
                            color: Color::WHITE,
                        },
                    )
                    .with_style(Style {
                        size: Size::height(Val::Px(25.)),
                        margin: UiRect {
                            top: Val::Percent(5.0),
                            ..default()
                        },
                        ..default()
                    }),
                    Label,
                    Name::new("Title"),
                ));
                // StartMenu Button Container
                parent
                    .spawn((
                        NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                align_content: AlignContent::SpaceEvenly,
                                justify_content: JustifyContent::SpaceEvenly,
                                position: UiRect {
                                    top: Val::Percent(30.0),
                                    ..default()
                                },
                                gap: Size {
                                    width: Val::Px(20.0),
                                    height: Val::Px(20.0),
                                },
                                ..default()
                            },
                            ..default()
                        },
                        Name::new("Button Container"),
                    ))
                    .with_children(|parent| {
                        //  Play Button
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: Style {
                                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                                        // horizontally center child text
                                        justify_content: JustifyContent::Center,
                                        // vertically center child text
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                Name::new("Continue Button"),
                                ContinueButton,
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section(
                                    "Continue Game",
                                    TextStyle {
                                        font: fonts.main_font.clone(),
                                        font_size: 24.0,
                                        color: Color::rgb(1.0, 0.2, 0.1),
                                    },
                                ));
                            });

                        // Settings Button
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: Style {
                                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                                        // horizontally center child text
                                        justify_content: JustifyContent::Center,
                                        // vertically center child text
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                Name::new("Settings Button"),
                                SettingsButton,
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section(
                                    "Settings",
                                    TextStyle {
                                        font: fonts.main_font.clone(),
                                        font_size: 24.0,
                                        color: Color::rgb(1.0, 0.2, 0.1),
                                    },
                                ));
                            });
                        // Exit Button
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: Style {
                                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                                        // horizontally center child text
                                        justify_content: JustifyContent::Center,
                                        // vertically center child text
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                Name::new("Exit Button"),
                                ExitButton,
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section(
                                    "Exit Game",
                                    TextStyle {
                                        font: fonts.main_font.clone(),
                                        font_size: 24.0,
                                        color: Color::rgb(1.0, 0.2, 0.1),
                                    },
                                ));
                            });
                    });
            });
    });
}

/// handle button interactions for pausemenu
pub fn button_system(
    mut nextstate: ResMut<NextState<GameStage>>,
    mut text_query: Query<&mut Text>,
    mut appexit: EventWriter<AppExit>,
    mut continue_button_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (
            Changed<Interaction>,
            (
                With<ContinueButton>,
                Without<ExitButton>,
                Without<SettingsButton>,
            ),
        ),
    >,
    mut exit_button_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (
            Changed<Interaction>,
            (
                With<ExitButton>,
                Without<ContinueButton>,
                Without<SettingsButton>,
            ),
        ),
    >,
    mut settings_button_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (
            Changed<Interaction>,
            (
                With<SettingsButton>,
                Without<ContinueButton>,
                Without<ExitButton>,
            ),
        ),
    >,
) {
    for (interaction, mut color, children) in &mut continue_button_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                nextstate.set(GameStage::PlayingGame);
                text.sections[0].value = "Continue Game".to_string();
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                text.sections[0].value = "CLICK ME".to_string();
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                text.sections[0].value = "Continue Game".to_string();
                *color = NORMAL_BUTTON.into();
            }
        }
    }

    for (interaction, mut color, children) in &mut exit_button_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                appexit.send(AppExit);
                text.sections[0].value = "Exit Game".to_string();
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                text.sections[0].value = "Exit Game".to_string();
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                text.sections[0].value = "Exit Game".to_string();
                *color = NORMAL_BUTTON.into();
            }
        }
    }

    for (interaction, mut color, children) in &mut settings_button_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Does Nothing".to_string();
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                text.sections[0].value = "Does Nothing".to_string();
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                text.sections[0].value = "Settings".to_string();
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
