use bevy::prelude::*;
use bevy_ecs_ldtk::{LdtkIntCell, IntGridCell};
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
