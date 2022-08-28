use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::prelude::*;

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans_ttf: Handle<Font>,
    #[asset(path = "fonts/FiraSans-Bold.kayak_font")]
    pub fira_sans_msdf: Handle<kayak_ui::font::KayakFont>,

    #[asset(path = "fonts/FantasqueSansMonoNF.kayak_font")]
    pub fantasque_sans_msdf: Handle<kayak_ui::font::KayakFont>,
}

#[derive(AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "audio/ost/expansion.ogg")]
    pub gamesoundtrack: Handle<AudioSource>,
}

#[derive(AssetCollection, Clone)]
pub struct RexTextureAssets {
    #[asset(texture_atlas(tile_size_x = 18., tile_size_y = 36., columns = 2, rows = 1))]
    #[asset(path = "characters/heroes/rex-idle.png")]
    pub idle: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 13., tile_size_y = 36., columns = 3, rows = 1))]
    #[asset(path = "characters/heroes/rex-walkeast.png")]
    pub walkeast: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 18., tile_size_y = 36., columns = 5, rows = 1))]
    #[asset(path = "characters/heroes/rex-walknorth.png")]
    pub walknorth: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 18., tile_size_y = 36., columns = 5, rows = 1))]
    #[asset(path = "characters/heroes/rex-walksouth.png")]
    pub walksouth: Handle<TextureAtlas>,
}

#[derive(AssetCollection, Clone)]
pub struct UiTextureAssets {
    #[asset(path = "textures/splashscreen.png")]
    pub splash_image: Handle<Image>,

    #[asset(path = "kenny/panel_brown.png")]
    pub panel_brown_png: Handle<Image>,

    #[asset(path = "kenny/buttonSquare_blue_pressed.png")]
    pub button_blue_pressed_png: Handle<Image>,

    #[asset(path = "kenny/buttonSquare_blue.png")]
    pub button_blue_png: Handle<Image>,
}
