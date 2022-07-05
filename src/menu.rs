use bevy::prelude::{AssetServer, Commands, Handle, Plugin, Res, ResMut, SystemSet, World};
use kayak_ui::bevy::{BevyContext, BevyKayakUIPlugin, FontMapping, ImageManager, UICameraBundle};
use kayak_ui::core::{
    render, rsx,
    styles::{Edge, LayoutType, Style, StyleProp, Units},
    widget, Bound, Children, EventType, MutableBound, OnEvent, WidgetProps,
};
use kayak_ui::widgets::{App, NinePatch, Text};

use crate::{
    loading::{FontAssets, UiTextureAssets},
    GameState,
};

#[derive(WidgetProps, Clone, Debug, Default, PartialEq)]
struct BlueButtonProps {
    #[prop_field(Styles)]
    styles: Option<Style>,
    #[prop_field(Children)]
    children: Option<Children>,
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugin(BevyKayakUIPlugin)
            .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(startup));
    }
}

#[widget]
fn BlueButton(props: BlueButtonProps) {
    let (blue_button_handle, blue_button_hover_handle) = {
        let world = context.get_global_mut::<World>();
        if world.is_err() {
            return;
        }

        let mut world = world.unwrap();

        let (handle1, handle2) = {
            let asset_server = world.get_resource::<AssetServer>().unwrap();
            let handle1: Handle<bevy::render::texture::Image> =
                asset_server.load("../assets/kenny/buttonSquare_blue_pressed.png");
            let handle2: Handle<bevy::render::texture::Image> =
                asset_server.load("../assets/kenny/buttonSquare_blue.png");

            (handle1, handle2)
        };

        let mut image_manager = world.get_resource_mut::<ImageManager>().unwrap();
        let blue_button_handle = image_manager.get(&handle1);
        let blue_button_hover_handle = image_manager.get(&handle2);

        (blue_button_handle, blue_button_hover_handle)
    };

    let current_button_handle = context.create_state::<u16>(blue_button_handle).unwrap();

    let button_styles = Style {
        width: StyleProp::Value(Units::Pixels(200.0)),
        height: StyleProp::Value(Units::Pixels(50.0)),
        padding: StyleProp::Value(Edge::all(Units::Stretch(1.0))),
        ..props.styles.clone().unwrap_or_default()
    };

    let cloned_current_button_handle = current_button_handle.clone();
    let on_event = OnEvent::new(move |_, event| match event.event_type {
        EventType::MouseIn(..) => {
            cloned_current_button_handle.set(blue_button_hover_handle);
        }
        EventType::MouseOut(..) => {
            cloned_current_button_handle.set(blue_button_handle);
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

fn startup(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    ui_assets: Res<UiTextureAssets>,
    mut image_manager: ResMut<ImageManager>,
    mut font_mapping: ResMut<FontMapping>,
) {
    commands.spawn_bundle(UICameraBundle::new());

    let main_font = font_assets.fira_sans_msdf.clone();
    font_mapping.add("FiraSans-Bold", main_font.clone());

    // let handle: Handle<bevy::render::texture::Image> = asset_server.load("kenny/panel_brown.png");
    let panel_brown_handle = image_manager.get(&ui_assets.panel_brown_png);

    let context = BevyContext::new(|context| {
        let nine_patch_styles = Style {
            layout_type: StyleProp::Value(LayoutType::Column),
            width: StyleProp::Value(Units::Pixels(512.0)),
            height: StyleProp::Value(Units::Pixels(512.0)),
            left: StyleProp::Value(Units::Stretch(1.0)),
            right: StyleProp::Value(Units::Stretch(1.0)),
            top: StyleProp::Value(Units::Stretch(1.0)),
            bottom: StyleProp::Value(Units::Stretch(1.0)),
            padding: StyleProp::Value(Edge::all(Units::Stretch(1.0))),
            ..Style::default()
        };

        let header_styles = Style {
            bottom: StyleProp::Value(Units::Stretch(1.0)),
            ..Style::default()
        };

        let options_button_styles = Style {
            top: StyleProp::Value(Units::Pixels(15.0)),
            ..Style::default()
        };

        let main_font_id = font_mapping.get(&main_font);

        render! {
            <App>
                <NinePatch
                    styles={Some(nine_patch_styles)}
                    border={Edge::all(30.0)}
                    handle={panel_brown_handle}
                >
                    <Text
                        styles={Some(header_styles)}
                        size={35.0}
                        content={"Vanilla Coffee".to_string()}
                        font={main_font_id}
                    />
                    <BlueButton>
                        <Text line_height={Some(50.0)} size={20.0} content={"Play".to_string()} font={main_font_id} />
                    </BlueButton>
                    <BlueButton styles={Some(options_button_styles)}>
                        <Text line_height={Some(50.0)} size={20.0} content={"Options".to_string()} font={main_font_id} />
                    </BlueButton>
                    <BlueButton styles={Some(options_button_styles)}>
                        <Text line_height={Some(50.0)} size={20.0} content={"Quit".to_string()} font={main_font_id} />
                    </BlueButton>
                </NinePatch>
            </App>
        }
    });

    commands.insert_resource(context);
}

// use crate::loading::FontAssets;
// use crate::GameState;
// use bevy::prelude::*;

// pub struct MenuPlugin;

// /// This plugin is responsible for the game menu (containing only one button...)
// /// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
// impl Plugin for MenuPlugin {
//     fn build(&self, app: &mut App) {
//         app.init_resource::<ButtonColors>()
//             .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(setup_menu))
//             .add_system_set(SystemSet::on_update(GameState::Menu).with_system(click_play_button));
//     }
// }

// struct ButtonColors {
//     normal: UiColor,
//     hovered: UiColor,
// }

// impl Default for ButtonColors {
//     fn default() -> Self {
//         ButtonColors {
//             normal: Color::rgb(0.15, 0.15, 0.15).into(),
//             hovered: Color::rgb(0.25, 0.25, 0.25).into(),
//         }
//     }
// }

// fn setup_menu(
//     mut commands: Commands,
//     font_assets: Res<FontAssets>,
//     button_colors: Res<ButtonColors>,
// ) {
//     commands
//         .spawn_bundle(ButtonBundle {
//             style: Style {
//                 size: Size::new(Val::Px(120.0), Val::Px(50.0)),
//                 margin: Rect::all(Val::Auto),
//                 justify_content: JustifyContent::Center,
//                 align_items: AlignItems::Center,
//                 ..Default::default()
//             },
//             color: button_colors.normal,
//             ..Default::default()
//         })
//         .with_children(|parent| {
//             parent.spawn_bundle(TextBundle {
//                 text: Text {
//                     sections: vec![TextSection {
//                         value: "Play".to_string(),
//                         style: TextStyle {
//                             font: font_assets.fira_sans.clone(),
//                             font_size: 40.0,
//                             color: Color::rgb(0.9, 0.9, 0.9),
//                         },
//                     }],
//                     alignment: Default::default(),
//                 },
//                 ..Default::default()
//             });
//         });

//         commands
//         .spawn_bundle(ButtonBundle {
//             style: Style {
//                 size: Size::new(Val::Px(120.0), Val::Px(50.0)),
//                 margin: Rect::all(Val::Auto),
//                 justify_content: JustifyContent::Center,
//                 align_items: AlignItems::Center,
//                 ..Default::default()
//             },
//             color: button_colors.normal,
//             ..Default::default()
//         })
//         .with_children(|parent| {
//             parent.spawn_bundle(TextBundle {
//                 text: Text {
//                     sections: vec![TextSection {
//                         value: "Settings".to_string(),
//                         style: TextStyle {
//                             font: font_assets.fira_sans.clone(),
//                             font_size: 40.0,
//                             color: Color::rgb(0.9, 0.9, 0.9),
//                         },
//                     }],
//                     alignment: Default::default(),
//                 },
//                 ..Default::default()
//             });
//         });

// }

// fn click_play_button(
//     mut commands: Commands,
//     button_colors: Res<ButtonColors>,
//     mut state: ResMut<State<GameState>>,
//     mut interaction_query: Query<
//         (Entity, &Interaction, &mut UiColor),
//         (Changed<Interaction>, With<Button>),
//     >,
// ) {
//     for (button, interaction, mut color) in interaction_query.iter_mut() {
//         match *interaction {
//             Interaction::Clicked => {
//                 commands.entity(button).despawn_recursive();
//                 state.set(GameState::Playing).unwrap();
//             }
//             Interaction::Hovered => {
//                 *color = button_colors.hovered;
//             }
//             Interaction::None => {
//                 *color = button_colors.normal;
//             }
//         }
//     }
// }
