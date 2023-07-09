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

/// newtype for `Timer` for use with bullet lifetimes
#[derive(Component, Default, Reflect, Deref, DerefMut)]
#[reflect(Component)]
pub struct TimeToLive(pub Timer);

/// player data
#[derive(Component, Reflect, Clone, Copy, Default)]
pub struct Player {
    /// if player can sprint
    pub sprint_available: bool,
    /// if player just moved
    pub just_moved: bool,
    /// if player wants too teleport
    pub wants_to_teleport: bool,
    /// if player enter dungeon requested
    pub enter_dungeon_requested: bool,
}

/// projectile data
#[derive(Component, Reflect, Clone, Copy, Default)]
pub struct ProjectileStats {
    /// damage too apply
    pub damage: f32,
    /// velocit of projectile
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
    /// gives chance too deal 200% damage, atleast 5% chance, 150% if pvp
    pub critical: f32,
    /// rate at which melee auto-attacks are dealt,
    /// cast time, tick rate of dot, hot, and
    /// global cooldown, and regen rate of energy
    pub haste: f32,
    /// reduces reload time, bow shoot time, and general channeled stuff
    pub mastery: f32,
    /// increases outgoing damage,healing,absorb, and reduce incoming damage.
    pub versatality: f32,
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
    /// provied chance too hit more than 1 enemy
    pub multistrike: f32,
}

/// actor stats derived from primary and tertiary stats
#[derive(Component, Reflect, Clone, Copy)]
pub struct ActorDerivedAttributes {
    // Offensive derived stats
    /// derived from weapon damage and attackpower
    pub weapondamage: f32,
    /// derived from strength and agility,
    pub attackpower: f32,
    /// derived from weapon speed or cast time and haste, affects gcd too
    pub attackspeed: f32,
    /// derived from crit on gear + base crit
    pub criticalstrike: f32,
    /// derives from intellect
    pub spellpower: f32,
    /// derived from intellect and mastery
    pub regen_speed: f32,

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
    /// life point buffer, subtracted before lifepoints,
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
            versatality: 10.0,
        }
    }
}

impl Default for ActorTertiaryAttributes {
    fn default() -> Self {
        Self {
            avoidance: 0.0,
            leech: 0.0,
            multistrike: 0.0,
            speed: 100.0,
        }
    }
}

impl Default for ActorDerivedAttributes {
    fn default() -> Self {
        Self {
            weapondamage: 5.0,
            attackpower: 5.0,
            attackspeed: 1.0,
            criticalstrike: 0.05,
            spellpower: 5.0,
            regen_speed: 5.0,
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
