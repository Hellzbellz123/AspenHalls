use crate::game::interface::random_color;
use bevy::prelude::*;

/// spawns styled menu button
pub fn spawn_button<T: Component>(
    buttons: &mut ChildSpawnerCommands,
    font: Handle<Font>,
    text: &str,
    component: T,
) {
    buttons
        .spawn((
            Name::new(format!("{text} Button")),
            component,
            Button,
            Node {
                width: Val::Px(100.0),
                height: Val::Px(60.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(crate::colors::PURPLE.into()),
            BorderColor(crate::colors::PINK.into()),
        ))
        .with_children(|button_text| {
            button_text.spawn((
                Name::new("ButtonText"),
                Text::new(text),
                TextFont {
                    font,
                    font_size: 14.0,
                    ..default()
                },
            ));
        });
}

/// spawns a text bundle with alignment center
/// styling for this component makes
/// it a good title for menu like interfaces
pub fn spawn_menu_title(
    child_builder: &mut ChildSpawnerCommands,
    font: Handle<Font>,
    text: &str,
    font_size: f32,
) {
    child_builder.spawn((
        Name::new("Title"),
        Text::new(text),
        TextLayout::new(JustifyText::Center, LineBreak::WordBoundary),
        TextFont {
            font,
            font_size,
            ..default()
        },
        BackgroundColor(random_color(Some(0.6))),
        Node {
            justify_items: JustifyItems::Center,
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
        },
    ));
}
