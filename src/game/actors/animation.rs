use crate::game::{actors::animation::components::*, TimeInfo};
use bevy::prelude::*;

// TODO: redo player animations to be based on where the mouse cursor is pointing, not player velocity
// this will probably look better and makes the player animations look a bit less funky

/// plays animations for all actors with ([`AnimState`], [`AnimationSheet`], [`TextureAtlasSprite`])
pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_playing_animation, frame_animation));
    }
}

/// iterates over actors with animstate and animsheet and
/// updates selected animation based on facing direction
fn update_playing_animation(
    mut sprites_query: Query<(&mut AnimState, &AnimationSheet), Changed<AnimState>>,
) {
    sprites_query.for_each_mut(|(mut animation, anim_sheet)| {
        if matches!(
            animation.facing,
            ActorAnimationType::Right | ActorAnimationType::Left
        ) {
            animation.animation_frames = anim_sheet.right_animation.to_vec();
        } else if animation.facing == ActorAnimationType::Up {
            animation.animation_frames = anim_sheet.up_animation.to_vec();
        } else if animation.facing == ActorAnimationType::Down {
            animation.animation_frames = anim_sheet.down_animation.to_vec();
        } else if animation.facing == ActorAnimationType::Idle {
            animation.animation_frames = anim_sheet.idle_animation.to_vec();
        }
    });
}

/// play next frame of animations
fn frame_animation(
    timeinfo: ResMut<TimeInfo>,
    mut sprites_query: Query<(&mut TextureAtlasSprite, &mut AnimState)>,
    time: Res<Time>,
) {
    sprites_query.for_each_mut(|(mut sprite, mut animation)| {
        animation.timer.tick(time.delta());
        if !timeinfo.game_paused && animation.timer.just_finished() {
            if animation.animation_frames.is_empty() {
                warn!("no animations available ?");
            } else {
                animation.active_frame =
                    (animation.active_frame + 1) % animation.animation_frames.len();
                sprite.index = animation.animation_frames[animation.active_frame];
            }
        }
    });
}

/// animation components
pub mod components {
    use bevy::prelude::*;

    /// holds what frames belong too what animations
    #[derive(Default, Component, Reflect)]
    pub struct AnimationSheet {
        /// handle too texture atlas
        pub handle: Handle<TextureAtlas>,
        /// idle animation
        pub idle_animation: [usize; 5],
        /// walk up
        pub up_animation: [usize; 5],
        /// walk down
        pub down_animation: [usize; 5],
        /// walk right, mirrored for walk left
        pub right_animation: [usize; 5],
    }

    /// different animations player can use
    #[derive(Component, Default, Clone, Copy, Reflect, PartialEq, Eq, PartialOrd, Ord, Debug)]
    pub enum ActorAnimationType {
        /// doing nothing
        #[default]
        Idle,
        /// walk south
        Down,
        /// walk west
        Left,
        /// walk north
        Up,
        /// walk east
        Right,
    }

    /// animation direction, current frames, current frame, and timer
    #[derive(Component, Default, Reflect)]
    #[reflect(Component)]
    pub struct AnimState {
        /// direction player is facing
        pub facing: ActorAnimationType,
        /// animation timer
        pub timer: Timer,
        /// frames belonging too selected AnimationType
        pub animation_frames: Vec<usize>,
        /// active frame if AnimationType
        pub active_frame: usize,
    }
}
