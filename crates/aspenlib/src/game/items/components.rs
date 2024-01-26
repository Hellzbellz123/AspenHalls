use bevy::reflect::Reflect;

/// items function in the game
#[derive(Debug, Reflect, Copy, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum ItemType {
    /// item is used too damage characters
    Weapon,
    /// item is used for stats and damage reduction
    Armor,
    /// item is used for stats and special effects on timers
    Trinket,
    /// item is used too heal stats and give special effects for duration
    Food,
}
