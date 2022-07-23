use crate::action_manager::actions::GameActions;
use crate::action_manager::bindings::PlayerInput;
use crate::loading::GameTextureAssets;
use crate::GameState;

use bevy::prelude::*;

use bevy_inspector_egui::Inspectable;

use leafwing_input_manager::prelude::ActionState;

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
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(player_movement_system)
                .with_system(player_sprint),
        );
    }
}

#[derive(Component, Inspectable, Reflect)]
pub struct Player {
    speed: f32,
    sprint_available: bool,
}

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
            player: Player {
                speed: 100.0,
                sprint_available: false,
            },
            sprite: SpriteBundle {
                texture: textures.texture_player.clone(),
                transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
                ..Default::default()
            },
            input_map: PlayerInput::default(),
        })
        .insert(Player {
            speed: 100.0,
            sprint_available: false,
        });
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn player_movement_system(
    query_action_state: Query<&ActionState<GameActions>>,
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &mut Player)>,
) {
    let _movement_dir = Vec3::ZERO;
    let (mut player_transform, mut player) = player_query.single_mut();

    if player.sprint_available {
        player.speed = 255.0
    }

    if !player.sprint_available {
        player.speed = 100.0
    }

    Vec3::clamp(
        player_transform.translation,
        Vec3::ZERO,
        Vec3::new(255., 255., 255.),
    );

    for action_state in query_action_state.iter() {
        if action_state.pressed(GameActions::Right) {
            player_transform.translation.x += 1.0 * player.speed * time.delta_seconds();
        }

        if action_state.pressed(GameActions::Left) {
            player_transform.translation.x += -1.0 * player.speed * time.delta_seconds();
        }

        if action_state.pressed(GameActions::Up) {
            player_transform.translation.y += 1.0 * player.speed * time.delta_seconds();
        }

        if action_state.pressed(GameActions::Down) {
            player_transform.translation.y += -1.0 * player.speed * time.delta_seconds();
        }
    }
}

fn player_sprint(
    query: Query<&ActionState<GameActions>, With<Player>>,
    mut player_query: Query<(&mut Transform, &mut Player)>,
) {
    let action_state = query.single();
    let (mut _player_transform, mut player) = player_query.single_mut();

    if action_state.pressed(GameActions::Dash) {
        player.sprint_available = true;
    }

    if action_state.released(GameActions::Dash) {
        player.sprint_available = false;
    }
}
