use bevy::{
    prelude::{Component, Deref, DerefMut, Entity, Event, ReflectComponent, Vec2},
    reflect::Reflect,
    time::Timer,
};
use rand::{distributions::Standard, prelude::Distribution, Rng};
use strum::{Display, EnumString, EnumVariantNames, VariantNames};

use crate::game::actors::ai::components::ActorType;

/// different enemy types in the game,
#[derive(
    Component, Debug, Reflect, Default, Clone, Copy, EnumVariantNames, EnumString, Display,
)]
// #[strum(serialize_all = "lowercase")]
pub enum EnemyType {
    /// bony monster, pretty weak, but can come in large amounts
    #[default]
    Skeleton,
    /// squishy gelatinous monster, pretty resistent but not very strong
    Slime,
}

/// weapons that can be spawned in the game
#[derive(Component, Debug, Reflect, Default, Clone, EnumVariantNames, EnumString, Display)]
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
    /// spawning
    Enemy,
}

/// spawner for enemies
#[derive(Component, Default, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Spawner {
    /// what too spawn
    pub enemy_type: EnemyType,
    /// random enemies?
    pub random_enemy: bool,
    /// how far away can spawn
    pub spawn_radius: f32,
    /// max enemies in spawner radius
    pub max_enemies: i32,
    /// list of enemies spawned by spawner
    pub spawned_enemies: Vec<Entity>,
}

/// event for spawning enemies
#[derive(Debug, Reflect, Clone, Event)]
pub struct SpawnActorEvent {
    /// type of actor
    pub actor_type: ActorType,
    /// string that is deserialized too variant of ActorType::Actor.value
    pub what_to_spawn: String,
    /// where too spawn actor, extended too Vec3 later
    pub spawn_position: Vec2,
    /// how many actors too spawn.
    /// prefer setting this too amount you want instead of looping till value is reached
    /// uses spawn batch for actual spawning (not just yet)
    //TODO impl spawn batch for spawning actors
    pub spawn_count: i32,
    /// set too spawner that requested this entity, none if its spawned by a player or some other reason
    pub spawner: Option<Entity>,
}

// /// event for spawning enemies
// #[derive(Component, Debug, Reflect, Copy, Clone, Event)]
// #[reflect(Component)]
// pub struct SpawnEnemyEvent {
//     /// what too spawn
//     /// empty entity too build with
//     pub enemy_to_spawn: Entity,
//     /// where too spawn
//     pub spawn_position: Vec2,
//     /// how many too spawn
//     pub spawn_count: i32,
// }

// /// event for spawning weapons
// #[derive(Component, Debug, Reflect, Default, Event)]
// #[reflect(Component)]
// pub struct SpawnWeaponEvent {
//     /// what too spawn
//     pub weapon_to_spawn: WeaponType,
//     /// where too spawn
//     pub spawn_position: Vec2,
//     /// how many too spawn
//     pub spawn_count: i32,
// }

// impl Default for SpawnEnemyEvent {
//     fn default() -> Self {
//         Self { enemy_to_spawn: Entity::PLACEHOLDER, spawn_position: Default::default(), spawn_count: Default::default() }
//     }
// }
