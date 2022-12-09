use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use bevy_kira_audio::{prelude::AudioControl, AudioApp, AudioChannel, AudioPlugin};

use crate::{
    components::actors::{
        animation::FacingDirection,
        general::{ActorState, Player},
    },
    game::GameStage,
    loading::assets::AudioHandles,
};

/// music is played in this channel
#[derive(Resource, Component)]
pub struct Music;
/// nothing is currently played here, inteded for menu sounds/creaking/etc atmospheric sounds
#[derive(Resource, Component)]
pub struct Ambience;
/// this audio is for everything gameplay related, footsteps of npc/enemy can be used to tell if enemys exist?
#[derive(Resource, Component)]
pub struct Sound;

#[derive(Resource)]
pub struct WalkingSound {
    pub timer: Timer,
    pub is_first_time: bool,
}
//TODO: make this serialize into a settings.toml file in a saves folder
/// modify to change sound volume settings
#[derive(Inspectable, Debug, Serialize, Deserialize)]
pub struct SoundSettings {
    #[inspectable(min = 0.0, max = 1.0)]
    pub mastervolume: f64,
    #[inspectable(min = 0.0, max = 1.0)]
    pub ambiencevolume: f64,
    #[inspectable(min = 0.0, max = 1.0)]
    pub musicvolume: f64,
    #[inspectable(min = 0.0, max = 1.0)]
    pub soundvolume: f64,
}

// impl FromWorld for SoundSettings {
//     #[allow(unused_variables, reason = "clippy says its unused but it isnt")]
//     fn from_world(world: &mut World) -> Self {
//         SoundSettings {
//             mastervolume: 0.5,
//             ambiencevolume: 1.0,
//             musicvolume: 0.1,
//             soundvolume: 0.5,
//         }
//     }
// }

pub struct InternalAudioPlugin;

// This plugin is responsible to control the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .add_audio_channel::<Music>()
            .add_audio_channel::<Ambience>()
            .add_audio_channel::<Sound>()
            .insert_resource(WalkingSound {
                timer: Timer::from_seconds(0.65, TimerMode::Repeating),
                is_first_time: true,
            })
            .add_system_set(
                SystemSet::on_exit(GameStage::Loading).with_system(play_background_audio),
            )
            .add_system_set(
                SystemSet::on_update(GameStage::Playing).with_system(player_walking_sound_system),
            );
    }
}

fn play_background_audio(audio_assets: Res<AudioHandles>, audio: Res<AudioChannel<Music>>) {
    audio.play(audio_assets.gamesoundtrack.clone()).looped();
}

fn player_walking_sound_system(
    audio_assets: Res<AudioHandles>,
    mut player_query: Query<&mut ActorState, With<Player>>,
    mut walksound_res: ResMut<WalkingSound>,
    audio: Res<AudioChannel<Sound>>,
    time: Res<Time>,
) {
    let playerdata = player_query.single_mut();
    if playerdata.facing == FacingDirection::Idle {
        walksound_res.timer.reset();
        walksound_res.is_first_time = true;
    } else {
        if !playerdata.sprint_available {
            walksound_res.timer.set_duration(Duration::from_millis(650));
        } else if playerdata.sprint_available {
            walksound_res.timer.set_duration(Duration::from_millis(150));
        }

        walksound_res.timer.tick(time.delta());
        if walksound_res.timer.finished() || walksound_res.is_first_time {
            if walksound_res.is_first_time {
                walksound_res.is_first_time = false;
                walksound_res.timer.reset();
            }
            let mut index = rand::thread_rng();

            let chandle = audio_assets
                .footsteps
                .choose(&mut index)
                .expect("SHOULD NEVER BE EMPTY")
                .clone();
            audio.play(chandle);
        }
    }
}
