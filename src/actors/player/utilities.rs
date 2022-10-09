use crate::{
    action_manager::bindings::PlayerInput,
    actors::{
        animation::{AnimState, AnimationSheet, FacingDirection},
        player::{ActorState, PlayerBundle},
    },
    loading::assets::PlayerTextureHandles,
    utilities::game::{PhysicsLayers, PLAYER_SIZE, TILE_SIZE},
};
use bevy::prelude::*;
use heron::{CollisionLayers, CollisionShape, PhysicMaterial, RotationConstraints, Velocity};

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

pub fn spawn_player(mut commands: Commands, selected_player: Res<PlayerTextureHandles>) {
    commands
        .spawn_bundle(PlayerBundle {
            player_animationstate: AnimState {
                timer: Timer::from_seconds(0.2, true),
                current_frames: vec![0, 1, 2, 3, 4],
                current_frame: 0,
            },
            available_animations: AnimationSheet {
                handle: selected_player.rex_full_sheet.clone(),
                idle_animation: [0, 1, 2, 3, 4],
                down_animation: [5, 6, 7, 8, 9],
                up_animation: [10, 11, 12, 13, 14],
                right_animation: [15, 16, 17, 18, 19],
            },
            player_state: ActorState {
                speed: 150.0,
                sprint_available: false,
                facing: FacingDirection::Idle,
                just_moved: false,
            },
            player_sprite_sheet: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    custom_size: Some(PLAYER_SIZE), //character is 1 tile wide by 2 tiles wide
                    ..default()
                },
                texture_atlas: selected_player.rex_full_sheet.clone(),
                transform: Transform::from_xyz(-60.0, 1090.0, 8.0),
                // global_transform:  , // Vec3::new(0.0, 0.0, 8.0)
                ..default()
            },
            player_input_map: PlayerInput::default(),
            name: Name::new("player"),
            rigidbody: super::RigidBodyBundle {
                rigidbody: heron::RigidBody::Dynamic,
                velocity: Velocity::default(),
                rconstraints: RotationConstraints::lock(),
                collision_layers: CollisionLayers::all_masks::<PhysicsLayers>()
                    .with_group(PhysicsLayers::Player),
                physicsmat: PhysicMaterial {
                    restitution: 0.1,
                    density: 100.0,
                    friction: 0.5,
                }, //PhysicsLayers::Player.layers()
            },
            player: super::Player {
                just_teleported: false,
            },
        })
        .with_children(|parent| {
            parent
                .spawn()
                .insert(CollisionShape::Cuboid {
                    half_extends: Vec3::new(TILE_SIZE.x / 2.0, TILE_SIZE.y / 2.0, 0.0),
                    border_radius: None,
                })
                .insert(Transform::from_translation(Vec3::new(0., -24., 0.)));
        });
}
