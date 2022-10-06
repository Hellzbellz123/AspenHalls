use bevy::{prelude::{Commands, ResMut, Query, Transform, With, info, Name, default, BuildChildren, Vec3}, time::Timer, sprite::{SpriteSheetBundle, TextureAtlasSprite}};
use heron::{Velocity, PhysicMaterial, RotationConstraints, CollisionLayers, CollisionShape};
use leafwing_input_manager::prelude::ActionState;

use crate::{loading::assets::EnemyTextureHandles, action_manager::actions::PlayerBindables, actors::{player::Player, enemies::{skeleton::SkeletonBundle, Enemy}, ActorState, animation::FacingDirection, RigidBodyBundle}, utilities::game::{PLAYER_SIZE, PhysicsLayers, TILE_SIZE}};

pub fn spawn_skeleton_button(
    mut commands: Commands,
    enemyassets: ResMut<EnemyTextureHandles>,
    query_action_state: Query<&ActionState<PlayerBindables>>,
    player_query: Query<(&Transform, With<Player>)>,
) {
    if !query_action_state.is_empty() {
        let actions = query_action_state.get_single().expect("no ents?");

        if actions.just_released(PlayerBindables::Heal) {
            info!("Pressed devtest button");

            if !player_query.is_empty() {
                // let player_transform = ;

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
                        sprite: SpriteSheetBundle {
                            sprite: TextureAtlasSprite {
                                custom_size: Some(PLAYER_SIZE), //character is 1 tile wide by 2 tiles wide
                                ..default()
                            },
                            texture_atlas: enemyassets.skele_full_sheet.clone(),
                            transform: Transform {
                                translation:  player_query.get_single().expect("always a player, hopefully").0.translation,
                                ..default()
                            },
                            ..default()
                        },
                        rigidbody: RigidBodyBundle {
                            rigidbody: heron::RigidBody::Static,
                            velocity: Velocity::default(),
                            physicsmat: PhysicMaterial {
                                friction: 1.0,
                                density: 100.0,
                                ..Default::default()
                            },
                            rconstraints: RotationConstraints::lock(),
                            collisionlayers: CollisionLayers::none()
                                .with_group(PhysicsLayers::Enemies)
                                // .without_mask(PhysicsLayers::TeleportSensor)
                                // .with_mask(PhysicsLayers::World)
                                // .with_mask(PhysicsLayers::Player),
                        },
                    })
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

