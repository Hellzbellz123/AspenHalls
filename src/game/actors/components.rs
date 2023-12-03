use bevy::prelude::Component;

/// tag for player collider
#[derive(Component)]
pub struct PlayerColliderTag;

/// tag for enemy collider
#[derive(Component)]
pub struct EnemyColliderTag;

/// tag for enemy projectile
#[derive(Component)]
pub struct EnemyProjectileTag;

/// tag for player projectile
#[derive(Component)]
pub struct PlayerProjectileTag;

/// tag for enemy projectile collider
#[derive(Component)]
pub struct EnemyProjectileColliderTag;

/// tag for player projectile collider
#[derive(Component)]
pub struct PlayerProjectileColliderTag;

use bevy::prelude::*;

use super::combat::components::Damage;

/// new type for `Timer` for use with bullet lifetimes
#[derive(Component, Default, Reflect, Deref, DerefMut)]
#[reflect(Component)]
pub struct TimeToLive(pub Timer);

/// is this actor allowed too sprint?
pub enum MoveStatus {
    /// actor is allowed too walk
    CanWalk,
    /// actor is allowed too run
    CanSprint,
    /// actor is not allowed too move
    NotAllowed,
}

/// entity teleport status
pub enum TeleportStatus {
    /// entity has requested a teleport too this pos
    Requested(Vec2),
    /// entity is in process of teleporting
    Teleporting,
    /// entity has finished teleporting
    Done,
}

/// player data
#[derive(Component, Reflect, Clone, Copy, Default)]
pub struct Player {
    /// if player can sprint
    pub sprint_available: bool,
    /// if player just moved
    pub just_moved: bool,
    /// if player wants too teleport
    pub wants_to_teleport: bool,
}

/// projectile data
#[derive(Component, Reflect, Clone, Copy, Default)]
pub struct ProjectileStats {
    /// damage too apply
    pub damage: f32,
    /// velocity of projectile
    pub speed: f32,
    /// size of projectile
    pub size: f32,
}

/// actor primary stats
#[derive(Component, Reflect, Clone, Copy)]
pub struct ActorPrimaryAttributes {
    /// gives health per point
    pub stamina: f32,
    /// gives attack power and parry
    pub strength: f32,
    /// gives attack power and dodge
    pub agility: f32,
    /// gives spell power and mana
    pub intellect: f32,
}

/// secondary attributes affected by Primary attributes
#[derive(Component, Reflect, Clone, Copy)]
pub struct ActorSecondaryAttributes {
    /// gives chance too deal 200% damage, at least 5% chance, 150% if pvp
    pub critical: f32,
    /// rate at which melee auto-attacks are dealt,
    /// cast time, tick rate of dot, hot, and
    /// global cool down, and regeneration rate of energy
    pub haste: f32,
    /// reduces reload time, bow shoot time, and general channeled stuff
    pub mastery: f32,
    /// increases outgoing damage,healing,absorb, and reduce incoming damage.
    pub versatility: f32,
}

/// extra attributes given by guns and armor
#[derive(Component, Reflect, Clone, Copy)]
pub struct ActorTertiaryAttributes {
    /// reduce damage take from aoe attacks
    pub avoidance: f32,
    /// heal based on damage dealt
    pub leech: f32,
    /// increases movement speed
    pub speed: f32,
    /// provides chance too hit more than 1 enemy
    pub multi_strike: f32,
}

/// actor stats derived from primary and tertiary stats
#[derive(Component, Reflect, Clone, Copy)]
pub struct ActorDerivedAttributes {
    // Offensive derived stats
    /// derived from weapon damage and Attack Power
    pub weapon_damage: f32,
    /// derived from strength and agility,
    pub attack_power: f32,
    /// derived from weapon speed or cast time and haste, affects gcd too
    pub attack_speed: f32,
    /// derived from crit on gear + base crit
    pub critical_strike: f32,
    /// derives from intellect
    pub spell_power: f32,
    /// derived from intellect and mastery
    pub regeneration_speed: f32,

    // Defensive derived stats
    /// gives damage reduction % + shield points, derived from equipment and stamina
    pub armor: f32,
    /// chance too dodge physical attacks, derived from agility
    pub dodge: f32,
    /// chance too parry melee attacks, derived from strength
    pub parry: f32,
    /// chance too block with shield, derived from mastery
    pub block: f32,
}

/// denote life/energy/shield level for actors
#[derive(Component, Reflect, Clone, Copy)]
pub struct ActorCombatStats {
    /// actual life points, is dead when 0
    pub health: f32,
    /// energy points, used for casting and some actions
    pub energy: f32,
    /// life point buffer, subtracted before life points,
    pub shield: f32,
}

/// queue of damage too be dealt with once per frame
#[derive(Component, Reflect, Deref, DerefMut)]
pub struct DamageQueue(Vec<Damage>);

impl Default for ActorPrimaryAttributes {
    fn default() -> Self {
        Self {
            stamina: 10.0,
            strength: 10.0,
            agility: 10.0,
            intellect: 10.0,
        }
    }
}

impl Default for ActorSecondaryAttributes {
    fn default() -> Self {
        Self {
            critical: 0.05,
            haste: 10.0,
            mastery: 10.0,
            versatility: 10.0,
        }
    }
}

impl Default for ActorTertiaryAttributes {
    fn default() -> Self {
        Self {
            avoidance: 0.0,
            leech: 0.0,
            multi_strike: 0.0,
            speed: 100.0,
        }
    }
}

impl Default for ActorDerivedAttributes {
    fn default() -> Self {
        Self {
            weapon_damage: 5.0,
            attack_power: 5.0,
            attack_speed: 1.0,
            critical_strike: 0.05,
            spell_power: 5.0,
            regeneration_speed: 5.0,
            armor: 10.0,
            dodge: 0.05,
            parry: 0.05,
            block: 0.05,
        }
    }
}

impl Default for ActorCombatStats {
    fn default() -> Self {
        Self {
            health: 50.0,
            energy: 100.0,
            shield: 50.0,
        }
    }
}
