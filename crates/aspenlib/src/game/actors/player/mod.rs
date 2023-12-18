use bevy::{prelude::*, utils::hashbrown::HashMap};

use crate::{
    bundles::{ActorAttributesBundle, ActorBundle, ActorColliderBundle, RigidBodyBundle},
    consts::{
        actor_collider, AspenCollisionLayer, ACTOR_PHYSICS_Z_INDEX, ACTOR_SCALE, ACTOR_SIZE,
        ACTOR_Z_INDEX,
    },
    game::actors::{
        ai::components::{ActorType, Faction},
        animation::components::{ActorAnimationType, AnimState, AnimationSheet},
        components::{
            ActorColliderTag, ActorMoveState, AllowedMovement, CurrentMovement, PlayerColliderTag,
            TeleportStatus,
        },
    },
    game::{
        actors::{
            combat::components::WeaponSlots,
            player::movement::{camera_movement_system, update_player_velocity},
        },
        input::action_maps::PlayerBindings,
    },
    loading::assets::ActorTextureHandles,
};

use bevy_rapier2d::prelude::{
    ColliderMassProperties, CollisionGroups, Damping, Friction, LockedAxes, Restitution, RigidBody,
    Velocity,
};

use self::{
    actions::{equip_closest_weapon, spawn_custom_on_button},
    actions::{player_attack_sender, ShootEvent},
};

use super::{combat::components::WeaponSocket, components::Player};

/// new type for animations
#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

/// player actions
pub mod actions;
/// player movement functions
mod movement;

/// handles player events, and fn
pub struct PlayerPlugin;
/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ShootEvent>().add_systems(
            Update,
            (
                build_player.run_if(
                    (|player: Query<&Player>| player.is_empty())
                        .and_then(resource_exists::<ActorTextureHandles>()),
                ),
                update_player_velocity,
                camera_movement_system,
                spawn_custom_on_button,
                player_attack_sender,
                equip_closest_weapon,
            ),
        );
    }
}

/// spawns player with no weapons
pub fn build_player(mut commands: Commands, selected_player: Res<ActorTextureHandles>) {
    info!("spawning player");
    commands
        .spawn((
            Player,
            PlayerBindings::default(),
            WeaponSocket {
                drawn_slot: Some(WeaponSlots::Slot1), // entity id of currently equipped weapon
                weapon_slots: empty_weapon_slots(),
            },
            ActorBundle {
                name: Name::new("Player"),
                faction: ActorType::Npc(Faction::Player),
                move_state: ActorMoveState {
                    move_status: CurrentMovement::None,
                    move_perms: AllowedMovement::Run,
                    teleport_status: TeleportStatus::default(),
                },
                animation_state: AnimState {
                    animation_type: ActorAnimationType::Idle,
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
                rigidbody_bundle: RigidBodyBundle {
                    rigidbody: RigidBody::Dynamic,
                    velocity: Velocity::default(),
                    friction: Friction::coefficient(0.7),
                    how_bouncy: Restitution::coefficient(0.3),
                    mass_prop: ColliderMassProperties::Density(0.3),
                    rotation_locks: LockedAxes::ROTATION_LOCKED,
                    damping_prop: Damping {
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
                    tag: ActorColliderTag,
                    name: Name::new("PlayerCollider"),
                    transform_bundle: TransformBundle {
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
                    collider: actor_collider(),
                    collision_groups: CollisionGroups::new(
                        AspenCollisionLayer::ACTOR,
                        AspenCollisionLayer::EVERYTHING,
                    ),
                },
            ));
        });
}

/// creates empty weapon slots
pub fn empty_weapon_slots() -> HashMap<WeaponSlots, Option<Entity>> {
    let mut weapon_slots = HashMap::new();
    weapon_slots.insert(WeaponSlots::Slot1, None::<Entity>);
    weapon_slots.insert(WeaponSlots::Slot2, None::<Entity>);
    weapon_slots.insert(WeaponSlots::Slot3, None::<Entity>);
    weapon_slots.insert(WeaponSlots::Slot4, None::<Entity>);
    weapon_slots
}
