use bevy::{prelude::*, utils::hashbrown::HashMap};

use crate::{
    bundles::{ActorAttributesBundle, ActorBundle, ActorColliderBundle, RigidBodyBundle},
    consts::{ACTOR_COLLIDER, ACTOR_PHYSICS_Z_INDEX, ACTOR_SCALE, ACTOR_SIZE, ACTOR_Z_INDEX},
    game::{
        actors::{
            ai::components::{ActorType, Faction},
            animation::components::{ActorAnimationType, AnimState, AnimationSheet},
            components::PlayerColliderTag,
        },
        AppStage,
    },
    game::{
        actors::{
            combat::components::WeaponSlots,
            player::movement::{camera_movement_system, player_movement_system, player_sprint},
        },
        input::actions::PlayerBindings,
    },
    loading::assets::ActorTextureHandles,
};

use bevy_rapier2d::prelude::{
    Collider, ColliderMassProperties, CollisionGroups, Damping, Friction, Group, LockedAxes,
    Restitution, RigidBody, Velocity,
};

use self::{
    actions::{equip_closest_weapon, spawn_skeleton_button},
    actions::{player_attack_sender, ShootEvent},
};

use super::{combat::components::WeaponSocket, components::Player};

/// new type for animations
#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

/// player actions
pub mod actions;
/// player movment functions
mod movement;

/// handles player events, and fn
pub struct PlayerPlugin;
/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ShootEvent>()
            .add_systems(Update,
                (
                    build_player
                        .run_if((|player: Query<&Player>| player.is_empty()).and_then(resource_exists::<ActorTextureHandles>())),
                    player_movement_system,
                    camera_movement_system,
                    player_sprint,
                    spawn_skeleton_button,
                    player_attack_sender,
                    equip_closest_weapon,
                )
            );
    }
}

/// spawns player with no weapons
pub fn build_player(mut commands: Commands, selected_player: Res<ActorTextureHandles>) {
    info!("spawning player");
    commands
        .spawn((
            Player {
                wants_to_teleport: false,
                enter_dungeon_requested: false,
                sprint_available: false,
                just_moved: false,
            },
            PlayerBindings::default(),
            WeaponSocket {
                drawn_slot: Some(WeaponSlots::Slot1), // entity id of currently equipped weapon
                weapon_slots: init_weapon_slots(),
            },
            ActorBundle {
                name: Name::new("Player"),
                actortype: ActorType(Faction::Player),
                animationstate: AnimState {
                    facing: ActorAnimationType::Idle,
                    timer: Timer::from_seconds(0.2, TimerMode::Repeating),
                    animation_frames: vec![0, 1, 2, 3, 4],
                    active_frame: 0,
                },
                available_animations: AnimationSheet {
                    handle: selected_player.rex_sheet.clone(),
                    idle_animation: [0, 1, 2, 3, 4],
                    down_animation: [5, 6, 7, 8, 9],
                    up_animation: [10, 11, 12, 13, 14],
                    right_animation: [15, 16, 17, 18, 19],
                },
                stats: ActorAttributesBundle::default(),
                sprite: SpriteSheetBundle {
                    sprite: TextureAtlasSprite {
                        custom_size: Some(ACTOR_SIZE), //character is 1 tile wide by 2 tiles wide
                        ..default()
                    },
                    texture_atlas: selected_player.rex_sheet.clone(),
                    transform: (Transform {
                        translation: Vec3 {
                            x: 816.0,
                            y: 464.0,
                            z: ACTOR_Z_INDEX,
                        },
                        rotation: Quat::default(),
                        scale: ACTOR_SCALE,
                    }),
                    ..default()
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
            },
        ))
        .with_children(|child| {
            child.spawn((
                PlayerColliderTag,
                ActorColliderBundle {
                    name: Name::new("PlayerCollider"),
                    transformbundle: TransformBundle {
                        local: (Transform {
                            // transform relative to parent
                            translation: (Vec3 {
                                x: 0.,
                                y: -2.,
                                z: ACTOR_PHYSICS_Z_INDEX,
                            }),
                            ..default()
                        }),
                        ..default()
                    },
                    collider: Collider::capsule(
                        ACTOR_COLLIDER.0,
                        ACTOR_COLLIDER.1,
                        ACTOR_COLLIDER.2,
                    ),
                    collisiongroups: CollisionGroups::new(Group::ALL, Group::GROUP_30),
                },
            ));
        });
}

/// creates empty weaponslots
pub fn init_weapon_slots() -> HashMap<WeaponSlots, Option<Entity>> {
    let mut weaponslots = HashMap::new();
    weaponslots.insert(WeaponSlots::Slot1, None::<Entity>);
    weaponslots.insert(WeaponSlots::Slot2, None::<Entity>);
    weaponslots.insert(WeaponSlots::Slot3, None::<Entity>);
    weaponslots.insert(WeaponSlots::Slot4, None::<Entity>);
    warn!("{:#?}", weaponslots);
    weaponslots
}
