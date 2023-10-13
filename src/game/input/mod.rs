/// player input map
pub mod actions;
pub mod kbm;
mod touch;

use bevy::{
    ecs::system::Command,
    input::InputSystem,
    prelude::{
        default, App, Color, Commands, Component, GlobalTransform, IntoSystemConfigs,
        IntoSystemSetConfig, IntoSystemSetConfigs, Name, Plugin, PreUpdate, Query, Startup,
        SystemSet, Transform, Update, Vec2, With,
    },
    window::{PrimaryWindow, Window},
};

use bevy_prototype_lyon::{
    prelude::{Fill, GeometryBuilder, ShapeBundle, Stroke},
    shapes,
};
use leafwing_input_manager::{
    axislike::DualAxisData,
    plugin::InputManagerSystem,
    prelude::{ActionState, InputManagerPlugin},
};

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InternalInputSet {
    KBMInput,
    TouchInput,
    SoftwareCursor,
}

use super::actors::components::Player;
/// player input plugin

pub struct ActionsPlugin;

// holds default bindings for game
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<actions::Gameplay>::default());
        app.add_plugins(touch::TouchInputPlugin);
        app.add_plugins(kbm::KBMPlugin);
        app.add_systems(Startup, spawn_software_cursor);

        app.add_systems(
            PreUpdate,
            update_software_cursor
                .in_set(InternalInputSet::SoftwareCursor)
                .after(InternalInputSet::TouchInput),
        );

        app.configure_sets(
            PreUpdate,
            (
                InternalInputSet::KBMInput,
                InternalInputSet::TouchInput,
                InternalInputSet::SoftwareCursor,
            )
                .chain()
                .in_set(InputManagerSystem::ManualControl)
                .before(InputManagerSystem::ReleaseOnDisable)
                .after(InputManagerSystem::Tick)
                .after(InputManagerSystem::Update)
                .after(InputSystem),
        );
    }
}

#[derive(Component)]
pub struct SoftWareCursorTag;

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
            ..default()
        },
        Fill::color(Color::CYAN),
        Stroke::new(Color::BLACK, 2.0),
    ));
}

//TODO: hide software cursor when close too player (when within margin of screen_dimensions / 2.0)
fn update_software_cursor(
    window_query: Query<&Window, With<PrimaryWindow>>,
    player_input: Query<&ActionState<actions::Gameplay>, With<Player>>,
    mut soft_ware_cursor: Query<&mut Transform, With<SoftWareCursorTag>>,
) {
    player_input.for_each(|player_input| {
        // let look_local_data = player_input.action_data(actions::Gameplay::LookLocal);
        let window = window_query.single();
        let look_global_data = player_input.action_data(actions::Gameplay::LookWorld);
        let mut software_cursor = soft_ware_cursor.single_mut();

        software_cursor.translation = look_global_data
            .axis_pair
            .unwrap_or_else(|| {
                DualAxisData::from_xy(Vec2 {
                    x: window.width() / 2.0,
                    y: window.height() / 2.0,
                })
            })
            .xy()
            .extend(7.0);
    })
}
