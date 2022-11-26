//TODO: refactor enemy spawning into events.
// event should look kinda like
//
// struct SpawnSkeletonEvent {
//     position_to_spawn: Vec3
// }
//
// enemy spawner entity with position
//struct SpawnerEntityBundle {
//  transform: <>
//  spawner: {
//      Spawner {
//          enemy_to_spawn: <>
//          spawn_radius: <>
//          max_enemy_in_area: <>
//          spawn_timer: <>
//  }
//}
//}
// or possibly just a catch all event with what type of enemy to spawn along with position, amount to spawn should also be added, along with a radius, select random vector3 from within the radius and spawn 1 enemy at that point.

//not sure how to deal with enemys being spawned in colliders. can possible scan in each direction and move to whichever direction has the least amount of colliders? maybe check spawning positon for collider first, if no collider then spawn?

use bevy::{prelude::*, time::Timer};

pub enum EnemyType {
    Skeleton
}

pub struct Spawner {
    pub enemy_to_spawn: EnemyType,
    pub spawn_radius: f32,
    pub max_enemies:  i32,
    pub spawn_timer: Timer,
}

pub struct SpawnerEntityBundle {
    pub name: Name,
    pub spatialbundle: SpatialBundle,
    pub spawner: Spawner,
}
