mod debug_dirs;
// #[cfg(feature = "dev")]
pub mod debug_plugin {
    use bevy::{
        diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
        prelude::{App, *},
    };
    use bevy_debug_text_overlay::OverlayPlugin;
    use bevy_ecs_ldtk::{GridCoords, IntGridCell, LayerMetadata};
    use bevy_inspector_egui::quick::{StateInspectorPlugin, WorldInspectorPlugin};
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
    use std::{fs, time::Duration};

    use crate::{
        actors::combat::components::{
            CurrentlySelectedWeapon, DamageType, WeaponSlots, WeaponSocket, WeaponStats, WeaponTag,
        },
        components::{
            actors::{
                ai::{
                    AIAttackState, AICanChase, AICanWander, AIChaseAction, AIEnemy, AIWanderAction,
                    ActorType, AggroScore, TypeEnum,
                },
                animation::{AnimState, AnimationSheet, FacingDirection},
                general::{CombatStats, DefenseStats, MovementState, Player, TimeToLive},
                spawners::Spawner,
            },
            DebugTimer, MainCameraTag,
        },
        dev_tools::debug_dirs::debugdir,
        game::{GameStage, TimeInfo},
        // kayak_ui::MenuState,
        utilities::game::AppSettings,
    };

    pub struct DebugPlugin;

    impl Plugin for DebugPlugin {
        fn build(&self, app: &mut App) {
            debugdir();
            app
                // .add_plugin()
                .add_plugin(OverlayPlugin {
                    font_size: 32.0,
                    ..Default::default()
                })
                .add_plugin(WorldInspectorPlugin::default())
                .add_plugin(StateInspectorPlugin::<GameStage>::default())
                .add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin {
                    wait_duration: Duration::from_secs(20),
                    ..Default::default()
                })
                // .add_plugin(InspectorPlugin::<crate::game_world::homeworld::components::ReflectData>::new())
                .register_type::<Timer>()
                //rapier Reflects in this plugin
                // .add_plugin(ReflectRapierPlugin)
                .add_plugin(RapierDebugRenderPlugin::default())
                //custom Reflects not from plugins
                .register_type::<AppSettings>()
                // .register_type::<MenuState>()
                .register_type::<MovementState>()
                .register_type::<CombatStats>()
                .register_type::<DefenseStats>()
                .register_type::<Player>()
                .register_type::<AIEnemy>()
                .register_type::<TypeEnum>()
                .register_type::<AnimationSheet>()
                .register_type::<FacingDirection>()
                .register_type::<TimeInfo>()
                .register_type::<MainCameraTag>() // tells bevy-inspector-egui how to display the struct in the world inspector
                .register_type::<Spawner>()
                // .register_type::<PlayerActions>()
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
                .register_type::<LayerMetadata>()
                .register_type::<IntGridCell>()
                .register_type::<GridCoords>()
                // bigbrain AI
                .register_type::<AggroScore>()
                .register_type::<AICanWander>()
                .register_type::<AICanChase>()
                .register_type::<AIChaseAction>()
                .register_type::<AIWanderAction>()
                .register_type::<ActorType>()
                .add_systems((debug_visualize_spawner, debug_visualize_weapon_spawn_point))
                .insert_resource(DebugTimer(Timer::from_seconds(10.0, TimerMode::Repeating)))
                // TODO: refactor these systems into nice sets and stages
                .add_systems(
                    (debug_visualize_spawner, debug_visualize_weapon_spawn_point)
                        .in_set(OnUpdate(GameStage::PlaySubStage)),
                );
        }
    }

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

    fn debug_visualize_weapon_spawn_point(
        mut cmds: Commands,
        #[allow(clippy::type_complexity)]
        // trunk-ignore(clippy/type_complexity)
        weapon_query: Query<
            // this is equivelent to if player has a weapon equipped and out
            (Entity, &WeaponStats, &Transform),
            (With<Parent>, With<CurrentlySelectedWeapon>, Without<Player>),
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

    pub fn debug_dump_graphs(app: &mut App) {
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

        let schedule_graph = bevy_mod_debugdump::schedule_graph_dot(
            app,
            CoreSchedule::Main,
            &schedule_graph_settings,
        );
        let render_graph = bevy_mod_debugdump::render_graph_dot(app, &render_graph_settings);

        fs::write("zschedulegraph.dot", schedule_graph.clone())
            .expect("couldnt write render schedule to file");
        fs::write("zrendergraph.dot", render_graph).expect("couldnt write render schedule to file");
    }
}
