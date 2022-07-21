use crate::loading::AudioAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioPlugin};

pub struct InternalAudioPlugin;

// This plugin is responsible to control the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(start_audio));
        // .add_system_set(
        //     SystemSet::on_update(GameState::Playing).with_system(control_flying_sound),
        // );
    }
}

fn start_audio(audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    audio.set_volume(0.1);
    audio.play_looped(audio_assets.gamesoundtrack.clone());
}

// /// Tag for basic (1 row) animation
// #[derive(Component)]
// pub struct BasicAnimationComponent;

// /// Animate basic (1 row) animations
// pub fn basic_sprite_animation_system(
//     time: Res<Time>,
//     texture_atlases: Res<Assets<TextureAtlas>>,
//     mut query: Query<
//         (&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>),
//         With<BasicAnimationComponent>,
//     >,
// ) {
//     for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
//         timer.tick(time.delta());
//         if timer.finished() {
//             let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
//             sprite.index = (sprite.index as usize + 1) % texture_atlas.textures.len();
//         }
//     }
// }

// /// Animate a characters (people, includes player)
// pub fn animate_character_system(
//     time: Res<Time>,
//     character_animations: Res<CharacterAnimationResource>,
//     mut animation_query: Query<(&mut CharacterAnimationComponent, &mut TextureAtlasSprite)>,
// ) {
//     for (mut character_animation, mut sprite) in animation_query.iter_mut() {
//         character_animation.timer.tick(time.delta());

//         if character_animation.timer.finished() {
//             let animation_idxs =
//                 character_animations.animations[&character_animation.animation_type];
//             if sprite.index == animation_idxs.1 as usize {
//                 sprite.index = animation_idxs.0 as usize;
//             } else {
//                 sprite.index += 1;
//             }
//         }
//     }
// }

// /// Stores data about character animations frames (data/character_animations.ron)
// #[derive(Deserialize)]
// pub struct CharacterAnimationResource {
//     // start and end indexes of animations
//     pub animations: HashMap<CharacterAnimationType, (u32, u32, f32)>,
// }

// /// Types of character animations
// #[derive(Deserialize, Hash, PartialEq, Eq, Clone)]
// pub enum CharacterAnimationType {
//     ForwardIdle,
//     LeftIdle,
//     BackwardIdle,
//     RightIdle,
//     ForwardMove,
//     LeftMove,
//     BackwardMove,
//     RightMove,
// }

// impl CharacterAnimationType {
//     fn is_idle(&self) -> bool {
//         matches!(
//             self,
//             CharacterAnimationType::ForwardIdle
//                 | CharacterAnimationType::BackwardIdle
//                 | CharacterAnimationType::LeftIdle
//                 | CharacterAnimationType::RightIdle
//         )
//     }
// }

// /// Used for tracking animations of a character entity
// #[derive(Component)]
// pub struct CharacterAnimationComponent {
//     pub timer: Timer,
//     pub animation_type: CharacterAnimationType,
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

// pub const PLAYER_SPRITE_SCALE: f32 = 2.0;

// /// Stores core attributes of player
// #[derive(Debug, Component)]
// pub struct PlayerComponent {
//     pub speed: f32,
// }

// /// Spawns a player
// pub fn spawn_player(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     character_animations: Res<CharacterAnimationResource>,
//     mut texture_atlases: ResMut<Assets<TextureAtlas>>,
// ) {
//     // spawn player
//     let character_starting_animation = CharacterAnimationType::ForwardIdle;
//     let texture_handle = asset_server.load("textures/player_spritesheet.png");
//     let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 46.0), 6, 8);
//     let texture_atlas_handle = texture_atlases.add(texture_atlas);

//     let sprite_transform = Transform {
//         translation: Vec3::new(0.0, 0.0, 1),
//         scale: Vec3::new(PLAYER_SPRITE_SCALE, PLAYER_SPRITE_SCALE, 0.0),
//         ..Default::default()
//     };

//     commands
//         .spawn()
//         .insert(PlayerComponent {
//             speed: 1.5,
//         })
//         .insert(CharacterAnimationComponent {
//             timer: Timer::from_seconds(
//                 character_animations.animations[&character_starting_animation].2,
//                 true,
//             ),
//             animation_type: character_starting_animation.clone(),
//         })
//         .insert_bundle(SpriteSheetBundle {
//             texture_atlas: texture_atlas_handle,
//             transform: sprite_transform,
//             sprite: TextureAtlasSprite {
//                 index: character_animations.animations[&character_starting_animation].0 as usize,
//                 ..Default::default()
//             },
//             ..Default::default()
//         })
//         .insert_bundle((
//             Name::new("Player"),
//             PlayerComponent {
//                 speed: 1.5,
//             },
//         ));
// }

// /// Set the player's animation based on what the player is doing
// pub fn set_player_animation_system(
//     keyboard_input: Res<Input<KeyCode>>,
//     character_animations: Res<CharacterAnimationResource>,
//     mut player_query: Query<
//         (
//             &mut CharacterAnimationComponent,
//             &mut TextureAtlasSprite,
//         ),
//         With<PlayerComponent>,
//     >,
// ) {
//     for (mut character_animation, mut sprite, rb_vels) in player_query.iter_mut() {
//         let mut restart_animation = false;

//         // set to idle animation if velocity is 0 and key is released
//         if rb_vels.linvel.x == 0.0 && rb_vels.linvel.y == 0.0 {
//             if keyboard_input.just_released(KeyCode::A)
//                 || keyboard_input.just_released(KeyCode::Left)
//             {
//                 character_animation.animation_type = CharacterAnimationType::LeftIdle;
//                 restart_animation = true;
//             } else if keyboard_input.just_released(KeyCode::D)
//                 || keyboard_input.just_released(KeyCode::Right)
//             {
//                 character_animation.animation_type = CharacterAnimationType::RightIdle;
//                 restart_animation = true;
//             } else if keyboard_input.just_released(KeyCode::W)
//                 || keyboard_input.just_released(KeyCode::Up)
//             {
//                 character_animation.animation_type = CharacterAnimationType::BackwardIdle;
//                 restart_animation = true;
//             } else if keyboard_input.just_released(KeyCode::S)
//                 || keyboard_input.just_released(KeyCode::Down)
//             {
//                 character_animation.animation_type = CharacterAnimationType::ForwardIdle;
//                 restart_animation = true;
//             }
//         }
//         // set to move animation if key pressed
//         if keyboard_input.just_pressed(KeyCode::A) || keyboard_input.just_pressed(KeyCode::Left) {
//             character_animation.animation_type = CharacterAnimationType::LeftMove;
//             restart_animation = true;
//         } else if keyboard_input.just_pressed(KeyCode::D)
//             || keyboard_input.just_pressed(KeyCode::Right)
//         {
//             character_animation.animation_type = CharacterAnimationType::RightMove;
//             restart_animation = true;
//         } else if keyboard_input.just_pressed(KeyCode::W)
//             || keyboard_input.just_pressed(KeyCode::Up)
//         {
//             character_animation.animation_type = CharacterAnimationType::BackwardMove;
//             restart_animation = true;
//         } else if keyboard_input.just_pressed(KeyCode::S)
//             || keyboard_input.just_pressed(KeyCode::Down)
//         {
//             character_animation.animation_type = CharacterAnimationType::ForwardMove;
//             restart_animation = true;
//         }

//         // if animation changed restart the timer, sprite, and set animation type
//         if restart_animation {
//             let animation_data =
//                 character_animations.animations[&character_animation.animation_type];
//             sprite.index = animation_data.0 as usize;
//             character_animation.timer = Timer::from_seconds(animation_data.2, true);
//         }
//     }
// }

// // Hide player in designated states
// pub fn hide_player_system(
//     app_state: Res<State<GameState>>,
//     mut player_sprite_query: Query<&mut TextureAtlasSprite, With<PlayerComponent>>,
// ) {
//     for mut sprite in player_sprite_query.iter_mut() {
//         sprite.color = if app_state.current() == &GameState::PlayerSleepingState
//             || (app_state.current() == &GameState::GameOverState)
//         {
//             Color::NONE
//         } else {
//             Color::WHITE
//         }
//     }
// }

// // Move player by modifying velocity with input
// pub fn player_movement_system(
//     keyboard_input: Res<Input<KeyCode>>,
//     mut player_info: Query<&PlayerComponent>,
//     app_state: Res<State<GameState>>,
// ) {
//     // if we are not playing the game prevent the player from moving
//     if app_state.current() != &GameState::MainGame {
//         return;
//     }

//     for (player, mut rb_vels) in player_info.iter_mut() {
//         // get key presses
//         let up = keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up);
//         let down = keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down);
//         let left = keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
//         let right = keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);

//         // convert to axis multipliers
//         let x_axis = -(left as i8) + right as i8;
//         let y_axis = -(down as i8) + up as i8;

//         // handle movement in x direction
//         if x_axis != 0 {
//             // accelerate to the player's maximum speed stat
//             rb_vels.linvel.x = player.speed * (x_axis as f32) * 1;
//         } else {
//             rb_vels.linvel.x = 0.0;
//         }

//         // handle movement in y direction
//         if y_axis != 0 {
//             // accelerate to the player's maximum speed stat
//             rb_vels.linvel.y = player.speed * (y_axis as f32) * 1;
//         } else {
//             rb_vels.linvel.y = 0.0;
//         }
//     }
// }
