use belly::prelude::StyleSheet;
use bevy::{
    asset::Handle, ecs::system::Resource, reflect::TypeUuid, render::texture::Image,
    sprite::TextureAtlas, text::Font,
};
use bevy_asset_loader::prelude::AssetCollection;
use bevy_ecs_ldtk::prelude::LdtkProject;

/// ui textures
#[derive(AssetCollection, Resource, Clone)]
pub struct InitAssetHandles {
    /// global style sheet
    #[asset(key = "global_style")]
    pub global_style_sheet: Handle<StyleSheet>,
    /// style sheet for game menus
    #[asset(key = "menu_style")]
    pub menu_style_sheet: Handle<StyleSheet>,
    /// game icon
    #[asset(key = "favicon")]
    pub img_favicon: Handle<Image>,
    /// splash screen
    #[asset(key = "splash_img")]
    pub img_splashscreen: Handle<Image>,

    /// large fancy font
    #[asset(key = "title_font")]
    pub font_title: Handle<Font>,
    /// main font, default for all text
    #[asset(key = "main_font")]
    pub font_regular: Handle<Font>,
    /// bold font for EMPHASIS
    #[asset(key = "bold_font")]
    pub font_bold: Handle<Font>,

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

/// asset handles for touch controls UI
#[derive(AssetCollection, Resource, Debug)]
pub struct TouchControlAssetHandles {
    /// knob with arrows
    #[asset(key = "knob_arrows")]
    pub knob_arrows: Handle<Image>,
    /// knob with no arrows
    #[asset(key = "knob_no_arrows")]
    pub knob_no_arrows: Handle<Image>,
    /// knob container with arrows
    #[asset(key = "outline_arrows")]
    pub outline_arrows: Handle<Image>,
    /// knob container without arrows
    #[asset(key = "outline_no_arrows")]
    pub outline_no_arrows: Handle<Image>,
    // interact button image handle
    // #[asset(key = "interact_button")]
    // pub interact_button: Handle<Image>,
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

/// miscellaneous texture asset handles for tiles
#[derive(AssetCollection, Resource, Clone, Debug, TypeUuid)]
#[uuid = "c904a07a-9d11-4a5b-91e3-c6ee60db599a"]
pub struct SingleTileTextureHandles {
    /// full tile grass texture
    #[asset(key = "grass")]
    pub grass: Handle<Image>,
    /// full tile dirt texture
    #[asset(key = "dirt")]
    pub dirt: Handle<Image>,
    ///grass and dirt on 1 side
    #[asset(key = "grass_dirt_top")]
    pub grass_dirt_top: Handle<Image>,
}
