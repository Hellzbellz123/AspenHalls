// #[must_use]
// pub fn is_player(layers: CollisionLayers) -> bool {
//     //determines if entity is in player collision layer
//     layers.contains_group(PhysicsLayers::Player)
//         && !layers.contains_group(PhysicsLayers::Enemy)
//         && !layers.contains_group(PhysicsLayers::Sensor)
//         && !layers.contains_group(PhysicsLayers::World)
//     // && !layers.contains_group(PhysicsLayers::Projectile)
// }

// #[must_use]
// pub fn is_enemy(layers: CollisionLayers) -> bool {
//     //determines if entity is in enemy collision layer
//     layers.contains_group(PhysicsLayers::Enemy)
//         && !layers.contains_group(PhysicsLayers::Player)
//         && !layers.contains_group(PhysicsLayers::Sensor)
//         && !layers.contains_group(PhysicsLayers::World)
//     // && !layers.contains_group(PhysicsLayers::Projectile)
// }

// #[must_use]
// pub fn is_sensor(layers: CollisionLayers) -> bool {
//     layers.contains_group(PhysicsLayers::Sensor)
//         && !layers.contains_group(PhysicsLayers::Player)
//         && !layers.contains_group(PhysicsLayers::World)
//         && !layers.contains_group(PhysicsLayers::Enemy)
//     // && !layers.contains_group(PhysicsLayers::Projectile)
// }

// #[must_use]
// pub fn is_wall(layers: CollisionLayers) -> bool {
//     layers.contains_group(PhysicsLayers::World)
//         && !layers.contains_group(PhysicsLayers::Player)
//         && !layers.contains_group(PhysicsLayers::Enemy)
//         && !layers.contains_group(PhysicsLayers::Enemy)
//     // && !layers.contains_group(PhysicsLayers::Projectile)
// }

// #[derive(PhysicsLayer, Reflect)]
// pub enum PhysicsLayers {
//     World,
//     Player,
//     Enemy,
//     Sensor,
//     PlayerAttack,
//     EnemyAttack,
// }

// impl PhysicsLayers {
//     #[must_use]
//     pub fn layers(&self) -> CollisionLayers {
//         match self {
//             PhysicsLayers::Player => CollisionLayers::none()
//                 .with_group(PhysicsLayers::Player)
//                 .with_masks(vec![
//                     PhysicsLayers::Enemy,
//                     PhysicsLayers::Sensor,
//                     PhysicsLayers::Player,
//                     PhysicsLayers::World,
//                 ]),
//             PhysicsLayers::Enemy => CollisionLayers::none()
//                 .with_group(PhysicsLayers::Enemy)
//                 .with_masks(vec![
//                     PhysicsLayers::Player,
//                     PhysicsLayers::Enemy,
//                     PhysicsLayers::World,
//                 ]),
//             PhysicsLayers::World => CollisionLayers::none()
//                 .with_group(PhysicsLayers::World)
//                 .with_masks(vec![PhysicsLayers::Player, PhysicsLayers::Enemy]),
//             PhysicsLayers::Sensor => CollisionLayers::none()
//                 .with_group(PhysicsLayers::Sensor)
//                 .with_masks(vec![PhysicsLayers::Player]),
//             PhysicsLayers::PlayerAttack => CollisionLayers::none()
//                 .with_group(PhysicsLayers::PlayerAttack)
//                 .with_masks(vec![PhysicsLayers::Enemy, PhysicsLayers::World]),
//             PhysicsLayers::EnemyAttack => CollisionLayers::none()
//                 .with_group(PhysicsLayers::EnemyAttack)
//                 .with_masks(vec![PhysicsLayers::Player, PhysicsLayers::World]),
//         }
//     }
// }
