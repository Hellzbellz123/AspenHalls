#![allow(clippy::type_complexity)]
/// holds walk_dirs function
/// outputs cwd too console
mod debug_dirs;

/// debug plugin for vanillacoffee
/// holds type registration, diagnostics, and inspector stuff
#[cfg(feature = "dev")]
pub mod debug_plugin {
    use bevy::{
        diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
        prelude::{App, *}, app::RunFixedUpdateLoop,
    };
    // use bevy_debug_grid::DebugGridPlugin;
    use bevy_debug_text_overlay::OverlayPlugin;
    use bevy_ecs_ldtk::{prelude::LdtkLevel, GridCoords, IntGridCell, LayerMetadata};
    use bevy_inspector_egui::quick::{
        ResourceInspectorPlugin, StateInspectorPlugin, WorldInspectorPlugin,
    };
    use bevy_mod_debugdump::{render_graph, schedule_graph};
    use bevy_prototype_lyon::{
        prelude::{
            // DrawMode,
            Fill,
            FillOptions,
            GeometryBuilder,
        },
        // render::Shape,
        shapes,
    };
    use bevy_rapier2d::render::RapierDebugRenderPlugin;
    // use grid_plane::GridPlanePlugin;
    use std::{fs, time::Duration};

    use crate::{
        launch_config::{DifficultyScale, GeneralSettings, SoundSettings, WindowSettings},
        dev_tools::debug_dirs::debugdir,
        game::{
            actors::combat::components::{
                CurrentlySelectedWeapon, DamageType, WeaponSlots, WeaponSocket, WeaponStats,
                WeaponTag,
            },
            game_world::dungeonator::DungeonGeneratorSettings,
        },
        // kayak_ui::MenuState,
        game::{
            actors::{
                ai::components::*, animation::components::*, components::*,
                spawners::components::Spawner,
            },
            game_world::dungeonator::GeneratorStage,
            interface::RequestedMenu,
        },
        game::{AppStage, TimeInfo},
        loading::splashscreen::MainCameraTag,
    };

    /// actual plugin too insert
    pub struct DebugPlugin;

    impl Plugin for DebugPlugin {
        fn build(&self, app: &mut App) {
            debugdir();
            app
                // .add_plugin(InspectorPlugin::<crate::game_world::homeworld::components::ReflectData>::new())
                .register_type::<Timer>()
                //rapier Reflects in this plugin
                // .add_plugin(ReflectRapierPlugin)
                .add_plugin(RapierDebugRenderPlugin::default())
                //custom Reflects not from plugins
                .register_type::<DifficultyScale>()
                .register_type::<WindowSettings>()
                .register_type::<GeneralSettings>()
                .register_type::<SoundSettings>()
                // .register_type::<MenuState>()
                .register_type::<Player>()
                .register_type::<Faction>()
                .register_type::<AnimationSheet>()
                .register_type::<ActorAnimationType>()
                .register_type::<TimeInfo>()
                .register_type::<MainCameraTag>() // tells bevy-inspector-egui how to display the struct in the world inspector
                .register_type::<Spawner>()
                // .register_type::<actions::Combat>()
                .register_type::<AnimState>()
                .register_type::<AIAttackState>()
                .register_type::<TimeToLive>()
                .register_type::<WeaponTag>()
                // weapon stuff
                .register_type::<CurrentlySelectedWeapon>()
                .register_type::<DamageType>()
                .register_type::<WeaponStats>()
                .register_type::<WeaponSlots>()
                .register_type::<WeaponSocket>()
                // LDTK debug data
                .register_type::<LdtkLevel>()
                .register_type::<Handle<LdtkLevel>>()
                .register_type::<LayerMetadata>()
                .register_type::<IntGridCell>()
                .register_type::<GridCoords>()
                // bigbrain AI
                .register_type::<AggroScore>()
                .register_type::<AICanWander>()
                .register_type::<AICanAggro>()
                .register_type::<AIChaseAction>()
                .register_type::<AIWanderAction>()
                .register_type::<ActorType>()
                .add_plugin(OverlayPlugin {
                    font_size: 32.0,
                    ..Default::default()
                })
                .add_plugin(WorldInspectorPlugin::default())
                .add_plugin(
                    ResourceInspectorPlugin::<DungeonGeneratorSettings>::default()
                        .run_if(state_exists_and_equals(GeneratorStage::Finished)),
                )
                .add_plugin(StateInspectorPlugin::<AppStage>::default())
                .add_plugin(StateInspectorPlugin::<RequestedMenu>::default())
                .add_plugin(StateInspectorPlugin::<GeneratorStage>::default())
                .add_plugin(FrameTimeDiagnosticsPlugin)
                .add_plugin(LogDiagnosticsPlugin {
                    wait_duration: Duration::from_secs(20),
                    ..Default::default()
                })
                // .insert_resource(DebugTimer(Timer::from_seconds(10.0, TimerMode::Repeating)))
                // TODO: refactor these systems into nice sets and stages
                .add_systems(
                    Update,
                    (debug_visualize_spawner, debug_visualize_weapon_spawn_point)
                        .run_if(state_exists_and_equals(AppStage::PlayingGame)),
                );

            debug_dump_graphs(app);
        }
    }

    /// querys spawners and creates debug representations for spawner area
    fn debug_visualize_spawner(
        mut cmds: Commands,
        spawner_query: Query<(Entity, &Transform, &Spawner), Without<Fill>>,
    ) {
        for (entity, _transform, spawner) in &spawner_query {
            let spawner_box_visual = shapes::Rectangle {
                extents: Vec2 { x: 40.0, y: 40.0 },
                origin: shapes::RectangleOrigin::Center,
            };

            let spawner_radius_visual = shapes::Circle {
                radius: spawner.spawn_radius,
                center: Vec2::ZERO,
            };

            info!("adding visual too spawner {:?}", entity);
            let spawner_visual_bundle = GeometryBuilder::new()
                .add(&spawner_box_visual)
                .add(&spawner_radius_visual)
                .build();

            cmds.entity(entity)
                .insert(spawner_visual_bundle)
                .insert(Fill {
                    options: FillOptions::default(),
                    color: Color::Hsla {
                        hue: 334.0,
                        saturation: 0.83,
                        lightness: 0.3,
                        alpha: 0.25,
                    },
                });
        }
    }

    /// spawn red dot where weapon bullets spawn
    fn debug_visualize_weapon_spawn_point(
        mut cmds: Commands,
        weapon_query: Query<
            // this is equivelent to if player has a weapon equipped and out
            (Entity, &WeaponStats, &Transform),
            (With<Parent>, With<CurrentlySelectedWeapon>),
        >,
    ) {
        for (ent, _wstats, _trans) in &weapon_query {
            let spawner_box_visual = shapes::Rectangle {
                extents: Vec2 { x: 2.0, y: 2.0 },
                origin: shapes::RectangleOrigin::Center,
            };

            let spawner_visual_bundle = GeometryBuilder::new().add(&spawner_box_visual).build();

            cmds.entity(ent).insert(spawner_visual_bundle);
        }
    }

    /// dumps scheduling graphs for given App
    pub fn debug_dump_graphs(app: &mut App) {
        warn!("Dumping graphs");

        let schedule_theme = schedule_graph::settings::Style::dark_github();
        let render_theme = render_graph::settings::Style::dark_github();

        let schedule_graph_settings = schedule_graph::Settings {
            ambiguity_enable: false,
            ambiguity_enable_on_world: false,
            style: schedule_theme,
            collapse_single_system_sets: true,
            prettify_system_names: false,
            ..Default::default()
        };

        let render_graph_settings = bevy_mod_debugdump::render_graph::Settings {
            style: render_theme,
        };

        let update_schedule_graph = bevy_mod_debugdump::schedule_graph_dot(app, Update, &schedule_graph_settings);

        // let startup_schedule_graph =
        //     bevy_mod_debugdump::schedule_graph_dot(app, Main, &schedule_graph_settings);

        let render_graph = bevy_mod_debugdump::render_graph_dot(app, &render_graph_settings);

        fs::write("zmainschedulegraph.dot", update_schedule_graph)
            .expect("couldnt write render schedule to file");
        fs::write("zrendergraph.dot", render_graph).expect("couldnt write render schedule to file");
    }

    // /// takes all actors and sums z then divied by actor count
    // pub fn debug_test_transform_z(actor_query: Query<(&ActorType, &GlobalTransform)>) {
    //     let mut total = 0;
    //     let mut z_value = 0.0;
    //     actor_query.for_each(|(_thing, trans)| {
    //         z_value += trans.translation().z;
    //         total += 1;
    //     });
    //     let a: f32 = z_value / actor_query.iter().len() as f32;
    //     info!("average z value {}", a);
    // }
}
