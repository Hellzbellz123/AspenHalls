use crate::actors::ActorState;
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::game::{GameStage, TimeInfo};

pub struct GraphicsPlugin;

#[derive(Default, Component, Inspectable)]
pub struct AnimationSheet {
    pub handle: Handle<TextureAtlas>,
    pub idle_animation: [usize; 5],
    pub up_animation: [usize; 5],
    pub down_animation: [usize; 5],
    pub right_animation: [usize; 5],
}

#[derive(
    Component, Default, Clone, Copy, Inspectable, PartialEq, Eq, PartialOrd, Ord, Debug, Reflect,
)]
pub enum FacingDirection {
    #[default]
    Idle,
    Down,
    Left,
    Up,
    Right,
}

#[derive(Component, Default, Inspectable)]
pub struct PlayerGraphics {
    pub facing: FacingDirection,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
#[allow(clippy::module_name_repetitions)]
pub struct AnimState {
    pub timer: Timer,
    pub current_frames: Vec<usize>,
    pub current_frame: usize,
}

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameStage::Playing)
                .with_system(Self::update_current_animation)
                .with_system(Self::frame_animation),
        );
    }
}

impl GraphicsPlugin {
    fn update_current_animation(
        mut sprites_query: Query<
            (&ActorState, &mut AnimState, &AnimationSheet),
            Changed<ActorState>,
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
                    info!("no animations available ?");
                } else {
                    animation.current_frame =
                        (animation.current_frame + 1) % animation.current_frames.len();
                    sprite.index = animation.current_frames[animation.current_frame];
                }
            }
        }
    }
}
