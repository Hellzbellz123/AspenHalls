use bevy::{prelude::*, render::primitives::Aabb};

use crate::{
    game::{characters::player::PlayerSelectedHero, input::AspenCursorPosition},
    loading::{assets::AspenInitHandles, registry::RegistryIdentifier},
    GameStage, WindowSettings,
};

use super::AspenInputSystemSet;

/// adds software cursor functionality too app
pub struct SoftwareCursorPlugin;

impl Plugin for SoftwareCursorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<SoftWareCursor>();

        app
            // .add_systems(
            //     Update,
            //     update_software_cursor_position.run_if(
            //         resource_exists::<AspenCursorPosition>
            //             .and(any_with_component::<SoftWareCursor>)
            //             .and(|res: Res<WindowSettings>| res.software_cursor_enabled),
            //     ),
            // )
            .add_systems(
                Update,
                (
                    cursor_grab_system,
                    control_software_cursor.run_if(resource_exists::<AspenInitHandles>),
                    (
                        update_software_cursor_image,
                        update_software_cursor_position,
                    )
                        .run_if(
                            resource_exists::<AspenCursorPosition>
                                .and(any_with_component::<SoftWareCursor>)
                                .and(|res: Res<WindowSettings>| res.software_cursor_enabled),
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
    cfg: Res<WindowSettings>,
) {
    let Ok(mut window) = windows.single_mut() else {
        return;
    };

    if !cfg.software_cursor_enabled {
        if !window.cursor_options.visible {
            window.cursor_options.visible = true;
        }
        return;
    }

    //TODO: disable system if software cursor is disabled.
    if btn.just_pressed(MouseButton::Left) && window.cursor_options.visible {
        // locking cursor causes loss of mouse movement on windows?
        // window.cursor.grab_mode = bevy::window::CursorGrabMode::Confined;

        window.cursor_options.visible = false;
    }

    if key.just_pressed(KeyCode::Escape) && !window.cursor_options.visible {
        // window.cursor.grab_mode = bevy::window::CursorGrabMode::None;

        window.cursor_options.visible = true;
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
fn control_software_cursor(
    mut cmds: Commands,
    cfg: Res<WindowSettings>,
    tex: Res<AspenInitHandles>,
    software_cursor: Query<Entity, With<SoftWareCursor>>,
) {
    if !cfg.software_cursor_enabled {
        for cursor in &software_cursor {
            cmds.entity(cursor).despawn();
        }
    }

    if cfg.software_cursor_enabled && software_cursor.is_empty() {
        cmds.spawn((
            Name::new("SoftwareCursor"),
            Pickable {
                should_block_lower: false,
                is_hoverable: false,
            },
            SoftWareCursor {
                offset: Vec2 { x: 0.0, y: 0.0 },
                hide_distance: 50.0,
                hide_alpha: 0.4,
                show_alpha: 0.8,
            },
            ZIndex(15),
            BackgroundColor(crate::colors::WHITE.with_alpha(0.0).into()),
            ImageNode {
                image: tex.cursor_image.clone(),
                texture_atlas: Some(TextureAtlas::from(tex.cursor_layout.clone())),
                ..default()
            },
        ));
    }
}

/// changes software cursor image based upon certain conditions
fn update_software_cursor_image(
    os_cursor_pos: Res<AspenCursorPosition>,
    player: Query<&GlobalTransform, With<PlayerSelectedHero>>,
    interactables: Query<
        (&GlobalTransform, &Aabb),
        (Without<PlayerSelectedHero>, With<RegistryIdentifier>),
    >,
    mut software_cursor: Query<(&mut SoftWareCursor, &mut ImageNode, &Node, &ComputedNode)>,
    game_state: Option<Res<State<GameStage>>>,
) {
    let Ok((mut cursor_data, mut cursor_image, _node, node_size)) = software_cursor.single_mut()
    else {
        return;
    };

    let distance = player
        .single()
        .map_or(cursor_data.hide_distance + 25.0, |transform| {
            transform
                .translation()
                .truncate()
                .distance(os_cursor_pos.world)
        });

    if distance.le(&cursor_data.hide_distance)
        && game_state
            .as_ref()
            .is_some_and(|f| f.get() != &GameStage::StartMenu)
    {
        cursor_image.color = cursor_image.color.with_alpha(cursor_data.hide_alpha);
    } else {
        cursor_image.color = cursor_image.color.with_alpha(cursor_data.show_alpha);
    };

    let Some(cursor_atlas) = &mut cursor_image.texture_atlas else {
        warn!("Software cursor image did not have a texture atlas");
        return;
    };

    if game_state.is_some_and(|f| f.get() != &GameStage::StartMenu) {
        // if cursor is over 'interactable actor/enemy' set TextureAtlas.index too 'HasTarget' otherwise 'NoTarget'
        for (interactble_pos, interactable_aabb) in &interactables {
            let pos = interactble_pos.translation(); // + Vec3::from(interactable_aabb.center);
            if Rect::from_center_half_size(
                pos.truncate(),
                Vec3::from(interactable_aabb.half_extents).truncate(),
            )
            .contains(os_cursor_pos.world)
            {
                cursor_atlas.index = CursorType::NoTarget as usize;
            } else {
                cursor_atlas.index = CursorType::NoTarget as usize;
            }

            cursor_data.offset = node_size.size() / 2.0;
        }
    } else {
        cursor_atlas.index = CursorType::Default as usize;
        cursor_data.offset = Vec2 { x: 0.0, y: 0.0 };
    }
}

/// cursor image type
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum CursorType {
    /// default arrow shaped cursor
    Default,
    /// undimmed crosshair cursor
    HasTarget,
    /// dimmed crosshair cursor
    NoTarget,
}

/// updates software cursor position based on `LookLocal` (`LookLocal` is just `winit::Window.cursor_position()`)
fn update_software_cursor_position(
    mut software_cursor: Query<(&mut Node, &SoftWareCursor)>,
    cursor_pos: Res<AspenCursorPosition>,
    window_query: Query<&Window>,
) {
    let Ok((mut cursor_style, cursor_data)) = software_cursor.single_mut() else {
        error!("no software cursor too update");
        return;
    };
    let Ok(window) = window_query.single() else {
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
