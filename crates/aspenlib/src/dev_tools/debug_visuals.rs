use crate::{colors, dev_tools::DebugConfig, game::game_world::components::CharacterSpawner};
use bevy::{platform::collections::HashMap, prelude::*, render::primitives::Aabb};

/// enables visual debug tools for game
pub struct DebugVisualsPlugin;

impl Plugin for DebugVisualsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                debug_draws.run_if(any_with_component::<DebugDraw>),
                debugdraw_aabb.run_if(|cfg: Res<DebugConfig>| cfg.aabb_draw),
                init_debug_visualize_spawner.run_if(any_with_component::<CharacterSpawner>),
            )
                .run_if(|cfg: Res<DebugConfig>| cfg.enabled),
        );
    }
}

/// visualizes aabbs on entitys with an `Aabb` and a `GlobalTransform`
fn debugdraw_aabb(mut gizmos: Gizmos, query: Query<(&GlobalTransform, &Aabb)>) {
    for (t, a) in &query {
        gizmos.rect_2d(
            Isometry2d::from_translation(t.translation().truncate() + a.center.truncate()),
            a.half_extents.truncate() * 2.0,
            colors::AQUA,
        );
    }
}

/// debug item descriptor
#[allow(clippy::derived_hash_with_manual_eq)]
#[derive(Debug, Reflect, Default, Hash, PartialOrd, Deref, DerefMut, Component, Clone)]
pub struct DebugDraw {
    /// identifier for `DebugDraw`
    /// only data used for `Hash`
    #[deref]
    pub id: DrawId,
    /// display type for this `DebugDraw`
    pub shape: DebugShape,
}

/// shape too display for this `DebugDraw`
#[derive(Debug, Reflect, Default, PartialEq, Eq, PartialOrd, Component, Clone, Hash)]
pub enum DebugShape {
    /// displays the x/y axis for this `DebugDraw`
    #[default]
    Axes,
    /// displays a box for entity
    Box,
    /// displays circle and center for spawner with variable radius
    Spawner {
        /// how large a circle this gimoz should display as
        radius: i32,
    },
}

/// identifier for `DebugDraw`s
#[derive(
    Debug, Reflect, Default, Hash, PartialEq, Eq, PartialOrd, Deref, DerefMut, Component, Clone,
)]
pub struct DrawId(String);

impl Eq for DebugDraw {}
impl PartialEq for DebugDraw {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

/// Resource of `DebugDraws` to create gizmos for
#[derive(Debug, Reflect, Default, Deref, DerefMut, Clone)]
pub struct DrawnMap(HashMap<DrawId, bool>);

/// This system draws the axes based on the cube's transform, with length based on the size of
/// the entity's axis-aligned bounding box `Aabb`.
fn debug_draws(
    mut gizmos: Gizmos,
    mut debug_cfg: ResMut<DebugConfig>,
    query: Query<(&GlobalTransform, Option<&Aabb>, &DebugDraw)>,
) {
    let draws = &debug_cfg.drawmap;

    for (&transform, aabb, draw) in &query {
        let Some(&enabled) = draws.get(&draw.id) else {
            error!("'DebugDraw' item was not registered, registering {draw:?} in DrawMap");

            debug_cfg.drawmap.insert(draw.id.clone(), false);
            return;
        };

        if !enabled {
            continue;
        }

        match draw.shape {
            DebugShape::Axes => {
                let length = 0.5;
                gizmos.axes(transform, length);
            }
            DebugShape::Box => {
                let size =
                    aabb.map_or_else(|| Vec3::splat(0.5), |aabb| (aabb.half_extents * 2.0).into());

                gizmos.rounded_cuboid(
                    transform.to_isometry(),
                    size,
                    colors::AZURE,
                );
            }
            DebugShape::Spawner { radius } => {
                gizmos.circle_2d(
                    transform.translation().truncate(),
                    radius as f32,
                    colors::CRIMSON.with_alpha(0.7),
                );
            }
        }
    }
}

/// query's spawners and creates debug representations for spawner area
fn init_debug_visualize_spawner(
    mut cmds: Commands,
    spawner_query: Query<(Entity, &CharacterSpawner), Without<DebugDraw>>,
) {
    for (entity, spawner) in &spawner_query {
        let debug_draw = DebugDraw {
            id: DrawId("CharacterSpawner".into()),
            shape: DebugShape::Spawner {
                radius: (spawner.spawn_radius) as i32,
            },
        };

        cmds.entity(entity).insert(debug_draw);
    }
}
