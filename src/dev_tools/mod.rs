mod debug_dirs;

// #[cfg(feature = "dev")]
pub mod debug_plugin {
    use bevy::{
        diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
        prelude::{EventReader, *},
    };
    use bevy_ecs_ldtk::{GridCoords, IntGridCell, LayerMetadata};
    use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};
    use bevy_inspector_egui_rapier::InspectableRapierPlugin;
    use bevy_prototype_lyon::{
        prelude::{DrawMode, FillMode, GeometryBuilder},
        render::Shape,
        shapes,
    };
    use bevy_rapier2d::{
        prelude::{CollisionEvent, ContactForceEvent, *},
        render::RapierDebugRenderPlugin,
    };

    use leafwing_input_manager::prelude::ActionState;
    use std::time::Duration;

    use crate::{
        action_manager::actions::PlayerBindables,
        actors::weapons::{DamageType, WeaponStats, WeaponTag},
        components::{
            actors::{
                ai::{
                    AIAttackTimer, AICanChase, AICanWander, AIChaseAction, AIEnemy, AIWanderAction,
                    ActorType, AggroScore, TypeEnum,
                },
                animation::{AnimState, AnimationSheet, FacingDirection},
                bundles::RigidBodyBundle,
                general::{CombatStats, DefenseStats, MovementState, Player, TimeToLive},
                spawners::Spawner,
            },
            DebugTimer, MainCameraTag,
        },
        dev_tools::debug_dirs::debugdir,
        game::{GameStage, TimeInfo},
        loading::assets::GameTextureHandles,
        ui::MenuState,
        utilities::game::{SystemLabels, ACTOR_SIZE},
    };
    use crate::{actors::weapons::WeaponBundle, components::actors::bundles::ActorColliderBundle};

    pub struct DebugPlugin;

    impl Plugin for DebugPlugin {
        fn build(&self, app: &mut App) {
            debugdir();
            app
                // .add_plugin()
                .add_plugin(WorldInspectorPlugin::new())
                .add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin {
                    wait_duration: Duration::from_secs(20),
                    ..Default::default()
                })
                // .add_plugin(InspectorPlugin::<crate::game_world::homeworld::components::InspectableData>::new())
                .register_type::<Timer>()
                //rapier inspectables in this plugin
                .add_plugin(InspectableRapierPlugin)
                .add_plugin(RapierDebugRenderPlugin::default())
                //custom inspectables not from plugins
                .register_inspectable::<MenuState>()
                .register_inspectable::<Spawner>()
                .register_inspectable::<MovementState>()
                .register_inspectable::<CombatStats>()
                .register_inspectable::<DefenseStats>()
                .register_inspectable::<Player>()
                .register_inspectable::<AIEnemy>()
                .register_inspectable::<TypeEnum>()
                .register_inspectable::<AnimationSheet>()
                .register_inspectable::<FacingDirection>()
                .register_inspectable::<TimeInfo>()
                .register_inspectable::<MainCameraTag>() // tells bevy-inspector-egui how to display the struct in the world inspector
                .register_type::<PlayerBindables>()
                .register_type::<AnimState>()
                .register_type::<AIAttackTimer>()
                .register_type::<TimeToLive>()
                // LDTK debug data
                .register_type::<LayerMetadata>()
                .register_type::<IntGridCell>()
                .register_type::<GridCoords>()
                // bigbrain AI
                .register_inspectable::<AggroScore>()
                .register_inspectable::<AICanWander>()
                .register_inspectable::<AICanChase>()
                .register_inspectable::<AIChaseAction>()
                .register_inspectable::<AIWanderAction>()
                .register_inspectable::<ActorType>()
                .add_system_to_stage(CoreStage::PostUpdate, debug_logging)
                .add_system_set(
                    SystemSet::on_update(GameStage::Playing)
                        .with_system(debug_f2_action)
                        .with_system(debug_visualize_spawner)
                        .after(SystemLabels::Spawn),
                )
                .insert_resource(DebugTimer(Timer::from_seconds(10.0, TimerMode::Repeating)));
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

    pub fn debug_f2_action(
        selected_player: Res<GameTextureHandles>,
        query_action_state: Query<&ActionState<PlayerBindables>>,
        mut cmds: Commands,
    ) {
        if !query_action_state.is_empty() {
            let actions = query_action_state.get_single().expect("no ents?");

            if actions.just_released(PlayerBindables::DebugF2) {
                debug!("debug f2 action requested: spawn smg");

                cmds.spawn(WeaponBundle {
                    name: Name::new("Small SMG"),
                    tag: WeaponTag {
                        parent: None,
                        stored_weapon_slot: None,
                    },
                    weaponstats: WeaponStats {
                        damage: 2.0,
                        speed: 0.2,
                    },
                    damagetype: DamageType::KineticRanged,
                    rigidbodybundle: RigidBodyBundle {
                        rigidbody: RigidBody::Dynamic,
                        velocity: Velocity::default(),
                        friction: Friction::coefficient(0.7),
                        howbouncy: Restitution::coefficient(0.3),
                        massprop: ColliderMassProperties::Density(0.3),
                        rotationlocks: LockedAxes::empty(),
                        dampingprop: Damping {
                            linear_damping: 1.0,
                            angular_damping: 1.0,
                        },
                    },
                    spritesheetbundle: SpriteSheetBundle {
                        sprite: TextureAtlasSprite {
                            custom_size: Some(ACTOR_SIZE), //character is 1 tile wide by 2 tiles wide
                            ..default()
                        },
                        texture_atlas: selected_player.small_smg.clone(),
                        transform: Transform::from_xyz(-60.0, 1090.0, 8.0),
                        ..default()
                    },
                })
                .with_children(|child| {
                    child.spawn((
                        ActorColliderBundle {
                            transform_bundle: TransformBundle::default(),
                            collider: Collider::capsule(
                                Vec2 { x: 0.0, y: -32.1 },
                                Vec2 { x: 0.0, y: 17.0 },
                                4.0,
                            ),
                        },
                        CollisionGroups::new(Group::NONE, Group::GROUP_30),
                    ));
                });
            }
        }
    }
}

// fn log_collisions(mut events: EventReader<CollisionEvent>) {
//     for event in events.iter() {
//         if event.is_started() {
//             info!("{:?}", event);
//         }
//     }
// }

// fn debug_collision_events(mut commands: Commands, mut events: EventReader<CollisionEvent>) {
//     events
//         .iter()
//         // We care about when the entities "start" to collide
//         .filter(|e| e.is_started())
//         .filter_map(|event| {
//             let (entity_1, entity_2) = event.rigid_body_entities();
//             let (layers_1, layers_2) = event.collision_layers();
//             if is_player(layers_1) && is_enemy(layers_2) | is_player(layers_2) && is_enemy(layers_1)
//             {
//                 info!("player and enemy collided");
//                 Some(entity_1)
//             } else if is_player(layers_2) && is_sensor(layers_1)
//                 || is_player(layers_1) && is_sensor(layers_2)
//             {
//                 info!("player and sensor collided");
//                 layers_1.groups_bits();
//                 Some(entity_1)
//             } else {
//                 info!("not player or enemy or sensor, we can ignore");
//                 // This event is not the collision between an enemy and the player. We can ignore it.
//                 None
//             }
//         })
//         .for_each(|entity| {
//             // let player = entity.id();
//             info!("{}", entity.id());
//         })
// }
