use crate::GameState;
use bevy::prelude::*;

pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>().add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(set_movement_actions).with_system(set_ui_actions),
        );
    }
}

#[derive(Default)]
pub struct Actions {
    pub player_movement: Option<Vec2>,
    pub pause_game: bool,
}

fn set_ui_actions(mut actions: ResMut<Actions>, keyboard_input: Res<Input<KeyCode>>) {
    if GameControl::Escape.just_pressed(&keyboard_input) && actions.pause_game == false {
        actions.pause_game = true;
        
    }
    else {
        actions.pause_game = false;
    }
}

fn set_movement_actions(mut actions: ResMut<Actions>, keyboard_input: Res<Input<KeyCode>>) {
    if GameControl::Up.just_released(&keyboard_input)
        || GameControl::Up.pressed(&keyboard_input)
        || GameControl::Left.just_released(&keyboard_input)
        || GameControl::Left.pressed(&keyboard_input)
        || GameControl::Down.just_released(&keyboard_input)
        || GameControl::Down.pressed(&keyboard_input)
        || GameControl::Right.just_released(&keyboard_input)
        || GameControl::Right.pressed(&keyboard_input)
    {
        let mut player_movement = Vec2::ZERO;

        if GameControl::Up.just_released(&keyboard_input)
            || GameControl::Down.just_released(&keyboard_input)
        {
            if GameControl::Up.pressed(&keyboard_input) {
                player_movement.y = 1.;
            } else if GameControl::Down.pressed(&keyboard_input) {
                player_movement.y = -1.;
            } else {
                player_movement.y = 0.;
            }
        } else if GameControl::Up.just_pressed(&keyboard_input) {
            player_movement.y = 1.;
        } else if GameControl::Down.just_pressed(&keyboard_input) {
            player_movement.y = -1.;
        } else {
            player_movement.y = actions.player_movement.unwrap_or(Vec2::ZERO).y;
        }

        if GameControl::Right.just_released(&keyboard_input)
            || GameControl::Left.just_released(&keyboard_input)
        {
            if GameControl::Right.pressed(&keyboard_input) {
                player_movement.x = 1.;
            } else if GameControl::Left.pressed(&keyboard_input) {
                player_movement.x = -1.;
            } else {
                player_movement.x = 0.;
            }
        } else if GameControl::Right.just_pressed(&keyboard_input) {
            player_movement.x = 1.;
        } else if GameControl::Left.just_pressed(&keyboard_input) {
            player_movement.x = -1.;
        } else {
            player_movement.x = actions.player_movement.unwrap_or(Vec2::ZERO).x;
        }

        if player_movement != Vec2::ZERO {
            player_movement = player_movement.normalize();
            actions.player_movement = Some(player_movement);
        }
    } else {
        actions.player_movement = None;
    }
}

enum GameControl {
    Up,
    Down,
    Left,
    Right,
    Escape,
}

impl GameControl {
    fn just_released(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
        match self {
            GameControl::Up => {
                keyboard_input.just_released(KeyCode::W)
            }
            GameControl::Down => {
                keyboard_input.just_released(KeyCode::S)
            }
            GameControl::Left => {
                keyboard_input.just_released(KeyCode::A)
            }
            GameControl::Right => {
                keyboard_input.just_released(KeyCode::D)
            }
            GameControl::Escape => {
                keyboard_input.just_released(KeyCode::Escape)
            } 
            
        }
    }

    fn pressed(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
        match self {
            GameControl::Up => {
                keyboard_input.pressed(KeyCode::W)
            }
            GameControl::Down => {
                keyboard_input.pressed(KeyCode::S)
            }
            GameControl::Left => {
                keyboard_input.pressed(KeyCode::A)
            }
            GameControl::Right => {
                keyboard_input.pressed(KeyCode::D)
            }
            GameControl::Escape => {
                keyboard_input.pressed(KeyCode::Escape)
            }
        }
    }

    fn just_pressed(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
        match self {
            GameControl::Up => {
                keyboard_input.just_pressed(KeyCode::W)
            }
            GameControl::Down => {
                keyboard_input.just_pressed(KeyCode::S)

            }
            GameControl::Left => {
                keyboard_input.just_pressed(KeyCode::A)
            }
            GameControl::Right => {
                keyboard_input.just_pressed(KeyCode::D)
            }
            GameControl::Escape => {
                    keyboard_input.just_pressed(KeyCode::Escape)
            }
        }
    }
}
