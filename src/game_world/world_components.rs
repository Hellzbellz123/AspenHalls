use bevy::prelude::*;
use bevy_ecs_ldtk::LdtkIntCell;
use bevy_inspector_egui::Inspectable;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Component, Inspectable)]
pub struct Collides {
    active: bool,
}

impl Default for Collides {
    fn default() -> Self {
        Collides { active: true }
    }
}

#[derive(Clone, Debug, Bundle, LdtkIntCell, Default)]
pub struct ColliderBundle {
    active: Collides,
}

// // not used yet maybe not at all
// impl From<IntGridCell> for ColliderBundle {
//     fn from(int_grid_cell: IntGridCell) -> ColliderBundle {
//         if int_grid_cell.value == 1 {
//             ColliderBundle {
//                 collider: CollisionShape::Cuboid {
//                     half_extends: Vec3::new(8., 8., 0.),
//                     border_radius: None,
//                 },
//                 rigidbody: RigidBody::Static,
//                 ..Default::default()
//             }
//         } else {

//         }
//     }
// }

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
