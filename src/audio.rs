use std::time::Duration;

use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl, AudioPlugin};
use rand::Rng;

use crate::{
    actors::player::{animation::FacingDirection, PlayerState},
    game::GameStage,
    loading::assets::AudioHandles,
};

pub struct InternalAudioPlugin;

// This plugin is responsible to control the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .insert_resource(WalkingSound {
                timer: Timer::from_seconds(0.65, true),
                first_time: true,
            })
            .add_system_set(SystemSet::on_enter(GameStage::Playing).with_system(start_audio))
            .add_system_set(
                SystemSet::on_update(GameStage::Playing).with_system(player_walking_sound_system),
            );
    }
}

fn start_audio(audio_assets: Res<AudioHandles>, audio: Res<Audio>) {
    audio.set_volume(0.1);
    audio.play(audio_assets.gamesoundtrack.clone()).looped();
}

pub struct WalkingSound {
    pub timer: Timer,
    pub first_time: bool,
}

pub fn player_walking_sound_system(
    mut ws_res: ResMut<WalkingSound>,
    mut player_query: Query<&mut PlayerState>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
) {
    let playerdata = player_query.single_mut();
    if playerdata.facing == FacingDirection::Idle {
        ws_res.timer.reset();
        ws_res.first_time = true;
    } else {
        if !playerdata.sprint_available {
            ws_res.timer.set_duration(Duration::from_millis(650));
        } else if playerdata.sprint_available {
            ws_res.timer.set_duration(Duration::from_millis(150));
        }

        ws_res.timer.tick(time.delta());
        if ws_res.timer.finished() || ws_res.first_time {
            if ws_res.first_time {
                ws_res.first_time = false;
                ws_res.timer.reset();
            }

            let index = rand::thread_rng().gen_range(1..8);
            audio
                .play(asset_server.load(format!("audio/footstep/footstep-{}.ogg", index).as_str()));
        }
    }
}
