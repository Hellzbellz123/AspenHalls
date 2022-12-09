use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

pub mod actors {
    pub mod spawners {
        use bevy::{
            prelude::{Component, Deref, DerefMut, ReflectComponent, Transform},
            reflect::Reflect,
            time::Timer,
        };
        use bevy_inspector_egui::Inspectable;

        #[derive(Component)]
        pub struct EnemyContainerTag;

        #[derive(Debug, Component, DerefMut, Deref)]
        pub struct SpawnerTimer(pub Timer);

        #[derive(Component, Inspectable, Debug, Reflect, Default)]
        pub enum EnemyType {
            #[default]
            Skeleton,
            Slime,
        }

        #[derive(Component, Inspectable)]
        pub struct Spawner {
            pub enemy_to_spawn: EnemyType,
            pub spawn_radius: f32,
            pub max_enemies: i32,
        }

        #[derive(Component, Debug, Reflect, Default)]
        #[reflect(Component)]
        pub struct SpawnEvent {
            pub enemy_to_spawn: EnemyType,
            pub spawn_position: Transform,
            pub spawn_count: i32,
        }
    }
    pub mod bundles {
        use crate::components::actors::{
            ai::{AIAttackTimer, AICanChase, AICanWander, ActorType},
            general::TimeToLive,
        };
        use bevy::prelude::*;
        use bevy_rapier2d::prelude::*;
        use big_brain::thinker::ThinkerBuilder;

        #[derive(Bundle)]
        pub struct SkeletonAiBundle {
            pub actortype: ActorType,
            pub aggrodistance: AICanChase,
            pub canmeander: AICanWander,
            pub aiattacktimer: AIAttackTimer,
            pub thinker: ThinkerBuilder,
        }

        #[derive(Bundle)]
        pub struct ProjectileBundle {
            pub name: Name,
            pub ttl: TimeToLive,
            #[bundle]
            pub sprite_bundle: SpriteBundle,

            #[bundle]
            pub rigidbody_bundle: RigidBodyBundle,
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

        #[derive(Bundle)]
        pub struct ActorColliderBundle {
            pub name: Name,
            pub transformbundle: TransformBundle,
            pub collider: Collider,
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
        pub struct AIEnemy;

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
        pub struct CombatStats {
            pub stamina: f64,  // gives health per point
            pub agility: f64,  // gives speed per point
            pub strength: f64, // gives damage per point
            pub armor: f64,    // gives damage reduction % + shield points
        }

        #[derive(Component, Inspectable, Clone, Copy, Default)]
        pub struct DefenseStats {
            pub health: f64,
            pub shield: f64,
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
