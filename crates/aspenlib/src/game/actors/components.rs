use bevy::{
    ecs::reflect::ReflectComponent,
    prelude::{Component, Deref, DerefMut, Reflect, Timer},
};

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

/// new type for `Timer` for use with bullet lifetimes
#[derive(Debug, Component, Default, Reflect, Deref, DerefMut)]
#[reflect(Component)]
pub struct TimeToLive(pub Timer);

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
    /// actor is allowed too run
    #[default]
    Run,
    /// actor is allowed too walk
    Walk,
    /// actor is not allowed too move
    None,
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

impl TeleportStatus {
    /// are we not teleporting or just finished?
    pub fn teleport_allowed(&self) -> bool {
        self == &Self::None
    }

    /// is teleport requested?
    pub fn wants_teleport(&self) -> bool {
        self == &Self::Requested
    }

    /// was teleport not requested?
    pub fn teleport_not_requested(&self) -> bool {
        self != &Self::Requested
    }

    /// are we currently teleproting?
    pub fn is_teleporting(&self) -> bool {
        self == &Self::Teleporting
    }

    /// is teleport done?
    pub fn finished_teleport(&self) -> bool {
        self == &Self::Done
    }
}

/// actor move state
/// move state and permission
/// current teleport status
#[derive(Debug, Component, Reflect, Clone, Default)]
pub struct ActorMoveState {
    /// what movment is this actor doing currently
    pub move_status: CurrentMovement,
    /// how is this actor allowed too move
    pub move_perms: AllowedMovement,
    /// actors teleport status
    pub teleport_status: TeleportStatus,
}

impl ActorMoveState {
    /// full speed with no requested teleport
    pub const DEFAULT: Self = Self {
        move_status: CurrentMovement::None,
        move_perms: AllowedMovement::Run,
        teleport_status: TeleportStatus::None,
    };
}
