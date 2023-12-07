use bevy::prelude::*;
use bevy_kira_audio::{prelude::AudioControl, AudioApp, AudioChannel, AudioPlugin};
use rand::seq::SliceRandom;
use std::time::Duration;

use crate::{
    game::{
        actors::{
            animation::components::{ActorAnimationType, AnimState},
            components::{ActorMoveState, MoveStatus, Player},
        },
        AppState,
    },
    loading::assets::AudioHandles,
    loading::config::SoundSettings,
};

/// OST music is played on this channel.
#[derive(Resource, Component)]
pub struct MusicSoundChannel;

/// Sound Channel intended for menu sounds/creaking/1etc atmospheric sounds
#[derive(Resource, Component)]
pub struct AmbienceSoundChannel;

/// `AudioChannel` for footsteps/grunts/etc of npc/enemy/player, weapon sounds.
/// can be used to tell if enemies exist?
#[derive(Resource, Component)]
pub struct GameSoundChannel;

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
            .add_audio_channel::<MusicSoundChannel>()
            .add_audio_channel::<AmbienceSoundChannel>()
            .add_audio_channel::<GameSoundChannel>()
            .insert_resource(WalkingSoundTimer {
                timer: Timer::from_seconds(0.65, TimerMode::Repeating),
                is_first_time: true,
            })
            .add_systems(
                Update,
                (
                    player_walking_sound_system
                        .run_if(state_exists_and_equals(AppState::PlayingGame)),
                    play_background_audio.run_if(run_once()),
                )
                    .run_if(resource_exists::<AudioHandles>()),
            )
            .add_systems(OnEnter(AppState::Loading), setup_sound_volume);
    }
}

/// initial volume from sound settings
fn setup_sound_volume(
    sound_settings: ResMut<SoundSettings>,
    music_channel: Res<AudioChannel<MusicSoundChannel>>,
    ambience_channel: Res<AudioChannel<AmbienceSoundChannel>>,
    sound_channel: Res<AudioChannel<GameSoundChannel>>,
) {
    let mastervolume = sound_settings.master_volume;
    music_channel.set_volume(sound_settings.music_volume * mastervolume);
    ambience_channel.set_volume(sound_settings.ambience_volume * mastervolume);
    sound_channel.set_volume(sound_settings.sound_volume * mastervolume);
}

/// play game soundtrack
fn play_background_audio(
    audio_assets: Res<AudioHandles>,
    audio: Res<AudioChannel<MusicSoundChannel>>,
) {
    audio.play(audio_assets.game_soundtrack.clone()).looped();
}

// TODO: make generic across actors and use spatial sound emitters on entitys
/// play walking sound
fn player_walking_sound_system(
    mut walk_sound_res: ResMut<WalkingSoundTimer>,
    game_sound: Res<AudioChannel<GameSoundChannel>>,
    actor_query: Query<&ActorMoveState>,
    audio: Res<AudioHandles>,
    time: Res<Time>,
) {
    for actor in &actor_query {
        match &actor.move_status {
            MoveStatus::NoMovement => {
                walk_sound_res.timer.reset();
                walk_sound_res.is_first_time = true;
            }
            moving => {
                match moving {
                    MoveStatus::Run => {
                        walk_sound_res
                            .timer
                            .set_duration(Duration::from_millis(150));
                    }
                    MoveStatus::Walk => {}
                    _ => panic!("wild"),
                }
                walk_sound_res.timer.tick(time.delta());
                if walk_sound_res.timer.finished() || walk_sound_res.is_first_time {
                    if walk_sound_res.is_first_time {
                        walk_sound_res.is_first_time = false;
                        return;
                    }
                    let mut index = rand::thread_rng();

                    let step_sound_handle = audio
                        .footsteps
                        .choose(&mut index)
                        .expect("SHOULD NEVER BE EMPTY")
                        .clone();
                    game_sound.play(step_sound_handle);
                }
            }
        }
    }
}
