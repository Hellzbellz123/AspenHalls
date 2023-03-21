use bevy::prelude::*;
use bevy_kira_audio::{prelude::AudioControl, AudioApp, AudioChannel, AudioPlugin};
use rand::seq::SliceRandom;
use std::time::Duration;

use crate::{
    app_config::SoundSettings,
    components::actors::{
        animation::FacingDirection,
        general::{MovementState, Player},
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
pub struct WalkingSoundTimer {
    pub timer: Timer,
    pub is_first_time: bool,
}

pub struct InternalAudioPlugin;

// This plugin is responsible to control the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .add_audio_channel::<Music>()
            .add_audio_channel::<Ambience>()
            .add_audio_channel::<Sound>()
            .insert_resource(WalkingSoundTimer {
                timer: Timer::from_seconds(0.65, TimerMode::Repeating),
                is_first_time: true,
            })
            .add_system(play_background_audio.in_schedule(OnEnter(GameStage::StartMenu)))
            .add_system(player_walking_sound_system.in_set(OnUpdate(GameStage::PlaySubStage)))
            .add_startup_system(setup_sound_volume);
    }
}

fn setup_sound_volume(
    sound_settings: ResMut<SoundSettings>,
    bgm: Res<AudioChannel<Music>>,
    bga: Res<AudioChannel<Ambience>>,
    bgs: Res<AudioChannel<Sound>>,
) {
    let mastervolume = sound_settings.mastervolume;
    bgm.set_volume(sound_settings.musicvolume * mastervolume);
    bga.set_volume(sound_settings.ambiencevolume * mastervolume);
    bgs.set_volume(sound_settings.soundvolume * mastervolume);
}

fn play_background_audio(audio_assets: Res<AudioHandles>, audio: Res<AudioChannel<Music>>) {
    audio.play(audio_assets.gamesoundtrack.clone()).looped();
}

fn player_walking_sound_system(
    audio_assets: Res<AudioHandles>,
    mut player_query: Query<&mut MovementState, With<Player>>,
    mut walksound_res: ResMut<WalkingSoundTimer>,
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
