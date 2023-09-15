// allow type complexity for whole file because i cant allow it for individual fn arguments
#![allow(clippy::type_complexity)]

use bevy::{app::AppExit, prelude::*};

use crate::{game::AppStage, loading::assets::FontHandles};

use super::{
    components::{ExitButton, PlayButton, SettingsButton, StartMenuRoot, UiRoot},
    HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON,
};

/// spawns startmenu as child of ui_root_node
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
                        border: UiRect::all(Val::Px(1.0)),
                        flex_direction: FlexDirection::Column,
                        width: Val::Percent(35.0),
                        // flex_wrap: FlexWrap::Wrap,
                        // justify_content: JustifyContent::Center,
                        // align_content: AlignContent::Start,
                        // align_items: AlignItems::Center,
                        // margin: UiRect {
                        //     top: Val::Percent(10.0),
                        //     bottom: Val::Percent(5.0),
                        //     left: Val::Percent(75.0),
                        //     ..default()
                        // },
                        ..default()
                    },
                    border_color: BorderColor(Color::AQUAMARINE),
                    background_color: Color::rgba(0.22, 0.22, 0.43, 0.22).into(),
                    z_index: ZIndex::Local(1),
                    ..default()
                },
                Name::new("Start Menu"),
                StartMenuRoot,
            ))
            .with_children(|parent| {
                // Title
                parent.spawn((
                    TextBundle::from_section(
                        "Vanilla Coffee",
                        TextStyle {
                            font: fonts.title_font.clone(),
                            font_size: 40.,
                            color: Color::WHITE,
                        },
                    )
                    .with_text_alignment(TextAlignment::Center)
                    .with_style(Style {
                        border: UiRect::all(Val::Px(1.0)),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        height: Val::Px(25.0),
                        // margin: UiRect {
                        //     top: Val::Percent(5.0),
                        //     bottom: Val::Percent(5.0),
                        //     ..default()
                        // },
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
                                border: UiRect::all(Val::Px(1.0)),
                                flex_direction: FlexDirection::Column,
                                // flex_wrap: FlexWrap::Wrap,
                                align_content: AlignContent::Center,
                                justify_content: JustifyContent::Center,
                                row_gap: Val::Px(20.0),
                                ..default()
                            },
                            border_color: BorderColor(Color::RED),
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
                                        width: Val::Px(150.0),
                                        height: Val::Px(65.0),
                                        // horizontally center child text
                                        justify_content: JustifyContent::Center,
                                        // vertically center child text
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                Name::new("Play Button"),
                                PlayButton,
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section(
                                    "Play Game",
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
                                        width: Val::Px(150.0),
                                        height: Val::Px(65.0),
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
                                        width: Val::Px(150.0),
                                        height: Val::Px(65.0),
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

/// handles start menu button interactions
pub fn button_system(
    mut nextstate: ResMut<NextState<AppStage>>,
    mut text_query: Query<&mut Text>,
    mut appexit: EventWriter<AppExit>,
    mut play_button_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (
            Changed<Interaction>,
            (
                With<PlayButton>,
                Without<ExitButton>,
                Without<SettingsButton>,
            ),
        ),
    >,
    #[allow(clippy::type_complexity)] mut exit_button_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (
            Changed<Interaction>,
            (
                With<ExitButton>,
                Without<PlayButton>,
                Without<SettingsButton>,
            ),
        ),
    >,
    #[allow(clippy::type_complexity)] mut settings_button_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (
            Changed<Interaction>,
            (
                With<SettingsButton>,
                Without<PlayButton>,
                Without<ExitButton>,
            ),
        ),
    >,
) {
    for (interaction, mut color, children) in &mut play_button_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                nextstate.set(AppStage::PlayingGame);
                text.sections[0].value = "Played".to_string();
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                text.sections[0].value = "CLICK ME".to_string();
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                text.sections[0].value = "Play Game".to_string();
                *color = NORMAL_BUTTON.into();
            }
        }
    }

    for (interaction, mut color, children) in &mut exit_button_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
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
            Interaction::Pressed => {
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
