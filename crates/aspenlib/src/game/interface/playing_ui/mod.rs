use crate::{
    game::interface::InterfaceRootTag, loading::assets::AspenTouchHandles, register_types, AppState,
};
use bevy::prelude::*;

/// player actions ui widgets
mod action_buttons;

/// player weapon ui widgets
pub mod gun_hud;

/// player vital ui widgets
mod stat_hud;

/// in game ui elements module
pub struct PlayingUiPlugin;

impl Plugin for PlayingUiPlugin {
    fn build(&self, app: &mut App) {
        register_types!(app, [stat_hud::StatBar, gun_hud::PlayerAmmoBar]);
        app.add_systems(OnExit(AppState::Loading), spawn_playing_ui)
            .add_systems(
                Update,
                ((
                    toggle_playing_ui.run_if(state_changed::<AppState>),
                    (
                        stat_hud::update_player_hp_bar,
                        gun_hud::update_ui_ammo_counter,
                        gun_hud::update_ui_ammo_slots,
                    )
                        .run_if(in_state(AppState::PlayingGame)),
                ),),
            )
            .add_systems(
                OnExit(AppState::StartMenu),
                stat_hud::update_player_portrait,
            );
    }
}

#[derive(Component)]
/// marker component for playing ui container element
pub struct PlayingUiTag;

/// toggles visibility of gameplay ui elements
fn toggle_playing_ui(
    mut playing_ui_query: Query<&mut Style, (With<Node>, With<PlayingUiTag>)>,
    game_state: Res<State<AppState>>,
) {
    let Ok(mut playing_ui_style) = playing_ui_query.get_single_mut() else {
        return;
    };
    if game_state.get() == &AppState::PlayingGame && playing_ui_style.display != Display::Flex {
        playing_ui_style.display = Display::Flex;
    } else {
        playing_ui_style.display = Display::None;
    }
}

/// spawns start menu with buttons
fn spawn_playing_ui(
    mut cmds: Commands,
    // assets: Res<AspenInitHandles>,
    touch_assets: Res<AspenTouchHandles>,
    interface_root: Query<Entity, With<InterfaceRootTag>>,
) {
    cmds.entity(interface_root.single())
        .with_children(|children| {
            children
                .spawn((
                    Name::new("PlayingUi"),
                    PlayingUiTag,
                    NodeBundle {
                        style: Style {
                            display: Display::None,
                            position_type: PositionType::Absolute,
                            flex_direction: FlexDirection::Row,
                            margin: UiRect::all(Val::Px(0.0)),
                            height: Val::Percent(100.0),
                            width: Val::Percent(100.0),
                            align_self: AlignSelf::Center,
                            ..default()
                        },
                        ..default()
                    },
                ))
                .with_children(|playing_ui_parts| {
                    // TODO: update portrait based on selected player
                    gun_hud::create_gun_hud(playing_ui_parts);
                    create_hud_container(playing_ui_parts, touch_assets);
                });
        });
}

/// create portrait and hp and action buttons container
fn create_hud_container(hud_hud_parts: &mut ChildBuilder, touch_assets: Res<AspenTouchHandles>) {
    hud_hud_parts
        .spawn((
            Name::new("HudContainer"),
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Row,
                    margin: UiRect {
                        left: Val::default(),
                        right: Val::default(),
                        top: Val::default(),
                        bottom: Val::Px(10.0),
                    },
                    height: Val::Percent(30.0),
                    width: Val::Percent(50.0),
                    align_self: AlignSelf::Center,

                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|playing_ui_parts| {
            stat_hud::create_player_hud(playing_ui_parts);
            action_buttons::create_action_buttons(playing_ui_parts, touch_assets);
        });
}

/// ui preset colors for no rainbow
mod colors {
    use bevy::prelude::Color;

    /// background color dark
    pub const BACKDARK: Color = Color::Rgba {
        red: 0.125,
        green: 0.125,
        blue: 0.125,
        alpha: 0.95,
    };

    /// background color light
    pub const BACKLIGHT: Color = Color::Rgba {
        red: 0.225,
        green: 0.225,
        blue: 0.225,
        alpha: 0.85,
    };

    /// ui accent color
    pub const ACCENT: Color = Color::Rgba {
        red: 0.425,
        green: 0.225,
        blue: 0.425,
        alpha: 0.85,
    };

    /// ui highlight color
    pub const HIGHLIGHT: Color = Color::Rgba {
        red: 0.94,
        green: 0.97,
        blue: 1.0,
        alpha: 1.0,
    };

    /// ui outline color
    pub const OUTLINE: Color = Color::Rgba {
        red: 0.0,
        green: 0.0,
        blue: 0.0,
        alpha: 1.0,
    };

    /// hp full color
    pub const HPFULL: Color = Color::Rgba {
        red: 0.0,
        green: 1.0,
        blue: 0.0,
        alpha: 1.0,
    };

    /// hp empty color
    pub const HPEMPTY: Color = Color::Rgba {
        red: 1.0,
        green: 0.0,
        blue: 0.0,
        alpha: 1.0,
    };

    /// mana full color
    pub const MANAFULL: Color = Color::Rgba {
        red: 0.0,
        green: 0.0,
        blue: 1.0,
        alpha: 1.0,
    };

    /// mana empty color
    pub const UTILITYEMPTY: Color = Color::Rgba {
        red: 0.49,
        green: 1.0,
        blue: 0.83,
        alpha: 1.0,
    };
}
