use crate::loading::registry::RegistryIdentifier;
use bevy::prelude::*;

/// misc components
pub mod components;
/// item spawner system
pub mod item_spawner;
/// weapon item plugin
pub mod weapons;

/// item functionality for game
pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EventSpawnItem>();
        app.add_plugins((
            // TODO: impl other items
            weapons::WeaponItemPlugin,
        ));
        app.add_systems(
            Update,
            item_spawner::spawn_item_on_event.run_if(on_event::<EventSpawnItem>()),
        );
    }
}

/// requested item spawn
#[derive(Debug, Reflect, Clone, Event)]
pub struct EventSpawnItem {
    /// id of what too spawn and how many too spawn
    pub spawn_data: (RegistryIdentifier, i32),
    /// id of who requested spawn, if none, near player?
    pub requester: Entity,
    // TODO: send optional equip event?
}
