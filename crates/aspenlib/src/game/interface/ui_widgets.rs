use crate::game::interface::random_color;
use bevy::prelude::*;

/// spawns styled menu button
pub fn spawn_button<T: Component>(
    buttons: &mut ChildBuilder,
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
pub fn spawn_menu_title(child_builder: &mut ChildBuilder, font: Handle<Font>, text: &str) {
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
        .with_text_justify(JustifyText::Center)
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
