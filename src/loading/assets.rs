use bevy::{prelude::*, reflect::TypeUuid};
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::LdtkAsset;
use kayak_ui::prelude::KayakFont;

/// game data folder is data, anything thats not ron/toml/json/etc goes in assets
pub const SPLASHASSETPATH: &str = "assets/textures/splash/splashL.png";

#[derive(AssetCollection, Resource)]
pub struct FontHandles {
    /// default font
    #[asset(key = "main_font")]
    pub main_font: Handle<KayakFont>,

    #[asset(key = "title_font")]
    pub title_font: Handle<KayakFont>,
}

#[derive(AssetCollection, Resource, Debug)]
pub struct AudioHandles {
    /// looping background soundtrack
    #[asset(key = "ost")]
    pub gamesoundtrack: Handle<bevy_kira_audio::AudioSource>,

    /// a vector of footstep files, currently 8
    #[asset(key = "footsteps", collection(typed))]
    pub footsteps: Vec<Handle<bevy_kira_audio::AudioSource>>,
}

#[derive(AssetCollection, Resource, Clone)]
pub struct ActorTextureHandles {
    /// player character 1 texture handle
    #[asset(key = "hero_rex")]
    pub rex_sheet: Handle<TextureAtlas>,

    /// skeleton enemy asset
    #[asset(key = "skeleton")]
    pub skeleton_sheet: Handle<TextureAtlas>,

    /// slime enemy asset
    #[asset(key = "slime")]
    pub slime_sheet: Handle<TextureAtlas>,

    /// first weapon, small smg
    #[asset(key = "small_smg")]
    pub small_smg: Handle<TextureAtlas>,

    /// second weapon a small pistol
    #[asset(key = "small_pistol")]
    pub small_pistol: Handle<TextureAtlas>,

    /// bevy icon
    #[asset(key = "bevy_icon")]
    pub bevy_icon: Handle<Image>,
}

#[derive(AssetCollection, Resource, Clone)]
pub struct UiTextureHandles {
    /// ui containing image
    #[asset(key = "panel_brown")]
    pub panel_brown: Handle<Image>,

    /// blue button texture
    #[asset(key = "button_unpressed")]
    pub button_blue: Handle<Image>,

    /// blue button darkend for pressing
    #[asset(key = "button_pressed")]
    pub button_blue_pressed: Handle<Image>,
}

#[derive(AssetCollection, Resource, Clone, Debug, TypeUuid)]
#[uuid = "a8923dfa-1245-1ab2-901b-129264012320"]
pub struct MapAssetHandles {
    /// homeworld level asset
    #[asset(path = "levels/homeworld.ldtk")]
    pub homeworld: Handle<LdtkAsset>,
}

// BEVY ODDIO ASSET CONFIG
// #[derive(AssetCollection, Debug)]
// pub struct AudioHandles {
//     #[asset(path = "audio/ost/expansion.wav")]
//     pub gamesoundtrack: Handle<bevy_oddio::AudioSource<Stereo>>,//Handle<bevy_kira_audio::AudioSource>,
//     #[asset(path = "audio/footstep", collection(typed))]
//     pub footsteps: Vec<Handle<bevy_oddio::AudioSource<Stereo>>>, //Vec<Handle<bevy_kira_audio::AudioSource>>,

//     #[asset(path = "audio/ost/expansion.wav")]
//     pub gamesoundtracktwo: Handle<bevy_oddio::AudioSource<Stereo>>
// }
