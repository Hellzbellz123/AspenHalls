use bevy::prelude::*;

use crate::{
    game::{characters::player::PlayerSelectedHero, input::AspenCursorPosition},
    loading::assets::AspenInitHandles,
    AppState,
};

use super::AspenInputSystemSet;

/// adds software cursor functionality too app
pub struct SoftwareCursorPlugin;

impl Plugin for SoftwareCursorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<SoftWareCursor>();
        app.add_systems(
            OnEnter(AppState::Loading),
            spawn_software_cursor.run_if(not(any_with_component::<SoftWareCursor>)),
        );

        app.add_systems(
            PreUpdate,
            update_software_cursor_position
                .run_if(
                    resource_exists::<AspenCursorPosition>
                        .and_then(any_with_component::<SoftWareCursor>),
                )
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
    /// distance before hiding
    hide_distance: f32,
    /// alpha too hide
    /// clamped to 0.0-1.0
    hide_alpha: f32,
    /// alpha when should be visible
    /// clamped to 0.0-1.0
    show_alpha: f32,
}

/// creates software cursor entity
/// image selected from `init_resources.custom_cursor` ?
fn spawn_software_cursor(mut cmds: Commands, tex: Res<AspenInitHandles>) {
    cmds.spawn((
        Name::new("SoftwareCursor"),
        SoftWareCursor {
            offset: Vec2 { x: 0.0, y: 0.0 },
            hide_distance: 80.0,
            hide_alpha: 0.4,
            show_alpha: 0.8,
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
    os_cursor_pos: Res<AspenCursorPosition>,
    player: Query<&GlobalTransform, With<PlayerSelectedHero>>,
    mut software_cursor: Query<(&mut Style, &SoftWareCursor, &mut BackgroundColor), With<Node>>,
    window_query: Query<&Window>,
) {
    let (mut cursor_style, cursor_data, mut cursor_color) = software_cursor.single_mut();
    let Ok(ptrans) = player.get_single() else {
        let Ok(window) = window_query.get_single() else {
            error!("no window too update software cursor");
            return;
        };

        let window_cur_pos = window.cursor_position().unwrap_or(Vec2 {
            x: window.width() / 2.0,
            y: window.height() / 2.0,
        });

        cursor_style.left = Val::Px(window_cur_pos.x - cursor_data.offset.x);
        cursor_style.top = Val::Px(window_cur_pos.y - cursor_data.offset.y);
        return;
    };

    let (look_local, look_world) = (os_cursor_pos.screen, os_cursor_pos.world);
    let color = cursor_color.0;

    if ptrans
        .translation()
        .distance(look_world.extend(0.0))
        .le(&cursor_data.hide_distance)
    {
        *cursor_color = BackgroundColor(color.with_a(cursor_data.hide_alpha.clamp(0.0, 1.0)));
    } else {
        *cursor_color = BackgroundColor(color.with_a(cursor_data.show_alpha.clamp(0.0, 1.0)));
    };

    cursor_style.left = Val::Px(look_local.x - cursor_data.offset.x);
    cursor_style.top = Val::Px(look_local.y - cursor_data.offset.y);
}

// TODO: software cursor image should change based on button interaction

// it would be cool if:
// actually playing game it was a target like looking thingy
// a menu was visible, it would be a hand, and if the buttons get pressed the hand goes to 1 finger
