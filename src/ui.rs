use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_inspector_egui::egui::{RichText, Color32};

use crate::game::{self, GameStage, TimeInfo};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .add_event::<PlayButtonEvent>()
            .add_system_set(
                SystemSet::on_enter(GameStage::Menu)
                    .with_system(configure_visuals)
            )
            .add_system_set(SystemSet::on_update(GameStage::Menu)
                    .with_system(play_button_event)
                    .with_system(left_menu_bar)
            );
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn configure_visuals(mut egui_ctx: ResMut<EguiContext>) {
    let mut fonts = egui::FontDefinitions::default();
    // install your own font (.ttf and .otf supported)
    fonts.font_data.insert(
        "FantasqueSansNF".to_string(),
        egui::FontData::from_static(include_bytes!("../assets/fonts/FantasqueSansMono NF.ttf")),
    );
    // insert it at the beginning for highest priority
    fonts
        .families
        .get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "FantasqueSansNF".to_owned());
    egui_ctx.ctx_mut().set_fonts(fonts);
}

fn left_menu_bar(mut commands: Commands, mut egui_ctx: ResMut<bevy_egui::EguiContext>, windows: ResMut<Windows>) {
    let window = windows.get_primary().expect("window was probably closed before the menu could draw");
    egui::Area::new("my_area")
        .fixed_pos(egui::pos2(window.width() / 2.0, window.height() / 2.0))
        .show(egui_ctx.ctx_mut(), |ui| {
            if ui.button(RichText::new("Play").size(52.0).color(Color32::RED)).clicked() {
                commands.add(|w: &mut World| {
                    let mut events_resource = w.resource_mut::<Events<_>>();
                    events_resource.send(PlayButtonEvent);
                });
            }
            if ui.button(RichText::new("Quit").size(52.0).color(Color32::RED)).clicked() {
                std::process::exit(0);
            }
        });
}

pub struct PlayButtonEvent;

pub fn play_button_event(
    mut reader: EventReader<PlayButtonEvent>,
    mut state: ResMut<bevy::prelude::State<game::GameStage>>,
    mut timeinfo: ResMut<TimeInfo>,
) {
    for _ in reader.iter() {
        if *state.current() == game::GameStage::Menu {
            info!("play button was pressed");
            let _ = state.set(game::GameStage::Playing);
        }

        if *state.current() == game::GameStage::Playing {
            info!("resume button pressed");
            timeinfo.pause_menu = false;
            timeinfo.game_paused = false;
            timeinfo.time_step = 1.0;
        }
    }
}
