use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::{
    characters::player::PlayerComponent,
    game::{GameStage, TimeInfo},
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

#[derive(Component, Default, Clone, Copy, Inspectable, PartialEq, Eq, PartialOrd, Ord, Reflect)]
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
pub struct FrameAnimation {
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
    fn load_graphics(
        mut commands: Commands,
        assets: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    ) {
        let columns = 5;
        let image = assets.load("characters/heroes/rex-sheet.png");
        let atlas = TextureAtlas::from_grid(
            image,
            Vec2::new(18.0, 36.0),
            columns,
            4,
            // Vec2::new(0., 0.),
            // Vec2::new(0., 1.),
        );
        let atlas_handle = texture_atlases.add(atlas);

        commands.insert_resource(CharacterSheet {
            handle: atlas_handle,
            player_idle: [0, 1],
            player_down: [5, 6, 7, 8, 9],
            player_up: [10, 11, 12, 13, 14],
            player_right: [15, 16, 17],
        });
    }

    fn update_player_graphics(
        mut sprites_query: Query<(&PlayerComponent, &mut FrameAnimation), Changed<PlayerComponent>>,
        characters: Res<CharacterSheet>,
    ) {
        for (player_compontent, mut animation) in sprites_query.iter_mut() {
            if player_compontent.facing == FacingDirection::Idle {
                animation.frames = characters.player_idle.to_vec()
            }
            if player_compontent.facing == FacingDirection::Up {
                animation.frames = characters.player_up.to_vec()
            } else if player_compontent.facing == FacingDirection::Down {
                animation.frames = characters.player_down.to_vec()
            } else if player_compontent.facing == FacingDirection::Left {
                animation.frames = characters.player_right.to_vec()
            } else if player_compontent.facing == FacingDirection::Right {
                animation.frames = characters.player_right.to_vec()
            }
            // animation.frames = match graphics.facing {
            //     FacingDirection::Up => characters.player_up.to_vec(),
            //     FacingDirection::Down => characters.player_down.to_vec(),
            //     FacingDirection::Left => characters.player_right.to_vec(),
            //     FacingDirection::Right => characters.player_right.to_vec(),
            // }
        }
    }

    fn frame_animation(
        timeinfo: ResMut<TimeInfo>,
        mut sprites_query: Query<(&mut TextureAtlasSprite, &mut FrameAnimation)>,
        time: Res<Time>,
    ) {
        for (mut sprite, mut animation) in sprites_query.iter_mut() {
            animation.timer.tick(time.delta());
            if !timeinfo.game_paused {
                if animation.timer.just_finished() {
                    if !animation.frames.is_empty() {
                        animation.current_frame =
                            (animation.current_frame + 1) % animation.frames.len();
                        sprite.index = animation.frames[animation.current_frame];
                    } else {
                        info!("no animations available ?")
                    }
                }
            }
        }
    }
}
