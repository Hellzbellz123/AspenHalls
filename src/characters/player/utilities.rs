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
                facing: FacingDirection::Down,
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
        .with_children(|children| {
            children
                .spawn()
                .insert(CollisionShape::Cuboid {
                    half_extends: Vec3::new(TILE_SIZE.x / 2.0, TILE_SIZE.y, 0.0),
                    border_radius: None,
                })
                .insert(Transform::from_translation(Vec3::new(0., 0., 0.)));
        });
}

// pub fn animate_sprite(
//     timeinfo: ResMut<TimeInfo>,
//     texture_atlases: Res<Assets<TextureAtlas>>,
//     mut query: Query<(
//         &mut AnimationTimer,
//         &mut TextureAtlasSprite,
//         &Handle<TextureAtlas>,
//     )>,
// ) {
//     for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
//         timer.tick(Duration::from_secs_f32(0.3));
//         if timer.just_finished() & !timeinfo.game_paused {
//             let texture_atlas = texture_atlases
//                 .get(texture_atlas_handle)
//                 .expect("where are our animations?");
//             let tal = texture_atlas.textures.len();

//             info!("sprite index: {:?}, max index: {:?} ", sprite.index, tal);

//             // match tal {
//             //     tal if tal-1 == sprite.index => sprite.index = 0,
//             //     tal if sprite.index < tal-1 => sprite.index += 1, // % texture_atlas.textures.len()
//             //     _ => print!("match error?"),
//             // };

//             if sprite.index >= tal-1 {
//                 info!("sprite index longer than TAL");
//                 sprite.index = 0;
//             } else if tal == sprite.index {
//                 info!("resetting animation loop");
//                 sprite.index = 0;
//             } else if sprite.index < tal-1 {
//                 sprite.index += 1 % texture_atlas.textures.len();
//                 info!("sprite index being increased");
//             }
//         }
//     }
// }

// pub struct WalkingSound {
//     pub timer: Timer,
//     pub first_time: bool,
// }

// pub fn player_walking_sound_system(
//     mut ws_res: ResMut<WalkingSound>,
//     mut player_query: Query<&mut CharacterAnimationComponent, With<PlayerComponent>>,
//     audio: Res<Audio>,
//     asset_server: Res<AssetServer>,
//     time: Res<Time>,
// ) {
//     for char_animation in player_query.iter_mut() {
//         if !char_animation.animation_type.is_idle() {
//             ws_res.timer.tick(time.delta());
//             if ws_res.timer.finished() || ws_res.first_time {
//                 if ws_res.first_time {
//                     ws_res.first_time = false;
//                     ws_res.timer.reset();
//                 }

//                 let index = rand::thread_rng().gen_range(1..8);
//                 audio.play(
//                     asset_server.load(format!("audio/footstep/footstep-{}.mp3", index).as_str()),
//                 );
//             }
//         } else {
//             ws_res.timer.reset();
//             ws_res.first_time = true;
//         }
//     }
// }
