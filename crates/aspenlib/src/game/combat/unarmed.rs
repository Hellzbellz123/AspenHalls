use bevy::prelude::*;

pub struct UnArmedPlugin;

impl Plugin for UnArmedPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, unarmed_attacks);
    }
}

pub fn unarmed_attacks() {}

// TODO:
// implement fist attack as an ability?
// possibly a hidden ability
/// attacked without weapon using fist
#[derive(Debug, Event)]
pub struct EventAttackFist {
    /// who requested fist attack
    pub requester: Entity,
}
