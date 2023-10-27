use bevy::{
    asset::{AssetServer, Handle},
    ecs::system::Resource,
    reflect::TypeUuid,
    render::texture::Image,
    sprite::TextureAtlas,
    text::Font,
};
use bevy_asset_loader::prelude::AssetCollection;
use bevy_ecs_ldtk::prelude::LdtkProject;

/// ui textures
#[derive(AssetCollection, Resource, Clone)]
pub struct InitAssetHandles {
    #[asset(key = "favicon")]
    pub img_favicon: Handle<Image>,
    /// ui containing image
    #[asset(key = "panel_brown")]
    pub img_panel_brown: Handle<Image>,

    /// blue button texture
    #[asset(key = "button_unpressed")]
    pub img_button_blue: Handle<Image>,

    /// blue button darkened for pressing
    #[asset(key = "button_pressed")]
    pub img_button_blue_pressed: Handle<Image>,
}

/// game data folder is data, anything thats not ron/toml/json/etc goes in assets
/// font resources available too use
#[derive(AssetCollection, Resource, Clone)]
pub struct FontHandles {
    /// default font
    #[asset(key = "main_font")]
    pub main_font: Handle<Font>,
    /// large fancy font
    #[asset(key = "title_font")]
    pub title_font: Handle<Font>,
}
/// audio resource available
#[derive(AssetCollection, Resource, Debug)]
pub struct AudioHandles {
    /// looping background soundtrack
    #[asset(key = "ost")]
    pub game_soundtrack: Handle<bevy_kira_audio::AudioSource>,
    /// a vector of footstep files, currently 8
    #[asset(key = "footsteps", collection(typed))]
    pub footsteps: Vec<Handle<bevy_kira_audio::AudioSource>>,
}
/// Actor Textures
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

/// map asset handle
#[derive(AssetCollection, Resource, Clone, Debug, TypeUuid)]
#[uuid = "a8923dfa-1245-1ab2-901b-129264012320"]
pub struct MapAssetHandles {
    /// default levels asset
    #[asset(key = "start_level")]
    pub start_level: Handle<LdtkProject>,

    /// dungeons
    #[asset(key = "dungeons")]
    pub dungeons: Handle<LdtkProject>,
}

#[derive(AssetCollection, Resource, Clone, Debug, TypeUuid)]
#[uuid = "c904a07a-9d11-4a5b-91e3-c6ee60db599a"]
pub struct SingleTileTextureHandles {
    #[asset(key = "grass")]
    pub grass: Handle<Image>,
    #[asset(key = "dirt")]
    pub dirt: Handle<Image>,
    #[asset(key = "grassdirt_top")]
    pub grassdirt_top: Handle<Image>,
}

// BEVY ODDIO ASSET CONFIG
// #[derive(AssetCollection, Debug)]
// pub struct AudioHandles {
//     #[asset(path = "audio/ost/expansion.wav")]
//     pub game_soundtrack: Handle<bevy_oddio::AudioSource<Stereo>>,//Handle<bevy_kira_audio::AudioSource>,
//     #[asset(path = "audio/footstep", collection(typed))]
//     pub footsteps: Vec<Handle<bevy_oddio::AudioSource<Stereo>>>, //Vec<Handle<bevy_kira_audio::AudioSource>>,

//     #[asset(path = "audio/ost/expansion.wav")]
//     pub game_soundtrack_two: Handle<bevy_oddio::AudioSource<Stereo>>
// }
