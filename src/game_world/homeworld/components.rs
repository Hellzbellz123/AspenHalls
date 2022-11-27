use bevy::prelude::*;
use bevy_ecs_ldtk::{IntGridCell, LdtkIntCell};
use bevy_inspector_egui::reflect::ReflectedUI;
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::prelude::{ActiveEvents, Collider, RigidBody, Sensor};

#[derive(Inspectable, Default, Debug, Resource)]
pub struct InspectableData {
    // and for most of bevy's types
    timer: ReflectedUI<Timer>,
}
/// just a marker for sensors, saying whether active
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct HomeWorldTeleportSensor {
    pub active: bool,
}

#[derive(Component, Clone, Debug)]
pub struct TeleportTimer {
    pub timer: Timer,
}

#[derive(Clone, Debug, Bundle, LdtkIntCell)]
pub struct CollisionBundle {
    name: Name,
    rigidbody: RigidBody,
    collision_shape: Collider,
}

#[derive(Bundle, LdtkIntCell)]
pub struct RapierCollisionBundle {
    #[from_int_grid_cell]
    collisionbundle: CollisionBundle,
}

impl From<IntGridCell> for CollisionBundle {
    fn from(_int_grid_cell: IntGridCell) -> CollisionBundle {
        CollisionBundle {
            name: Name::new("Cbundle"),
            rigidbody: RigidBody::Fixed,
            collision_shape: Collider::cuboid(7.8, 7.8),
        }
    }
}

#[derive(Bundle, LdtkIntCell)]
pub struct RapierSensorBundle {
    #[from_int_grid_cell]
    sensorbundle: SensorBundle,
}

#[derive(Clone, Debug, Bundle, LdtkIntCell)]
pub struct SensorBundle {
    name: Name,
    sensor: Sensor,
    homeworldsensor: HomeWorldTeleportSensor,
    collision_shape: Collider,
    events: ActiveEvents,
}

impl From<IntGridCell> for SensorBundle {
    fn from(_int_grid_cell: IntGridCell) -> SensorBundle {
        SensorBundle {
            name: Name::new("SensorBundle"),
            collision_shape: Collider::cuboid(8., 8.),
            sensor: Sensor,
            events: ActiveEvents::COLLISION_EVENTS,
            homeworldsensor: HomeWorldTeleportSensor { active: true },
        }
    }
}
