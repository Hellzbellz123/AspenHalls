use crate::action_manager::bindings::GameActions;
use crate::loading::GameTextureAssets;
use crate::GameState;
use bevy::prelude::*;
use leafwing_input_manager::{InputManagerBundle, action_state};
use leafwing_input_manager::prelude::ActionState;


pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    // This bundle must be added to your player entity
    // (or whatever else you wish to control)
    #[bundle]
    input_manager: InputManagerBundle<GameActions>,
}


/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(spawn_player)
                .with_system(spawn_camera),
        );
        // .add_system_set(SystemSet::on_update(GameState::Playing).with_system(move_player));
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_player(mut commands: Commands, textures: Res<GameTextureAssets>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: textures.texture_player.clone(),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            ..Default::default()
        })
        .insert(Player);
}

fn move_player(
    time: Res<Time>,
    // actions: Res<Actions>,
    mut player_query: Query<&mut Transform, With<Player>>,
    query_action_state: Query<&ActionState<GameActions>,
>,
) {

    // if actions.player_movement.is_none() {
    //     return;
    // }
    // let speed = 250.;
    // let movement = Vec3::new(
    //     actions.player_movement.unwrap().x * speed * time.delta_seconds(),
    //     actions.player_movement.unwrap().y * speed * time.delta_seconds(),
    //     0.,
    // );
    // for mut player_transform in player_query.iter_mut() {
    //     player_transform.translation += movement;
    // }
}

fn horizontal(
    query_action_state: Query<
        &ActionState<GameActions>,
    >,
    mut commands: Commands,
    // axes: Res<Axis<GamepadAxis>>,
    mut query_player: Query<
        (
            Entity,
            &mut TextureAtlasSprite,
            // Option<&AnimationTi
        ),
        With<Player>,
    >,){

    }
