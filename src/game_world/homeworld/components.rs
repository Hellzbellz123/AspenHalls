use bevy::prelude::*;
use bevy_ecs_ldtk::{IntGridCell, LdtkIntCell};
use heron::{CollisionLayers, CollisionShape, RigidBody};

use crate::utilities::game::PhysicsLayers;

#[derive(Bundle, LdtkIntCell)]
pub struct HeronCollisonBundle {
    #[from_int_grid_cell]
    #[bundle]
    collisionbundle: CollisionBundle,
}

#[derive(Clone, Debug, Bundle, LdtkIntCell)]
pub struct CollisionBundle {
    name: Name,
    rigidbody: RigidBody,
    collision_shape: CollisionShape,
    collision_layers: CollisionLayers,
    sensor: WorldSensor,
}

/// just a marker for sensors, saying whether active
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct WorldSensor {
    pub active: bool,
}

impl From<IntGridCell> for CollisionBundle {
    fn from(int_grid_cell: IntGridCell) -> CollisionBundle {
        if int_grid_cell.value == 1 {
            CollisionBundle {
                name: Name::new("CollisonBundle"),
                rigidbody: RigidBody::Static,
                collision_shape: CollisionShape::Cuboid {
                    half_extends: Vec3 {
                        x: 16.0,
                        y: 16.0,
                        z: 0.0,
                    },
                    border_radius: None,
                },
                collision_layers: PhysicsLayers::World.layers(),
                sensor: WorldSensor { active: false },
            }
        } else if int_grid_cell.value == 2 {
            CollisionBundle {
                name: Name::new("SensorBundle"),
                rigidbody: RigidBody::Sensor,
                collision_shape: CollisionShape::Cuboid {
                    half_extends: Vec3 {
                        x: 16.0,
                        y: 16.0,
                        z: 0.0,
                    },
                    border_radius: None,
                },
                collision_layers: CollisionLayers::all_masks::<PhysicsLayers>()
                    .with_group(PhysicsLayers::Sensor),
                sensor: WorldSensor { active: true },
            }
        } else {
            debug!("we hit some weird shit");
            CollisionBundle {
                name: Name::new("SHOULDNT EXIST!!!"),
                rigidbody: RigidBody::Static,
                collision_shape: CollisionShape::default(),
                collision_layers: CollisionLayers::none(),
                sensor: WorldSensor { active: false },
            }
        }
    }
}
