use bevy::prelude::{App, Plugin, SystemSet, *};
use bevy_rapier2d::prelude::*;
use big_brain::{prelude::FirstToScore, thinker::Thinker, BigBrainPlugin};
// use heron::{CollisionShape, PhysicMaterial, RotationConstraints, Velocity};
use rand::prelude::*;

use crate::{
    actors::enemies::skeleton::{actions::on_shoot, utilities::update_skeleton_graphics},
    components::actors::{
        ai::{
            AIAggroDistance, AIAttackAction, AIAttackTimer, AIEnemy, AIIsAggroed, AIMeanderAction,
        },
        animation::{AnimState, AnimationSheet, FacingDirection},
        bundles::{ActorColliderBundle, RigidBodyBundle},
        general::ActorState,
    },
    game::GameStage,
    loading::assets::EnemyTextureHandles,
    utilities::game::{ACTOR_PHYSICS_LAYER, PLAYER_SIZE},
};

pub mod simple_ai;
pub mod skeleton;

const MAX_ENEMIES: i32 = 10;

fn on_enter(mut commands: Commands, enemyassets: Res<EnemyTextureHandles>) {
    let mut rng = rand::thread_rng();

    commands
        .spawn((
            Name::new("EnemyContainer"),
            SpatialBundle {
                visibility: Visibility::VISIBLE,
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
        ))
        .with_children(|parent| {
            for _ in 0..MAX_ENEMIES {
                parent
                    .spawn((
                        skeleton::SkeletonBundle {
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
                                    custom_size: Some(PLAYER_SIZE), //character is 1 tile wide by 2 tiles wide
                                    ..default()
                                },
                                texture_atlas: enemyassets.skele_full_sheet.clone(),
                                transform: Transform::from_xyz(
                                    rng.gen_range(-470.0..520.0),
                                    rng.gen_range(2818.0..3805.0),
                                    8.0,
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
            }
        });
    info!("this only runs when switching to gamestage::playing, setup enemys here")
}

fn on_update() {
    // info!("this runs every frame in gamestage::playing \"sorta\" ");
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BigBrainPlugin)
            .add_plugin(simple_ai::SimpleAIPlugin)
            .add_system_set(
                SystemSet::on_update(GameStage::Playing)
                    .with_system(on_update)
                    .with_system(skeleton::utilities::spawn_skeleton_button)
                    .with_system(on_shoot)
                    .with_system(update_skeleton_graphics),
            )
            .add_system_set(SystemSet::on_enter(GameStage::Playing).with_system(on_enter));
    }
}
