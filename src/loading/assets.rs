use bevy::{prelude::*, reflect::TypeUuid};
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::LdtkAsset;

#[derive(AssetCollection, Resource)]
pub struct FontHandles {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans_ttf: Handle<Font>,
    // #[asset(path = "fonts/FiraSans-Bold.kayak_font")]
    // pub fira_sans_msdf: Handle<kayak_ui::font::KayakFont>,

    // #[asset(path = "fonts/FantasqueSansMonoNF.kayak_font")]
    // pub fantasque_sans_msdf: Handle<kayak_ui::font::KayakFont>,
}

#[derive(AssetCollection, Resource, Debug)]
pub struct AudioHandles {
    #[asset(path = "audio/ost/expansion.wav")]
    pub gamesoundtrack: Handle<bevy_kira_audio::AudioSource>,
    #[asset(path = "audio/footstep", collection(typed))]
    pub footsteps: Vec<Handle<bevy_kira_audio::AudioSource>>,
}

#[derive(AssetCollection, Resource, Clone)]
pub struct PlayerTextureHandles {
    #[asset(texture_atlas(tile_size_x = 18., tile_size_y = 36., columns = 5, rows = 4))]
    #[asset(path = "characters/heroes/rex-sheet.png")]
    pub rex_full_sheet: Handle<TextureAtlas>,

    #[asset(path = "textures/bevy.png")]
    pub rex_attack: Handle<Image>,
}

#[derive(AssetCollection, Resource, Clone)]
pub struct EnemyTextureHandles {
    #[asset(texture_atlas(tile_size_x = 18., tile_size_y = 36., columns = 5, rows = 4))]
    #[asset(path = "characters/enemies/skele-sheet.png")]
    pub skele_full_sheet: Handle<TextureAtlas>,
}

#[derive(AssetCollection, Resource, Clone)]
pub struct UiTextureHandles {
    // #[asset(path = "textures/splashscreen.png")]
    // pub splash_image: Handle<Image>,
    #[asset(path = "kenny/panel_brown.png")]
    pub panel_brown_png: Handle<Image>,

    #[asset(path = "kenny/buttonSquare_blue_pressed.png")]
    pub button_blue_pressed_png: Handle<Image>,

    #[asset(path = "kenny/buttonSquare_blue.png")]
    pub button_blue_png: Handle<Image>,
}

#[derive(AssetCollection, Resource, Clone, Debug, TypeUuid)]
#[uuid = "a8923dfa-1245-1ab2-901b-129264012320"]
pub struct MapAssetHandles {
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
