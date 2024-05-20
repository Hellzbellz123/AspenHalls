use crate::loading::assets::AspenTouchHandles;
use bevy::prelude::*;

/// create action buttons widget
pub fn create_action_buttons(
    playing_ui_parts: &mut ChildBuilder,
    touch_assets: Res<'_, AspenTouchHandles>,
) {
    playing_ui_parts
        .spawn((
            Name::new("ActionButtonsRow"),
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    position_type: PositionType::Relative,
                    flex_direction: FlexDirection::Row,
                    align_self: AlignSelf::Center,
                    align_content: AlignContent::Center,
                    justify_content: JustifyContent::SpaceEvenly,
                    margin: UiRect {
                        left: Val::Percent(0.0),
                        right: Val::Auto,
                        top: Val::Auto,
                        bottom: Val::Px(0.0),
                    },
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|action_buttons_holder| {
            let bottom = Val::Percent(0.0);
            spawn_actionbutton(
                Val::Percent(40.0),
                bottom,
                action_buttons_holder,
                ActionNumber::One,
                touch_assets.action_one.clone(),
            );
            spawn_actionbutton(
                Val::Percent(40.0),
                bottom,
                action_buttons_holder,
                ActionNumber::Two,
                touch_assets.action_two.clone(),
            );
            spawn_actionbutton(
                Val::Percent(40.0),
                bottom,
                action_buttons_holder,
                ActionNumber::Three,
                touch_assets.action_three.clone(),
            );
            spawn_actionbutton(
                Val::Percent(40.0),
                bottom,
                action_buttons_holder,
                ActionNumber::Four,
                touch_assets.action_four.clone(),
            );
            spawn_actionbutton(
                Val::Percent(40.0),
                bottom,
                action_buttons_holder,
                ActionNumber::Five,
                touch_assets.action_five.clone(),
            );
        });
}

#[derive(Copy, Clone)]
/// avaialable action slots
pub enum ActionNumber {
    /// action slot 1
    One,
    /// action slot 2
    Two,
    /// action slot 3
    Three,
    /// action slot 4
    Four,
    /// action slot 5
    Five,
}

impl From<ActionNumber> for String {
    fn from(val: ActionNumber) -> Self {
        use ActionNumber::{Five, Four, One, Three, Two};
        match val {
            One => "One".to_string(),
            Two => "Two".to_string(),
            Three => "Three".to_string(),
            Four => "Four".to_string(),
            Five => "Five".to_string(),
        }
    }
}

#[derive(Component)]
/// what action this widget is tied too
pub struct ActionButton(ActionNumber);

/// reusable action button widget
fn spawn_actionbutton(
    _xpos: Val,
    _ypos: Val,
    button_parent: &mut ChildBuilder,
    slot: ActionNumber,
    image: Handle<Image>,
) {
    let slot_name = format!(
        "ActionButton({})",
        <ActionNumber as Into<String>>::into(slot)
    );

    button_parent.spawn((
        Name::new(slot_name),
        ActionButton(slot),
        ButtonBundle {
            style: Style {
                // left: xpos,
                // top: ypos,
                width: Val::Px(50.0),
                height: Val::Px(50.0),
                // display: Display::default(),
                // position_type: PositionType::default(),
                // overflow: Overflow::default(),
                // direction: Direction::default(),
                // right: Val::default(),
                // bottom: Val::default(),
                // width: Val::default(),
                // height: Val::default(),
                // min_width: Val::default(),
                // min_height: Val::default(),
                // max_width: Val::default(),
                // max_height: Val::default(),
                // aspect_ratio: None,
                // align_items: AlignItems::default(),
                // justify_items: JustifyItems::default(),
                // align_self: AlignSelf::default(),
                // justify_self: JustifySelf::default(),
                // align_content: AlignContent::default(),
                // justify_content: JustifyContent::default(),
                // margin: UiRect::default(),
                // padding: UiRect::default(),
                // border: UiRect::default(),
                // flex_direction: FlexDirection::default(),
                // flex_wrap: FlexWrap::default(),
                // flex_grow: 0.0,
                // flex_shrink: 0.0,
                // flex_basis: Val::default(),
                // row_gap: Val::default(),
                // column_gap: Val::default(),
                ..default()
            },
            background_color: BackgroundColor::default(),
            border_color: BorderColor(Color::WHITE),
            image: UiImage::new(image),
            ..default()
        },
    ));
}
