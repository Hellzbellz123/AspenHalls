use rand::seq::SliceRandom;
use std::time::Duration;

use bevy::{prelude::*, utils::HashMap};
use bevy_kira_audio::{
    prelude::{AudioControl, AudioEmitter, SpatialAudio},
    AudioApp, AudioChannel, AudioPlugin as InternalAudioPlugin, AudioSettings,
};
use bevy_rapier2d::dynamics::Velocity;

use crate::{
    consts::MIN_VELOCITY, game::characters::{components::{CharacterMoveState, CurrentMovement}, player::PlayerSelectedHero}, loading::{assets::AspenAudioHandles, config::SoundSettings, splashscreen::MainCamera}, register_types, AppState
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
#[derive(Debug, Component, Deref, DerefMut, Reflect)]
#[reflect(Component)]
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
        register_types!(app, [
            ActorSoundMap,
            ActorSoundTimers,
            ActorSoundTimer
        ]);

        app.insert_resource(AudioSettings {
            command_capacity: 256,
            sound_capacity: 128,
        })
        .add_plugins(InternalAudioPlugin)
        .add_audio_channel::<MusicSoundChannel>()
        .add_audio_channel::<AmbienceSoundChannel>()
        .add_audio_channel::<GameSoundChannel>()
        .insert_resource(SpatialAudio {
            max_distance: 250.0,
        })
        .add_systems(OnEnter(AppState::Loading), setup_sound_volume)
        .add_systems(
            Update,
            (
                prepare_actor_spatial_sound,
                actor_footstep_sound_system.run_if(in_state(AppState::PlayingGame)),
                play_background_audio.run_if(in_state(AppState::StartMenu).and_then(run_once())),
            )
                .run_if(resource_exists::<AspenAudioHandles>),
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
    audio_assets: Res<AspenAudioHandles>,
    audio: Res<AudioChannel<MusicSoundChannel>>,
) {
    info!("starting background soundtrack");
    audio.play(audio_assets.game_soundtrack.clone()).looped();
}

use bevy_kira_audio::prelude::AudioReceiver;

/// applies sound data mapps and a spacial emitter for actors that dont already have emitters
fn prepare_actor_spatial_sound(
    audio: Res<AspenAudioHandles>,
    sound_channel: Res<AudioChannel<GameSoundChannel>>,
    mut cmds: Commands,
    camera: Query<Entity, (With<MainCamera>, Without<AudioReceiver>)>,
    actors: Query<Entity, (With<CharacterMoveState>, Without<AudioEmitter>)>,
) {
    if let Ok(unconfigured_camera) = camera.get_single() {
        cmds.entity(unconfigured_camera).insert(AudioReceiver);
    }

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

        sound_map.insert(key.to_string(), footstep_handle.clone_weak());
        sound_timers.insert(key.to_string(), footstep_timer);

        let footstep_instance = sound_channel.play(footstep_handle.clone_weak()).handle();

        cmds.entity(actor).insert((
            ActorSoundMap(sound_map),
            ActorSoundTimers(sound_timers),
            AudioEmitter { instances: vec![footstep_instance] },
        ));
    }
}

use bevy_kira_audio::AudioSource;

/// map of sound assets too "soundactionid"
#[derive(Debug, Component, Deref, DerefMut, Reflect)]
#[reflect(Component)]
pub struct ActorSoundMap(HashMap<String, Handle<AudioSource>>);

/// map of timers too "soundactionid"
#[derive(Debug, Component, Deref, DerefMut, Reflect)]
#[reflect(Component)]
pub struct ActorSoundTimers(HashMap<String, ActorSoundTimer>);

// TODO: make generic across actors and use spatial sound emitters on entitys
/// play walking sound
fn actor_footstep_sound_system(
    // mut audio_instances: ResMut<Assets<AudioInstance>>,
    game_sound: Res<AudioChannel<GameSoundChannel>>,
    mut actor_query: Query<(
        &CharacterMoveState,
        &ActorSoundMap,
        &mut AudioEmitter,
        &mut ActorSoundTimers,
        &Velocity,
    ),
    // With<PlayerSelectedHero>
    >,
    time: Res<Time>,
) {
    for (move_state, sound_map, mut spatial_emmiter, mut sound_timers, velocity) in &mut actor_query {
        let key = "Footstep".to_string();
        let footstep_timer = sound_timers
            .get_mut(&key)
            .expect("timer did not exist in ActorSoundTimers");
        let footstep_handle = sound_map
            .get(&key)
            .expect("audio source did not exist in ActorSoundMap.")
            .to_owned();

        let run_dur = Duration::from_secs_f32(0.3);
        let walk_dur = Duration::from_secs_f32(0.45);

        footstep_timer.tick(time.delta());

        match &move_state.move_status {
            (CurrentMovement::Run, _) => {
                if footstep_timer.duration() != run_dur {
                    footstep_timer.set_duration(run_dur);
                }

                if footstep_timer.finished() {
                    let mut snd = game_sound.play(footstep_handle);
                    spatial_emmiter.instances.push(snd.handle());
                    footstep_timer.reset();

                    // game_sound.play(footstep_handle);
                    // footstep_timer.reset();
                }
            }
            (CurrentMovement::Walk, _) => {
                if footstep_timer.duration() != walk_dur {
                    footstep_timer.set_duration(walk_dur);
                }

                if footstep_timer.finished() {
                    let mut snd = game_sound.play(footstep_handle);
                    spatial_emmiter.instances.push(snd.handle());
                    footstep_timer.reset();

                    // game_sound.play(footstep_handle);
                    // footstep_timer.reset();
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
