use bevy::prelude::*;
use leafwing_input_manager::prelude::InputMap;

use crate::{
    action_manager::actions::*,
};

pub struct GamepadPlugin;

pub struct gamepad_axis_values {
    leftstick: Vec2,
    rightstick: Vec2,
}

impl Plugin for GamepadPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(gamepad_connections)
            // .add_system(gamepad_input)
            .add_system(on_change_gamepad);
        // .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(gamepad_input));
    }
}
/// Simple resource to store the ID of the
/// connected gamepad. We need to know which
/// gamepad to use for player input.
#[derive(Debug)]
pub struct MyGamepad(pub Gamepad);

fn gamepad_connections(
    mut commands: Commands,
    my_gamepad: Option<Res<MyGamepad>>,
    mut gamepad_evr: EventReader<GamepadEvent>,
) {
    for GamepadEvent(id, kind) in gamepad_evr.iter() {
        match kind {
            GamepadEventType::Connected => {
                println!("New gamepad connected with ID: {:?}", id);

                // if we don't have any gamepad yet, use
                // this one
                if my_gamepad.is_none() {
                    commands.insert_resource(MyGamepad(*id));
                }
            }
            GamepadEventType::Disconnected => {
                println!("Lost gamepad connection with ID: {:?}", id);

                // if it's the one we previously associated
                // with the player,
                // disassociate it:
                if let Some(MyGamepad(old_id)) = my_gamepad.as_deref() {
                    if old_id == id {
                        commands.remove_resource::<MyGamepad>();
                    }
                }
            }
            // other events are irrelevant
            _ => {}
        }
    }
}

fn on_change_gamepad(
    gamepad: Option<Res<MyGamepad>>,
    mut input_map: Query<&mut InputMap<GameActions>>,
) {
    if let Some(gamepad) = gamepad {
        if gamepad.is_changed() {
            for mut map in input_map.iter_mut() {
                map.set_gamepad(gamepad.0);
            }
        }
    }
}
fn gamepad_input(
    // mut player_query: Query<(&mut Sprite, With<Player>)>,
    _commands: Commands,
    axes: Res<Axis<GamepadAxis>>,
    buttons: Res<Input<GamepadButton>>,
    my_gamepad: Option<Res<MyGamepad>>,
) {
    let gamepad = if let Some(gp) = my_gamepad {
        // info!("a gamepad is connected, we have the id");
        gp.0
    } else {
        // info!("no gamepad connected"); // no gamepad is connected
        return;
    };

    // The joysticks are represented using a separate
    // axis for X and Y
    let axis_lx = GamepadAxis(gamepad, GamepadAxisType::LeftStickX);
    let axis_ly = GamepadAxis(gamepad, GamepadAxisType::LeftStickY);

    // let mut sprite = player_query.single_mut();

    if let (Some(x), Some(y)) = (axes.get(axis_lx), axes.get(axis_ly)) {
        // combine X and Y into one vector
        let left_stick_pos = Vec2::new(x, y);
        if left_stick_pos.x != 0.0 {
            match left_stick_pos.x.signum() {
                -1.0 => {
                    // info!("flipped");
                    // sprite.0.flip_x = true;
                }
                1.0 => {
                    // info!("not flipped");
                    // sprite.0.flip_x = false;
                }
                _ => {}
            };
        };

        if left_stick_pos.length() > 0.9 && left_stick_pos.y > 0.5 {
            // info!("left stick pushed up")
        }

        if left_stick_pos.length() > 0.9 && left_stick_pos.y > -0.5 {
            // info!("left stick pushed down?")
        }
    }

    // In a real game, the buttons would be
    // configurable, but here we hardcode them
    let _jump_button = GamepadButton(gamepad, GamepadButtonType::South);
    let heal_button = GamepadButton(gamepad, GamepadButtonType::East);

    if buttons.pressed(heal_button) {
        // button being held down: heal the player
        dbg!("circle");
    }
}
