use crate::action_manager::actions::GameActions;
use crate::action_manager::bindings::PlayerInput;
use crate::loading::GameTextureAssets;
use crate::GameState;

use bevy::prelude::*;

use leafwing_input_manager::prelude::ActionState;
use leafwing_input_manager::{errors::NearlySingularConversion, orientation::Direction};

pub struct PlayerPlugin;
/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(spawn_player)
                .with_system(spawn_camera),
        )
        .add_event::<PlayerWalk>()
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(player_walks_reader));
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    // This bundle must be added to your player entity
    // (or whatever else you wish to control)
    #[bundle]
    input_map: PlayerInput,
    #[bundle]
    sprite: SpriteBundle,
}

fn spawn_player(mut commands: Commands, textures: Res<GameTextureAssets>) {
    commands
        .spawn_bundle(PlayerBundle {
            player: Player,
            sprite: SpriteBundle {
                texture: textures.texture_player.clone(),
                transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
                ..Default::default()
            },
            input_map: PlayerInput::default(),
        })
        .insert(Player);
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn player_walks_reader(
    query_action_state: Query<&ActionState<GameActions>>,
    time: Res<Time>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    let mut movement_dir = Vec3::ZERO;

    let speed = 250.;

    for action_state in query_action_state.iter() {
        if action_state.just_pressed(GameActions::Right) {
            movement_dir.x = 1.0 * speed * time.delta_seconds();
        }

        if action_state.just_pressed(GameActions::Left) {
            movement_dir.x = -1.0 * speed * time.delta_seconds();
        }

        if action_state.just_pressed(GameActions::Up) {
            movement_dir.y = 1.0 * speed * time.delta_seconds();
        }

        if action_state.just_pressed(GameActions::Down) {
            movement_dir.y = -1.0 * speed * time.delta_seconds();
        }

        for mut player_transform in player_query.iter_mut() {
            player_transform.translation += movement_dir;
        }
    }
}

pub struct PlayerWalk {
    pub direction: Direction,
}

fn player_walks(
    action_query: Query<&ActionState<GameActions>, With<Player>>,
    mut event_writer: EventWriter<PlayerWalk>,
) {
    let action_state = action_query.single();

    let mut direction_vector = Vec2::ZERO;

    for input_direction in GameActions::DIRECTIONS {
        if action_state.pressed(input_direction) {
            if let Some(direction) = input_direction.direction() {
                // Sum the directions as 2D vectors
                direction_vector += Vec2::from(direction);
            }
        }
    }
    let net_direction: Result<Direction, NearlySingularConversion> = direction_vector.try_into();

    if let Ok(direction) = net_direction {
        event_writer.send(PlayerWalk { direction });
    }
}
