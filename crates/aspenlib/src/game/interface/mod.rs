use crate::game::AppState;
use bevy::prelude::*;
use rand::Rng;

/// pause menu module
pub mod pause_menu;
/// playing game ui
pub mod playing_ui;
/// game configuration menu
pub mod settings_menu;
/// start menu module
pub mod start_menu;
/// resuseable ui parts
pub mod ui_widgets;

/// ui plugin
pub struct InterfacePlugin;

/// simple marker component
#[derive(Debug, Component)]
pub struct InterfaceRootTag;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            // start menu
            start_menu::StartMenuPlugin,
            // pause menu
            pause_menu::PauseMenuPlugin,
            // settings
            settings_menu::SettingsMenuPlugin,
            //playing ui
            playing_ui::PlayingUiPlugin,
        ));
        app.add_systems(OnEnter(AppState::BootingApp), (spawn_interface_root,));
        app.add_systems(Update, (update_button_color,));
    }
}

/// spawns entity that all UI is parented under
fn spawn_interface_root(mut cmds: Commands) {
    cmds.spawn((
        Name::new("InterfaceRoot"),
        InterfaceRootTag,
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                // extra
                // justify_content: JustifyContent::FlexStart,
                flex_direction: FlexDirection::Column,
                flex_wrap: FlexWrap::NoWrap,
                margin: UiRect::all(Val::Auto),
                ..default()
            },
            ..default()
        },
    ));
}

/// unpressed unhovered color
const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
/// cursor goes over or finger drags over color
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
/// cursor click or finger lift color
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

/// updates color of all buttons with text for interactions
#[allow(clippy::type_complexity)]
fn update_button_color(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = crate::colors::RED.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = crate::colors::WHITE.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = crate::colors::BLACK.into();
            }
        }
    }
}

/// generated random Rgba color with alpha between 0.8-1.0
pub fn random_color(alpha: Option<f32>) -> Color {
    let mut rng = rand::thread_rng();
    Color::Srgba(Srgba {
        red: rng.gen(),
        green: rng.gen(),
        blue: rng.gen(),
        alpha: { alpha.map_or_else(|| rng.gen_range(0.8..=1.0), |alpha| alpha) },
    })
}
