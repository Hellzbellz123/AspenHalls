use bevy::prelude::*;
use bevy_kira_audio::{prelude::AudioControl, AudioApp, AudioChannel, AudioPlugin};
use rand::seq::SliceRandom;
use std::time::Duration;

use crate::{
    game::{
        actors::{
            animation::components::{ActorAnimationType, AnimState},
            components::Player,
        },
        AppStage,
    },
    loading::assets::AudioHandles,
    loading::config::SoundSettings,
};

/// music is played in this channel
#[derive(Resource, Component)]
pub struct Music;
/// nothing is currently played here, intended for menu sounds/creaking/etc atmospheric sounds
#[derive(Resource, Component)]
pub struct Ambience;
/// this audio is for everything gameplay related, footsteps of npc/enemy can be used to tell if enemies exist?
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
    music_channel: Res<AudioChannel<Music>>,
    ambience_channel: Res<AudioChannel<Ambience>>,
    sound_channel: Res<AudioChannel<Sound>>,
) {
    let mastervolume = sound_settings.master_volume;
    music_channel.set_volume(sound_settings.music_volume * mastervolume);
    ambience_channel.set_volume(sound_settings.ambience_volume * mastervolume);
    sound_channel.set_volume(sound_settings.sound_volume * mastervolume);
}

/// play game soundtrack
fn play_background_audio(audio_assets: Res<AudioHandles>, audio: Res<AudioChannel<Music>>) {
    audio.play(audio_assets.game_soundtrack.clone()).looped();
}

// TODO: make generic across actors and use spatial sound emitters on entitys
/// play walking sound
fn player_walking_sound_system(
    audio_assets: Res<AudioHandles>,
    mut player_query: Query<(&mut AnimState, &mut Player)>,
    mut walk_sound_res: ResMut<WalkingSoundTimer>,
    audio: Res<AudioChannel<Sound>>,
    time: Res<Time>,
) {
    let (anim_data, player_data) = player_query.single_mut();
    if anim_data.animation_type == ActorAnimationType::Idle {
        walk_sound_res.timer.reset();
        walk_sound_res.is_first_time = true;
    } else {
        if !player_data.sprint_available {
            walk_sound_res
                .timer
                .set_duration(Duration::from_millis(650));
        } else if player_data.sprint_available {
            walk_sound_res
                .timer
                .set_duration(Duration::from_millis(150));
        }

        walk_sound_res.timer.tick(time.delta());
        if walk_sound_res.timer.finished() || walk_sound_res.is_first_time {
            if walk_sound_res.is_first_time {
                walk_sound_res.is_first_time = false;
                walk_sound_res.timer.reset();
            }
            let mut index = rand::thread_rng();

            let step_sound_handle = audio_assets
                .footsteps
                .choose(&mut index)
                .expect("SHOULD NEVER BE EMPTY")
                .clone();
            audio.play(step_sound_handle);
        }
    }
}
