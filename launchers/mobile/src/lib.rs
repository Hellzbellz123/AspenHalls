use aspen_halls_game::{ConfigFile, GameDifficulty};
use bevy::prelude::{bevy_main, Vec2};

#[bevy_main]
fn main() {
    let ctx = ndk_context::android_context();
    let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }.unwrap();
    let env = vm.attach_current_thread().unwrap();
    let config = ConfigFile {
        window_settings: aspen_halls_game::WindowSettings {
            v_sync: true,
            frame_rate_target: 120.0,
            full_screen: true,
            resolution: Vec2 {
                x: 1920.0,
                y: 1080.0,
            },
            window_scale_override: 1.75,
        },
        sound_settings: aspen_halls_game::SoundSettings {
            master_volume: 1.0,
            ambience_volume: 0.5,
            music_volume: 0.5,
            sound_volume: 0.5,
        },
        general_settings: aspen_halls_game::GeneralSettings {
            camera_zoom: 3.5,
            game_difficulty: GameDifficulty::Medium,
        },
    };
    println!("Starting launcher: Mobile");
    aspen_halls_game::start_app(config).run();
}

// TODO: use bevy_fluent for localization, keep below functions
// for android localization and maybe a homepage button

// use jni::objects::JObject;
// use jni::*;
// fn open_url(url: &str) {
//     let ctx = ndk_context::android_context();
//     let vm = unsafe { JavaVM::from_raw(ctx.vm().cast()) }.unwrap();
//     let context = unsafe { JObject::from_raw(ctx.context().cast()) };
//     let mut env = vm.attach_current_thread().unwrap();

//     let url = env.new_string(url).unwrap();

//     env.call_method(
//         context,
//         "openUrl",
//         "(Ljava/lang/String;)V",
//         &[(&url).into()],
//     )
//     .unwrap();
// }

// fn get_lang() -> aspen_hall_game::LocaleLangs {
//     let ctx = ndk_context::android_context();
//     let vm = unsafe { JavaVM::from_raw(ctx.vm().cast()) }.unwrap();
//     let mut env = vm.attach_current_thread().unwrap();

//     let lang = env.find_class("java/util/Locale").unwrap();
//     let lang = env
//         .call_static_method(lang, "getDefault", "()Ljava/util/Locale;", &[])
//         .unwrap();
//     let lang = env
//         .call_method(
//             lang.l().unwrap(),
//             "getLanguage",
//             "()Ljava/lang/String;",
//             &[],
//         )
//         .unwrap();
//     let lang = lang.l().unwrap();
//     let lang = env.get_string((&lang).into()).unwrap();
//     let lang = lang.to_str().unwrap();
//     let lang = lang.to_lowercase();

//     match lang.as_str() {
//         "es" => LocaleLangs::ES,
//         _ => LocaleLangs::EN,
//     }
// }
