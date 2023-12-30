use bevy::prelude::Reflect;
use bevy_asepritesheet::animator::AnimatedSpriteBundle;

use crate::{
    prelude::{
        engine::{
            Bundle, Collider, ColliderMassProperties, CollisionGroups, Damping, Friction,
            LockedAxes, Name, Restitution, RigidBody, SpriteBundle,
            ThinkerBuilder, TransformBundle, Velocity,
        },
        game::{
            AIShootConfig, AIWanderConfig, ActorType,
            TimeToLive,
        },
    },
    game::actors::{
        ai::components::AICombatConfig,
        components::{ActorMoveState, ProjectileTag, ProjectileColliderTag, CharacterColliderTag}, attributes_stats::{CharacterStatBundle, ProjectileStats},
    }, loading::custom_assets::npc_definition::{AiSetupConfig, RegistryIdentifier},
};

/// bundle used too spawn "actors"
#[derive(Bundle, Reflect, Clone)]
pub struct CharacterBundle {
    /// actor name
    pub name: Name,
    /// id too get actor definition
    pub identifier: RegistryIdentifier,
    /// actors current movement data
    pub move_state: ActorMoveState,
    /// actor type
    pub actor_type: ActorType,
    /// actor stats
    pub stats: CharacterStatBundle,
    /// is character ai controlled or player controlled
    pub controller: AiSetupConfig,
    /// texture and animations
    #[reflect(ignore)]
    pub aseprite: AnimatedSpriteBundle,
    /// actor collisions and movement
    #[reflect(ignore)]
    pub rigidbody_bundle: RigidBodyBundle,
    // /// animation state
    // pub animation_state: AnimState,
    // /// sprite sheet bundle old
    // pub sprite: SpriteBundle,
    // /// available animations
    // pub available_animations: AnimationSheet,
}

/// collider bundle for actors
#[derive(Bundle)]
pub struct CharacterColliderBundle {
    /// name of collider
    pub name: Name,
    /// location of collider
    pub transform_bundle: TransformBundle,
    /// collider shape
    pub collider: Collider,
    /// collision groups
    pub collision_groups: CollisionGroups,
    /// tag
    pub tag: CharacterColliderTag,
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
    /// tag
    pub tag: ProjectileTag,
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
    /// tag
    pub tag: ProjectileColliderTag,
}

/// All Components needed for `stupid_ai` functionality
#[derive(Bundle)]
pub struct StupidAiBundle {
    /// ai chase/attack config
    pub combat_config: AICombatConfig,
    /// stupid wander action
    pub wander_config: AIWanderConfig,
    /// stupid shoot action
    pub shoot_config: AIShootConfig,
    /// chooses action
    pub thinker: ThinkerBuilder,
}

// /// all attributes actor can possess
// #[derive(Bundle, Default, Debug, Clone)]
// pub struct ActorAttributesBundle {
//     /// derived from attributes, working stats
//     combat_stat: ActorCombatStats,
//     /// base stats, buffed from equipment
//     primary: ActorPrimaryAttributes,
//     /// secondary stats, buffed from primary
//     secondary: ActorSecondaryAttributes,
//     /// buffed from primary and equipment
//     tertiary: ActorTertiaryAttributes,
//     /// final attribute values
//     /// used for most calculations
//     derived: ActorDerivedAttributes,
// }

/// bundle for collisions and movement
/// REQUIRES child collider too work properly
#[derive(Bundle, Clone)]
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

impl RigidBodyBundle {
    pub const ENEMY: RigidBodyBundle = RigidBodyBundle {
        rigidbody: bevy_rapier2d::prelude::RigidBody::Dynamic,
        velocity: Velocity::zero(),
        friction: Friction::coefficient(0.7),
        how_bouncy: Restitution::coefficient(0.3),
        mass_prop: ColliderMassProperties::Density(0.3),
        rotation_locks: LockedAxes::ROTATION_LOCKED,
        damping_prop: Damping {
            linear_damping: 1.0,
            angular_damping: 1.0,
        },
    };
}

impl Default for RigidBodyBundle {
    fn default() -> Self {
        Self::ENEMY
    }
}
