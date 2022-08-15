use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

use crate::game::GameStage;

pub struct LoadingPlugin;

/// This plugin loads all assets using \[AssetLoader\] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at https://bevy-cheatbook.github.io/features/assets.html
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        LoadingState::new(GameStage::Loading)
            .with_collection::<FontAssets>()
            .with_collection::<AudioAssets>()
            .with_collection::<GameTextureAssets>()
            .with_collection::<UiTextureAssets>()
            .continue_to_state(GameStage::Splash)
            .build(app);
        info!("asset loader init")
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see https://github.com/NiklasEi/bevy_asset_loader)

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans_ttf: Handle<Font>,
    // #[asset(path = "fonts/FiraSans-Bold.kayak_font")]
    // pub fira_sans_msdf: Handle<kayak_ui::font::KayakFont>,

    // #[asset(path = "fonts/FantasqueSansMonoNF.kayak_font")]
    // pub fantasque_sans_msdf: Handle<kayak_ui::font::KayakFont>,
}

#[derive(AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "audio/ost/expansion.ogg")]
    pub gamesoundtrack: Handle<AudioSource>,
}

#[derive(AssetCollection, Clone)]
pub struct GameTextureAssets {
    #[asset(path = "textures/Player/player.png")]
    pub texture_player: Handle<Image>,
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
