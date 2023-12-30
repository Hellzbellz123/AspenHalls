use crate::game::{
    // characters::animation::components::{ActorAnimationType, AnimState, AnimationSheet},
    TimeInfo,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

// TODO: redo player animations to be based on where the mouse cursor is pointing, not player velocity
// this will probably look better and makes the player animations look a bit less funky

/// plays animations for all actors with ([`AnimState`], [`AnimationSheet`], [`TextureAtlasSprite`])
pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(
        //     Update,
        //     (
        //     //     update_selected_animation_sheet,
        //     //     update_facing_direction,
        //     //     frame_animation,
        //     ),
        // );
    }
}

/// updates character animation based on velocity
// pub fn update_facing_direction(
//     mut animation_states: Query<
//         (&mut Velocity, &mut AnimState, &mut TextureAtlasSprite),
//         Changed<Velocity>,
//     >,
// ) {
//     for (velocity, mut anim_state, mut sprite) in &mut animation_states {
//         if velocity.linvel.abs().length() <= 0.05 || velocity.linvel == Vec2::ZERO {
//             anim_state.animation_type = ActorAnimationType::Idle;
//         }
//         let horizontal = velocity.linvel.x;
//         let vertical = velocity.linvel.y;

//         // Calculate absolute values for horizontal and vertical movement
//         let abs_horizontal = horizontal.abs();
//         let abs_vertical = vertical.abs();

//         if abs_horizontal > abs_vertical {
//             // Horizontal movement is greater
//             if horizontal < 0.0 {
//                 sprite.flip_x = true;
//                 anim_state.animation_type = ActorAnimationType::Right;
//             } else if horizontal > 0.0 {
//                 sprite.flip_x = false;
//                 anim_state.animation_type = ActorAnimationType::Left;
//             }
//         } else if abs_vertical > abs_horizontal {
//             // Vertical movement is greater
//             if vertical < 0.0 {
//                 anim_state.animation_type = ActorAnimationType::Down;
//             } else if vertical > 0.0 {
//                 anim_state.animation_type = ActorAnimationType::Up;
//             }
//         }
//     }
// }

/// iterates over actors with `AnimState` and `AnimationSheet` and
/// updates selected animation based on facing direction
// fn update_selected_animation_sheet(
//     mut sprites_query: Query<(&mut AnimState, &AnimationSheet), Changed<AnimState>>,
// ) {
//     for (mut animation, anim_sheet) in &mut sprites_query {
//         if matches!(
//             animation.animation_type,
//             ActorAnimationType::Right | ActorAnimationType::Left
//         ) {
//             animation.animation_frames = anim_sheet.right_animation.to_vec();
//         } else if animation.animation_type == ActorAnimationType::Up {
//             animation.animation_frames = anim_sheet.up_animation.to_vec();
//         } else if animation.animation_type == ActorAnimationType::Down {
//             animation.animation_frames = anim_sheet.down_animation.to_vec();
//         } else if animation.animation_type == ActorAnimationType::Idle {
//             animation.animation_frames = anim_sheet.idle_animation.to_vec();
//         }
//     }
// }

/// play next frame of animations
// fn frame_animation(
//     time_info: ResMut<TimeInfo>,
//     mut sprites_query: Query<(&mut TextureAtlasSprite, &mut AnimState)>,
//     time: Res<Time>,
// ) {
//     for (mut sprite, mut animation) in &mut sprites_query {
//         animation.timer.tick(time.delta());
//         if !time_info.game_paused && animation.timer.just_finished() {
//             if animation.animation_frames.is_empty() {
//                 warn!("no animations available ?");
//             } else {
//                 animation.active_frame =
//                     (animation.active_frame + 1) % animation.animation_frames.len();
//                 sprite.index = animation.animation_frames[animation.active_frame];
//             }
//         }
//     }
// }

/// animation components
pub mod components {
    use bevy::prelude::*;

    // /// holds what frames belong too what animations
    // #[derive(Debug, Clone, Default, Component, Reflect)]
    // pub struct AnimationSheet {
    //     /// handle too texture atlas
    //     pub handle: Handle<TextureAtlas>,
    //     /// idle animation
    //     pub idle_animation: [usize; 5],
    //     /// walk up
    //     pub up_animation: [usize; 5],
    //     /// walk down
    //     pub down_animation: [usize; 5],
    //     /// walk right, mirrored for walk left
    //     pub right_animation: [usize; 5],
    // }

    // /// different animations player can use
    // #[derive(Component, Default, Clone, Copy, Reflect, PartialEq, Eq, PartialOrd, Ord, Debug)]
    // pub enum ActorAnimationType {
    //     /// doing nothing
    //     #[default]
    //     Idle,
    //     /// walk south
    //     Down,
    //     /// walk west
    //     Left,
    //     /// walk north
    //     Up,
    //     /// walk east
    //     Right,
    //     /// shoot action, per actor
    //     Shoot,
    // }

    // /// animation direction, current frames, current frame, and timer
    // #[derive(Debug, Clone, Component, Default, Reflect)]
    // #[reflect(Component)]
    // pub struct AnimState {
    //     /// direction player is facing
    //     pub animation_type: ActorAnimationType,
    //     /// animation timer
    //     pub timer: Timer,
    //     /// frames belonging too selected AnimationType
    //     pub animation_frames: Vec<usize>,
    //     /// active frame if AnimationType
    //     pub active_frame: usize,
    // }
}
