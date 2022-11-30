use bevy::prelude::{Res, ResMut, World};
use kayak_ui::{
    bevy::ImageManager,
    core::{
        rsx,
        styles::{
            Corner, Edge, LayoutType, Style, StyleProp,
            Units,
        },
        widget, Binding, Bound, Color, EventType, Handler,
        OnEvent, WidgetProps,
    },
    widgets::{Element, NinePatch, Text},
};

use super::button;
use crate::{
    assets::ImageAssets,
    settings::{AudioSettings, GameSettings},
    ui::{
        checkbox::Checkbox, snake_selector::SnakeSelector,
    },
};

#[derive(WidgetProps, Clone, Debug, Default, PartialEq)]
pub struct SettingsMenuProps {
    pub back: Handler<()>,
}

#[widget]
pub fn SettingsMenu(props: SettingsMenuProps) {
    let container_styles = Style {
        border_radius: StyleProp::Value(Corner::all(15.0)),
        background_color: StyleProp::Value(Color::WHITE),
        bottom: StyleProp::Value(Units::Stretch(1.0)),
        height: StyleProp::Value(Units::Pixels(500.0)),
        layout_type: StyleProp::Value(LayoutType::Column),
        left: StyleProp::Value(Units::Stretch(1.0)),
        padding: StyleProp::Value(Edge::all(
            Units::Stretch(1.0),
        )),
        right: StyleProp::Value(Units::Stretch(1.0)),
        row_between: StyleProp::Value(Units::Pixels(20.0)),
        top: StyleProp::Value(Units::Stretch(1.0)),
        width: StyleProp::Value(Units::Pixels(360.0)),
        ..Default::default()
    };
    let checkbox_styles = Style {
        layout_type: StyleProp::Value(LayoutType::Row),
        col_between: StyleProp::Value(Units::Pixels(20.0)),
        ..Default::default()
    };

    let green_panel = context
        .query_world::<Res<ImageAssets>, _, _>(|assets| {
            assets.green_panel.clone()
        });

    let container = context
        .get_global_mut::<World>()
        .map(|mut world| {
            world
                .get_resource_mut::<ImageManager>()
                .unwrap()
                .get(&green_panel)
        })
        .unwrap();

    let settings = {
        let settings = context
                .query_world::<Res<Binding<GameSettings>>, _, _>(
                    move |settings| settings.clone(),
                );

        context.bind(&settings);
        settings.get()
    };
    let back = props.back.clone();
    let on_click_back = OnEvent::new(move |_, event| {
        match event.event_type {
            EventType::Click(..) => {
                back.call(());
            }
            _ => {}
        }
    });

    let on_click_audio = OnEvent::new(
        move |context, event| match event.event_type {
            EventType::Click(..) => {
                context.query_world::<ResMut<GameSettings>, _, _>(
            |mut settings| {
                settings.audio = match settings.audio {
                    AudioSettings::ON => AudioSettings::OFF,
                    AudioSettings::OFF => AudioSettings::ON,
                };
            },
        );
            }
            _ => {}
        },
    );

    let audio_checked = match settings.audio {
        AudioSettings::ON => true,
        AudioSettings::OFF => false,
    };

    rsx! {
        <NinePatch
            styles={Some(container_styles)}
            border={Edge::all(10.0)}
            handle={container}
        >
            <button::SnakeButton
                on_event={Some(on_click_back)}
                >
                <Text
                    size={20.0}
                    content={"Back".to_string()}
                />
            </button::SnakeButton>
            <Element styles={Some(checkbox_styles)}>
                <Checkbox
                    checked={audio_checked}
                    on_event={Some(on_click_audio)}
                />
                <Text
                    size={20.0}
                    content={"Play Audio".to_string()}
                />
            </Element>
            <SnakeSelector/>
        </NinePatch>
    }
}
