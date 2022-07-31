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
    game::{self, TimeInfo},
    loading::{FontAssets, UiTextureAssets},
    ui::menu_widgets::{ExitButton, PlayButton, SettingsButton},
};

pub struct PlayButtonEvent;
pub struct AppExitEvent;

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

pub fn destroy_menu(mut commands: Commands) {
    commands.remove_resource::<BevyContext>();
}

//if it has pub(crate) fn its probably actually a system and can probably be refactored into a seperate file.
pub fn play_button_event(
    mut reader: EventReader<PlayButtonEvent>,
    mut state: ResMut<bevy::prelude::State<game::GameStage>>,
    mut commands: Commands,
    mut timeinfo: ResMut<TimeInfo>,
) {
    for _ in reader.iter() {
        if *state.current() == game::GameStage::Menu {
            info!("play button was pressed");
            let _ = state.set(game::GameStage::Playing);
        }

        if *state.current() == game::GameStage::Playing {
            info!("resume button pressed");
            commands.remove_resource::<BevyContext>();
            timeinfo.pause_menu = false;
            timeinfo.game_paused = false;
            timeinfo.time_step = 1.0;
        }
    }
}

//if it has pub(crate) fn its probably actually a system and can probably be refactored into a seperate ui systems file or in mod.rs.
pub fn exit_system(
    mut reader: EventReader<AppExitEvent>,
    mut exit: EventWriter<bevy::app::AppExit>,
) {
    for _ in reader.iter() {
        exit.send(bevy::app::AppExit);
        info!("Exiting Game, AppExit Detected");
    }
}
