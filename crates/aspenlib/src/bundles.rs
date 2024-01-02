use bevy::prelude::Reflect;
use bevy_asepritesheet::animator::AnimatedSpriteBundle;

use crate::{
    game::actors::{
        ai::components::AICombatConfig,
        attributes_stats::{CharacterStatBundle, EquipmentStats, ProjectileStats},
        combat::components::{AttackDamage, WeaponForm, WeaponHolder},
        components::{ActorColliderType, ActorMoveState},
    },
    loading::{custom_assets::actor_definitions::AiSetupConfig, registry::RegistryIdentifier},
    prelude::{
        engine::{
            Bundle, Collider, ColliderMassProperties, CollisionGroups, Damping, Friction,
            LockedAxes, Name, Restitution, RigidBody, SpriteBundle, ThinkerBuilder,
            TransformBundle, Velocity,
        },
        game::{AIShootConfig, AIWanderConfig, ActorType, TimeToLive},
    },
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
}

// TODO: rename too ObjectBundle
// make ObjectType hold the important bits
/// bundle for spawning weapons
#[derive(Bundle, Reflect, Clone)]
pub struct WeaponBundle {
    /// weapon name
    pub name: Name,
    /// accesor for weapon definition
    pub identifier: RegistryIdentifier,
    /// weapon stored slot
    pub holder: WeaponHolder,
    /// weapons function when used
    pub damage: AttackDamage,
    /// how this weapon attacks, along with data for attack
    pub weapon_type: WeaponForm,
    /// stats applied too holder
    pub stats: EquipmentStats,
    /// sprite for weapon
    #[reflect(ignore)]
    pub sprite: AnimatedSpriteBundle,
    /// weapon physics
    #[reflect(ignore)]
    pub rigidbody_bundle: RigidBodyBundle,
    // /// weapon stats
    // pub weapon_stats: WeaponStats,
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

/// collider bundle for actors
#[derive(Bundle)]
pub struct ObjectColliderBundle {
    /// name of collider
    pub name: Name,
    /// type of collider
    pub tag: ActorColliderType,
    /// collider shape
    pub collider: Collider,
    /// collision groups
    pub collision_groups: CollisionGroups,
    /// collider transform
    pub transform_bundle: TransformBundle,
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

impl std::fmt::Debug for WeaponBundle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WeaponBundle")
            .field("name", &self.name)
            .field("holder", &self.holder)
            .field("damage", &self.damage)
            .field("weapon_type", &self.weapon_type)
            .field("stats", &self.stats)
            .field("sprite", &self.sprite.spritesheet)
            .field("rigidbody_bundle", &self.rigidbody_bundle.rigidbody)
            .finish()
    }
}
