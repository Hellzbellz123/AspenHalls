use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use super::RigidBodyBundle;

#[derive(Component, Default, Clone, Debug, Inspectable)]
pub struct Aggroable {
    pub distance: f32,
}

#[derive(Component, Default, Clone, Debug, Inspectable)]
pub struct Aggroed;

#[derive(Component, Default, Clone, Debug, Inspectable)]
pub struct AttackPlayer;

#[derive(Component, Default, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct Attacking {
    pub timer: Timer,
    pub is_attacking: bool,
}

#[derive(Component, Default, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct CanMeander;

#[derive(Component, Default, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct IsMeandering;

#[derive(Component, Inspectable)]
pub struct Enemy;

#[derive(Component, Inspectable)]
pub struct Player {
    pub just_teleported: bool,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct TimeToLive(pub Timer);

#[derive(Bundle)]
pub struct ProjectileBundle {
    pub ttl: TimeToLive,

    #[bundle]
    pub sprite_bundle: SpriteBundle,

    #[bundle]
    pub collider_bundle: RigidBodyBundle,
}
