use bevy::{prelude::*, utils::hashbrown::HashMap};

use crate::{
    actors::{
        combat::components::WeaponSlots,
        player::movement::{camera_movement_system, player_movement_system, player_sprint},
    },
    components::actors::{
        animation::FacingDirection,
        bundles::{PlayerColliderBundle, PlayerColliderTag},
    },
    components::actors::{
        animation::{AnimState, AnimationSheet},
        bundles::RigidBodyBundle,
        general::{CombatStats, DefenseStats, MovementState, Player},
    },
    consts::{ACTOR_PHYSICS_Z_INDEX, ACTOR_SIZE, ACTOR_Z_INDEX},
    game::GameStage,
    input::actions::PlayerBindings,
    loading::assets::ActorTextureHandles,
};

use bevy_rapier2d::prelude::{
    Collider, ColliderMassProperties, CollisionGroups, Damping, Friction, Group, LockedAxes,
    Restitution, RigidBody, Velocity,
};

use self::{
    actions::{equip_closest_weapon, spawn_skeleton_button},
    actions::{player_attack_sender, PlayerMeleeEvent, ShootEvent},
};

use super::combat::components::WeaponSocket;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

pub mod actions;
mod movement;

pub struct PlayerPlugin;
/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerMeleeEvent>()
            .add_event::<ShootEvent>()
            .add_system(
                build_player
                    .run_if(|player: Query<&Player>| player.is_empty())
                    .in_schedule(OnEnter(GameStage::PlayingGame)),
            )
            .add_systems(
                (
                    player_movement_system,
                    camera_movement_system,
                    player_sprint,
                    spawn_skeleton_button,
                    player_attack_sender,
                    equip_closest_weapon,
                )
                    .in_set(OnUpdate(GameStage::PlayingGame)),
            );
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    name: Name,
    pub player: Player,
    pub movement_state: MovementState,
    pub player_animationstate: AnimState,
    pub available_animations: AnimationSheet,
    pub sprite: TextureAtlasSprite,
    pub texture_atlas: Handle<TextureAtlas>,
    pub weapon_socket: WeaponSocket,
    pub combat_stats: CombatStats,
    pub defense_stats: DefenseStats,
    // This bundle must be added to your player entity
    // (or whatever else you wish to control)
    #[bundle]
    pub player_input_map: PlayerBindings,
    #[bundle]
    spatial: SpatialBundle,
    #[bundle]
    rigidbody: RigidBodyBundle,
}

// TODO: run after world is spawned, find LDTK player ent and add worldy too it, use that as location too spawn player
pub fn build_player(mut commands: Commands, selected_player: Res<ActorTextureHandles>) {
    info!("spawning player");
    commands
        .spawn((PlayerBundle {
            name: Name::new("Player"),
            player: Player {
                wants_to_teleport: false,
                just_teleported: false,
            },
            movement_state: MovementState {
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
                handle: selected_player.rex_sheet.clone(),
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
                health: 150.0,
                shield: 50.0,
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
            sprite: TextureAtlasSprite {
                custom_size: Some(ACTOR_SIZE), //character is 1 tile wide by 2 tiles wide
                ..default()
            },
            texture_atlas: selected_player.rex_sheet.clone(),
            spatial: SpatialBundle {
                transform: (Transform {
                    translation: Vec3 {
                        x: -60.0,
                        y: 1090.0,
                        z: ACTOR_Z_INDEX,
                    },
                    rotation: Quat::default(),
                    scale: Vec3::ONE,
                }),
                visibility: Visibility::Inherited,
                ..default()
            },
            player_input_map: PlayerBindings::default(),
            weapon_socket: WeaponSocket {
                drawn_slot: WeaponSlots::Slot1, // entity id of currently equipped weapon
                weapon_slots: init_weapon_slots(),
            },
        },))
        .with_children(|child| {
            child.spawn(PlayerColliderBundle {
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
                    Vec2 { x: 0.0, y: -12.0 },
                    Vec2 { x: 0.0, y: 18.8 },
                    13.12,
                ),
                tag: PlayerColliderTag,
                collisiongroups: CollisionGroups::new(Group::ALL, Group::GROUP_30),
            });
        });
}

pub fn init_weapon_slots() -> HashMap<WeaponSlots, Option<Entity>> {
    let mut weaponslots = HashMap::new();
    weaponslots.insert(WeaponSlots::Slot1, None::<Entity>);
    weaponslots.insert(WeaponSlots::Slot2, None::<Entity>);
    weaponslots.insert(WeaponSlots::Slot3, None::<Entity>);
    weaponslots.insert(WeaponSlots::Slot4, None::<Entity>);
    warn!("{:#?}", weaponslots);
    weaponslots
}
