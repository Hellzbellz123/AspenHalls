use crate::game::{characters::components::CharacterType, items::components::ItemType};
use bevy::prelude::*;

/// new type for `Timer` for use with actor life time
#[derive(Debug, Component, Default, Reflect, Deref, DerefMut)]
#[reflect(Component)]
pub struct TimeToLive(pub Timer);

/// actors function in game
#[derive(Debug, Reflect, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum ActorType {
    /// actor is an npc
    /// - `NpcType` decides freinds/enemies
    Character(CharacterType),
    /// actor is an item, can be equipped
    Item(ItemType),
}

/// collider tag, type of collider
#[derive(Debug, Copy, Clone, PartialEq, Eq, Reflect, Component, Default)]
#[reflect(Component)]
pub enum ActorColliderType {
    /// actor collider belongs too character
    #[default]
    Character,
    /// actor collider belongs too item
    Item,
    /// actor collider belongs too projectile
    Projectile,
}
