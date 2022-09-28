use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_kira_audio::prelude::*;
use rand::seq::SliceRandom;
use std::time::Duration;

use crate::{
    actors::player::{animation::FacingDirection, PlayerState},
    game::GameStage,
    loading::assets::AudioHandles,
};

/// music is played in this channel
pub struct Music;
/// nothing is currently played here, inteded for menu sounds/creaking/etc atmospheric sounds
pub struct Ambience;
/// this audio is for everything gameplay related, footsteps of npc/enemy can be used to tell if enemys exist?
pub struct Sound;

pub struct WalkingSound {
    pub timer: Timer,
    pub is_first_time: bool,
}
//TODO: make this serialize into a settings.toml file in a saves folder
/// modify to change sound volume settings
#[derive(Inspectable, Debug)]
pub struct SoundSettings {
    #[inspectable(min = 0.0, max = 1.0)]
    mastervolume: f64,
    #[inspectable(min = 0.0, max = 1.0)]
    ambiencevolume: f64,
    #[inspectable(min = 0.0, max = 1.0)]
    musicvolume: f64,
    #[inspectable(min = 0.0, max = 1.0)]
    soundvolume: f64,
}

impl FromWorld for SoundSettings {
    #[allow(unused_variables, reason = "clippy says its unused but it isnt")]
    fn from_world(world: &mut World) -> Self {
        SoundSettings {
            mastervolume: 0.5,
            ambiencevolume: 1.0,
            musicvolume: 0.1,
            soundvolume: 0.5,
        }
    }
}

pub struct InternalAudioPlugin;

// This plugin is responsible to control the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(bevy_kira_audio::AudioPlugin)
            .add_audio_channel::<Music>()
            .add_audio_channel::<Ambience>()
            .add_audio_channel::<Sound>()
            .insert_resource(SoundSettings {
                mastervolume: 1.0,
                ambiencevolume: 1.0,
                musicvolume: 1.0,
                soundvolume: 1.0,
            })
            .insert_resource(WalkingSound {
                timer: Timer::from_seconds(0.65, true),
                is_first_time: true,
            })
            .add_system(update_volumes)
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

pub fn player_walking_sound_system(
    audio_assets: Res<AudioHandles>,
    mut player_query: Query<&mut PlayerState>,
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

fn update_volumes(
    settings: Res<SoundSettings>,
    bgm: Res<AudioChannel<Music>>,
    bga: Res<AudioChannel<Ambience>>,
    bgs: Res<AudioChannel<Sound>>,
) {
    if settings.is_changed() {
        info!("volumes changed, applying settings");
        bgm.set_volume(settings.musicvolume * settings.mastervolume);
        bga.set_volume(settings.ambiencevolume * settings.mastervolume);
        bgs.set_volume(settings.soundvolume * settings.mastervolume);
    }
}

// fn play_music_oddio(audio_assets: Res<AudioHandles>, mut audio: ResMut<bevy_oddio::Audio<bevy_oddio::frames::Stereo>>) {
//     // let playing = audio.play(audio_assets.gamesoundtracktwo.clone(), 0.1);
//     // bevy_oddio::output::AudioSink
// }

// pub fn player_walking_oddio(
//     mut player_query: Query<&mut PlayerState>,
//     mut walk_sound_resource: ResMut<WalkingSound>,
//     // asset_server: Res<AssetServer>,
//     audio_assets: Res<AudioHandles>,
//     mut audio: ResMut<bevy_oddio::Audio<bevy_oddio::frames::Stereo>>,
//     // audio: Res<AudioChannel<Sound>>,
//     time: Res<Time>,
// ) {
//     let playerdata = player_query.single_mut();
//     if playerdata.facing == FacingDirection::Idle {
//         walk_sound_resource.timer.reset();
//         walk_sound_resource.is_first_time = true;
//     } else {
//         if !playerdata.sprint_available {
//             walk_sound_resource.timer.set_duration(Duration::from_millis(650));
//         } else if playerdata.sprint_available {
//             walk_sound_resource.timer.set_duration(Duration::from_millis(150));
//         }

//         walk_sound_resource.timer.tick(time.delta());
//         if walk_sound_resource.timer.finished() || walk_sound_resource.is_first_time {
//             if walk_sound_resource.is_first_time {
//                 walk_sound_resource.is_first_time = false;
//                 walk_sound_resource.timer.reset();
//             }

//             let mut rng = thread_rng();
//             let chandle = audio_assets.footsteps.choose(&mut rng).expect("this array will always be filled or the game wont start");

//             let bs = audio.play(chandle.clone(), 1.0);
//             info!("footstep sound {:?}", chandle);

//             // info!("play footsteps for player");
//             // let playing = audio.play(audio_assets.footsteps., 0.1);
//             // audio.play(asset_server.load(format!("audio/footstep/footstep-{}.wav", index).as_str()), 1.0);
//         }
//     }
// }
