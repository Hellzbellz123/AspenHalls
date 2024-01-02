use bevy::prelude::*;

// TODO: redo player animations to be based on where the mouse cursor is pointing, not player velocity
// this will probably look better and makes the player animations look a bit less funky

/// plays animations for all actors with ([`AnimState`], [`AnimationSheet`], [`TextureAtlasSprite`])
pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, _app: &mut App) {}
}
