// pub(crate) use bevy::prelude::{Handle, World, *};

use bevy::prelude::*;
use kayak_ui::{
    bevy::ImageManager,
    core::{
        rsx,
        styles::{Edge, Style, StyleProp, Units},
        widget, Bound, Children, EventType, MutableBound, OnEvent, WidgetProps,
    },
    widgets::NinePatch,
};

use crate::{
    loading::assets::UiTextureHandles,
    ui::events_handlers::{AppExitEvent, PlayButtonEvent}, //OptionsButtonEvent
};

#[derive(WidgetProps, Clone, Debug, Default, PartialEq)]
pub struct BlueButtonProps {
    #[prop_field(Styles)]
    pub styles: Option<Style>,
    #[prop_field(Children)]
    children: Option<Children>,
    // #[prop_field(OnEvent)]
    // on_event: Option<OnEvent>,
}

#[widget]
pub fn OptionsButton(props: BlueButtonProps) {
    let (blue_button_handle, blue_button_hover_handle) = {
        let world = context.get_global_mut::<World>();
        if world.is_err() {
            warn!("world is error in widget");
            return;
        }

        let mut world = world.expect("World should exist if we are being spawned");

        let (handle1, handle2) = {
            let uitexassets = world
                .get_resource::<UiTextureHandles>()
                .expect("no texture assets?");
            let handle1: Handle<bevy::render::texture::Image> =
                uitexassets.button_blue_pressed_png.clone();
            let handle2: Handle<bevy::render::texture::Image> = uitexassets.button_blue_png.clone();
            (handle1, handle2)
        };

        let mut image_manager = world
            .get_resource_mut::<ImageManager>()
            .expect("couldnt load image manager");
        let blue_button_handle = image_manager.get(&handle1);
        let blue_button_hover_handle = image_manager.get(&handle2);

        (blue_button_handle, blue_button_hover_handle)
    };

    let current_button_handle = context
        .create_state::<u16>(blue_button_handle)
        .expect("couldnt insert button state to kayakcontext");
    let cloned_current_button_handle = current_button_handle.clone();

    let button_styles = Style {
        width: StyleProp::Value(Units::Pixels(200.0)),
        height: StyleProp::Value(Units::Pixels(50.0)),
        padding: StyleProp::Value(Edge::all(Units::Stretch(1.0))),
        ..props.styles.clone().unwrap_or_default()
    };

    let on_event = OnEvent::new(move |_, event| match event.event_type {
        EventType::MouseOut(..) | EventType::MouseUp(..) => {
            cloned_current_button_handle.set(blue_button_handle);
        }
        EventType::MouseDown(..) => {
            cloned_current_button_handle.set(blue_button_hover_handle);
        }
        EventType::Click(..) => {
            info!("todo! make a settings menu and a saving system");
        }

        _ => (),
    });

    let children = props.get_children();
    rsx! {
        <NinePatch
            border={Edge::all(10.0)}
            handle={current_button_handle.get()}
            styles={Some(button_styles)}
            on_event={Some(on_event)}
        >
            {children}
        </NinePatch>
    }
}

#[widget]
pub fn PlayButton(props: BlueButtonProps) {
    let (blue_button_handle, blue_button_hover_handle) = {
        let world = context.get_global_mut::<World>();
        if world.is_err() {
            return;
        }

        let mut world = world.expect("World should exist if we are being spawned");

        let (handle1, handle2) = {
            let uitexassets = world
                .get_resource::<UiTextureHandles>()
                .expect("no texture assets?");
            let handle1: Handle<bevy::render::texture::Image> =
                uitexassets.button_blue_pressed_png.clone();
            let handle2: Handle<bevy::render::texture::Image> = uitexassets.button_blue_png.clone();
            (handle1, handle2)
        };

        let mut image_manager = world
            .get_resource_mut::<ImageManager>()
            .expect("couldnt load image manager");
        let blue_button_handle = image_manager.get(&handle1);
        let blue_button_hover_handle = image_manager.get(&handle2);

        (blue_button_handle, blue_button_hover_handle)
    };

    let current_button_handle = context
        .create_state::<u16>(blue_button_handle)
        .expect("couldnt insert button state to kayakcontext");
    let cloned_current_button_handle = current_button_handle.clone();

    let button_styles = Style {
        width: StyleProp::Value(Units::Pixels(200.0)),
        height: StyleProp::Value(Units::Pixels(50.0)),
        padding: StyleProp::Value(Edge::all(Units::Stretch(1.0))),
        ..props.styles.clone().unwrap_or_default()
    };

    let on_event = OnEvent::new(move |ctx, event| match event.event_type {
        EventType::MouseOut(..) | EventType::MouseUp(..) => {
            cloned_current_button_handle.set(blue_button_handle);
        }
        EventType::MouseDown(..) => {
            cloned_current_button_handle.set(blue_button_hover_handle);
        }
        EventType::Click(..) => {
            ctx.query_world::<EventWriter<PlayButtonEvent>, _, ()>(|mut writer| {
                writer.send(PlayButtonEvent);
            });
        }
        _ => (),
    });

    let children = props.get_children();
    rsx! {
        <NinePatch
            border={Edge::all(10.0)}
            handle={current_button_handle.get()}
            styles={Some(button_styles)}
            on_event={Some(on_event)}
        >
            {children}
        </NinePatch>
    }
}

#[widget]
pub fn ExitButton(props: BlueButtonProps) {
    let (blue_button_handle, blue_button_hover_handle) = {
        let world = context.get_global_mut::<World>();
        if world.is_err() {
            return;
        }

        let mut world = world.expect("World should exist if we are being spawned");

        let (handle1, handle2) = {
            let uitexassets = world
                .get_resource::<UiTextureHandles>()
                .expect("no texture assets?");
            let handle1: Handle<bevy::render::texture::Image> =
                uitexassets.button_blue_pressed_png.clone();
            let handle2: Handle<bevy::render::texture::Image> = uitexassets.button_blue_png.clone();
            (handle1, handle2)
        };

        let mut image_manager = world
            .get_resource_mut::<ImageManager>()
            .expect("couldnt load image manager");
        let blue_button_handle = image_manager.get(&handle1);
        let blue_button_hover_handle = image_manager.get(&handle2);

        (blue_button_handle, blue_button_hover_handle)
    };

    let current_button_handle = context
        .create_state::<u16>(blue_button_handle)
        .expect("couldnt insert button state to kayakcontext");
    let cloned_current_button_handle = current_button_handle.clone();

    let button_styles = Style {
        width: StyleProp::Value(Units::Pixels(200.0)),
        height: StyleProp::Value(Units::Pixels(50.0)),
        padding: StyleProp::Value(Edge::all(Units::Stretch(1.0))),
        ..props.styles.clone().unwrap_or_default()
    };

    let on_event = OnEvent::new(move |ctx, event| match event.event_type {
        EventType::MouseOut(..) | EventType::MouseUp(..) => {
            cloned_current_button_handle.set(blue_button_handle);
        }
        EventType::MouseDown(..) => {
            cloned_current_button_handle.set(blue_button_hover_handle);
        }
        EventType::Click(..) => {
            ctx.query_world::<EventWriter<AppExitEvent>, _, ()>(|mut writer| {
                writer.send(AppExitEvent);
            });
        }
        _ => (),
    });

    let children = props.get_children();
    rsx! {
        <NinePatch
            border={Edge::all(10.0)}
            handle={current_button_handle.get()}
            styles={Some(button_styles)}
            on_event={Some(on_event)}
        >
            {children}
        </NinePatch>
    }
}

#[widget]
pub fn SaveButton(props: BlueButtonProps) {
    //save system todo!
    let (blue_button_handle, blue_button_hover_handle) = {
        let world = context.get_global_mut::<World>();
        if world.is_err() {
            return;
        }

        let mut world = world.expect("World should exist if we are being spawned");

        let (handle1, handle2) = {
            let uitexassets = world
                .get_resource::<UiTextureHandles>()
                .expect("no texture assets?");
            let handle1: Handle<bevy::render::texture::Image> =
                uitexassets.button_blue_pressed_png.clone();
            let handle2: Handle<bevy::render::texture::Image> = uitexassets.button_blue_png.clone();
            (handle1, handle2)
        };

        let mut image_manager = world
            .get_resource_mut::<ImageManager>()
            .expect("couldnt load image manager");
        let blue_button_handle = image_manager.get(&handle1);
        let blue_button_hover_handle = image_manager.get(&handle2);

        (blue_button_handle, blue_button_hover_handle)
    };

    let current_button_handle = context
        .create_state::<u16>(blue_button_handle)
        .expect("couldnt insert button state to kayakcontext");
    let cloned_current_button_handle = current_button_handle.clone();

    let button_styles = Style {
        width: StyleProp::Value(Units::Pixels(200.0)),
        height: StyleProp::Value(Units::Pixels(50.0)),
        padding: StyleProp::Value(Edge::all(Units::Stretch(1.0))),
        ..props.styles.clone().unwrap_or_default()
    };

    let on_event = OnEvent::new(move |_ctx, event| match event.event_type {
        EventType::MouseOut(..) | EventType::MouseUp(..) => {
            cloned_current_button_handle.set(blue_button_handle);
        }
        EventType::MouseDown(..) => {
            cloned_current_button_handle.set(blue_button_hover_handle);
        }
        EventType::Click(..) => {
            info!("TODO: Savegame state, but we need a game first");
        }
        _ => (),
    });

    let children = props.get_children();
    rsx! {
        <NinePatch
            border={Edge::all(10.0)}
            handle={current_button_handle.get()}
            styles={Some(button_styles)}
            on_event={Some(on_event)}
        >
            {children}
        </NinePatch>
    }
}

#[widget]
pub fn SettingsButton(props: BlueButtonProps) {
    //settings system todo!
    let (blue_button_handle, blue_button_hover_handle) = {
        let world = context.get_global_mut::<World>();
        if world.is_err() {
            return;
        }

        let mut world = world.expect("World should exist if we are being spawned");

        let (handle1, handle2) = {
            let uitexassets = world
                .get_resource::<UiTextureHandles>()
                .expect("no texture assets?");
            let handle1: Handle<bevy::render::texture::Image> =
                uitexassets.button_blue_pressed_png.clone();
            let handle2: Handle<bevy::render::texture::Image> = uitexassets.button_blue_png.clone();
            (handle1, handle2)
        };

        let mut image_manager = world
            .get_resource_mut::<ImageManager>()
            .expect("couldnt load image manager");
        let blue_button_handle = image_manager.get(&handle1);
        let blue_button_hover_handle = image_manager.get(&handle2);

        (blue_button_handle, blue_button_hover_handle)
    };

    let current_button_handle = context
        .create_state::<u16>(blue_button_handle)
        .expect("couldnt insert button state to kayakcontext");
    let cloned_current_button_handle = current_button_handle.clone();

    let button_styles = Style {
        width: StyleProp::Value(Units::Pixels(200.0)),
        height: StyleProp::Value(Units::Pixels(50.0)),
        padding: StyleProp::Value(Edge::all(Units::Stretch(1.0))),
        ..props.styles.clone().unwrap_or_default()
    };

    let on_event = OnEvent::new(move |_, event| match event.event_type {
        EventType::MouseOut(..) | EventType::MouseUp(..) => {
            cloned_current_button_handle.set(blue_button_handle);
        }
        EventType::MouseDown(..) => {
            cloned_current_button_handle.set(blue_button_hover_handle);
        }
        EventType::Click(..) => {
            info!("should probably make a game before i get a settings menu built.");
        }
        _ => (),
    });

    let children = props.get_children();
    rsx! {
        <NinePatch
            border={Edge::all(10.0)}
            handle={current_button_handle.get()}
            styles={Some(button_styles)}
            on_event={Some(on_event)}
        >
            {children}
        </NinePatch>
    }
}

#[widget]
pub fn ResumeButton(props: BlueButtonProps) {
    let (blue_button_handle, blue_button_hover_handle) = {
        let world = context.get_global_mut::<World>();
        if world.is_err() {
            return;
        }

        let mut world = world.expect("World should exist if we are being spawned");

        let (handle1, handle2) = {
            let uitexassets = world
                .get_resource::<UiTextureHandles>()
                .expect("no texture assets?");
            let handle1: Handle<bevy::render::texture::Image> =
                uitexassets.button_blue_pressed_png.clone();
            let handle2: Handle<bevy::render::texture::Image> = uitexassets.button_blue_png.clone();
            (handle1, handle2)
        };

        let mut image_manager = world
            .get_resource_mut::<ImageManager>()
            .expect("couldnt load image manager");
        let blue_button_handle = image_manager.get(&handle1);
        let blue_button_hover_handle = image_manager.get(&handle2);

        (blue_button_handle, blue_button_hover_handle)
    };

    let current_button_handle = context
        .create_state::<u16>(blue_button_handle)
        .expect("couldnt insert button state to kayakcontext");
    let cloned_current_button_handle = current_button_handle.clone();

    let button_styles = Style {
        width: StyleProp::Value(Units::Pixels(200.0)),
        height: StyleProp::Value(Units::Pixels(50.0)),
        padding: StyleProp::Value(Edge::all(Units::Stretch(1.0))),
        ..props.styles.clone().unwrap_or_default()
    };

    let on_event = OnEvent::new(move |ctx, event| match event.event_type {
        EventType::MouseOut(..) | EventType::MouseUp(..) => {
            cloned_current_button_handle.set(blue_button_handle);
        }
        EventType::MouseDown(..) => {
            cloned_current_button_handle.set(blue_button_hover_handle);
        }
        EventType::Click(..) => {
            ctx.query_world::<EventWriter<PlayButtonEvent>, _, ()>(|mut writer| {
                writer.send(PlayButtonEvent);
            });
        }
        _ => (),
    });

    let children = props.get_children();
    rsx! {
        <NinePatch
            border={Edge::all(10.0)}
            handle={current_button_handle.get()}
            styles={Some(button_styles)}
            on_event={Some(on_event)}
        >
            {children}
        </NinePatch>
    }
}

// #[widget]
// pub fn ButtonTemplate(props: BlueButtonProps) {
//     let (blue_button_handle, blue_button_hover_handle) = {
//         let world = context.get_global_mut::<World>();
//         if world.is_err() {
//             return;
//         }

//         let mut world = world.expect("World should exist if we are being spawned");

//         let (handle1, handle2) = {
//             let uitexassets = world
//                 .get_resource::<UiTextureAssets>()
//                 .expect("no texture assets?");
//             let handle1: Handle<bevy::render::texture::Image> =
//                 uitexassets.button_blue_pressed_png.clone();
//             let handle2: Handle<bevy::render::texture::Image> = uitexassets.button_blue_png.clone();
//             (handle1, handle2)
//         };

//         let mut image_manager = world
//             .get_resource_mut::<ImageManager>()
//             .expect("couldnt load image manager");
//         let blue_button_handle = image_manager.get(&handle1);
//         let blue_button_hover_handle = image_manager.get(&handle2);

//         (blue_button_handle, blue_button_hover_handle)
//     };

//     let current_button_handle = context
//         .create_state::<u16>(blue_button_handle)
//         .expect("couldnt insert button state to kayakcontext");
//     let cloned_current_button_handle = current_button_handle.clone();

//     let button_styles = Style {
//         width: StyleProp::Value(Units::Pixels(200.0)),
//         height: StyleProp::Value(Units::Pixels(50.0)),
//         padding: StyleProp::Value(Edge::all(Units::Stretch(1.0))),
//         ..props.styles.clone().unwrap_or_default()
//     };

//     let on_event = OnEvent::new(move |ctx, event| match event.event_type {
//         EventType::MouseOut(..) => {
//             cloned_current_button_handle.set(blue_button_handle);
//         }
//         EventType::MouseUp(..) => {
//             cloned_current_button_handle.set(blue_button_handle);
//         }
//         EventType::MouseDown(..) => {
//             cloned_current_button_handle.set(blue_button_hover_handle);
//         }
//         EventType::Click(..) => {
//             ctx.query_world::<EventWriter<OptionsButtonEvent>, _, ()>(|mut writer| {
//                 writer.send(OptionsButtonEvent)
//             });
//         }
//         _ => (),
//     });

//     let children = props.get_children();
//     rsx! {
//         <NinePatch
//             border={Edge::all(10.0)}
//             handle={current_button_handle.get()}
//             styles={Some(button_styles)}
//             on_event={Some(on_event)}
//         >
//             {children}
//         </NinePatch>
//     }
// }
