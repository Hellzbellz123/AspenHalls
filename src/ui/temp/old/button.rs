use bevy::prelude::*;
use kayak_ui::{prelude::*, widgets::*};

use crate::assets::ImageAssets;

#[derive(Component, Default, PartialEq, Clone)]
pub struct BlockBreakerMenuButton {
    pub text: String,
}

impl Widget for BlockBreakerMenuButton {}

#[derive(Bundle)]
pub struct BlockBreakerMenuButtonBundle {
    pub button: BlockBreakerMenuButton,
    pub on_event: OnEvent,
    pub widget_name: WidgetName,
}

impl Default for BlockBreakerMenuButtonBundle {
    fn default() -> Self {
        Self {
            button: BlockBreakerMenuButton::default(),
            on_event: OnEvent::default(),
            widget_name: BlockBreakerMenuButton::default()
                .get_name(),
        }
    }
}
pub fn block_breaker_menu_button_render(
    In((widget_context, entity)): In<(
        KayakWidgetContext,
        Entity,
    )>,
    mut commands: Commands,
    images: Res<ImageAssets>,
    menu_button_query: Query<&BlockBreakerMenuButton>,
    button_state_query: Query<&ButtonState>,
) -> bool {
    let state_entity = widget_context.use_state(
        &mut commands,
        entity,
        ButtonState { hovering: false },
    );
    let button_text =
        menu_button_query.get(entity).unwrap().text.clone();
    let on_event = OnEvent::new(
        move |In((
            event_dispatcher_context,
            _,
            mut event,
            _entity,
        )): In<(
            EventDispatcherContext,
            WidgetState,
            Event,
            Entity,
        )>,
              mut query: Query<&mut ButtonState>| {
            if let Ok(mut button) =
                query.get_mut(state_entity)
            {
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

    if let Ok(button_state) =
        button_state_query.get(state_entity)
    {
        let button_image_handle = if button_state.hovering {
            images.button_pressed.clone()
        } else {
            images.button.clone()
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
