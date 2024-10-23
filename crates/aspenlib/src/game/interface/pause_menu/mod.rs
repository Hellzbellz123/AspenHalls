use crate::{
    game::{
        input::action_maps,
        interface::{
            random_color,
            settings_menu::SettingsMenuToggleButton,
            ui_widgets::{spawn_button, spawn_menu_title},
            InterfaceRootTag,
        },
        AppState,
    },
    loading::assets::AspenInitHandles,
};
use bevy::app::AppExit;
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

/// pause game functionality and pause menu ui
pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EventTogglePause>();
        app.add_systems(OnExit(AppState::Loading), spawn_pause_menu);
        app.add_systems(
            Update,
            (
                continue_button_interaction,
                exit_button_interaction,
                pause_event_handler.run_if(on_event::<EventTogglePause>()),
                keyboard_pause_sender,
            ),
        );
    }
}

/// Start menu marker component for querys
#[derive(Component)]
pub struct PauseMenuTag;

/// marks start button for query
#[derive(Debug, Component)]
pub struct ContinueGameTag;

/// marks start button for query
#[derive(Debug, Component)]
pub struct ExitGameTag;

/// spawns start menu with buttons
fn spawn_pause_menu(
    mut cmds: Commands,
    assets: Res<AspenInitHandles>,
    interface_root: Query<Entity, With<InterfaceRootTag>>,
) {
    cmds.entity(interface_root.single())
        .with_children(|children| {
            children
                .spawn((
                    Name::new("PauseMenu"),
                    PauseMenuTag,
                    NodeBundle {
                        style: Style {
                            display: Display::None,
                            position_type: PositionType::Absolute,
                            overflow: Overflow::clip(),
                            flex_direction: FlexDirection::Column,
                            min_height: Val::Percent(60.0),
                            min_width: Val::Percent(30.0),
                            // aspect_ratio: Some(0.8),
                            align_self: AlignSelf::Center,
                            justify_content: JustifyContent::FlexStart,
                            margin: UiRect {
                                left: Val::Percent(40.0),
                                right: Val::Px(0.0),
                                top: Val::Percent(10.0),
                                bottom: Val::Percent(10.0),
                            },
                            ..default()
                        },
                        background_color: BackgroundColor(random_color(Some(0.8))),
                        ..default()
                    },
                ))
                .with_children(|start_menu_container_childs| {
                    spawn_menu_title(
                        start_menu_container_childs,
                        assets.font_title.clone(),
                        "Pause Menu",
                    );
                    start_menu_container_childs
                        .spawn((
                            Name::new("ButtonContainer"),
                            NodeBundle {
                                style: Style {
                                    position_type: PositionType::Relative,
                                    flex_direction: FlexDirection::Column,
                                    justify_content: JustifyContent::SpaceEvenly,
                                    align_items: AlignItems::Center,
                                    width: Val::Percent(70.0),
                                    height: Val::Percent(70.0),
                                    // min_height: Val::Percent(20.0),
                                    // max_height: Val::Percent(85.0),
                                    margin: UiRect {
                                        left: Val::Auto,
                                        right: Val::Auto,
                                        top: Val::Px(5.0),
                                        bottom: Val::Px(15.0),
                                    },
                                    ..default()
                                },
                                border_color: BorderColor(random_color(None)),
                                ..default()
                            },
                        ))
                        .with_children(|buttons| {
                            spawn_button(
                                buttons,
                                assets.font_regular.clone(),
                                "Continue Game",
                                ContinueGameTag,
                            );
                            spawn_button(
                                buttons,
                                assets.font_regular.clone(),
                                "Settings",
                                SettingsMenuToggleButton,
                            );
                            spawn_button(
                                buttons,
                                assets.font_regular.clone(),
                                "Exit Game",
                                ExitGameTag,
                            );
                        });
                });
        });
}

/// send `EventTogglePause` request when pause menu continue button is pressed
fn continue_button_interaction(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<ContinueGameTag>)>,
    mut pauses: EventWriter<EventTogglePause>,
) {
    for interaction in &interaction_query {
        if matches!(interaction, Interaction::Pressed) {
            pauses.send(EventTogglePause);
        }
    }
}

/// send quit game request
fn exit_button_interaction(
    mut exit_event_writer: EventWriter<AppExit>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<ExitGameTag>)>,
) {
    for interaction in &interaction_query {
        if matches!(interaction, Interaction::Pressed) {
            exit_event_writer.send(AppExit::Success);
        }
    }
}

#[derive(Debug, Event)]
/// toggle game pause state
pub struct EventTogglePause;

/// send pause requests when pause button is pressed
fn keyboard_pause_sender(
    input: Res<ActionState<action_maps::Gameplay>>,
    mut pauses: EventWriter<EventTogglePause>,
) {
    if input.just_pressed(&action_maps::Gameplay::Pause) {
        pauses.send(EventTogglePause);
    }
}

/// takes pause requests and does things too pause game
fn pause_event_handler(
    mut pauses: EventReader<EventTogglePause>,
    game_state: Res<State<AppState>>,
    mut cmds: Commands,
    mut pause_menu_query: Query<&mut Style, (With<Node>, With<PauseMenuTag>)>,
    // mut rapier_cfg: Query<&mut RapierConfiguration>,
) {
    // let mut rapier_cfg = rapier_cfg.single_mut();

    for _event in pauses.read() {
        match game_state.get() {
            AppState::PlayingGame => {
                // rapier_cfg.timestep_mode = TimestepMode::Variable {
                //     max_dt: 1.0 / 144.0,
                //     time_scale: 0.0,
                //     substeps: 1,
                // };
                pause_menu_query.single_mut().display = Display::Flex;
                cmds.insert_resource(NextState::Pending(AppState::PauseMenu));
            }
            AppState::PauseMenu => {
                // rapier_cfg.timestep_mode = TimestepMode::Variable {
                //     max_dt: 1.0 / 144.0,
                //     time_scale: 1.0,
                //     substeps: 1,
                // };
                pause_menu_query.single_mut().display = Display::None;
                cmds.insert_resource(NextState::Pending(AppState::PlayingGame));
            }
            _ => {}
        }
    }
}
