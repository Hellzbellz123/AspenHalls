use bevy::{
    prelude::{Query, With, *},
    sprite::Sprite,
};
use leafwing_input_manager::prelude::ActionState;

use crate::{
    action_manager::actions::GameActions, characters::player::PlayerComponent, game::TimeInfo,
};

pub fn player_movement_system(
    timeinfo: ResMut<TimeInfo>,
    query_action_state: Query<&ActionState<GameActions>>,
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &mut PlayerComponent)>,
    mut sprite_query: Query<(&mut Sprite, With<PlayerComponent>)>,
) {
    let _movement_dir = Vec3::ZERO;
    let (mut player_transform, mut player) = player_query.single_mut();
    let mut sprite = sprite_query.single_mut();
    let timeinfo = timeinfo.as_ref();

    if player.sprint_available {
        player.speed = 255.0
    }

    if !player.sprint_available {
        player.speed = 100.0
    }

    Vec3::clamp(
        player_transform.translation,
        Vec3::ZERO,
        Vec3::new(255., 255., 255.),
    );

    let action_state = query_action_state.single();
    if action_state.pressed(GameActions::Move) {
        // Virtual direction pads are one of the types which return an AxisPair. The values will be
        // represented as `-1.0`, `0.0`, or `1.0` depending on the combination of buttons pressed.
        let axis_pair = action_state.axis_pair(GameActions::Move).unwrap();

        let horizontal = axis_pair.x();
        let vertical = axis_pair.y();
        let mut velocity = Vec3::ZERO;

        if horizontal <= 0.0 && !timeinfo.game_paused {
            velocity.x += horizontal
                * player.speed
                * time.delta_seconds()
                * timeinfo.time_step.clamp(-1.0, 1.0);
            sprite.0.flip_x = false;
        }

        if horizontal >= 0.0 && !timeinfo.game_paused {
            velocity.x += horizontal
                * player.speed
                * time.delta_seconds()
                * timeinfo.time_step.clamp(-1.0, 1.0);
            sprite.0.flip_x = true;
        }

        if vertical <= 0.0 && !timeinfo.game_paused {
            velocity.y += vertical * player.speed * time.delta_seconds() * timeinfo.time_step;
        }

        if vertical >= 0.0 && !timeinfo.game_paused {
            player_transform.translation.y +=
                vertical * player.speed * time.delta_seconds() * timeinfo.time_step;
        }
        player_transform.translation += velocity; //.clamp(Vec3::new(-1.0, -1.0, 0.0), Vec3::new(1.0, 1.0, 0.0))
    }
}

pub fn player_sprint(
    input_query: Query<&ActionState<GameActions>, With<PlayerComponent>>,
    mut player_query: Query<(&mut Sprite, &mut PlayerComponent)>,
) {
    let action_state = input_query.single();
    let (mut _player_sprite, mut player) = player_query.single_mut();

    if action_state.pressed(GameActions::Dash) {
        player.sprint_available = true;
    }

    if action_state.released(GameActions::Dash) {
        player.sprint_available = false;
    }
}

// pub const PLAYER_SPRITE_SCALE: f32 = 2.0;

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
