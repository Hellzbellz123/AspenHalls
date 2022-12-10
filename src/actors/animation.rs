use bevy::prelude::*;

use crate::{
    components::actors::{
        animation::{AnimState, AnimationSheet, FacingDirection},
        general::MovementState,
    },
    game::{GameStage, TimeInfo},
};

// TODO: redo player animations to be based on where the mouse cursor is pointing, not player velocity
// this will probably look better and makes the player animations look a bit less funky

/// plays animations for all actors with ([`ActorState`], [`AnimState`], [`AnimationSheet`], [`TextureAtlasSprite`])
pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameStage::Playing)
                .with_system(Self::update_current_animation)
                .with_system(Self::frame_animation),
        );
    }
}

impl AnimationPlugin {
    fn update_current_animation(
        mut sprites_query: Query<
            (&MovementState, &mut AnimState, &AnimationSheet),
            Changed<MovementState>,
        >,
    ) {
        for (player_compontent, mut animation, anim_sheet) in sprites_query.iter_mut() {
            if matches!(
                player_compontent.facing,
                FacingDirection::Right | FacingDirection::Left
            ) {
                animation.current_frames = anim_sheet.right_animation.to_vec();
            } else if player_compontent.facing == FacingDirection::Up {
                animation.current_frames = anim_sheet.up_animation.to_vec();
            } else if player_compontent.facing == FacingDirection::Down {
                animation.current_frames = anim_sheet.down_animation.to_vec();
            } else if player_compontent.facing == FacingDirection::Idle {
                animation.current_frames = anim_sheet.idle_animation.to_vec();
            }
        }
    }

    fn frame_animation(
        timeinfo: ResMut<TimeInfo>,
        mut sprites_query: Query<(&mut TextureAtlasSprite, &mut AnimState)>,
        time: Res<Time>,
    ) {
        for (mut sprite, mut animation) in sprites_query.iter_mut() {
            animation.timer.tick(time.delta());
            if !timeinfo.game_paused && animation.timer.just_finished() {
                if animation.current_frames.is_empty() {
                    warn!("no animations available ?");
                } else {
                    animation.current_frame =
                        (animation.current_frame + 1) % animation.current_frames.len();
                    sprite.index = animation.current_frames[animation.current_frame];
                }
            }
        }
    }
}
