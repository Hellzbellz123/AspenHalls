use std::time::Duration;

use bevy::{platform::collections::HashMap, prelude::*};
use bevy_aseprite_ultra::prelude::AnimationState;
use bevy_kira_audio::{
    prelude::AudioControl, AudioApp, AudioChannel, AudioPlugin as InternalAudioPlugin,
    AudioSettings, AudioSource, DefaultSpatialRadius as GlobalSpatialRadius, SpatialAudioEmitter,
    SpatialAudioPlugin, SpatialAudioReceiver,
};

use crate::{
    consts::ACTOR_Z_INDEX,
    game::{
        characters::{
            components::{CharacterMoveState, CurrentMovement},
            player::PlayerSelectedHero,
        },
        items::weapons::components::WeaponDescriptor,
    },
    loading::{assets::AspenAudioHandles, registry::RegistryIdentifier},
    playing_game, register_types, AppStage,
};

/// OST music is played on this channel.
#[derive(Resource)]
pub struct MusicSoundChannel;

/// Sound Channel intended for menu sounds/creaking/1etc atmospheric sounds
#[derive(Resource)]
pub struct AmbienceSoundChannel;

/// `AudioChannel` for footsteps/grunts/etc of npc/player, weapon sounds.
/// can be used to tell if enemies exist?
#[derive(Resource)]
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
        register_types!(app, [ActorSoundMap, ActorSoundTimers, ActorSoundTimer]);

        // pretty sure the max sound amount is different per platform?
        app.add_event::<EventPlaySpatialSound>()
            .insert_resource(AudioSettings {
                command_capacity: 512,
                sound_capacity: 512,
            })
            .insert_resource(GlobalSpatialRadius { radius: 350.0 })
            .add_plugins((InternalAudioPlugin, SpatialAudioPlugin))
            .add_audio_channel::<MusicSoundChannel>()
            .add_audio_channel::<AmbienceSoundChannel>()
            .add_audio_channel::<GameSoundChannel>()
            .add_systems(
                OnExit(AppStage::Loading),
                (setup_sound_volume, play_background_audio),
            )
            .add_systems(
                Update,
                (
                    process_event_sounds.run_if(on_event::<EventPlaySpatialSound>),
                    prepare_actor_spatial_sound,
                    update_audio_listener,
                    actor_footstep_sounds,
                )
                    .run_if(playing_game()),
            );
    }
}

// TODO: yeet these
/// gunshot sound tag
pub const S_GUNSHOT: &str = "Gunshot";
/// footstep sound handle
pub const S_FOOTSTEP: &str = "Footstep";

/// map of sound assets too "soundactionid"
#[derive(Debug, Component, Deref, DerefMut, Reflect)]
#[reflect(Component)]
pub struct ActorSoundMap(HashMap<&'static str, Handle<AudioSource>>);

/// map of timers too "soundactionid"
#[derive(Debug, Component, Deref, DerefMut, Reflect)]
#[reflect(Component)]
pub struct ActorSoundTimers(HashMap<&'static str, ActorSoundTimer>);

/// event for playing spatialized sound across different modules
#[derive(Debug, Event)]
pub struct EventPlaySpatialSound {
    /// entity with emitter too play sound from
    pub emitter_id: Entity,
    /// id for sound, should be a const
    pub sound_id: &'static str,
}

fn process_event_sounds(
    game_sound: Res<AudioChannel<GameSoundChannel>>,
    mut event_reader: EventReader<EventPlaySpatialSound>,
    mut emitters: Query<(Entity, &mut SpatialAudioEmitter, &ActorSoundMap)>,
) {
    for event in event_reader.read() {
        let EventPlaySpatialSound {
            emitter_id,
            sound_id,
        } = event;

        let Ok((_emitter, mut sound_emitter, sound_map)) = emitters.get_mut(*emitter_id) else {
            continue;
        };
        // play fire sound
        let Some(sound) = sound_map.get(sound_id) else {
            continue;
        };
        let mut snd = game_sound.play(sound.clone());
        sound_emitter.instances.push(snd.handle());
    }
}

/// initial volume from sound settings
fn setup_sound_volume(
    sound_settings: ResMut<crate::loading::config::AudioSettings>,
    music_channel: Res<AudioChannel<MusicSoundChannel>>,
    ambience_channel: Res<AudioChannel<AmbienceSoundChannel>>,
    sound_channel: Res<AudioChannel<GameSoundChannel>>,
) {
    let sound_settings = sound_settings.volume_config;

    let mastervolume = sound_settings.master;
    music_channel.set_volume(sound_settings.music * mastervolume);
    ambience_channel.set_volume(sound_settings.ambience * mastervolume);
    sound_channel.set_volume(sound_settings.gameplay * mastervolume);
}

/// play game soundtrack
fn play_background_audio(
    audio_assets: Res<AspenAudioHandles>,
    audio: Res<AudioChannel<MusicSoundChannel>>,
) {
    info!("starting background soundtrack");
    audio.play(audio_assets.game_soundtrack.clone()).looped();
}

/// adds spatial audio listener too player controlled character
fn update_audio_listener(
    mut cmds: Commands,
    player_hero: Query<Entity, With<PlayerSelectedHero>>,
    audio_reciever: Query<Entity, (With<ChildOf>, With<SpatialAudioReceiver>)>,
) {
    // use fake listener that is at player position or else camera position?
    // camera plane is 999 but thats too far for audio too work correctly?
    if let Ok(hero) = player_hero.single()
        && audio_reciever.is_empty()
    {
        cmds.entity(hero).with_children(|f| {
            f.spawn((
                Name::new("PlayerAudioReciever"),
                SpatialAudioReceiver,
                Transform::from_translation(Vec3::ZERO.with_z(ACTOR_Z_INDEX + 1.0)),
            ));
        });
    }
}

/// applies sound data maps and a spacial emitter for actors that dont already have emitters
fn prepare_actor_spatial_sound(
    audio: Res<AspenAudioHandles>,
    mut cmds: Commands,
    actors: Query<
        (
            Entity,
            &RegistryIdentifier,
            Option<&CharacterMoveState>,
            Option<&WeaponDescriptor>,
        ),
        Without<SpatialAudioEmitter>,
    >,
) {
    // let mut rng = rand::thread_rng();

    for (actor, _registery_id, is_character, is_weapon) in &actors {
        let mut sound_timers: HashMap<&'static str, ActorSoundTimer> = HashMap::new();
        let mut sound_map: HashMap<&'static str, Handle<AudioSource>> = HashMap::new();

        // TODO: get pregenerated sound map and timer with registery_id

        // footsteps
        let (audio_handle, key) = if is_character.is_some() {
            (audio.footstep_light.clone(), S_FOOTSTEP)
        } else if is_weapon.is_some() {
            (audio.gunshot_quiet.clone(), S_GUNSHOT)
        } else {
            continue;
        };

        let sound_timer = ActorSoundTimer {
            timer: Timer::new(Duration::from_millis(1000), TimerMode::Once),
            is_first_time: true,
        };

        sound_map.insert(key, audio_handle.clone_weak());
        sound_timers.insert(key, sound_timer);

        cmds.entity(actor).insert((
            ActorSoundMap(sound_map),
            ActorSoundTimers(sound_timers),
            SpatialAudioEmitter {
                instances: Vec::new(),
            },
        ));
    }
}

// TODO: make this use actor velocity and gait too calculate footsteps
/// play footstep sound sound for actor
fn actor_footstep_sounds(
    sound_cfg: Res<crate::loading::config::AudioSettings>,
    game_sound: Res<AudioChannel<GameSoundChannel>>,
    mut actor_query: Query<(
        &AnimationState,
        &CharacterMoveState,
        &ActorSoundMap,
        &mut SpatialAudioEmitter,
        &GlobalTransform,
    )>,
    listener: Query<&GlobalTransform, With<SpatialAudioReceiver>>,
) {
    let Ok(listener) = listener.single() else {
        return;
    };

    for (animator_state, move_state, sound_map, mut spatial_emmiter, transform) in &mut actor_query
    {
        if move_state.move_status.0 == CurrentMovement::None
            || listener.translation().distance(transform.translation()) > sound_cfg.max_distance
        {
            continue;
        }

        let key = S_FOOTSTEP;
        let footstep_handle = sound_map
            .get(&key)
            .expect("audio source did not exist in ActorSoundMap.")
            .to_owned();

        let current_frame = animator_state.current_frame();

        if current_frame & 1 != 0 && spatial_emmiter.instances.is_empty() {
            let snd = game_sound
                .play(footstep_handle)
                .with_playback_rate(1.25)
                .handle();
            spatial_emmiter.instances.push(snd);
        }
    }
}
