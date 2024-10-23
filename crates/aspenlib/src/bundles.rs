use bevy::prelude::*;
use bevy_asepritesheet::animator::AnimatedSpriteBundle;
use bevy_rapier2d::prelude::*;

use crate::{
    game::{
        attributes_stats::{CharacterStatBundle, EquipmentStats, ProjectileStats},
        characters::{
            ai::components::AiType,
            components::{CharacterMoveState, CharacterType},
        },
        components::{ActorColliderType, TimeToLive},
        items::weapons::components::{AttackDamage, WeaponDescriptor, WeaponHolder},
    },
    loading::registry::RegistryIdentifier,
};

/// bundle used too spawn "actors"
#[derive(Bundle, Reflect, Clone)]
pub struct CharacterBundle {
    /// actor name
    pub name: Name,
    /// id too get actor definition
    pub identifier: RegistryIdentifier,
    /// actors current movement data
    pub move_state: CharacterMoveState,
    /// actor type
    pub actor_type: CharacterType,
    /// actor stats
    pub stats: CharacterStatBundle,
    /// is character ai controlled or player controlled
    pub controller: AiType,
    /// texture and animations
    #[reflect(ignore)]
    pub aseprite: AnimatedSpriteBundle,
    /// actor collisions and movement
    #[reflect(ignore)]
    pub rigidbody_bundle: RigidBodyBundle,
}

/// bundle for spawning weapons
#[derive(Bundle, Reflect, Clone)]
pub struct WeaponBundle {
    /// weapon name
    pub name: Name,
    /// accesor for weapon definition
    pub identifier: RegistryIdentifier,
    /// who is holding this weapon, and what slot it is in
    pub holder: WeaponHolder,
    /// weapons damage when used
    pub damage: AttackDamage,
    /// how this weapon applies its damage, along with config
    pub weapon_type: WeaponDescriptor,
    /// stats applied too holder
    pub stats: EquipmentStats,
    /// sprite for weapon
    #[reflect(ignore)]
    pub sprite: AnimatedSpriteBundle,
    /// weapon physics
    #[reflect(ignore)]
    pub rigidbody_bundle: RigidBodyBundle,
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
pub struct ActorColliderBundle {
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
    /// `RigidBody` Mass
    pub mass_prop: ColliderMassProperties,
    /// rotation locks
    pub rotation_locks: LockedAxes,
    /// velocity damping
    pub damping_prop: Damping,
}

impl RigidBodyBundle {
    /// default enemy rigidbody stats
    pub const DEFAULT_CHARACTER: Self = Self {
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
        Self::DEFAULT_CHARACTER
    }
}

impl std::fmt::Debug for WeaponBundle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WeaponBundle")
            .field("name", &self.name)
            .field("identifier", &self.identifier)
            .field("holder", &self.holder)
            .field("damage", &self.damage)
            .field("weapon_type", &self.weapon_type)
            .field("stats", &self.stats)
            .field("sprite", &self.sprite.spritesheet)
            .field("rigidbody_bundle", &self.rigidbody_bundle.rigidbody)
            .finish()
    }
}
