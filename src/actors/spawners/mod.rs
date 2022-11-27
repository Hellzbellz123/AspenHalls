//TODO: refactor enemy spawning into events.
// event should look kinda like
//
// struct SpawnSkeletonEvent {
//     position_to_spawn: Vec3
// }
//
// enemy spawner entity with position
//struct SpawnerEntityBundle {
//  transform: <>
//  spawner: {
//      Spawner {
//          enemy_to_spawn: <>
//          spawn_radius: <>
//          max_enemy_in_area: <>
//          spawn_timer: <>
//  }
//}
//}
// or possibly just a catch all event with what type of enemy to spawn along with position, amount to spawn should also be added, along with a radius, select random vector3 from within the radius and spawn 1 enemy at that point.
//
// Transform::from_xyz(
//     rng.gen_range(-470.0..520.0),
//     rng.gen_range(2818.0..3805.0),
//     8.0,
// ),
//
//not sure how to deal with enemys being spawned in colliders. can possible scan in each direction and move to whichever direction has the least amount of colliders? maybe check spawning positon for collider first, if no collider then spawn?

pub mod test;

use bevy::{prelude::*, time::Timer};
use bevy_rapier2d::prelude::{
    Collider, ColliderMassProperties, Damping, Friction, LockedAxes, Restitution, Velocity,
};
use big_brain::{
    prelude::FirstToScore,
    thinker::{Thinker, ThinkerBuilder},
};

use crate::{
    actors::enemies::skeleton::SkeletonBundle,
    components::actors::{
        ai::{
            AIAggroDistance, AIAttackAction, AIAttackTimer, AIEnemy, AIIsAggroed, AIMeanderAction,
            ActorType, TypeEnum,
        },
        animation::{AnimState, AnimationSheet, FacingDirection},
        bundles::{ActorColliderBundle, BigBrainBundle, RigidBodyBundle},
        general::ActorState,
        spawners::{EnemyContainerTag, EnemyType, SpawnEvent, Spawner, SpawnerTimer},
    },
    game::GameStage,
    loading::assets::EnemyTextureHandles,
    utilities::game::{SystemLabels, ACTOR_PHYSICS_LAYER, ACTOR_SIZE},
};

// commands
//     .spawn((
//         Name::new("EnemyContainer"),
//         SpatialBundle {
//             visibility: Visibility::VISIBLE,
//             transform: Transform::from_xyz(0.0, 0.0, 0.0),
//             ..default()
//         },
//     ))
//     .with_children(|parent| {
//         }
//     });
pub struct SpawnerPlugin;

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnEvent>().add_system_set(
            SystemSet::on_enter(GameStage::Playing)
                .with_system(on_enter)
                .with_system(catch_spawn_event)
                .label(SystemLabels::Spawn),
        );
    }
}

pub fn on_enter(mut cmds: Commands) {
    info!("spawning enemy container");
    cmds.spawn((
        Name::new("EnemyContainer"),
        EnemyContainerTag,
        SpatialBundle {
            visibility: Visibility::VISIBLE,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
    ));

    // spawn spawner at x: -644.16, y: 2342, z: 9.0
    // spawn spawner at x: 47, y: 3293, z: 8.0
    info!("spawning entity spawners");
    cmds.spawn((
        Name::new("Spawner_Outside"),
        Spawner {
            enemy_to_spawn: EnemyType::Skeleton,
            spawn_radius: 300.0,
            max_enemies: 7,
        },
        SpawnerTimer(Timer::from_seconds(5.0, TimerMode::Repeating)),
        Transform {
            translation: Vec3::new(-644.16, 2342.0, 8.0),
            ..default()
        },
    ));
}

pub fn catch_spawn_event(
    entity_container: Query<Entity, With<EnemyContainerTag>>,
    mut events: EventReader<SpawnEvent>,
    mut commands: Commands,
    enemyassets: Res<EnemyTextureHandles>,
) {
    for event in events.iter() {
        match event.enemy_to_spawn {
            EnemyType::Skeleton => {
                for _ in 0..event.spawn_count {
                    commands
                        .get_entity(entity_container.single())
                        .expect("should always be atleast one entity container. if this panics we probably made more than 1")
                        .add_children(|parent| {
                            parent
                                .spawn((
                                    SkeletonBundle {
                                        name: Name::new("Skeleton"),
                                        actortype: AIEnemy,
                                        actorstate: ActorState {
                                            speed: 100.0,
                                            sprint_available: false,
                                            facing: FacingDirection::Idle,
                                            just_moved: false,
                                        },
                                        animation_state: AnimState {
                                            timer: Timer::from_seconds(0.2, TimerMode::Repeating),
                                            current_frames: vec![0, 1, 2, 3, 4],
                                            current_frame: 0,
                                        },
                                        available_animations: AnimationSheet {
                                            handle: enemyassets.skele_full_sheet.clone(),
                                            idle_animation: [0, 1, 2, 3, 4],
                                            down_animation: [5, 6, 7, 8, 9],
                                            up_animation: [10, 11, 12, 13, 14],
                                            right_animation: [15, 16, 17, 18, 19],
                                        },
                                        sprite: SpriteSheetBundle {
                                            sprite: TextureAtlasSprite {
                                                custom_size: Some(ACTOR_SIZE), //character is 1 tile wide by 2 tiles wide
                                                ..default()
                                            },
                                            texture_atlas: enemyassets.skele_full_sheet.clone(),
                                            transform: Transform::from_translation(
                                                event.spawn_position,
                                            ),
                                            ..default()
                                        },
                                        rigidbody: RigidBodyBundle {
                                            rigidbody: bevy_rapier2d::prelude::RigidBody::Dynamic,
                                            velocity: Velocity::zero(),
                                            friction: Friction::coefficient(0.7),
                                            howbouncy: Restitution::coefficient(0.3),
                                            massprop: ColliderMassProperties::Density(0.3),
                                            rotationlocks: LockedAxes::ROTATION_LOCKED,
                                            dampingprop: Damping {
                                                linear_damping: 1.0,
                                                angular_damping: 1.0,
                                            },
                                        },
                                        brain: BigBrainBundle {
                                            actortype: ActorType(TypeEnum::Enemy),
                                            aggrodistance: AIAggroDistance { distance: 200.0 },
                                            aiattacktimer: AIAttackTimer {
                                                timer: Timer::from_seconds(
                                                    9.5,
                                                    TimerMode::Repeating,
                                                ),
                                                is_attacking: false,
                                                is_near: false,
                                            },
                                            thinker: ThinkerBuilder::default(),
                                        },
                                    },
                                    //ai components
                                    AIAggroDistance { distance: 200.0 },
                                    AIAttackTimer {
                                        timer: Timer::from_seconds(2., TimerMode::Repeating),
                                        is_attacking: false,
                                        is_near: false,
                                    },
                                    Thinker::build()
                                        .picker(FirstToScore { threshold: 1.0 })
                                        .when(AIIsAggroed, AIAttackAction)
                                        .otherwise(AIMeanderAction), // .otherwise(IsMeandering),
                                ))
                                .with_children(|child| {
                                    child.spawn(ActorColliderBundle {
                                        transform_bundle: TransformBundle {
                                            local: (Transform {
                                                translation: (Vec3 {
                                                    x: 0.,
                                                    y: -5.,
                                                    z: ACTOR_PHYSICS_LAYER,
                                                }),
                                                ..default()
                                            }),
                                            ..default()
                                        },
                                        collider: Collider::capsule_y(10.4, 13.12),
                                    });
                                });
                        })
                }
            }
            EnemyType::Slime => {
                info!("not implemented yet")
            }
        }
    }
}
