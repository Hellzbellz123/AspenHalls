/// character spawner system
mod character_spawner;

use bevy::{
    platform::collections::HashSet,
    prelude::{EventReader, Local, Query, Transform, With},
};
use bevy_ecs_ldtk::{LevelEvent, LevelIid};
pub use character_spawner::character_spawners_system;
use log::warn;

// TODO: we might be able to yeet this with transform propogation improvements
/// only run system if all spawned levels have had a transform event fired for them
pub fn all_levels_transformed(
    levels: Query<&LevelIid, With<Transform>>,
    mut level_events: EventReader<LevelEvent>,
    mut transformed_levels: Local<HashSet<LevelIid>>,
) -> bool {
    for event in level_events.read() {
        match event {
            LevelEvent::Transformed(level_iid) => {
                if transformed_levels.contains(level_iid) {
                    warn!("transformed levels already contains this id, did we miss the despawn event?");
                } else {
                    transformed_levels.insert(level_iid.clone());
                }
            }
            LevelEvent::Despawned(level_iid) => {
                if transformed_levels.contains(level_iid) {
                    transformed_levels.remove(level_iid);
                } else {
                    warn!("transformed levels doesnt contain this id, did we miss the transform event?");
                }
            }
            _ => continue,
        }
    }

    if levels.iter().all(|iid| transformed_levels.contains(iid)) {
        return true;
    }
    false
}
