use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};

use crate::{components::OnSplashScreen, game::GameStage, utilities::despawn_with};

pub struct BevyUiPlugin;

impl Plugin for BevyUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(despawn_with::<OnSplashScreen>.in_schedule(OnExit(GameStage::Loading)))
            .add_system(despawn_with::<ROOT>.in_schedule(OnExit(GameStage::StartMenu)))
            .add_system(setup.in_schedule(OnEnter(GameStage::StartMenu)))
            .add_system(button_system.in_set(OnUpdate(GameStage::StartMenu)));
    }
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn button_system(
    mut nextstate: ResMut<NextState<GameStage>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                nextstate.set(GameStage::PlaySubStage);
                text.sections[0].value = "Played".to_string();
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                text.sections[0].value = "CLICK ME".to_string();
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                text.sections[0].value = "Play".to_string();
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

#[derive(Component)]
pub struct ROOT;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .insert(ROOT)
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play Button",
                        TextStyle {
                            font: asset_server.load("assets/fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(1.0, 0.2, 0.1),
                        },
                    ));
                });
        });
}
