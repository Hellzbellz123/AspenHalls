use crate::{
    action_manager::bindings::PlayerInput,
    characters::player::{PlayerBundle, PlayerComponent},
    loading::assets::RexTextureAssets,
};
use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

pub fn spawn_player(
    mut commands: Commands,
    rex_animations: Res<RexTextureAssets>,
    _animation_set: ResMut<Assets<TextureAtlas>>,
) {
    commands
        .spawn_bundle(PlayerBundle {
            player: PlayerComponent {
                speed: 100.0,
                sprint_available: false,
            },
            psprite: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: (1),
                    custom_size: Some(Vec2::new(32., 64.)),
                    ..Default::default()
                },
                texture_atlas: rex_animations.idle.clone(),
                // transform: todo!(),
                // global_transform: todo!(),
                ..Default::default()
            },
            pinput_map: PlayerInput::default(),
        })
        .insert(AnimationTimer(Timer::from_seconds(0.01, true)));
}

pub fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).expect("where are our animations?");
            let atg = texture_atlas.textures.len();

            if sprite.index == atg {
                sprite.index = 1
            }
            else if sprite.index != atg {
                if sprite.index > atg {
                info!("sprites arent the same length, Skipping.....");
                sprite.index = 1
                }
                else {
                    sprite.index = (sprite.index + 1) % atg;
                }
            }
        }
    }
}

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
