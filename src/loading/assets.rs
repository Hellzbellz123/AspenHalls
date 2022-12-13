use bevy::{prelude::*, reflect::TypeUuid};
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::LdtkAsset;
use kayak_ui::prelude::KayakFont;

#[derive(AssetCollection, Resource)]
pub struct FontHandles {
    /// default font
    #[asset(path = "fonts/kttf/FantasqueSansMonoNF.kayak_font")]
    pub fantasque_sans_msdf: Handle<KayakFont>,
}

#[derive(AssetCollection, Resource, Debug)]
pub struct AudioHandles {
    /// looping background soundtrack
    #[asset(path = "audio/ost/expansion.ogg")]
    pub gamesoundtrack: Handle<bevy_kira_audio::AudioSource>,

    /// a vector of footstep files, currently 8
    #[asset(path = "audio/footstep", collection(typed))]
    pub footsteps: Vec<Handle<bevy_kira_audio::AudioSource>>,
}

#[derive(AssetCollection, Resource, Clone)]
pub struct ActorTextureHandles {
    /// player character 1 texture handle
    #[asset(texture_atlas(tile_size_x = 18., tile_size_y = 36., columns = 5, rows = 4))]
    #[asset(path = "textures/actors/heroes/rex-sheet.png")]
    pub rex_sheet: Handle<TextureAtlas>,

    /// skeleton enermy icon
    #[asset(texture_atlas(tile_size_x = 18., tile_size_y = 36., columns = 5, rows = 4))]
    #[asset(path = "textures/actors/enemies/skeleton-sheet.png")]
    pub skeleton_sheet: Handle<TextureAtlas>,

    /// skeleton enermy icon
    #[asset(texture_atlas(tile_size_x = 18., tile_size_y = 36., columns = 5, rows = 4))]
    #[asset(path = "textures/actors/enemies/slime-sheet.png")]
    pub slime_sheet: Handle<TextureAtlas>,

    /// first weapon, small smg
    #[asset(texture_atlas(tile_size_x = 18., tile_size_y = 36., columns = 1, rows = 1))]
    #[asset(path = "textures/actors/weapons/smallsmg.png")]
    pub small_smg: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 18., tile_size_y = 36., columns = 1, rows = 1))]
    #[asset(path = "textures/actors/weapons/smallpistol.png")]
    pub small_pistol: Handle<TextureAtlas>,

    /// bevy icon
    #[asset(path = "textures/bevy.png")]
    pub bevy_icon: Handle<Image>,
}

#[derive(AssetCollection, Resource, Clone)]
pub struct UiTextureHandles {
    /// ui containing image
    #[asset(path = "ui/panel_brown.png")]
    pub panel_brown: Handle<Image>,

    /// blue button darkend for pressing
    #[asset(path = "ui/buttonSquare_blue_pressed.png")]
    pub button_blue_pressed: Handle<Image>,

    /// blue button texture
    #[asset(path = "ui/buttonSquare_blue.png")]
    pub button_blue: Handle<Image>,
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
