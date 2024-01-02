use bevy::{
    asset::Handle, ecs::system::Resource, render::texture::Image, text::Font, prelude::Reflect,
};
use bevy_asset_loader::prelude::AssetCollection;
use bevy_ecs_ldtk::prelude::LdtkProject;

use crate::loading::custom_assets::actor_definitions::{CharacterDefinition, ObjectDefinition};

/// ui textures
#[derive(AssetCollection, Resource, Clone, Debug, Reflect)]
pub struct AspenInitHandles {
    /// game icon
    #[asset(key = "favicon")]
    pub img_favicon: Handle<Image>,
    /// splash screen
    #[asset(key = "splash_img")]
    pub img_splashscreen: Handle<Image>,
    /// default cursor
    #[asset(key = "cursor_default")]
    pub cursor_default: Handle<Image>,
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
#[derive(AssetCollection, Resource, Clone, Debug, Reflect)]
pub struct AspenTouchHandles {
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
}

/// audio resource available
#[derive(AssetCollection, Resource, Clone, Debug, Reflect)]
pub struct AspenAudioHandles {
    /// looping background soundtrack
    #[asset(key = "ost")]
    pub game_soundtrack: Handle<bevy_kira_audio::AudioSource>,
    /// a vector of footstep files, currently 8
    #[asset(key = "footsteps", collection(typed))]
    pub footsteps: Vec<Handle<bevy_kira_audio::AudioSource>>,
}

/// map asset handle
#[derive(AssetCollection, Resource, Clone, Debug, Reflect)]
pub struct AspenMapHandles {
    /// dungeons
    #[asset(key = "default_levels")]
    pub default_levels: Handle<LdtkProject>,
}

/// miscellaneous texture asset handles for tiles
#[derive(AssetCollection, Resource, Clone, Debug, Reflect)]
pub struct AspenTextureHandles {
    /// debug tile for dev purposes
    #[asset(key = "debug")]
    pub debug: Handle<Image>,
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

/// list of definitions that should be loaded as assets
#[derive(AssetCollection, Resource, Clone, Debug, Reflect)]
pub struct AspenDefinitionHandles {
    /// character definitons
    #[asset(key = "character_definitions", collection(typed))]
    pub characters: Vec<Handle<CharacterDefinition>>,

    /// weapon item etc definitions
    #[asset(key = "object_definitions", collection(typed))]
    pub objects: Vec<Handle<ObjectDefinition>>,
}