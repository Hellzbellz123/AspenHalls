use bevy::{app::AppExit, prelude::*};
use kayak_ui::{
    prelude::*,
    widgets::{
        ButtonState, ElementBundle, KayakAppBundle, KayakWidgetsContextPlugin, NinePatch,
        NinePatchBundle, TextProps, TextWidgetBundle,
    },
};

use crate::{
    loading::assets::FontHandles,
    ui::widgets::button::{self, menu_button_render, MenuButton},
};

pub fn failed_load_ui(
    mut commands: Commands,
    assetserver: Res<AssetServer>,
    mut font_mapping: ResMut<FontMapping>,
) {
    font_mapping.set_default(assetserver.load("fonts/kttf/FantasqueSansMonoNF.kayak_font"));

    let mut widget_context = KayakRootContext::new();
    widget_context.add_plugin(KayakWidgetsContextPlugin);

    let parent_id = None;

    widget_context.add_widget_data::<MenuButton, ButtonState>();
    widget_context.add_widget_system(
        MenuButton::default().get_name(),
        widget_update::<MenuButton, ButtonState>,
        menu_button_render,
    );

    let on_click_exit = OnEvent::new(
        move |In((event_dispatcher_context, _, event, _entity)): In<(
            EventDispatcherContext,
            WidgetState,
            Event,
            Entity,
        )>,
              mut exit: EventWriter<AppExit>| {
            if let EventType::Click(..) = event.event_type {
                exit.send(AppExit);
            }
            (event_dispatcher_context, event)
        },
    );

    let ninepatch_style = KStyle {
        border_radius: StyleProp::Value(Corner::all(15.0)),
        background_color: StyleProp::Value(Color::WHITE),
        bottom: StyleProp::Value(Units::Percentage(70.0)),
        top: StyleProp::Value(Units::Percentage(30.0)),
        left: StyleProp::Value(Units::Percentage(50.0)),
        right: StyleProp::Value(Units::Percentage(50.0)),
        layout_type: StyleProp::Value(LayoutType::Column),
        row_between: StyleProp::Value(Units::Pixels(50.0)),
        padding: StyleProp::Value(Edge::axis(Units::Stretch(20.0), Units::Stretch(0.0))),
        height: StyleProp::Value(Units::Pixels(500.0)),
        width: StyleProp::Value(Units::Pixels(460.0)),
        ..Default::default()
    };

    rsx! {
        <KayakAppBundle>
                    <NinePatchBundle styles={ninepatch_style} nine_patch={NinePatch {border:{Edge::all(1.0)}, ..default()}}>
                    <TextWidgetBundle text={TextProps { content: "loading game failed. there was missing assets".to_string(), size: 32.0, alignment: Alignment::Middle, ..default()}}/>
                    <ElementBundle/>
                    <button::MenuButtonBundle button={ MenuButton { text: "exit game".into()}} on_event={on_click_exit}/>
                    </NinePatchBundle>
        </KayakAppBundle>
    };
    commands.spawn((UICameraBundle::new(widget_context), Name::new("UI Camera")));
}
