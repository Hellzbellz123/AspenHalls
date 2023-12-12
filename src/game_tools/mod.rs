#[cfg(feature = "develop")]
#[cfg(not(any(target_os = "android", target_family = "wasm")))]
/// dump game directory function.
///
/// useful too see where the game is trying too load assets from
/// holds `walk_dirs` function
/// outputs cwd too console
mod debug_dirs;

/// debug plugin for vanillacoffee
/// holds type registration, diagnostics, and inspector stuff
#[cfg(feature = "develop")]
pub mod debug_plugin {
    use bevy::input::{common_conditions::input_toggle_active, keyboard::KeyCode};

    #[cfg(feature = "develop")]
    #[cfg(not(any(target_os = "android", target_family = "wasm")))]
    use crate::game_tools::debug_dirs::debug_directory;

    use std::{
        fs,
        path::{Path, PathBuf},
        time::Duration,
    };

    use crate::ahp::{
        engine::{
            info, render_graph, render_graph_dot, schedule_graph, schedule_graph_dot,
            state_exists_and_equals, svg_shapes, warn, App, Color, Commands, Entity, Fill,
            FillOptions, First, GeometryBuilder, GridCoords, Handle, IntGridCell,
            IntoSystemConfigs, Last, LayerMetadata, LdtkProject, Parent, Plugin, PostStartup,
            PostUpdate, PreStartup, PreUpdate, Query, Startup, Timer, Transform, Update, Vec2,
            With, Without,
        },
        game::{
            AIChaseAction, AIChaseConfig, AIShootAction, AIShootConfig, AIWanderAction,
            AIWanderConfig, ActorAnimationType, ActorType, AnimState, AnimationSheet, AppState,
            ChaseScore, CurrentlySelectedWeapon, DamageType, DifficultyScales, GeneralSettings,
            MainCamera, Player, SoundSettings, Spawner, TimeInfo, TimeToLive, Type, Weapon,
            WeaponSlots, WeaponSocket, WeaponStats, WindowSettings,
        },
        plugins::{
            FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin, OverlayPlugin,
            RapierDebugRenderPlugin, StateInspectorPlugin, WorldInspectorPlugin,
        },
    };

    /// debug plugin for Aspen Halls.
    /// registers types from plugins and the game, prints diagnostics too the console, and spawns an `world` inspector and a `state` inspector
    pub struct DebugPlugin;

    impl Plugin for DebugPlugin {
        fn build(&self, app: &mut App) {
            #[cfg(not(any(target_os = "android", target_family = "wasm")))]
            debug_directory();

            app.register_type::<Timer>()
                //custom Reflects not from plugins
                .register_type::<DifficultyScales>()
                .register_type::<WindowSettings>()
                .register_type::<GeneralSettings>()
                .register_type::<SoundSettings>()
                .register_type::<Player>()
                .register_type::<Type>()
                .register_type::<AnimationSheet>()
                .register_type::<ActorAnimationType>()
                .register_type::<TimeInfo>()
                .register_type::<MainCamera>() // tells bevy-inspector-egui how to display the struct in the world inspector
                .register_type::<Spawner>()
                .register_type::<AnimState>()
                .register_type::<TimeToLive>()
                .register_type::<Weapon>()
                // weapon stuff
                .register_type::<CurrentlySelectedWeapon>()
                .register_type::<DamageType>()
                .register_type::<WeaponStats>()
                .register_type::<WeaponSlots>()
                .register_type::<WeaponSocket>()
                // LDTK debug data
                .register_type::<LdtkProject>()
                .register_type::<Handle<LdtkProject>>()
                .register_type::<LayerMetadata>()
                .register_type::<IntGridCell>()
                .register_type::<GridCoords>()
                // bigbrain AI
                .register_type::<ChaseScore>()
                .register_type::<AIWanderConfig>()
                .register_type::<AIChaseConfig>()
                .register_type::<AIShootConfig>()
                .register_type::<AIChaseAction>()
                .register_type::<AIWanderAction>()
                .register_type::<AIShootAction>()
                .register_type::<ActorType>()
                .add_plugins((
                    WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::F3)),
                    StateInspectorPlugin::<AppState>::default()
                        .run_if(input_toggle_active(true, KeyCode::F3)),
                    RapierDebugRenderPlugin::default(),
                    OverlayPlugin {
                        font_size: 32.0,
                        ..Default::default()
                    },
                    FrameTimeDiagnosticsPlugin,
                    LogDiagnosticsPlugin {
                        wait_duration: Duration::from_secs(20),
                        ..Default::default()
                    },
                    // ResourceInspectorPlugin::<DungeonSettings>::default()
                    //     .run_if(state_exists_and_equals(GeneratorStage::Finished)),
                    // StateInspectorPlugin::<RequestedMenu>::default(),
                    // StateInspectorPlugin::<GeneratorStage>::default(),
                ))
                // .insert_resource(DebugTimer(Timer::from_seconds(10.0, TimerMode::Repeating)))
                // TODO: refactor these systems into nice sets and stages
                .add_systems(
                    Update,
                    (debug_visualize_spawner, debug_visualize_weapon_spawn_point)
                        .run_if(state_exists_and_equals(AppState::PlayingGame)),
                );

            debug_dump_graphs(app);
        }
    }

    /// query's spawners and creates debug representations for spawner area
    fn debug_visualize_spawner(
        mut cmds: Commands,
        spawner_query: Query<(Entity, &Transform, &Spawner), Without<Fill>>,
    ) {
        for (entity, _transform, spawner) in &spawner_query {
            let spawner_box_visual = svg_shapes::Rectangle {
                extents: Vec2 { x: 40.0, y: 40.0 },
                origin: svg_shapes::RectangleOrigin::Center,
            };

            let spawner_radius_visual = svg_shapes::Circle {
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
    #[allow(clippy::type_complexity)]
    fn debug_visualize_weapon_spawn_point(
        mut cmds: Commands,
        weapon_query: Query<
            // this is equivalent to if player has a weapon equipped and out
            (Entity, &WeaponStats, &Transform),
            (With<Parent>, With<CurrentlySelectedWeapon>),
        >,
    ) {
        for (ent, _w_stats, _trans) in &weapon_query {
            let spawner_box_visual = svg_shapes::Rectangle {
                extents: Vec2 { x: 2.0, y: 2.0 },
                origin: svg_shapes::RectangleOrigin::Center,
            };

            let spawner_visual_bundle = GeometryBuilder::new().add(&spawner_box_visual).build();

            cmds.entity(ent).insert(spawner_visual_bundle);
        }
    }

    /// dumps scheduling graphs for given App
    pub fn debug_dump_graphs(app: &mut App) {
        let target = Path::new(".schedule");
        match fs::try_exists(target) {
            Err(error) => {
                warn!("problem with {:?} directory: {}", target, error);
            }
            Ok(exists) => {
                if !exists {
                    warn!(
                        "Not dumping schedules because {:?} directory does not exist",
                        target
                    );
                    warn!(
                        "Create {:?} directory in cwd too dump schedule graphs",
                        target
                    );
                    return;
                }
                warn!("Dumping graphs");

                let schedule_theme = schedule_graph::settings::Style::dark_github();
                let render_theme = render_graph::settings::Style::dark_github();

                let settings = schedule_graph::Settings {
                    ambiguity_enable: false,
                    ambiguity_enable_on_world: false,
                    style: schedule_theme,
                    collapse_single_system_sets: true,
                    prettify_system_names: true,
                    ..Default::default()
                };

                let render_graph_settings = render_graph::Settings {
                    style: render_theme,
                };

                let pre_startup_graph = schedule_graph_dot(app, PreStartup, &settings);
                let main_startup_graph = schedule_graph_dot(app, Startup, &settings);
                let post_startup_graph = schedule_graph_dot(app, PostStartup, &settings);
                let first_schedule = schedule_graph_dot(app, First, &settings);
                let pre_update_schedule = schedule_graph_dot(app, PreUpdate, &settings);
                let main_update_schedule = schedule_graph_dot(app, Update, &settings);
                let post_update_schedule = schedule_graph_dot(app, PostUpdate, &settings);
                let last_schedule = schedule_graph_dot(app, Last, &settings);

                let render_graph = render_graph_dot(app, &render_graph_settings);

                write_graphs(
                    target.to_path_buf(),
                    (
                        pre_startup_graph,
                        main_startup_graph,
                        post_startup_graph,
                        first_schedule,
                        pre_update_schedule,
                        main_update_schedule,
                        post_update_schedule,
                        last_schedule,
                        render_graph,
                    ),
                );
            }
        }
    }

    /// dumps schedule as a graph
    fn write_graphs(
        folder: PathBuf,
        dotfiles: (
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
        ),
    ) {
        let (
            pre_startup_graph,
            main_startup_graph,
            post_startup_graph,
            first_schedule,
            pre_update_schedule,
            main_update_schedule,
            post_update_schedule,
            last_schedule,
            render_graph,
        ) = dotfiles;

        match fs::write(folder.join("0-pre_startup_schedule.dot"), pre_startup_graph) {
            Ok(()) => {}
            Err(e) => warn!("{}", e),
        }
        match fs::write(
            folder.join("1-main_startup_schedule.dot"),
            main_startup_graph,
        ) {
            Ok(()) => {}
            Err(e) => warn!("{}", e),
        }
        match fs::write(folder.join("2-post_startup_graph.dot"), post_startup_graph) {
            Ok(()) => {}
            Err(e) => warn!("{}", e),
        }
        match fs::write(folder.join("3-first_schedule.dot"), first_schedule) {
            Ok(()) => {}
            Err(e) => warn!("{}", e),
        }
        match fs::write(
            folder.join("4-pre_update_schedule.dot"),
            pre_update_schedule,
        ) {
            Ok(()) => {}
            Err(e) => warn!("{}", e),
        }
        match fs::write(
            folder.join("5-main_update_schedule.dot"),
            main_update_schedule,
        ) {
            Ok(()) => {}
            Err(e) => warn!("{}", e),
        }
        match fs::write(
            folder.join("6-post_update_schedule.dot"),
            post_update_schedule,
        ) {
            Ok(()) => {}
            Err(e) => warn!("{}", e),
        }
        match fs::write(folder.join("7-last_schedule.dot"), last_schedule) {
            Ok(()) => {}
            Err(e) => warn!("{}", e),
        }

        match fs::write(folder.join("Z-render_graph.dot"), render_graph) {
            Ok(()) => {}
            Err(e) => warn!("{}", e),
        }
    }
}
