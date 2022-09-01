use super::assets::RexTextureHandles;
use bevy::prelude::*;

pub fn drawatlas(_player_animations: Res<RexTextureHandles>) {
    info!("loading player animations");

    // let rex_idle_anim = TextureAtlas::from_grid(
    //     player_animations.idle.clone(),
    //     tile_size,
    //     2,
    //     1,
    // );

    // let rex_walknorth_anim = TextureAtlas::from_grid(
    //     player_animations.walknorth.clone(),
    //     tile_size,
    //     5,
    //     1
    // );

    //     let rex_walksouth_anim = TextureAtlas::from_grid(
    //     player_animations.walknorth.clone(),
    //     tile_size,
    //     5,
    //     1
    // );

    // let rex_walkeast_anim = TextureAtlas::from_grid(
    //     player_animations.walknorth.clone(),
    //     tile_size,
    //     5,
    //     1
    // );
}
