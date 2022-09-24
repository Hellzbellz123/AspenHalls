use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::{
    actors::player::PlayerState,
    game::{GameStage, TimeInfo},
    loading::assets::PlayerTextureHandles,
};

pub struct GraphicsPlugin;

#[derive(Default, Component, Inspectable)]
pub struct CharacterSheet {
    pub handle: Handle<TextureAtlas>,
    pub player_idle: [usize; 2],
    pub player_up: [usize; 5],
    pub player_down: [usize; 5],
    pub player_right: [usize; 3],
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
    pub frames: Vec<usize>,
    pub current_frame: usize,
}

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameStage::Menu).with_system(Self::load_graphics))
            .add_system_set(
                SystemSet::on_update(GameStage::Playing)
                    .with_system(Self::update_player_graphics)
                    .with_system(Self::frame_animation),
            );
    }
}

impl GraphicsPlugin {
    fn load_graphics(mut commands: Commands, selected_player: Res<PlayerTextureHandles>) {
        commands.insert_resource(CharacterSheet {
            handle: selected_player.rex_full_sheet.clone(),
            player_idle: [0, 1],
            player_down: [5, 6, 7, 8, 9],
            player_up: [10, 11, 12, 13, 14],
            player_right: [15, 16, 17],
        });
    }

    fn update_player_graphics(
        mut sprites_query: Query<(&PlayerState, &mut AnimState), Changed<PlayerState>>,
        characters: Res<CharacterSheet>,
    ) {
        for (player_compontent, mut animation) in sprites_query.iter_mut() {
            if matches!(
                player_compontent.facing,
                FacingDirection::Right | FacingDirection::Left
            ) {
                animation.frames = characters.player_right.to_vec();
            } else if player_compontent.facing == FacingDirection::Up {
                animation.frames = characters.player_up.to_vec();
            } else if player_compontent.facing == FacingDirection::Down {
                animation.frames = characters.player_down.to_vec();
            } else if player_compontent.facing == FacingDirection::Idle {
                animation.frames = characters.player_idle.to_vec();
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
                if animation.frames.is_empty() {
                    info!("no animations available ?");
                } else {
                    animation.current_frame =
                        (animation.current_frame + 1) % animation.frames.len();
                    sprite.index = animation.frames[animation.current_frame];
                }
            }
        }
    }
}
