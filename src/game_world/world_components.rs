
// im only keeping this for documentation purposes. this took me 3 weeks to figure out.
// pub fn name_colliders(
//     mut commands: Commands,
//     entity_query: Query<(Entity, &Parent, &CollisionShape, &Transform), Added<CollisionShape>>,
// ) {
//     for entity in entity_query.iter() {
//         info!("naming colliders: {}", entity.0.id());
//         commands
//             .entity(entity.0)
//             .insert(Name::new("levelCollider"))
//             .insert(RigidBody::Static)
//             .insert(CollisionShape::Cuboid {
//                 half_extends: Vec3::new(16.0, 16.0, 0.0),
//                 border_radius: None,
//             })
//             .insert(
//                 CollisionLayers::none()
//                     .with_group(PhysicsLayers::World)
//                     .with_mask(PhysicsLayers::Player),
//             );
//     }
// }

// pub fn name_colliders_simple(
//     mut commands: Commands,
//     entity_query: Query<(Entity, &Parent, &Transform), Added<CollisionShape>>,
// ) {
//     for entity in entity_query.iter() {
//         info!("naming colliders: {}", entity.0.id());
//         commands.entity(entity.0).insert(Name::new("levelCollider"));
//     }
// }

// impl FromWorld for ColliderBundle {
//     fn from_world(world: &mut World) -> Self {
//         Self {
//             name: Name::new("NamedCollider".to_string()),
//             rigidbody: RigidBody::Static,
//             collision_shape: CollisionShape::Cuboid {
//                 half_extends: Vec3::new(16.0, 16.0, 0.0),
//                 border_radius: None,
//             },
//             collision_layers: CollisionLayers::none()
//                 .with_group(PhysicsLayers::World)
//                 .with_mask(PhysicsLayers::Player),
//         }
//     }
// }

// // not used yet maybe not at all

// use bevy::prelude::*;
// use bevy_ecs_ldtk::{prelude::*, utils::ldtk_pixel_coords_to_translation_pivoted};

// use std::collections::HashSet;

// use heron::prelude::*;

// #[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
// pub struct ColliderBundle {
//     pub collider: CollisionShape,
//     pub rigid_body: RigidBody,
//     pub velocity: Velocity,
//     pub rotation_constraints: RotationConstraints,
//     pub physic_material: PhysicMaterial,
// }

// impl From<EntityInstance> for ColliderBundle {
//     fn from(entity_instance: EntityInstance) -> ColliderBundle {
//         let rotation_constraints = RotationConstraints::lock();

//         match entity_instance.identifier.as_ref() {
//             "Player" => ColliderBundle {
//                 collider: CollisionShape::Cuboid {
//                     half_extends: Vec3::new(6., 14., 0.),
//                     border_radius: None,
//                 },
//                 rigid_body: RigidBody::Dynamic,
//                 rotation_constraints,
//                 ..Default::default()
//             },
//             "Mob" => ColliderBundle {
//                 collider: CollisionShape::Cuboid {
//                     half_extends: Vec3::new(5., 5., 0.),
//                     border_radius: None,
//                 },
//                 rigid_body: RigidBody::KinematicVelocityBased,
//                 rotation_constraints,
//                 ..Default::default()
//             },
//             "Chest" => ColliderBundle {
//                 collider: CollisionShape::Cuboid {
//                     half_extends: Vec3::new(8., 8., 0.),
//                     border_radius: None,
//                 },
//                 rigid_body: RigidBody::Dynamic,
//                 rotation_constraints,
//                 physic_material: PhysicMaterial {
//                     friction: 0.5,
//                     density: 15.0,
//                     ..Default::default()
//                 },
//                 ..Default::default()
//             },
//             _ => ColliderBundle::default(),
//         }
//     }
// }
