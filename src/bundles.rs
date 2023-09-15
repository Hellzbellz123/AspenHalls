use crate::game::actors::{ai::components::*, animation::components::*, components::*};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use big_brain::thinker::ThinkerBuilder;

/// bundle used too spawn actors
#[derive(Bundle)]
pub struct ActorBundle {
    /// actor name
    pub name: Name,
    /// actor type
    pub actortype: ActorType,
    /// animation state
    pub animationstate: AnimState,
    /// available animations
    pub available_animations: AnimationSheet,
    /// texture data
    pub sprite: SpriteSheetBundle,
    /// actor stats
    pub stats: ActorAttributesBundle,
    /// actor collisions and movement
    pub rigidbody: RigidBodyBundle,
}

/// collider bundle for actors
#[derive(Bundle)]
pub struct ActorColliderBundle {
    /// name of collider
    pub name: Name,
    /// location of collider
    pub transformbundle: TransformBundle,
    /// collider shape
    pub collider: Collider,
    /// collision groups
    pub collisiongroups: CollisionGroups,
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
    /// collider transfoorm
    pub transformbundle: TransformBundle,
    /// collider shape
    pub collider: Collider,
    /// collison groups
    pub collisiongroups: CollisionGroups,
}

/// All Componenents needed for 'stupid_ai' functionality
#[derive(Bundle)]
pub struct StupidAiBundle {
    /// stupid chase action
    pub canaggro: AICanAggro,
    /// stupid wander action
    pub canmeander: AICanWander,
    /// stupid shoot action
    pub canshoot: AICanShoot,
    /// shoot data
    pub aiattacktimer: AIAttackState,
    /// chooses action
    pub thinker: ThinkerBuilder,
}

/// all attributes actor can possess
#[derive(Bundle, Default)]
pub struct ActorAttributesBundle {
    /// derived from attributes, working stats
    combatstat: ActorCombatStats,
    /// base stats, buffed from equipment
    primary: ActorPrimaryAttributes,
    /// secondar stats, buffed from primary
    secondary: ActorSecondaryAttributes,
    /// buffed from primary and equipment
    tertiary: ActorTertiaryAttributes,
    /// final attribute values
    /// used for most calculations
    derived: ActorDerivedAttributes,
}

/// bundle for collisions and movement
#[derive(Bundle)]
pub struct RigidBodyBundle {
    /// rigidbody
    pub rigidbody: RigidBody,
    /// velocity
    pub velocity: Velocity,
    /// friction
    pub friction: Friction,
    /// bounciness
    pub howbouncy: Restitution,
    /// RigidBody Mass
    pub massprop: ColliderMassProperties,
    /// rotation locks
    pub rotationlocks: LockedAxes,
    /// velocity damping
    pub dampingprop: Damping,
}
