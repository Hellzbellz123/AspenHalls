use bevy::prelude::{Commands, Res, ResMut, *};
use kayak_ui::{
    bevy::{BevyContext, FontMapping, ImageManager, UICameraBundle},
    core::{
        render,
        styles::{Edge, LayoutType, Style, StyleProp, Units},
    },
    widgets::{App, NinePatch, Text},
};

use crate::{
    loading::assets::{FontHandles, UiTextureHandles},
    ui::menu_widgets::{ExitButton, PlayButton, SettingsButton},
};

pub(crate) fn draw_mainmenu(
    mut commands: Commands,
    font_assets: Res<FontHandles>,
    ui_assets: Res<UiTextureHandles>,
    mut image_manager: ResMut<ImageManager>,
    mut font_mapping: ResMut<FontMapping>,
) {
    commands
        .spawn_bundle(UICameraBundle::new())
        .insert(Name::new("UI Camera config"));

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
            <PlayButton>
            <Text line_height={Some(40.0)} size={32.0} content={"Play".to_string()} font={main_font_id} />
            </PlayButton>
            <SettingsButton styles={Some(options_button_styles)}>
            <Text line_height={Some(40.0)} size={26.0} content={"Options".to_string()} font={main_font_id} />
            </SettingsButton>
            <ExitButton styles={Some(options_button_styles)}>
            <Text line_height={Some(40.0)} size={24.0} content={"Exit Game".to_string()} font={main_font_id} />
            </ExitButton>
            </NinePatch>
            </App>
        }
    });
    commands.insert_resource(context);
}
