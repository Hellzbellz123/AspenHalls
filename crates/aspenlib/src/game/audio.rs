use rand::seq::SliceRandom;
use std::time::Duration;

use bevy::{prelude::*, utils::HashMap};
use bevy_kira_audio::{
    prelude::{AudioControl, AudioEmitter, SpacialAudio},
    AudioApp, AudioChannel, AudioPlugin as InternalAudioPlugin, AudioSettings,
};
use bevy_rapier2d::dynamics::Velocity;

use crate::{
    consts::MIN_VELOCITY,
    game::characters::components::{CharacterMoveState, CurrentMovement},
    loading::assets::AspenAudioHandles,
    loading::config::SoundSettings,
    AppState,
};

/// OST music is played on this channel.
#[derive(Resource, Component)]
pub struct MusicSoundChannel;

/// Sound Channel intended for menu sounds/creaking/1etc atmospheric sounds
#[derive(Resource, Component)]
pub struct AmbienceSoundChannel;

/// `AudioChannel` for footsteps/grunts/etc of npc/player, weapon sounds.
/// can be used to tell if enemies exist?
#[derive(Resource, Component)]
pub struct GameSoundChannel;

/// footstep timer
#[derive(Debug, Component, Deref, DerefMut)]
pub struct ActorSoundTimer {
    /// timer for steps
    #[deref]
    pub timer: Timer,
    /// is first step?
    pub is_first_time: bool,
}

/// audio plugin
pub struct AudioPlugin;

// This plugin is responsible to control the game audio
impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AudioSettings {
            command_capacity: 512,
            sound_capacity: 512,
        })
        .add_plugins(InternalAudioPlugin)
        .add_audio_channel::<MusicSoundChannel>()
        .add_audio_channel::<AmbienceSoundChannel>()
        .add_audio_channel::<GameSoundChannel>()
        .insert_resource(SpacialAudio {
            max_distance: 150.0,
        })
        .add_systems(OnEnter(AppState::Loading), setup_sound_volume)
        .add_systems(
            Update,
            (
                prepare_actor_spatial_sound,
                actor_footstep_sound_system.run_if(state_exists_and_equals(AppState::PlayingGame)),
                play_background_audio.run_if(run_once()),
            )
                .run_if(resource_exists::<AspenAudioHandles>()),
        );
        // .add_systems(OnEnter(AppState::Loading), setup_sound_volume);
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
    audio_assets: Res<AspenAudioHandles>,
    audio: Res<AudioChannel<MusicSoundChannel>>,
) {
    audio.play(audio_assets.game_soundtrack.clone()).looped();
}

/// applies sound data mapps and a spacial emitter for actors that dont already have emitters
fn prepare_actor_spatial_sound(
    audio: Res<AspenAudioHandles>,
    mut cmds: Commands,
    actors: Query<Entity, (With<CharacterMoveState>, Without<AudioEmitter>)>,
) {
    let mut rng = rand::thread_rng();

    for actor in &actors {
        let mut sound_timers: HashMap<String, ActorSoundTimer> = HashMap::new();
        let mut sound_map: HashMap<String, Handle<AudioSource>> = HashMap::new();

        // footsteps
        let footstep_handle = audio
            .footsteps
            .choose(&mut rng)
            .expect("SHOULD NEVER BE EMPTY")
            .clone();

        let footstep_timer = ActorSoundTimer {
            timer: Timer::new(Duration::from_millis(1000), TimerMode::Once),
            is_first_time: true,
        };
        let key = "Footstep";
        sound_map.insert(key.to_string(), footstep_handle);
        sound_timers.insert(key.to_string(), footstep_timer);

        cmds.entity(actor).insert((
            ActorSoundMap(sound_map),
            ActorSoundTimers(sound_timers),
            AudioEmitter { instances: vec![] },
        ));
    }
}

use bevy_kira_audio::AudioSource;

/// map of sound assets too "soundactionid"
#[derive(Debug, Component, Deref, DerefMut)]
pub struct ActorSoundMap(HashMap<String, Handle<AudioSource>>);

/// map of timers too "soundactionid"
#[derive(Debug, Component, Deref, DerefMut)]
pub struct ActorSoundTimers(HashMap<String, ActorSoundTimer>);

// TODO: make generic across actors and use spatial sound emitters on entitys
/// play walking sound
fn actor_footstep_sound_system(
    // mut audio_instances: ResMut<Assets<AudioInstance>>,
    game_sound: Res<AudioChannel<GameSoundChannel>>,
    mut actor_query: Query<(
        &CharacterMoveState,
        &mut AudioEmitter,
        &ActorSoundMap,
        &mut ActorSoundTimers,
        &Velocity,
    )>,
    time: Res<Time>,
) {
    for (move_state, _spatial_emitter, sound_map, mut sound_timers, velocity) in &mut actor_query {
        let key = "Footstep".to_string();
        let footstep_timer = sound_timers
            .get_mut(&key)
            .expect("timer did not exist in ActorSoundTimers");
        let footstep_handle = sound_map
            .get(&key)
            .expect("audio source did not exist in ActorSoundMap.")
            .to_owned();

        let px_per_sec = velocity.linvel.abs().max_element();
        if px_per_sec <= MIN_VELOCITY {
            return;
        }

        let run_dur = Duration::from_secs_f32(0.3);
        let walk_dur = Duration::from_secs_f32(0.7);
        // info!("actor move state: {:?}", move_state);
        footstep_timer.tick(time.delta());

        match &move_state.move_status {
            (CurrentMovement::Run, _) => {
                if footstep_timer.duration() != run_dur {
                    footstep_timer.set_duration(run_dur);
                }

                if footstep_timer.finished() {
                    let _snd = game_sound.play(footstep_handle);
                    // spatial_emitter.instances.push(snd.handle());
                    footstep_timer.reset();
                }
            }
            (CurrentMovement::Walk, _) => {
                if footstep_timer.duration() != walk_dur {
                    footstep_timer.set_duration(walk_dur);
                }

                if footstep_timer.finished() {
                    game_sound.play(footstep_handle);
                    footstep_timer.reset();
                }
            }
            (CurrentMovement::None, _) => {
                footstep_timer.reset();
            }
        }
    }
}

// MoveStatus::NoMovement => {
//     walk_sound_res.timer.reset();
//     walk_sound_res.is_first_time = true;
// }

// moving => {
//     match moving {
//         MoveStatus::Run => {
//             walk_sound_res
//                 .timer
//                 .set_duration(Duration::from_millis(150));
//         }
//         MoveStatus::Walk => {}
//         _ => panic!("wild"),
//     }

//     walk_sound_res.timer.tick(time.delta());
//     if walk_sound_res.timer.finished() || walk_sound_res.is_first_time {
//         if walk_sound_res.is_first_time {
//             walk_sound_res.is_first_time = false;
//             return;
//         }
//         let mut index = rand::thread_rng();

//         let step_sound_handle = audio
//             .footsteps
//             .choose(&mut index)
//             .expect("SHOULD NEVER BE EMPTY")
//             .clone();
//         game_sound.play(step_sound_handle);
//     }
// }
