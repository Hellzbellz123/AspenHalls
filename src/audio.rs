use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl, AudioPlugin};

use crate::{game::GameStage, loading::assets::AudioHandles};

pub struct InternalAudioPlugin;

// This plugin is responsible to control the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .add_system_set(SystemSet::on_enter(GameStage::Playing).with_system(start_audio));
        // .add_system_set(
        //     SystemSet::on_update(GameState::Playing).with_system(control_flying_sound),
        // );
    }
}

fn start_audio(audio_assets: Res<AudioHandles>, audio: Res<Audio>) {
    audio.set_volume(0.1);
    audio.play(audio_assets.gamesoundtrack.clone()).looped();
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
