use bevy::prelude::{Commands, Res, ResMut, *};
use bevy_inspector_egui::Inspectable;
use kayak_ui::bevy::{BevyContext, FontMapping, ImageManager, UICameraBundle};
use kayak_ui::core::{
    render,
    styles::{Edge, LayoutType, Style, StyleProp, Units},
};
use kayak_ui::widgets::{App, NinePatch, Text};
use leafwing_input_manager::prelude::ActionState;

use crate::action_manager::actions::GameActions;
use crate::player::Player;
use crate::{
    loading::{FontAssets, UiTextureAssets},
    menu::menu_widgets::{ExitButton, IngameOptionsButton, PlayButton, SaveButton},
};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Component, Inspectable)]
pub enum PauseMenuState {
    Paused,
    Options,
    Closing,
    Closed,
}

pub fn listen_for_pause_event(
    input_query: Query<&ActionState<GameActions>, With<Player>>,
    commands: Commands,
    font_assets: Res<FontAssets>,
    ui_assets: Res<UiTextureAssets>,
    image_manager: ResMut<ImageManager>,
    font_mapping: ResMut<FontMapping>,
) {
    let action_state = input_query.single();

    if action_state.just_pressed(GameActions::Pause) {
        info!("pause button pressed");
        spawn_menu(
            commands,
            font_assets,
            ui_assets,
            image_manager,
            font_mapping,
        )
    }
}

fn spawn_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    ui_assets: Res<UiTextureAssets>,
    mut image_manager: ResMut<ImageManager>,
    mut font_mapping: ResMut<FontMapping>,
) {
    let main_font = font_assets.fira_sans_msdf.clone();
    let title_font = font_assets.fantasque_sans_msdf.clone();

    font_mapping.add("FiraSans-Bold", main_font.clone());
    font_mapping.add("FantasqueSansNF", title_font.clone());

    let panel_brown_handle = image_manager.get(&ui_assets.panel_brown_png);

    let context = BevyContext::new(|context| {
        let nine_patch_styles = Style {
            layout_type: StyleProp::Value(LayoutType::Column),
            width: StyleProp::Value(Units::Pixels(250.0)),
            height: StyleProp::Value(Units::Pixels(512.0)),
            left: StyleProp::Value(Units::Stretch(0.2)),
            right: StyleProp::Value(Units::Stretch(2.8)),
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
                    <kayak_ui::widgets::>
                    <kayak_ui::widgets::/>
                    <PlayButton>
                        <Text line_height={Some(40.0)} size={32.0} content={"Resume".to_string()} font={main_font_id} />
                    </PlayButton>
                    <SaveButton styles={Some(options_button_styles)}>
                        <Text line_height={Some(40.0)} size={26.0} content={"Save Game".to_string()} font={main_font_id} />
                    </SaveButton>
                    <IngameOptionsButton styles={Some(options_button_styles)}>
                        <Text line_height={Some(40.0)} size={26.0} content={"Options".to_string()} font={main_font_id} />
                    </IngameOptionsButton>
                    <ExitButton styles={Some(options_button_styles)}>
                        <Text line_height={Some(40.0)} size={24.0} content={"Exit Game".to_string()} font={main_font_id} />
                    </ExitButton>
                </NinePatch>
            </App>
        }
    });
    commands.insert_resource(context);
}
