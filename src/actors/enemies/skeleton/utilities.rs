use bevy::{
    prelude::{
        default, info, BuildChildren, Commands, Entity, Name, Query, ResMut, Transform, Vec3, With,
    },
    sprite::{SpriteSheetBundle, TextureAtlasSprite},
    time::Timer,
};

use big_brain::{
    prelude::{ActionState, Actor, FirstToScore},
    thinker::Thinker,
};

use heron::{CollisionShape, PhysicMaterial, RotationConstraints, Velocity};
use leafwing_input_manager::prelude::ActionState as LActionState;

use crate::{
    action_manager::actions::PlayerBindables,
    actors::{
        animation::{AnimationSheet, FacingDirection},
        components::{Aggroable, Aggroed, AttackPlayer, Attacking, Player},
        enemies::{skeleton::SkeletonBundle, Enemy},
        ActorState, RigidBodyBundle,
    },
    game::TimeInfo,
    loading::assets::EnemyTextureHandles,
    utilities::game::{PhysicsLayers, PLAYER_SIZE, TILE_SIZE},
};

pub fn spawn_skeleton_button(
    mut commands: Commands,
    enemyassets: ResMut<EnemyTextureHandles>,
    query_action_state: Query<&LActionState<PlayerBindables>>,
    player_query: Query<(&Transform, With<Player>)>,
) {
    if !query_action_state.is_empty() {
        let actions = query_action_state.get_single().expect("no ents?");

        if actions.just_released(PlayerBindables::Heal) {
            info!("Pressed devtest button");

            if !player_query.is_empty() {
                commands
                    .spawn_bundle(SkeletonBundle {
                        name: Name::new("Skeleton"),
                        actortype: Enemy,
                        actorstate: ActorState {
                            speed: 100.0,
                            sprint_available: false,
                            facing: FacingDirection::Idle,
                            just_moved: false,
                        },
                        animation_state: crate::actors::animation::AnimState {
                            timer: Timer::from_seconds(0.2, true),
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
                            transform: Transform {
                                translation: player_query
                                    .get_single()
                                    .expect("always a player, hopefully")
                                    .0
                                    .translation,
                                ..default()
                            },
                            ..default()
                        },
                        rigidbody: RigidBodyBundle {
                            rigidbody: heron::RigidBody::Dynamic,
                            velocity: Velocity::default(),
                            rconstraints: RotationConstraints::lock(),
                            collision_layers: PhysicsLayers::Enemy.layers(),
                            physicsmat: PhysicMaterial {
                                restitution: 0.1,
                                density: 1.0,
                                friction: 0.5,
                            },
                        },
                        aggroable: Aggroable { distance: 200.0 },
                    })
                    .insert(Attacking {
                        timer: Timer::from_seconds(5., true),
                        is_attacking: false,
                    })
                    .insert(
                        Thinker::build()
                            .picker(FirstToScore { threshold: 0.8 })
                            .when(Aggroed, AttackPlayer)
                    )
                    .with_children(|skele_parent| {
                        skele_parent
                            .spawn()
                            .insert(CollisionShape::Cuboid {
                                half_extends: Vec3::new(TILE_SIZE.x / 2.0, TILE_SIZE.y / 2.0, 0.0),
                                border_radius: None,
                            })
                            .insert(Transform::from_translation(Vec3::new(0., -24., 0.)));
                    });
            }
        };
    }
}

pub fn update_skeleton_graphics(
    timeinfo: ResMut<TimeInfo>,
    mut enemy_query: Query<(
        &mut Velocity,
        &mut ActorState,
        &mut TextureAtlasSprite,
        Entity,
        With<Enemy>,
    )>,
) {
    if !timeinfo.game_paused {
        enemy_query.for_each_mut(|(velocity, mut enemystate, mut sprite, _ent, _)| {

            if velocity.linear == Vec3::ZERO {
                enemystate.facing = FacingDirection::Idle;
            } else if velocity.linear.x > 5.0 {
                sprite.flip_x = false;
                enemystate.facing = FacingDirection::Right;
            } else if velocity.linear.x < -5.0 {
                sprite.flip_x = true;
                enemystate.facing = FacingDirection::Left;
            } else if velocity.linear.y < -5.0 {
                enemystate.facing = FacingDirection::Down;
            } else if velocity.linear.y > 5.0 {
                enemystate.facing = FacingDirection::Up;
            }
        })
    }
}

