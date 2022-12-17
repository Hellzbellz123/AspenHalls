pub mod error;

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

pub mod actors {
    pub mod spawners {

        use crate::components::error::ParseEnemyTypeError;
        use bevy::{
            prelude::{Component, Deref, DerefMut, ReflectComponent, Vec3},
            reflect::Reflect,
            time::Timer,
        };
        use bevy_inspector_egui::Inspectable;
        use std::str::FromStr;

        #[derive(Component)]
        pub struct EnemyContainerTag;

        #[derive(Debug, Component, DerefMut, Deref)]
        pub struct SpawnerTimer(pub Timer);

        #[derive(Component, Inspectable, Debug, Reflect, Default, Clone, Copy)]
        pub enum EnemyType {
            #[default]
            Skeleton,
            Slime,
        }

        impl FromStr for EnemyType {
            type Err = ParseEnemyTypeError;
            fn from_str(input: &str) -> Result<EnemyType, Self::Err> {
                match input.to_lowercase().as_str() {
                    "skeleton" => Ok(EnemyType::Skeleton),
                    "slime" => Ok(EnemyType::Slime),
                    _ => Err(ParseEnemyTypeError),
                }
            }
        }

        #[derive(Component, Inspectable, Debug, Reflect, Default, Clone, Copy)]
        pub enum WeaponType {
            #[default]
            SmallSMG,
            SmallPistol,
        }

        impl FromStr for WeaponType {
            type Err = ParseEnemyTypeError;
            fn from_str(input: &str) -> Result<WeaponType, Self::Err> {
                match input {
                    "smallpistol" => Ok(WeaponType::SmallPistol),
                    "smallsmg" => Ok(WeaponType::SmallSMG),
                    _ => Err(ParseEnemyTypeError),
                }
            }
        }

        #[derive(Component, Inspectable, Debug, Reflect)]
        pub enum SpawnType {
            Item,
            Weapon,
            EnemyType,
        }

        #[derive(Component, Inspectable)]
        pub struct Spawner {
            pub enemytype: EnemyType,
            pub spawn_radius: f32,
            pub max_enemies: i32,
        }

        #[derive(Component, Debug, Reflect, Default)]
        #[reflect(Component)]
        pub struct SpawnEnemyEvent {
            pub enemy_to_spawn: EnemyType,
            pub spawn_position: Vec3,
            pub spawn_count: i32,
        }

        #[derive(Component, Debug, Reflect, Default)]
        #[reflect(Component)]
        pub struct SpawnWeaponEvent {
            pub weapon_to_spawn: WeaponType,
            pub spawn_position: Vec3,
            pub spawn_count: i32,
        }
    }
    pub mod bundles {
        use crate::components::actors::{
            ai::{AIAttackTimer, AICanChase, AICanWander, ActorType},
            general::{ProjectileStats, TimeToLive},
        };
        use bevy::prelude::*;
        use bevy_rapier2d::prelude::*;
        use big_brain::thinker::ThinkerBuilder;

        #[derive(Bundle)]
        pub struct StupidAiBundle {
            pub actortype: ActorType,
            pub aggrodistance: AICanChase,
            pub canmeander: AICanWander,
            pub aiattacktimer: AIAttackTimer,
            pub thinker: ThinkerBuilder,
        }

        #[derive(Bundle)] //bundle for ease of use
        pub struct RigidBodyBundle {
            pub rigidbody: RigidBody,
            pub velocity: Velocity,
            pub friction: Friction,
            pub howbouncy: Restitution,
            pub massprop: ColliderMassProperties,
            pub rotationlocks: LockedAxes,
            pub dampingprop: Damping,
        }

        #[derive(Component)]
        pub struct PlayerColliderTag;

        #[derive(Component)]
        pub struct EnemyColliderTag;

        #[derive(Component)]
        pub struct EnemyProjectileTag;

        #[derive(Component)]
        pub struct PlayerProjectileTag;

        #[derive(Component)]
        pub struct EnemyProjectileColliderTag;

        #[derive(Component)]
        pub struct PlayerProjectileColliderTag;

        #[derive(Bundle)]
        pub struct PlayerProjectileBundle {
            pub name: Name,
            pub tag: PlayerProjectileTag,
            pub projectile_stats: ProjectileStats,
            pub ttl: TimeToLive,
            #[bundle]
            pub sprite_bundle: SpriteBundle,
            #[bundle]
            pub rigidbody_bundle: RigidBodyBundle,
        }

        #[derive(Bundle)]
        pub struct EnemyProjectileBundle {
            pub name: Name,
            pub tag: EnemyProjectileTag,
            pub projectile_stats: ProjectileStats,
            pub ttl: TimeToLive,
            #[bundle]
            pub sprite_bundle: SpriteBundle,
            #[bundle]
            pub rigidbody_bundle: RigidBodyBundle,
        }

        #[derive(Bundle)]
        pub struct EnemyColliderBundle {
            pub name: Name,
            pub tag: EnemyColliderTag,
            pub transformbundle: TransformBundle,
            pub collider: Collider,
            pub collisiongroups: CollisionGroups,
        }

        #[derive(Bundle)]
        pub struct PlayerColliderBundle {
            pub name: Name,
            pub tag: PlayerColliderTag,
            pub transformbundle: TransformBundle,
            pub collider: Collider,
            pub collisiongroups: CollisionGroups,
        }

        #[derive(Bundle)]
        pub struct EnemyProjectileColliderBundle {
            pub name: Name,
            pub tag: EnemyProjectileColliderTag,
            pub ttl: TimeToLive,
            pub transformbundle: TransformBundle,
            pub collider: Collider,
            pub collisiongroups: CollisionGroups,
        }

        #[derive(Bundle)]
        pub struct PlayerProjectileColliderBundle {
            pub name: Name,
            pub tag: PlayerProjectileColliderTag,
            pub ttl: TimeToLive,
            pub transformbundle: TransformBundle,
            pub collider: Collider,
            pub collisiongroups: CollisionGroups,
        }
    }

    pub mod ai {
        use bevy::prelude::*;
        use bevy_inspector_egui::Inspectable;

        #[derive(Inspectable)]
        pub enum TypeEnum {
            Enemy,
            Neutral,
            Friendly,
            Player,
        }

        #[derive(Component, Deref, DerefMut, Inspectable)]
        pub struct ActorType(pub TypeEnum);

        #[derive(Component, Inspectable)]
        pub enum AIEnemy {
            Skeleton,
            Slime,
        }

        /// enemies that can chase
        #[derive(Component, Default, Clone, Debug, Inspectable)]
        pub struct AICanChase {
            pub aggro_distance: f32,
        }

        /// enemies that can wander
        #[derive(Component, Default, Clone, Debug, Inspectable)]
        pub struct AICanWander {
            pub wander_target: Option<Vec3>,
            pub spawn_position: Option<Vec3>,
        }

        /// enemeies that can chase scorer
        #[derive(Component, Default, Clone, Debug, Inspectable)]
        pub struct AggroScore;

        /// enemies that wander scorer
        #[derive(Component, Default, Clone, Debug, Inspectable)]
        pub struct WanderScore;

        /// enemies with this tag are chasing a target
        #[derive(Component, Default, Clone, Debug, Inspectable)]
        pub struct AIChaseAction;

        /// enemies with this tag are wandering
        #[derive(Component, Default, Clone, Debug, Inspectable)]
        pub struct AIWanderAction;

        #[derive(Component, Default, Clone, Debug, Reflect)]
        #[reflect(Component)]
        pub struct AIAttackTimer {
            pub timer: Timer,
            pub is_attacking: bool,
            pub is_near: bool,
        }
    }

    pub mod animation {
        use bevy::prelude::*;
        use bevy_inspector_egui::Inspectable;

        #[derive(Default, Component, Inspectable)]
        pub struct AnimationSheet {
            pub handle: Handle<TextureAtlas>,
            pub idle_animation: [usize; 5],
            pub up_animation: [usize; 5],
            pub down_animation: [usize; 5],
            pub right_animation: [usize; 5],
        }

        #[derive(
            Component,
            Default,
            Clone,
            Copy,
            Inspectable,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Debug,
            Reflect,
        )]
        pub enum FacingDirection {
            #[default]
            Idle,
            Down,
            Left,
            Up,
            Right,
        }

        #[derive(Component, Default, Inspectable)]
        pub struct PlayerGraphics {
            pub facing: FacingDirection,
        }

        #[derive(Component, Default, Reflect)]
        #[reflect(Component)]
        #[allow(clippy::module_name_repetitions)]
        pub struct AnimState {
            pub timer: Timer,
            pub current_frames: Vec<usize>,
            pub current_frame: usize,
        }
    }

    pub mod general {
        use crate::components::actors::animation::FacingDirection;
        use bevy::prelude::*;
        use bevy_inspector_egui::Inspectable;

        #[derive(Component, Default, Reflect, Deref, DerefMut)]
        #[reflect(Component)]
        pub struct TimeToLive(pub Timer);

        #[derive(Component, Inspectable, Clone, Copy, Default)]
        pub struct Player {
            pub wants_to_teleport: bool,
            pub just_teleported: bool,
        }

        #[derive(Component, Inspectable, Clone, Copy, Default)]
        pub struct ProjectileStats {
            pub damage: f32,
            pub speed: f32,
            pub size: f32,
        }

        #[derive(Component, Inspectable, Clone, Copy, Default)]
        pub struct CombatStats {
            pub stamina: f64,  // gives health per point
            pub agility: f64,  // gives speed per point
            pub strength: f64, // gives damage per point
            pub armor: f64,    // gives damage reduction % + shield points
        }

        #[derive(Component, Inspectable, Clone, Copy, Default)]
        pub struct DefenseStats {
            pub health: f32,
            pub shield: f32,
        }

        #[derive(Component, Default, Inspectable)]
        pub struct MovementState {
            //stores actor information, all actors have this
            pub speed: f32,             //TODO: Refactor into stats
            pub sprint_available: bool, // refactor these into the movment system? facing direction too the graphics plugin somewhere maybe
            pub facing: FacingDirection,
            pub just_moved: bool,
        }
    }
}

#[derive(Component)]
pub struct OnSplashScreen;

#[derive(Component, Inspectable)]
pub struct MainCameraTag {
    pub is_active: bool,
}

// Newtype to use a `Timer` for this screen as a resource
#[derive(Resource, Deref, DerefMut)]
pub struct SplashTimer(pub Timer);

#[derive(Resource, Deref, DerefMut)]
pub struct DebugTimer(pub Timer);
