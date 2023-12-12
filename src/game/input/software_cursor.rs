use bevy::{
    ecs::{
        component::Component,
        schedule::common_conditions::not,
        system::Res,
    },
    log::warn,
    math::Vec2,
    prelude::{any_with_component, ImageBundle, PositionType, Reflect},
    ui::{Node, Style, Val, ZIndex},
};

use crate::ahp::{
    engine::{
        bevy, default, leafwing_input_manager::action_state::ActionState, Commands,
        IntoSystemConfigs, Name, OnEnter, Plugin, PreUpdate, Query, With,
    },
    game::{action_maps, AppState, InitAssetHandles, Player},
};

use super::AspenInputSystemSet;

/// adds software cursor functionality too app
pub struct SoftwareCursorPlugin;

impl Plugin for SoftwareCursorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<SoftWareCursor>();
        app.add_systems(
            OnEnter(AppState::Loading),
            spawn_software_cursor.run_if(not(any_with_component::<SoftWareCursor>())),
        );

        app.add_systems(
            PreUpdate,
            update_software_cursor_position
                .run_if(any_with_component::<SoftWareCursor>())
                .in_set(AspenInputSystemSet::SoftwareCursor),
        );
    }
}
/// tag for easy software cursor query
#[derive(Component, Reflect, Default)]
// #[reflect(Component)]
pub struct SoftWareCursor {
    /// offset too move cursor relative too actual winit cursor position.
    /// used too center cursors point
    offset: Vec2,
}

/// creates software cursor entity
/// image selected from `init_resources.custom_cursor` ?
fn spawn_software_cursor(mut cmds: Commands, tex: Res<InitAssetHandles>) {
    cmds.spawn((
        Name::new("SoftwareCursor"),
        SoftWareCursor {
            offset: Vec2 { x: 0.0, y: 0.0 },
        },
        ImageBundle {
            style: Style {
                width: Val::Vw(3.0),
                aspect_ratio: Some(1.0),
                position_type: PositionType::Absolute,
                left: Val::Auto,
                right: Val::Auto,
                top: Val::Auto,
                bottom: Val::Auto,
                ..default()
            },
            z_index: ZIndex::Global(15),
            image: tex.cursor_default.clone().into(),
            ..default()
        },
    ));
}

/// updates software cursor position based on player `LookLocal` (`LookLocal` is just `winit::Window.cursor_position()`)
fn update_software_cursor_position(
    player_input: Query<&ActionState<action_maps::Gameplay>, With<Player>>,
    mut software_cursor: Query<(&mut Style, &SoftWareCursor), With<Node>>,
) {
    let Ok(player) = player_input.get_single() else {
        warn!("no player action data too update cursor with");
        return;
    };

    let Some(look_data) = player
        .action_data(action_maps::Gameplay::LookLocal)
        .axis_pair
    else {
        warn!("No look data for software");
        return;
    };

    let axis_value = look_data.xy();
    let (mut cursor_style, cursor_data) = software_cursor.single_mut();

    cursor_style.left = Val::Px(axis_value.x - cursor_data.offset.x);
    cursor_style.top = Val::Px(axis_value.y - cursor_data.offset.y);
}

// TODO: software cursor image should change based on button interaction

// it would be cool if:
// actually playing game it was a target like looking thingy
// a menu was visible, it would be a hand, and if the buttons get pressed the hand goes to 1 finger
