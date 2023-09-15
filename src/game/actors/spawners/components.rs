use bevy::{
    prelude::{Component, Deref, DerefMut, ReflectComponent, Vec2, Event},
    reflect::Reflect,
    time::Timer,
};
use rand::{distributions::Standard, prelude::Distribution, Rng};
use strum::{EnumString, EnumVariantNames, VariantNames};

/// different enemy types in the game,
#[derive(Component, Debug, Reflect, Default, Clone, Copy, EnumVariantNames, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum EnemyType {
    /// bony monster, pretty weak, but can come in large amounts
    #[default]
    Skeleton,
    /// squishy gelatinous monster, pretty resistent but not very strong
    Slime,
}

/// weapons that can be spawned in the game
#[derive(Component, Debug, Reflect, Default, Clone, EnumVariantNames, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum WeaponType {
    /// small smg, fast fire rate, med damage
    #[default]
    SmallSMG,
    /// small pistol, slow fire large damage
    SmallPistol,
}

impl Distribution<EnemyType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> EnemyType {
        match rng.gen_range(0..=EnemyType::VARIANTS.len()) {
            0 => EnemyType::Skeleton,
            1 => EnemyType::Slime,
            _ => EnemyType::Slime,
        }
    }
}
/// Marker for enemy container
#[derive(Component)]
pub struct EnemyContainerTag;

/// timer for spawner
#[derive(Debug, Component, DerefMut, Deref, Default, Reflect, Clone)]
#[reflect(Component)]
pub struct SpawnerTimer(pub Timer);

/// type of thing too spawn
#[derive(Component, Reflect, Debug)]
pub enum SpawnType {
    /// spawning item
    Item,
    /// spawning weapon
    Weapon,
    /// spawnuing
    Enemy,
}

/// spawner for enemys
#[derive(Component, Default, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Spawner {
    /// what too spawn
    pub enemytype: EnemyType,
    /// random enemys?
    pub randomenemy: bool,
    /// how far away can spawn
    pub spawn_radius: f32,
    /// max enemys in spawner radius
    pub max_enemies: i32,
}

/// event for spawning enemys
#[derive(Component, Debug, Reflect, Default, Copy, Clone, Event)]
#[reflect(Component)]
pub struct SpawnEnemyEvent {
    /// what too spawn
    pub enemy_to_spawn: EnemyType,
    /// where too spawn
    pub spawn_position: Vec2,
    /// how many too spawn
    pub spawn_count: i32,
}

/// event for spawning weapons
#[derive(Component, Debug, Reflect, Default, Event)]
#[reflect(Component)]
pub struct SpawnWeaponEvent {
    /// what too spawn
    pub weapon_to_spawn: WeaponType,
    /// where too spawn
    pub spawn_position: Vec2,
    /// how many too spawn
    pub spawn_count: i32,
}
