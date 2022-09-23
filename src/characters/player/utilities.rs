use crate::{
    action_manager::bindings::PlayerInput,
    characters::player::{PlayerBundle, PlayerState},
    Layer, PLAYER_SIZE, TILE_SIZE,
};
use bevy::prelude::*;
use heron::{CollisionLayers, CollisionShape, PhysicMaterial, RotationConstraints, Velocity};

use super::animation::{self, AnimState, FacingDirection};

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

pub fn spawn_player(mut commands: Commands, characters: Res<animation::CharacterSheet>) {
    commands
        .spawn_bundle(PlayerBundle {
            player_animations: AnimState {
                timer: Timer::from_seconds(0.2, true),
                frames: characters.player_idle.to_vec(),
                current_frame: 0,
            },
            player_data: PlayerState {
                speed: 150.0,
                sprint_available: false,
                facing: FacingDirection::Idle,
                just_moved: false,
                target_positon: Some(Vec2::ZERO),
            },
            player_sprite_sheet: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    custom_size: Some(PLAYER_SIZE), //character is 1 tile wide by 2 tiles wide
                    ..default()
                },
                texture_atlas: characters.handle.clone(),
                transform: Transform::from_xyz(0.0, 30.0, 8.0),
                // global_transform:  , // Vec3::new(0.0, 0.0, 8.0)
                ..default()
            },
            player_input_map: PlayerInput::default(),
            name: Name::new("player"),
            rigidbody: super::RigidBodyBundle {
                rigidbody: heron::RigidBody::Dynamic,
                velocity: Velocity::default(),
                physicsmat: PhysicMaterial {
                    friction: 0.5,
                    density: 10.0,
                    ..Default::default()
                },
                rconstraints: RotationConstraints::lock(),
                collisionlayers: CollisionLayers::none()
                    .with_group(Layer::Player)
                    .with_mask(Layer::World),
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
