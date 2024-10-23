use bevy::{
    asset::Handle, ecs::system::Resource, prelude::{Reflect, TextureAtlasLayout}, render::texture::Image, text::Font,
};
use bevy_asset_loader::prelude::AssetCollection;
use bevy_ecs_ldtk::prelude::LdtkProject;

use crate::loading::custom_assets::actor_definitions::{CharacterDefinition, ItemDefinition};

/// ui textures
#[derive(AssetCollection, Resource, Clone, Debug, Reflect)]
pub struct AspenInitHandles {
    /// game icon
    #[asset(key = "favicon")]
    pub img_favicon: Handle<Image>,
    /// splash screen
    #[asset(key = "splash_img")]
    pub img_splashscreen: Handle<Image>,
    /// cursor image
    #[asset(key = "cursor_sheet")]
    pub cursor_image: Handle<Image>,
    /// cursor layout
    #[asset(key = "cursor_layout")]
    pub cursor_layout: Handle<TextureAtlasLayout>,
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
#[allow(clippy::struct_field_names)]
#[derive(AssetCollection, Resource, Clone, Debug, Reflect)]
pub struct AspenTouchHandles {
    // button handles
    /// zoom in button image
    #[asset(key = "zoom_plus_button")]
    pub zoom_plus_button: Handle<Image>,
    /// zoom out button image
    #[asset(key = "zoom_minus_button")]
    pub zoom_minus_button: Handle<Image>,
    /// menu button image
    #[asset(key = "menu_button")]
    pub menu_button: Handle<Image>,
    /// swap weapon button image
    #[asset(key = "swap_button")]
    pub swap_button: Handle<Image>,
    /// heal button image
    #[asset(key = "heal_button")]
    pub heal_button: Handle<Image>,

    // action buttons
    /// player abilties first slot
    #[asset(key = "action_one")]
    pub action_one: Handle<Image>,
    /// player abilties first slot
    #[asset(key = "action_two")]
    pub action_two: Handle<Image>,
    /// player abilties first slot
    #[asset(key = "action_three")]
    pub action_three: Handle<Image>,
    /// player abilties first slot
    #[asset(key = "action_four")]
    pub action_four: Handle<Image>,
    /// player abilties first slot
    #[asset(key = "action_five")]
    pub action_five: Handle<Image>,

    // Joy Stick handles
    /// knob with arrows
    #[asset(key = "move_knob")]
    pub move_knob: Handle<Image>,
    /// knob with no arrows
    #[asset(key = "look_knob")]
    pub look_knob: Handle<Image>,
    /// knob container with arrows
    #[asset(key = "move_outline")]
    pub move_outline: Handle<Image>,
    /// knob container without arrows
    #[asset(key = "look_outline")]
    pub look_outline: Handle<Image>,
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
    #[asset(key = "item_definitions", collection(typed))]
    pub items: Vec<Handle<ItemDefinition>>,
}
