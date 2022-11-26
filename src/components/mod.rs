use bevy::prelude::*;

pub mod actors {
    pub mod bundles {
        use crate::components::actors::{
            ai::{AIAggroDistance, AIAttackTimer},
            general::TimeToLive,
        };
        use bevy::prelude::*;
        use bevy_rapier2d::prelude::*;
        use big_brain::thinker::Thinker;

        #[derive(Bundle)]
        pub struct BigBrainBundle {
            pub aggrodistance: AIAggroDistance,
            pub aiattacktimer: AIAttackTimer,
            pub thinker: Thinker,
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
            pub transform_bundle: TransformBundle,
            pub collider: Collider,
        }
    }

    pub mod ai {
        use bevy::prelude::*;
        use bevy_inspector_egui::Inspectable;

        #[derive(Component, Inspectable)]
        pub struct AIEnemy;

        #[derive(Component, Default, Clone, Debug, Inspectable)]
        pub struct AIAggroDistance {
            pub distance: f32,
        }

        #[derive(Component, Default, Clone, Debug, Inspectable)]
        pub struct AIIsAggroed;

        #[derive(Component, Default, Clone, Debug, Inspectable)]
        pub struct AIAttackAction;

        #[derive(Component, Default, Clone, Debug, Reflect)]
        #[reflect(Component)]
        pub struct AIAttackTimer {
            pub timer: Timer,
            pub is_attacking: bool,
            pub is_near: bool,
        }

        #[derive(Component, Default, Clone, Debug, Reflect)]
        #[reflect(Component)]
        pub struct AICanMeander;

        #[derive(Component, Default, Clone, Debug, Reflect)]
        #[reflect(Component)]
        pub struct AIMeanderAction;
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

        #[derive(Component, Inspectable, Clone, Copy)]
        pub struct Player {
            pub wants_to_teleport: bool,
            pub just_teleported: bool,
        }

        #[derive(Component, Inspectable, Clone, Copy)]
        pub struct CombatStats {
            pub stamina: f64,  // gives health per point
            pub agility: f64,  // gives speed per point
            pub strength: f64, // gives damage per point
            pub armor: f64,    // gives damage reduction % + shield points
        }

        #[derive(Component, Inspectable, Clone, Copy)]
        pub struct DefenseStats {
            pub health: f64,
            pub shield: f64,
        }

        #[derive(Component, Default, Reflect, Inspectable)]
        #[reflect(Component)]
        pub struct ActorState {
            //stores actor information, all actors have this
            pub speed: f32,
            pub sprint_available: bool,
            pub facing: FacingDirection,
            pub just_moved: bool,
        }
    }
}

#[derive(Component)]
pub struct OnSplashScreen;

#[derive(Component)]
pub struct MainCamera {
    pub is_active: bool,
}

// Newtype to use a `Timer` for this screen as a resource
#[derive(Resource, Deref, DerefMut)]
pub struct SplashTimer(pub Timer);

#[derive(Resource, Deref, DerefMut)]
pub struct DebugTimer(pub Timer);
