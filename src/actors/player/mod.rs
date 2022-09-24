use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use heron::{CollisionLayers, PhysicMaterial, RigidBody, RotationConstraints, Velocity};

use crate::{
    action_manager::bindings::PlayerInput,
    actors::player::{
        animation::FacingDirection,
        movement::{camera_movement_system, player_movement_system, player_sprint},
        utilities::spawn_player,
    },
    game::GameStage,
};

use self::animation::AnimState;

pub mod animation;
mod movement;
mod utilities;

#[derive(Component, Default, Reflect, Inspectable)]
#[reflect(Component)]
pub struct PlayerState {
    //stores important player data
    pub target_positon: Option<Vec2>,
    pub speed: f32,
    pub sprint_available: bool,
    pub facing: FacingDirection,
    pub just_moved: bool,
}

#[derive(Bundle)]
pub struct RigidBodyBundle {
    rigidbody: RigidBody,
    collisionlayers: CollisionLayers,
    rconstraints: RotationConstraints,
    physicsmat: PhysicMaterial,
    velocity: Velocity,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    name: Name,
    pub player_data: PlayerState,
    pub player_animations: AnimState,
    #[bundle]
    rigidbody: RigidBodyBundle,
    // This bundle must be added to your player entity
    // (or whatever else you wish to control)
    #[bundle]
    pub player_input_map: PlayerInput,
    #[bundle]
    pub player_sprite_sheet: SpriteSheetBundle,
}

pub struct PlayerPlugin;
/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameStage::Playing).with_system(spawn_player))
            .add_system_set(
                SystemSet::on_update(GameStage::Playing)
                    .with_system(player_movement_system)
                    .with_system(camera_movement_system)
                    .with_system(player_sprint),
            );
    }
}
