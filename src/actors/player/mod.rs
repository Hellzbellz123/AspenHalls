use bevy::prelude::*;

use crate::{
    action_manager::bindings::PlayerInput,
    actors::player::movement::{camera_movement_system, player_movement_system, player_sprint},
    components::actors::{animation::FacingDirection, bundles::ActorColliderBundle},
    components::actors::{
        animation::{AnimState, AnimationSheet},
        bundles::RigidBodyBundle,
        general::{ActorState, CombatStats, DefenseStats, Player},
    },
    game::GameStage,
    loading::assets::PlayerTextureHandles,
    utilities::game::SystemLabels,
    utilities::game::{ACTOR_PHYSICS_LAYER, ACTOR_SIZE},
};

use bevy_rapier2d::prelude::{
    Collider, ColliderMassProperties, Damping, Friction, LockedAxes, Restitution, RigidBody,
    Velocity,
};

use self::actions::spawn_skeleton_button;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Component)]
pub struct PlayerColliderTag;

pub mod actions;
pub mod attack;
mod movement;

#[derive(Bundle)]
pub struct PlayerBundle {
    name: Name,
    pub player: Player,
    pub player_state: ActorState,
    pub player_animationstate: AnimState,
    pub available_animations: AnimationSheet,
    pub combat_stats: CombatStats,
    pub defense_stats: DefenseStats,
    #[bundle]
    rigidbody: RigidBodyBundle,
    #[bundle]
    pub player_sprite_sheet: SpriteSheetBundle,
    // This bundle must be added to your player entity
    // (or whatever else you wish to control)
    #[bundle]
    pub player_input_map: PlayerInput,
}

pub struct PlayerPlugin;
/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameStage::Playing)
                .with_system(spawn_player.label(SystemLabels::Spawn)),
        )
        .add_system_set(
            SystemSet::on_update(GameStage::Playing)
                .with_system(player_movement_system)
                .with_system(camera_movement_system)
                .with_system(player_sprint)
                .with_system(spawn_skeleton_button),
        );
    }
}

pub fn spawn_player(mut commands: Commands, selected_player: Res<PlayerTextureHandles>) {
    commands
        .spawn((PlayerBundle {
            name: Name::new("player"),
            player: Player {
                wants_to_teleport: false,
                just_teleported: false,
            },
            player_state: ActorState {
                speed: 150.0,
                sprint_available: false,
                facing: FacingDirection::Idle,
                just_moved: false,
            },
            player_animationstate: AnimState {
                timer: Timer::from_seconds(0.2, TimerMode::Repeating),
                current_frames: vec![0, 1, 2, 3, 4],
                current_frame: 0,
            },
            available_animations: AnimationSheet {
                handle: selected_player.rex_full_sheet.clone(),
                idle_animation: [0, 1, 2, 3, 4],
                down_animation: [5, 6, 7, 8, 9],
                up_animation: [10, 11, 12, 13, 14],
                right_animation: [15, 16, 17, 18, 19],
            },
            combat_stats: CombatStats {
                stamina: 10.0,
                agility: 10.0,
                strength: 10.0,
                armor: 10.0,
            },
            defense_stats: DefenseStats {
                health: 1.0,
                shield: 1.0,
            },
            rigidbody: RigidBodyBundle {
                rigidbody: RigidBody::Dynamic,
                velocity: Velocity::default(),
                friction: Friction::coefficient(0.7),
                howbouncy: Restitution::coefficient(0.3),
                massprop: ColliderMassProperties::Density(0.3),
                rotationlocks: LockedAxes::ROTATION_LOCKED,
                dampingprop: Damping {
                    linear_damping: 1.0,
                    angular_damping: 1.0,
                },
            },
            player_sprite_sheet: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    custom_size: Some(ACTOR_SIZE), //character is 1 tile wide by 2 tiles wide
                    ..default()
                },
                texture_atlas: selected_player.rex_full_sheet.clone(),
                transform: Transform::from_xyz(-60.0, 1090.0, 8.0),
                // global_transform:  , // Vec3::new(0.0, 0.0, 8.0)
                ..default()
            },
            player_input_map: PlayerInput::default(),
        },))
        .with_children(|child| {
            child.spawn((
                ActorColliderBundle {
                    transform_bundle: TransformBundle {
                        local: (Transform {
                            translation: (Vec3 {
                                x: 0.,
                                y: -5.,
                                z: ACTOR_PHYSICS_LAYER,
                            }),
                            ..default()
                        }),
                        ..default()
                    },
                    collider: Collider::capsule_y(10.4, 13.12),
                },
                PlayerColliderTag,
                Name::new("PlayerCollider"), // ActiveEvents::COLLISION_EVENTS, //adding this causes all player collisions to be listed.
            ));
        });
}
