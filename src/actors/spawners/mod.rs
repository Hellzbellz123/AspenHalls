//TODO: refactor enemy spawning into events.
// event should look kinda like
//
// struct SpawnSkeletonEvent {
//     position_to_spawn: Vec3
// }
// or possibly just a catch all event with what type of enemy to spawn along with position, amount to spawn should also be added, along with a radius, select random vector3 from within the radius and spawn 1 enemy at that point.

//not sure how to deal with enemys being spawned in colliders. can possible scan in each direction and move to whichever direction has the least amount of colliders? maybe check spawning positon for collider first, if no collider then spawn?
