use bevy::prelude::SpatialBundle;

use crate::ahp::{
    engine::{
        bevy, default, leafwing_input_manager::action_state::ActionState,
        svg_draw, svg_shapes, Color, Commands, Component, GeometryBuilder,
        IntoSystemConfigs, Name, OnEnter, Plugin, PreUpdate, Query,
        SvgBundle, Transform, With,
    },
    game::{action_maps, AppState, Player},
};

use super::AspenInputSystemSet;

/// adds software cursor functionality too app
pub struct SoftwareCursorPlugin;

impl Plugin for SoftwareCursorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(AppState::Loading), spawn_software_cursor);

        app.add_systems(
            PreUpdate,
            update_software_cursor
                .in_set(AspenInputSystemSet::SoftwareCursor),
        );
    }
}
/// tag for easy software cursor query
#[derive(Component)]
pub struct SoftWareCursorTag;

/// creates software cursor entity
/// image selected from `init_resources.custom_cursor` ?
fn spawn_software_cursor(mut cmds: Commands) {
    let shape = svg_shapes::RegularPolygon {
        sides: 6,
        feature: svg_shapes::RegularPolygonFeature::Radius(5.0),
        ..svg_shapes::RegularPolygon::default()
    };

    cmds.spawn((
        Name::new("SoftwareCursor"),
        SoftWareCursorTag,
        SvgBundle {
            path: GeometryBuilder::build_as(&shape),
            spatial: SpatialBundle::from_transform(Transform::from_xyz(
                0.0, 0.0, 50.0,
            )),
            ..default()
        },
        svg_draw::Fill::color(Color::CYAN),
        svg_draw::Stroke::new(Color::BLACK, 2.0),
    ));
}

//TODO: hide software cursor when close too player (when within margin of screen_dimensions / 2.0)
/// modifies software cursor position based on mouse position inside window
fn update_software_cursor(
    // window_query: Query<&Window, With<PrimaryWindow>>,
    player_input: Query<&ActionState<action_maps::Gameplay>, With<Player>>,
    mut soft_ware_cursor_query: Query<
        &mut Transform,
        With<SoftWareCursorTag>,
    >,
) {
    let Ok(player) = player_input.get_single() else {
        return;
    };
    // let look_local_data = player_input.action_data(actions::Gameplay::LookLocal);
    // let window = window_query.single();
    let look_global_data =
        player.action_data(action_maps::Gameplay::LookWorld);
    let mut soft_cursor_transform = soft_ware_cursor_query.single_mut();

    soft_cursor_transform.translation = look_global_data
        .axis_pair
        .unwrap()
        .xy()
        .extend(soft_cursor_transform.translation.z);
}
