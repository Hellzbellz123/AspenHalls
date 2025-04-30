use crate::{
    game::{
        characters::player::PlayerSelectedHero,
        game_world::{dungeonator_v2::components::Dungeon, hideout::systems::HideoutTag},
        input::action_maps,
        interface::{
            random_color,
            settings_menu::SettingsMenuToggleButton,
            ui_widgets::{spawn_button, spawn_menu_title},
            InterfaceRootTag,
        },
    },
    loading::{
        assets::{AspenInitHandles, AspenLevelsetHandles},
        registry::RegistryIdentifier,
    },
    AppStage, GameStage,
};
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_ecs_ldtk::{LdtkProjectHandle, LdtkWorldBundle, LevelSet};
use leafwing_input_manager::prelude::ActionState;

/// pause game functionality and pause menu ui
pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EventTogglePause>();
        app.add_systems(OnEnter(AppStage::Starting), spawn_pause_menu);
        app.add_systems(
            Update,
            (
                (
                    back_to_main_menu_interaction,
                    abandon_button_interaction,
                    continue_button_interaction,
                    exit_button_interaction,
                )
                    .run_if(in_state(GameStage::PausedGame)),
                keyboard_pause_sender,
                pause_menu_visibility.run_if(state_changed::<GameStage>),
                pause_event_handler.run_if(on_event::<EventTogglePause>),
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

/// marks abandon dungeon button for query
#[derive(Debug, Component)]
pub struct AbandonDungeonTag;

/// marks return too main menu button for query
#[derive(Debug, Component)]
pub struct BackToMainMenuTag;

/// spawns start menu with buttons
fn spawn_pause_menu(
    mut cmds: Commands,
    assets: Res<AspenInitHandles>,
    interface_root: Query<Entity, With<InterfaceRootTag>>,
) {
    let Ok(interface_root) = interface_root.single() else {
        return;
    };

    cmds.entity(interface_root)
        .with_children(|children| {
            children
                .spawn((
                    Name::new("PauseMenu"),
                    PauseMenuTag,
                    BackgroundColor(random_color(Some(0.8))),
                    Node {
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
                            top: Val::Px(50.0),
                            bottom: Val::Auto,
                        },
                        padding: UiRect::all(Val::Px(0.0)).with_top(Val::Px(5.0)),
                        ..default()
                    },
                ))
                .with_children(|start_menu_container_childs| {
                    spawn_menu_title(
                        start_menu_container_childs,
                        assets.font_title.clone(),
                        "Pause Menu",
                        48.0,
                    );
                    start_menu_container_childs
                        .spawn((
                            Name::new("ButtonContainer"),
                            Node {
                                flex_direction: FlexDirection::Column,
                                position_type: PositionType::Relative,
                                align_items: AlignItems::Center,
                                row_gap: Val::Px(15.0),
                                margin: UiRect {
                                    left: Val::Auto,
                                    right: Val::Auto,
                                    top: Val::Px(15.0),
                                    bottom: Val::Px(15.0),
                                },
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
                                "Back to Hideout",
                                AbandonDungeonTag,
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
                                "Main Menu",
                                BackToMainMenuTag,
                            );
                            #[cfg(not(target_family = "wasm"))]
                            spawn_button(
                                buttons,
                                assets.font_regular.clone(),
                                "Quit Game",
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
            pauses.write(EventTogglePause);
        }
    }
}

/// send `EventTogglePause` request when pause menu continue button is pressed
fn abandon_button_interaction(
    mut cmds: Commands,
    mut time: ResMut<Time<Virtual>>,
    mut next_state: ResMut<NextState<GameStage>>,
    maps: Res<AspenLevelsetHandles>,
    level_q: Query<Entity, With<Dungeon>>,
    hideout_q: Query<Entity, With<HideoutTag>>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<AbandonDungeonTag>)>,
    actor_q: Query<
        Entity,
        (
            With<RegistryIdentifier>,
            Without<PlayerSelectedHero>,
            Without<ChildOf>,
        ),
    >,
) {
    for interaction in &interaction_query {
        if matches!(interaction, Interaction::Pressed) {
            warn!("abandoning dungeon");
            for actor in &actor_q {
                cmds.entity(actor).despawn();
            }
            for level in &level_q {
                cmds.entity(level).despawn();
            }
            time.unpause();

            // TODO: most recent todo tag doesnt reflect my thoughts/feelings correctly
            // the below code should not be allowed by style.
            // this is a hack to fix a random issue that should not exist
            if hideout_q.is_empty() {
                cmds.spawn((
                    LdtkWorldBundle {
                        ldtk_handle: LdtkProjectHandle::from(maps.default_levels.clone()),
                        level_set: LevelSet::default(),
                        transform: Transform {
                            translation: Vec3 {
                                x: 0.0,
                                y: 0.0,
                                z: 0.0,
                            },
                            scale: Vec3 {
                                x: 1.0,
                                y: 1.0,
                                z: 1.0,
                            },
                            ..default()
                        },
                        ..default()
                    },
                    Name::new("HideOut"),
                    HideoutTag,
                ));
            }

            next_state.set(GameStage::PlayingGame);
        }
    }
}

fn back_to_main_menu_interaction(
    mut cmds: Commands,
    mut time: ResMut<Time<Virtual>>,
    mut game_stage: ResMut<NextState<AppStage>>,
    actor_q: Query<Entity, With<RegistryIdentifier>>,
    level_q: Query<Entity, With<LdtkProjectHandle>>,
    ui_root_q: Query<Entity, (With<Node>, With<InterfaceRootTag>)>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<BackToMainMenuTag>)>,
) {
    for interaction in &interaction_query {
        if matches!(interaction, Interaction::Pressed) {
            for actor in &actor_q {
                cmds.entity(actor).despawn();
            }
            for level in &level_q {
                cmds.entity(level).despawn();
            }
            for entity in &ui_root_q {
                // TODO: this might not be correct anymore
                cmds.entity(entity).despawn_related::<Children>();
            }

            time.unpause();
            game_stage.set(AppStage::Starting);
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
            exit_event_writer.write(AppExit::Success);
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
        pauses.write(EventTogglePause);
    }
}

fn pause_menu_visibility(
    game_state: Option<Res<State<GameStage>>>,
    mut pause_menu_query: Query<&mut Node, With<PauseMenuTag>>,
) {
    let Some(game_state) = game_state else {
        return;
    };
    let Ok(mut pause_menu) = pause_menu_query.single_mut() else {
        return;
    };

    match game_state.get() {
        GameStage::PausedGame => pause_menu.display = Display::Flex,
        _ => pause_menu.display = Display::None,
    }
}

/// takes pause requests and does things too pause game
fn pause_event_handler(
    mut pauses: EventReader<EventTogglePause>,
    // mut pause_menu_query: Query<&mut Style, (With<Node>, With<PauseMenuTag>)>,
    mut time: ResMut<Time<Virtual>>,
    game_state: Option<Res<State<GameStage>>>,
    mut next_state: ResMut<NextState<GameStage>>,
) {
    for _event in pauses.read() {
        let Some(game_state) = game_state.as_ref() else {
            return;
        };

        match game_state.get() {
            GameStage::PlayingGame => {
                time.pause();
                // pause_menu_query.single_mut().display = Display::Flex;
                next_state.set(GameStage::PausedGame);
            }
            GameStage::PausedGame => {
                time.unpause();
                // pause_menu_query.single_mut().display = Display::None;
                next_state.set(GameStage::PlayingGame);
            }
            _ => {}
        }
    }
}

// match game_state {
//     Some(state) => {
//         match state.get() {
//             GameStage::StartMenu => {
//                 // do the continue event?
//             },
//             GameStage::SelectCharacter => {
//                 // pause here may break stuff?
//             },
//             GameStage::PlayingGame => {

//             },
//             GameStage::PausedGame => todo!(),
//         }
//     },
//     None => {
//         return;
//     },
// }
