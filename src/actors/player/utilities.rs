use crate::{
    action_manager::bindings::PlayerInput,
    components::actors::{
        animation::{AnimState, AnimationSheet, FacingDirection},
        bundles::{ActorColliderBundle, RigidBodyBundle},
        general::{ActorState, CombatStats, DefenseStats, Player},
    },
    loading::assets::PlayerTextureHandles,
    utilities::game::{ACTOR_PHYSICS_LAYER, PLAYER_SIZE},
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::{
    Collider, ColliderMassProperties, Damping, Friction, LockedAxes, Restitution, RigidBody,
    Velocity,
};

use super::PlayerBundle;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Component)]
pub struct PlayerColliderTag;

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
                    custom_size: Some(PLAYER_SIZE), //character is 1 tile wide by 2 tiles wide
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
