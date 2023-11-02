use bevy::{
    prelude::{
        default, Color, Commands, Component, IntoSystemConfigs, Name, Plugin, PreUpdate, Query,
        Startup, Transform, Vec2, With, OnEnter,
    },
    window::{PrimaryWindow, Window},
};
use bevy_prototype_lyon::{
    prelude::{Fill, GeometryBuilder, ShapeBundle, Stroke},
    shapes,
};
use leafwing_input_manager::{axislike::DualAxisData, prelude::ActionState};

use crate::game::{actors::components::Player, AppStage};

use super::{action_maps::Gameplay, InternalInputSet};

/// adds software cursor functionality too app
pub struct SoftwareCursorPlugin;

impl Plugin for SoftwareCursorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(AppStage::Loading), spawn_software_cursor);

        app.add_systems(
            PreUpdate,
            update_software_cursor
                .in_set(InternalInputSet::SoftwareCursor)
                .after(InternalInputSet::TouchInput),
        );
    }
}
/// tag for easy software cursor query
#[derive(Component)]
pub struct SoftWareCursorTag;

/// creates software cursor entity
/// image selected from init_resources.custom_cursor?
fn spawn_software_cursor(mut cmds: Commands) {
    let shape = shapes::RegularPolygon {
        sides: 6,
        feature: shapes::RegularPolygonFeature::Radius(5.0),
        ..shapes::RegularPolygon::default()
    };

    cmds.spawn((
        Name::new("SoftwareCursor"),
        SoftWareCursorTag,
        ShapeBundle {
            path: GeometryBuilder::build_as(&shape),
            transform: Transform::from_xyz(0.0, 0.0, 50.0),
            ..default()
        },
        Fill::color(Color::CYAN),
        Stroke::new(Color::BLACK, 2.0),
    ));
}

//TODO: hide software cursor when close too player (when within margin of screen_dimensions / 2.0)
/// modifies software cursor position based on mouse position inside window
fn update_software_cursor(
    window_query: Query<&Window, With<PrimaryWindow>>,
    player_input: Query<&ActionState<Gameplay>, With<Player>>,
    mut soft_ware_cursor_query: Query<&mut Transform, With<SoftWareCursorTag>>,
) {
    player_input.for_each(|player_input| {
        // let look_local_data = player_input.action_data(actions::Gameplay::LookLocal);
        let window = window_query.single();
        let look_global_data = player_input.action_data(Gameplay::LookWorld);
        let mut soft_cursor_transform = soft_ware_cursor_query.single_mut();

        soft_cursor_transform.translation = look_global_data
            .axis_pair
            .unwrap_or_else(|| {
                DualAxisData::from_xy(Vec2 {
                    x: window.width() / 2.0,
                    y: window.height() / 2.0,
                })
            })
            .xy()
            .extend(soft_cursor_transform.translation.z);
    });
}
