use bevy::{
    prelude::{default, info, BuildChildren, Commands, Name, Query, ResMut, Transform, Vec3, With},
    sprite::{SpriteSheetBundle, TextureAtlasSprite},
    time::Timer,
};
use heron::{CollisionShape, RotationConstraints, Velocity};
use leafwing_input_manager::prelude::ActionState;

use crate::{
    action_manager::actions::PlayerBindables,
    actors::{
        animation::FacingDirection,
        enemies::{skeleton::SkeletonBundle, Enemy},
        player::Player,
        ActorState, RigidBodyBundle,
    },
    loading::assets::EnemyTextureHandles,
    utilities::game::{PhysicsLayers, PLAYER_SIZE, TILE_SIZE},
};

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
