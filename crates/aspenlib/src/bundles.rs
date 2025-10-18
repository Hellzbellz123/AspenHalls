use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::{AnimationState, AseAnimation};

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

// TODO: move these too the correct spots
/// tags this entity for collider creation
/// requires that entities have an Aabb for proper collider size
#[derive(Debug, Clone, Component)]
pub enum NeedsCollider {
    Aabb,
    Rectangle { x: f32, y: f32 },
}

/// empty locked axis bits
pub const NO_LOCKED_AXES: LockedAxes = LockedAxes::from_bits(0b00_000);

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
    /// actor stat
    pub stats: CharacterStatBundle,
    /// is character ai controlled or player controlled
    pub controller: AiType,
    #[reflect(ignore)]
    /// required components too render an Aseprite file as an Actor
    pub render: Aspen2dRenderBundle,
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
    #[reflect(ignore)]
    /// requirements too render weapon
    pub render: Aspen2dRenderBundle,
}

/// bundle too spawn projectiles
#[derive(Bundle, Clone)]
pub struct ProjectileBundle {
    /// projectile name
    pub name: Name,
    /// projectile stats
    pub projectile_stats: ProjectileStats,
    /// projectile lifetime
    pub ttl: TimeToLive,
    /// projectile collisions and movement
    pub rigidbody_bundle: Aspen2dPhysicsBundle,
}

/// The `AspenRenderBundle` holds all components needed to render Aseprite files as Actors.
#[derive(Bundle, Default)]
pub struct Aspen2dRenderBundle {
    /// asperite asset for this sprite
    pub handle: AseAnimation,
    /// animation play information
    pub animation_state: AnimationState,
    /// sprite configuration
    pub sprite: Sprite,
}

// TODO: yeet this clone, this is ugly
impl Clone for Aspen2dRenderBundle {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
            animation_state: AnimationState::default(),
            sprite: self.sprite.clone(),
        }
    }
}

/// collider bundle for actors
#[derive(Debug, Bundle)]
pub struct AspenColliderBundle {
    /// name of collider
    pub name: Name,
    /// type of collider
    pub tag: ActorColliderType,
    /// collider shape
    pub collider: NeedsCollider,
    /// collision groups
    pub collision_groups: CollisionLayers,
    /// collider transform
    pub transform: Transform,
}

#[derive(Bundle, Clone)]
pub struct Aspen2dPhysicsBundle {
    pub rigidbody: RigidBody,
    pub friction: Friction,
    pub how_bouncy: Restitution,
    pub mass_prop: MassPropertiesBundle,
    pub linear_damping: LinearDamping,
    pub angular_damping: AngularDamping,
    pub velocity: LinearVelocity,
    pub ang_vel: AngularVelocity,
    pub rotation_locks: LockedAxes,
    pub interpolate_loc: TranslationInterpolation,
}
pub const GRAVITY: f32 = 9.81;

pub fn sized_mass(
    // object weight in kg
    weight: f32,
    // 2d dimensions of this object
    dim2: Vec2,
) -> MassPropertiesBundle {
    let size = dim2;
    let collider = Collider::rectangle(size.x, size.y);
    let density = weight / GRAVITY;

    MassPropertiesBundle::from_shape(&collider, density)
}

impl Aspen2dPhysicsBundle {
    // TODO: impl this correctly
    pub fn new_projectile(velocity: Vec2) -> Self {
        Self {
            rigidbody: RigidBody::Dynamic,
            velocity: LinearVelocity(velocity),
            ang_vel: AngularVelocity::ZERO,
            friction: Friction {
                dynamic_coefficient: 0.3,
                static_coefficient: 0.3,
                combine_rule: CoefficientCombine::Average,
            },
            how_bouncy: Restitution {
                coefficient: 0.4,
                combine_rule: CoefficientCombine::Average,
            },
            mass_prop: sized_mass(0.5, Vec2 { x: 16.0, y: 32.0 }),
            rotation_locks: NO_LOCKED_AXES,
            linear_damping: LinearDamping(0.1),
            angular_damping: AngularDamping(0.1),
            interpolate_loc: TranslationInterpolation,
        }
    }

    pub fn default_character() -> Self {
        Self {
            rigidbody: RigidBody::Dynamic,
            velocity: LinearVelocity::ZERO,
            ang_vel: AngularVelocity::ZERO,
            friction: Friction {
                dynamic_coefficient: 0.7,
                static_coefficient: 0.7,
                combine_rule: CoefficientCombine::Average,
            },
            how_bouncy: Restitution {
                coefficient: 0.3,
                combine_rule: CoefficientCombine::Average,
            },
            mass_prop: sized_mass(65.0, Vec2 { x: 16.0, y: 32.0 }),
            rotation_locks: LockedAxes::ROTATION_LOCKED,
            linear_damping: LinearDamping(1.0),
            angular_damping: AngularDamping(1.0),
            interpolate_loc: TranslationInterpolation,
        }
    }

    pub fn default_item() -> Self {
        Self {
            rigidbody: RigidBody::Dynamic,
            velocity: LinearVelocity::ZERO,
            ang_vel: AngularVelocity::ZERO,
            friction: Friction {
                dynamic_coefficient: 0.5,
                static_coefficient: 0.5,
                combine_rule: CoefficientCombine::Average,
            },
            how_bouncy: Restitution {
                coefficient: 0.4,
                combine_rule: CoefficientCombine::Average,
            },
            mass_prop: sized_mass(10.0, Vec2 { x: 16.0, y: 32.0 }),
            rotation_locks: NO_LOCKED_AXES,
            linear_damping: LinearDamping(0.8),
            angular_damping: AngularDamping(0.6),
            interpolate_loc: TranslationInterpolation,
        }
    }
}

// TODO: upstream reflect/debug fixes and remove this manual impl
impl std::fmt::Debug for Aspen2dRenderBundle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Aspen2dRenderBundle")
            .field("sprite asset", &self.handle)
            .field("animation name", &self.handle.animation.tag)
            .field("sprite cfg", &self.sprite)
            .finish_non_exhaustive()
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
            .field("render", &self.render)
            .finish()
    }
}
