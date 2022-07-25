use bevy::prelude::{Commands, Plugin, Res, ResMut, SystemSet, *};
use kayak_ui::bevy::{BevyContext, BevyKayakUIPlugin, FontMapping, ImageManager, UICameraBundle};
use kayak_ui::core::{
    render,
    styles::{Edge, LayoutType, Style, StyleProp, Units},
};

use kayak_ui::widgets::{App, NinePatch, Text};

use crate::{
    loading::{FontAssets, UiTextureAssets},
    menu::menu_widgets::{BlueButton, BluePlayButton},
    GameState,
};
pub struct PlayButtonEvent;

pub(crate) fn startup(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    ui_assets: Res<UiTextureAssets>,
    mut image_manager: ResMut<ImageManager>,
    mut font_mapping: ResMut<FontMapping>,
) {
    commands.spawn_bundle(UICameraBundle::new());

    let main_font = font_assets.fira_sans_msdf.clone();
    let title_font = font_assets.fantasque_sans_msdf.clone();

    font_mapping.add("FiraSans-Bold", main_font.clone());
    font_mapping.add("FantasqueSansNF", title_font.clone());

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
        let title_font_id = font_mapping.get(&title_font);

        render! {
            <App>
                <NinePatch
                    styles={Some(nine_patch_styles)}
                    border={Edge::all(30.0)}
                    handle={panel_brown_handle}
                    >
                    <Text
                        styles={Some(header_styles)}
                        size={78.0}
                        content={"Vanilla Coffee".to_string()}
                        font={title_font_id}
                    />
                    <BluePlayButton>
                        <Text line_height={Some(40.0)} size={32.0} content={"Play".to_string()} font={main_font_id} />
                    </BluePlayButton>
                    <BlueButton styles={Some(options_button_styles)}>
                        <Text line_height={Some(40.0)} size={26.0} content={"Options".to_string()} font={main_font_id} />
                    </BlueButton>
                    <BlueButton styles={Some(options_button_styles)}>
                        <Text line_height={Some(40.0)} size={24.0} content={"Exit Game".to_string()} font={main_font_id} />
                    </BlueButton>
                </NinePatch>
            </App>
        }
    });
    commands.insert_resource(context);
}

pub(crate) fn destroy(mut commands: Commands) {
    commands.remove_resource::<BevyContext>();
}

pub(crate) fn play_button_event(
    mut reader: EventReader<PlayButtonEvent>,
    mut state: ResMut<bevy::prelude::State<crate::GameState>>,
) {
    for _ in reader.iter() {
        println!("play button was pressed");
        if *state.current() == crate::GameState::Menu {
            let _ = state.set(crate::GameState::Playing);
        }
    }
}
