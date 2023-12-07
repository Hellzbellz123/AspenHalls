use crate::{ahp::{
    engine::{
        Bundle, Collider, ColliderMassProperties, CollisionGroups, Damping, Friction, LockedAxes,
        Name, Restitution, RigidBody, SpriteBundle, SpriteSheetBundle, ThinkerBuilder,
        TransformBundle, Velocity,
    },
    game::{
        AIChaseConfig, AIShootConfig, AIWanderConfig, ActorCombatStats, ActorDerivedAttributes,
        ActorPrimaryAttributes, ActorSecondaryAttributes, ActorTertiaryAttributes, ActorType,
        AnimState, AnimationSheet, ProjectileStats, TimeToLive,
    },
}, game::actors::components::{ActorMoveState, ActorColliderTag}};

/// bundle used too spawn "actors"
#[derive(Bundle)]
pub struct ActorBundle {
    /// actor name
    pub name: Name,
    pub move_state: ActorMoveState,
    /// actor type
    pub faction: ActorType,
    /// actor stats
    pub stats: ActorAttributesBundle,
    /// animation state
    pub animation_state: AnimState,
    /// available animations
    pub available_animations: AnimationSheet,
    /// texture data
    pub sprite: SpriteSheetBundle,
    /// actor collisions and movement
    pub rigidbody_bundle: RigidBodyBundle,
}

/// collider bundle for actors
#[derive(Bundle)]
pub struct ActorColliderBundle {
    /// name of collider
    pub name: Name,
    /// location of collider
    pub transform_bundle: TransformBundle,
    /// collider shape
    pub collider: Collider,
    /// collision groups
    pub collision_groups: CollisionGroups,
    /// tag
    pub tag: ActorColliderTag,
}

/// bundle too spawn projectiles
#[derive(Bundle)]
pub struct ProjectileBundle {
    /// projectile name
    pub name: Name,
    /// projectile stats
    pub projectile_stats: ProjectileStats,
    /// projectile lifetime
    pub ttl: TimeToLive,
    /// projectile Sprite
    pub sprite_bundle: SpriteBundle,
    /// projectile collisions and movement
    pub rigidbody_bundle: RigidBodyBundle,
}

/// bundle for projectile colliders
#[derive(Bundle)]
pub struct ProjectileColliderBundle {
    /// collider name
    pub name: Name,
    /// collider lifetime
    pub ttl: TimeToLive,
    /// collider transform
    pub transform_bundle: TransformBundle,
    /// collider shape
    pub collider: Collider,
    /// collision groups
    pub collision_groups: CollisionGroups,
}

/// All Components needed for `stupid_ai` functionality
#[derive(Bundle)]
pub struct StupidAiBundle {
    /// stupid chase action
    pub aggro_config: AIChaseConfig,
    /// stupid wander action
    pub wander_config: AIWanderConfig,
    /// stupid shoot action
    pub shoot_config: AIShootConfig,
    /// chooses action
    pub thinker: ThinkerBuilder,
}

/// all attributes actor can possess
#[derive(Bundle, Default)]
pub struct ActorAttributesBundle {
    /// derived from attributes, working stats
    combat_stat: ActorCombatStats,
    /// base stats, buffed from equipment
    primary: ActorPrimaryAttributes,
    /// secondary stats, buffed from primary
    secondary: ActorSecondaryAttributes,
    /// buffed from primary and equipment
    tertiary: ActorTertiaryAttributes,
    /// final attribute values
    /// used for most calculations
    derived: ActorDerivedAttributes,
}

/// bundle for collisions and movement
/// REQUIRES child collider too work properly
#[derive(Bundle)]
pub struct RigidBodyBundle {
    /// rigidbody
    pub rigidbody: RigidBody,
    /// velocity
    pub velocity: Velocity,
    /// friction
    pub friction: Friction,
    /// bounciness
    pub how_bouncy: Restitution,
    /// RigidBody Mass
    pub mass_prop: ColliderMassProperties,
    /// rotation locks
    pub rotation_locks: LockedAxes,
    /// velocity damping
    pub damping_prop: Damping,
}
