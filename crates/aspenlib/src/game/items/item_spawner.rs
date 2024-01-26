use bevy::prelude::*;

use crate::{
    game::items::{components::ItemType, weapons, EventSpawnItem},
    loading::{custom_assets::actor_definitions::ItemDefinition, registry::ActorRegistry},
};

// TODO: implement leveled list like system
/// takes weapon spawn commands and spawns weapons in the world
pub fn spawn_item_on_event(
    mut commands: Commands,
    mut item_spawn_requests: EventReader<EventSpawnItem>,
    global_transforms: Query<&GlobalTransform>,
    registry: Res<ActorRegistry>,
    item_assets: Res<Assets<ItemDefinition>>,
) {
    for event in item_spawn_requests.read() {
        let Ok(requester_transform) = global_transforms.get(event.requester) else {
            error!("entity requesting teleport does not have a transform");
            continue;
        };
        let spawn_pos = &requester_transform.translation().truncate();

        let Some(item_type) = registry.items.get_item_type(&event.spawn_data.0) else {
            error!(
                "requested item did not exist in weapon registry: {:?}",
                event.spawn_data.0
            );
            continue;
        };

        match item_type {
            ItemType::Weapon => {
                info!("got weapon type");
                weapons::utils::spawn_weapon(
                    &registry,
                    &item_assets,
                    &event.spawn_data,
                    *spawn_pos,
                    &mut commands,
                );
            }
            a => {
                warn!("item type not implemented: {:?}", a);
                continue;
            }
        }
    }
}
