use bevy::{prelude::*, render::primitives::Aabb};

use crate::{
    game::{
        characters::{components::CharacterType, player::PlayerSelectedHero},
        components::ActorType,
        input::AspenCursorPosition,
    },
    loading::{assets::AspenInitHandles, registry::RegistryIdentifier},
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
            (
                cursor_grab_system,
                (
                    update_software_cursor_position,
                    update_software_cursor_image,
                )
                    .run_if(
                        resource_exists::<AspenCursorPosition>
                            .and_then(any_with_component::<SoftWareCursor>),
                    )
                    .in_set(AspenInputSystemSet::SoftwareCursor),
            ),
        );
    }
}

/// handle cursor lock for game
fn cursor_grab_system(
    mut windows: Query<&mut Window>,
    btn: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
) {
    let mut window = windows.single_mut();

    if btn.just_pressed(MouseButton::Left) {
        // if you want to use the cursor, but not let it leave the window,
        // use `Confined` mode:
        window.cursor.grab_mode = bevy::window::CursorGrabMode::Confined;

        if !cfg!(debug_assertions) {
            window.cursor.visible = false;
        }
    }

    if key.just_pressed(KeyCode::Escape) {
        window.cursor.grab_mode = bevy::window::CursorGrabMode::None;

        if !cfg!(debug_assertions) {
            window.cursor.visible = true;
        }
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
            hide_distance: 50.0,
            hide_alpha: 0.4,
            show_alpha: 0.8,
        },
        TextureAtlas::from(tex.cursor_layout.clone()),
        BorderColor(Color::RED),
        ImageBundle {
            style: Style {
                border: UiRect::all(Val::Px(2.0)),
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
            image: tex.cursor_image.clone().into(),
            ..default()
        },
    ));
}

fn update_software_cursor_image(
    os_cursor_pos: Res<AspenCursorPosition>,
    player: Query<&GlobalTransform, With<PlayerSelectedHero>>,
    interactables: Query<
        (&GlobalTransform, &Aabb),
        (Without<PlayerSelectedHero>, With<RegistryIdentifier>),
    >,
    mut software_cursor: Query<
        (&mut SoftWareCursor, &mut BackgroundColor, &mut TextureAtlas, &Node),
        With<Node>,
    >,
    game_state: Res<State<AppState>>,
) {
    let Ok((mut cursor_data, mut cursor_color, mut cursor_atlas, node_size)) = software_cursor.get_single_mut()
    else {
        return;
    };

    let distance = player
        .get_single()
        .map_or(cursor_data.hide_distance + 25.0, |transform| {
            transform
                .translation()
                .truncate()
                .distance(os_cursor_pos.world)
        });

    if distance.le(&cursor_data.hide_distance) && game_state.get() == &AppState::PlayingGame {
        *cursor_color = BackgroundColor(
            cursor_color
                .0
                .with_a(cursor_data.hide_alpha.clamp(0.0, 1.0)),
        );
    } else {
        *cursor_color = BackgroundColor(
            cursor_color
                .0
                .with_a(cursor_data.show_alpha.clamp(0.0, 1.0)),
        );
    };

    if game_state.get() == &AppState::PlayingGame {
        // if cursor is over 'interactable actor/enemy' set TextureAtlas.index too 'HasTarget' otherwise 'NoTarget'
        cursor_data.offset = node_size.size() / 2.0;
        for (interactble_pos, interactable_aabb) in &interactables {
            let pos = interactble_pos.translation() + Vec3::from(interactable_aabb.center);
            if Rect::from_center_half_size(
                pos.truncate(),
                Vec3::from(interactable_aabb.half_extents).truncate(),
            )
            .contains(os_cursor_pos.world)
            {
                cursor_atlas.index = CursorType::HasTarget as usize;
            } else {
                cursor_atlas.index = CursorType::NoTarget as usize;
            }
        }
    } else {
        cursor_atlas.index = CursorType::Default as usize;
        cursor_data.offset = Vec2 { x: 0.0, y: 0.0 };
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum CursorType {
    Default,
    HasTarget,
    NoTarget,
}

/// updates software cursor position based on `LookLocal` (`LookLocal` is just `winit::Window.cursor_position()`)
fn update_software_cursor_position(
    cursor_pos: Res<AspenCursorPosition>,
    mut software_cursor: Query<(&mut Style, &SoftWareCursor), With<Node>>,
    window_query: Query<&Window>,
) {
    let Ok((mut cursor_style, cursor_data)) = software_cursor.get_single_mut() else {
        error!("no software cursor too update");
        return;
    };
    let Ok(window) = window_query.get_single() else {
        error!("no window too position software cursor");
        return;
    };

    let percent_x = ((cursor_pos.screen.x - cursor_data.offset.x) / window.width()) * 100.0;
    let percent_y = ((cursor_pos.screen.y - cursor_data.offset.y) / window.height()) * 100.0;

    cursor_style.left = Val::Percent(percent_x.abs());
    cursor_style.top = Val::Percent(percent_y.abs());
}

// TODO: software cursor image should change based on button interaction

// it would be cool if:
// actually playing game it was a target like looking thingy
// a menu was visible, it would be a hand, and if the buttons get pressed the hand goes to 1 finger
