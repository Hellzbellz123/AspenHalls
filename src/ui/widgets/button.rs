use bevy::prelude::*;
use kayak_ui::{prelude::*, widgets::*};

use crate::loading::assets::UiTextureHandles;

#[derive(Component, Default, PartialEq, Clone)]
pub struct MenuButton {
    pub text: String,
}

impl Widget for MenuButton {}

#[derive(Bundle)]
pub struct MenuButtonBundle {
    pub button: MenuButton,
    pub on_event: OnEvent,
    pub widget_name: WidgetName,
}

impl Default for MenuButtonBundle {
    fn default() -> Self {
        Self {
            button: MenuButton::default(),
            on_event: OnEvent::default(),
            widget_name: MenuButton::default().get_name(),
        }
    }
}
pub fn menu_button_render(
    In((widget_context, entity)): In<(KayakWidgetContext, Entity)>,
    mut commands: Commands,
    images: Res<UiTextureHandles>,
    menu_button_query: Query<&MenuButton>,
    button_state_query: Query<&ButtonState>,
) -> bool {
    let state_entity =
        widget_context.use_state(&mut commands, entity, ButtonState { hovering: false });
    let button_text = menu_button_query.get(entity).unwrap().text.clone();
    let on_event = OnEvent::new(
        move |In((event_dispatcher_context, _, mut event, _entity)): In<(
            EventDispatcherContext,
            WidgetState,
            Event,
            Entity,
        )>,
              mut query: Query<&mut ButtonState>| {
            if let Ok(mut button) = query.get_mut(state_entity) {
                match event.event_type {
                    EventType::MouseIn(..) => {
                        event.stop_propagation();
                        button.hovering = true;
                    }
                    EventType::MouseOut(..) => {
                        button.hovering = false;
                    }
                    _ => {}
                }
            }
            (event_dispatcher_context, event)
        },
    );

    if let Ok(button_state) = button_state_query.get(state_entity) {
        let button_image_handle = if button_state.hovering {
            images.button_blue_pressed.clone()
        } else {
            images.button_blue.clone()
        };

        let parent_id = Some(entity);
        rsx! {
            <NinePatchBundle
                nine_patch={NinePatch {
                    handle: button_image_handle,
                    border: Edge::all(10.0),
                }}
                styles={KStyle {
                    background_color: StyleProp::Value(Color::BLACK),
                    height: StyleProp::Value(Units::Pixels(50.0)),
                    width: StyleProp::Value(Units::Pixels(200.0)),
                    left: StyleProp::Value(Units::Stretch(1.0)),
                    right: StyleProp::Value(Units::Stretch(1.0)),
                    cursor: StyleProp::Value(KCursorIcon(CursorIcon::Hand)),
                    ..default()
                }}
                on_event={on_event}
            >
                <TextWidgetBundle
                    text={TextProps {
                        alignment: Alignment::Middle,
                        content: button_text,
                        size: 28.0,
                        ..Default::default()
                    }}
                />
            </NinePatchBundle>
        }
    }
    true
}
