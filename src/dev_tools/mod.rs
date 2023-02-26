mod debug_dirs;
// #[cfg(feature = "dev")]
pub mod debug_plugin {
    use bevy::{
        diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
        prelude::{App, EventReader, *},
    };
    use bevy_debug_text_overlay::OverlayPlugin;
    use bevy_ecs_ldtk::{GridCoords, IntGridCell, LayerMetadata};
    use bevy_inspector_egui::quick::WorldInspectorPlugin;
    use bevy_mod_debugdump::{get_render_graph, get_render_schedule, get_schedule};
    use bevy_prototype_lyon::{
        prelude::{DrawMode, FillMode, GeometryBuilder},
        render::Shape,
        shapes,
    };
    use bevy_rapier2d::{
        prelude::{CollisionEvent, ContactForceEvent},
        render::RapierDebugRenderPlugin,
    };
    use std::{fs, time::Duration};

    use crate::{
        actions::PlayerActions,
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
        ui::MenuState,
        utilities::game::{SystemLabels, AppSettings},
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
                .add_plugin(WorldInspectorPlugin)
                // .add_plugin(StateInspectorPlugin::<GameStage>::default())
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
                .register_type::<MenuState>()
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
                .register_type::<PlayerActions>()
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
                // .add_system_to_stage(CoreStage::PostUpdate, debug_logging)
                .add_system_set(
                    SystemSet::on_update(GameStage::PlaySubStage)
                        .with_system(debug_visualize_spawner)
                        // .with_system(debug_visualize_weapon_spawn_point)
                        .after(SystemLabels::Spawn),
                )
                .insert_resource(DebugTimer(Timer::from_seconds(10.0, TimerMode::Repeating)));
            // .add_system(show_fps)
            // .add_system(show_cursor_position);
        }
    }

    fn debug_visualize_spawner(
        mut cmds: Commands,
        spawner_query: Query<(Entity, &Transform, &Spawner), Without<Shape>>,
    ) {
        for (entity, transform, spawner) in &spawner_query {
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
                .build(
                    DrawMode::Fill(FillMode::color(Color::Hsla {
                        hue: 334.0,
                        saturation: 0.83,
                        lightness: 0.3,
                        alpha: 0.25,
                    })),
                    *transform,
                );
            cmds.entity(entity).insert(spawner_visual_bundle);
        }
    }

    pub fn debug_dump_graphs(app: &mut App) {
        let rsched = get_render_schedule(app);
        let rgraph = get_render_graph(app);
        let appsched = get_schedule(app);
        fs::write("zrenderschedule.dot", rsched).expect("couldnt write render schedule to file");
        fs::write("zrendergraph.dot", rgraph).expect("couldnt write render schedule to file");
        fs::write("zappschedule.dot", appsched).expect("couldnt write render schedule to file");
    }

    fn debug_visualize_weapon_spawn_point(
        mut cmds: Commands,
        #[allow(clippy::type_complexity)]
        // trunk-ignore(clippy/type_complexity)
        _weapon_query: Query<
            // this is equivelent to if player has a weapon equipped and out
            (Entity, &WeaponStats, &Transform),
            (With<Parent>, With<CurrentlySelectedWeapon>, Without<Player>),
        >,
    ) {
        for (ent, _wstats, trans) in &_weapon_query {
            let spawner_box_visual = shapes::Rectangle {
                extents: Vec2 { x: 2.0, y: 2.0 },
                origin: shapes::RectangleOrigin::Center,
            };

            info!("adding visual too weapon {:?}", ent);
            let spawner_visual_bundle = GeometryBuilder::new()
                .add(&spawner_box_visual)
                // .add(&spawner_radius_visual)
                .build(
                    DrawMode::Fill(FillMode::color(Color::Hsla {
                        hue: 334.0,
                        saturation: 0.83,
                        lightness: 0.3,
                        alpha: 0.25,
                    })),
                    *trans,
                );
            cmds.entity(ent).add_children(|parent| {
                parent.spawn(spawner_visual_bundle);
            });
        }
    }

    fn debug_logging(
        time: Res<Time>,
        mut timer: ResMut<DebugTimer>,
        current_gamestate: Res<State<GameStage>>,
        mut collision_events: EventReader<CollisionEvent>,
        mut contact_force_events: EventReader<ContactForceEvent>,
    ) {
        for collision_event in collision_events.iter() {
            info!("Received collision event: {:?}", collision_event);
        }
        for contact_force_event in contact_force_events.iter() {
            info!("Received contact force event: {:?}", contact_force_event);
        }

        if timer.tick(time.delta()).finished() {
            info!("CURRENT GAMESTATE: {:?}", current_gamestate)
        }
    }
}
