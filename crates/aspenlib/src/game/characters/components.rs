use bevy::{
    ecs::{component::Component, reflect::ReflectComponent},
    prelude::{Entity, Vec2},
    reflect::{std_traits::ReflectDefault, Reflect},
    utils::HashMap,
};

use crate::loading::registry::RegistryIdentifier;

/// type of character for game
#[derive(
    Debug,
    Default,
    Component,
    Reflect,
    Copy,
    Clone,
    PartialEq,
    Eq,
    serde::Deserialize,
    serde::Serialize,
)]
#[reflect(Component)]
pub enum CharacterType {
    /// - final enemy of dungeon level
    /// - hostile too all npcs
    Boss,
    /// - random unique bosses
    /// - hostile too all npcs
    MiniBoss,
    /// - generic enemy for dungeon levels
    /// - passive too creep
    #[default]
    Creep,
    /// elite enemies in room
    /// - passive too creep
    CreepElite,
    /// - runs away from creeps
    /// - passive too self and freindly
    Critter,
    /// player pet
    HeroPet,
    /// passive too player
    Hero,
    /// sells stuff too player
    Shopkeep,
}

/// character move state and move permissions
/// current teleport status
#[derive(Debug, Component, Reflect, Clone, Default)]
#[reflect(Component)]
pub struct CharacterMoveState {
    /// what movment is this actor doing currently
    pub move_status: (CurrentMovement, MoveDirection),
    /// how is this actor allowed too move
    pub move_perms: AllowedMovement,
    /// actors teleport status
    pub teleport_status: TeleportStatus,
}

/// character items and value
#[derive(Debug, Component, Reflect, Clone, Default)]
#[reflect(Component)]
pub struct CharacterInventory {
    /// what items this character is carrying
    pub items: HashMap<Entity, (RegistryIdentifier, ItemSlot)>,
    /// if hero and player, is total coin count
    /// if enemy, how many coins enemy is worth
    pub coins: u64,
}

/// character available item slot
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Reflect, Default)]
pub struct ItemSlot(u32);

/// character available action slots
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Reflect, Default)]
#[reflect(Default)]
pub enum ActionSlot {
    /// useable action slot 1
    #[default]
    Slot1,
    /// useable action slot 2
    Slot2,
    /// useable action slot 3
    Slot3,
    /// useable action slot 4
    Slot4,
    /// useable action slot 5
    Slot5,
    /// useable action slot 6
    Slot6,
}

/// weapon slots for character
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Reflect, Default)]
pub enum WeaponSlot {
    #[default]
    /// first slot
    Slot1,
    /// second slot
    Slot2,
    /// third slot
    Slot3,
    /// fourth slot
    Slot4,
}

/// actor move permission
/// allowed too move?
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Reflect, Clone, Default, PartialEq, Eq)]
pub enum AllowedMovement {
    /// actor is allowed too run
    #[default]
    Run,
    /// actor is allowed too walk
    Walk,
    /// actor is not allowed too move
    None,
}

/// actors move state
#[derive(Debug, Reflect, Clone, Default, PartialEq, Eq)]
pub enum CurrentMovement {
    /// actor is running
    Run,
    /// actor is walking
    Walk,
    /// actor is not moving
    #[default]
    None,
}

/// actor 8 axis move direction
#[derive(Debug, Reflect, Clone, Default, PartialEq, Eq)]
pub enum MoveDirection {
    #[default]
    /// velocity -Y
    South,
    /// velocity +Y
    North,
    /// velocity +X
    East,
    /// velocity -X
    West,
    /// velocity +Y +Y
    NorthEast,
    /// velocity -Y +X
    SouthEast,
    /// velocity +Y -X
    NorthWest,
    /// velocity -Y -X
    SouthWest,
}

/// actor 8 axis move direction
#[derive(Debug, Reflect, Clone, Default, PartialEq, Eq)]
pub enum CardinalDirection {
    #[default]
    /// velocity -Y
    South,
    /// velocity +Y
    North,
    /// velocity +X
    East,
    /// velocity -X
    West,
}

impl CardinalDirection {
    /// turns a linear velocity into one of 4 cardinal directions
    pub fn from_velocitypi4(velocity: Vec2) -> Self {
        let angle = velocity.y.atan2(velocity.x).to_degrees() + 360.0;
        let index = (angle / 90.0) as usize % 4;

        match index {
            0 => Self::East,
            1 => Self::North,
            2 => Self::West,
            _ => Self::South,
        }
    }

    /// calculates a direction from `point_a` too `point_b`
    pub fn from_positions(from: Vec2, too: Vec2) -> Self {
        let delta = too - from;

        if delta.x.abs() > delta.y.abs() {
            if delta.x >= 0.0 {
                Self::East
            } else {
                Self::West
            }
        } else if delta.y >= 0.0 {
            Self::North
        } else {
            Self::South
        }
    }
}

/// entity teleport status
#[derive(Debug, Reflect, Clone, Default, PartialEq, Eq)]
pub enum TeleportStatus {
    /// no teleport
    #[default]
    None,
    /// entity has requested a teleport
    Requested,
    /// entity is in process of teleporting
    Teleporting,
    /// entity has finished teleporting
    Done,
}

impl CharacterMoveState {
    /// full speed with no requested teleport
    pub const DEFAULT: Self = Self {
        move_status: (CurrentMovement::None, MoveDirection::South),
        move_perms: AllowedMovement::Run,
        teleport_status: TeleportStatus::None,
    };
}

impl TeleportStatus {
    /// was teleport not requested?
    pub fn teleport_not_requested(&self) -> bool {
        self != &Self::Requested
    }

    // /// are we not teleporting or just finished?
    // pub fn teleport_allowed(&self) -> bool {
    //     self == &Self::None
    // }

    // /// is teleport requested?
    // pub fn wants_teleport(&self) -> bool {
    //     self == &Self::Requested
    // }

    // /// are we currently teleproting?
    // pub fn is_teleporting(&self) -> bool {
    //     self == &Self::Teleporting
    // }

    // /// is teleport done?
    // pub fn finished_teleport(&self) -> bool {
    //     self == &Self::Done
    // }
}
