use bevy::prelude::*;
use bevy_kira_audio::{prelude::AudioControl, AudioApp, AudioChannel, AudioPlugin};
use rand::seq::SliceRandom;
use std::time::Duration;

use crate::{
    launch_config::SoundSettings,
    game::{
        actors::{
            animation::components::{ActorAnimationType, AnimState},
            components::Player,
        },
        AppStage,
    },
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

/// footstep timer
#[derive(Resource)]
pub struct WalkingSoundTimer {
    /// timer for steps
    pub timer: Timer,
    /// is first step?
    pub is_first_time: bool,
}

/// audio plugin
pub struct InternalAudioPlugin;

// This plugin is responsible to control the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioPlugin)
            .add_audio_channel::<Music>()
            .add_audio_channel::<Ambience>()
            .add_audio_channel::<Sound>()
            .insert_resource(WalkingSoundTimer {
                timer: Timer::from_seconds(0.65, TimerMode::Repeating),
                is_first_time: true,
            })
            .add_systems(
                Update,
                (
                    player_walking_sound_system
                        .run_if(state_exists_and_equals(AppStage::PlayingGame)),
                    play_background_audio.run_if(run_once()),
                )
                    .run_if(resource_exists::<AudioHandles>()),
            )
            .add_systems(Startup, setup_sound_volume);
    }
}

/// initial volume from sound settings
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

/// play game soundtrack
fn play_background_audio(audio_assets: Res<AudioHandles>, audio: Res<AudioChannel<Music>>) {
    audio.play(audio_assets.gamesoundtrack.clone()).looped();
}

/// play walking sound
fn player_walking_sound_system(
    audio_assets: Res<AudioHandles>,
    mut player_query: Query<(&mut AnimState, &mut Player)>,
    mut walksound_res: ResMut<WalkingSoundTimer>,
    audio: Res<AudioChannel<Sound>>,
    time: Res<Time>,
) {
    let (anim_data, player_data) = player_query.single_mut();
    if anim_data.facing == ActorAnimationType::Idle {
        walksound_res.timer.reset();
        walksound_res.is_first_time = true;
    } else {
        if !player_data.sprint_available {
            walksound_res.timer.set_duration(Duration::from_millis(650));
        } else if player_data.sprint_available {
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
