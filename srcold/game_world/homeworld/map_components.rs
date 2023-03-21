use bevy::prelude::*;
use bevy_ecs_ldtk::{EntityInstance, IntGridCell, LdtkEntity, LdtkIntCell};

use bevy_rapier2d::prelude::{
    ActiveEvents, Collider, CollisionGroups, Group, RigidBody, Rot, Sensor, Vect,
};

use crate::{
    components::actors::spawners::{EnemyType, Spawner, SpawnerTimer},
    utilities::game::WORLD_COLLIDER_LAYER,
};

/// just a marker for sensors, saying whether active
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct HomeWorldTeleportSensor {
    pub active: bool,
}

#[derive(Resource, Clone, Debug, Default, Deref, DerefMut)]
pub struct TeleportTimer {
    pub timer: Timer,
}

#[derive(Clone, Debug, Bundle, LdtkIntCell)]
pub struct CollisionBundle {
    name: Name,
    rigidbody: RigidBody,
    collision_shape: Collider,
    collision_group: CollisionGroups,
}

#[derive(Bundle, LdtkIntCell)]
pub struct LdtkCollisionBundle {
    #[from_int_grid_cell]
    collisionbundle: CollisionBundle,
}

impl From<IntGridCell> for CollisionBundle {
    fn from(int_grid_cell: IntGridCell) -> CollisionBundle {
        // 90 degrees radian
        let ndgs = std::f32::consts::FRAC_PI_2;
        match int_grid_cell.value {
            1 => {
                let shape: Vec<(Vect, Rot, Collider)> =
                    vec![(Vec2::new(0.0, 6.0), 0.0, Collider::cuboid(8.0, 2.0))];
                CollisionBundle {
                    name: Name::new("CollideDown"),
                    rigidbody: RigidBody::Fixed,
                    collision_shape: Collider::compound(shape),
                    collision_group: CollisionGroups {
                        memberships: Group::all(),
                        filters: WORLD_COLLIDER_LAYER,
                    },
                }
            }
            2 => {
                let shape: Vec<(Vect, Rot, Collider)> =
                    vec![(Vec2::new(0.0, -6.), 0.0, Collider::cuboid(8.0, 2.0))];
                CollisionBundle {
                    name: Name::new("CollideUp"),
                    rigidbody: RigidBody::Fixed,
                    collision_shape: Collider::compound(shape),
                    collision_group: CollisionGroups {
                        memberships: Group::all(),
                        filters: WORLD_COLLIDER_LAYER,
                    },
                }
            }
            3 => {
                let shape: Vec<(Vect, Rot, Collider)> =
                    vec![(Vec2::new(6.0, 0.0), 0.0, Collider::cuboid(2.0, 8.0))];
                CollisionBundle {
                    name: Name::new("CollideLeft"),
                    rigidbody: RigidBody::Fixed,
                    collision_shape: Collider::compound(shape),
                    collision_group: CollisionGroups {
                        memberships: Group::all(),
                        filters: WORLD_COLLIDER_LAYER,
                    },
                }
            }
            4 => {
                let shape: Vec<(Vect, Rot, Collider)> =
                    vec![(Vec2::new(-6.0, 0.0), 0.0, Collider::cuboid(2.0, 8.0))];
                CollisionBundle {
                    name: Name::new("CollideRight"),
                    rigidbody: RigidBody::Fixed,
                    collision_shape: Collider::compound(shape),
                    collision_group: CollisionGroups {
                        memberships: Group::all(),
                        filters: WORLD_COLLIDER_LAYER,
                    },
                }
            }
            5 => {
                let shape: Vec<(Vect, Rot, Collider)> =
                    vec![(Vec2::new(0.0, 7.0), 0.0, Collider::cuboid(8.0, 2.0))];

                CollisionBundle {
                    name: Name::new("CollideWall"),
                    rigidbody: RigidBody::Fixed,
                    collision_shape: Collider::compound(shape),
                    collision_group: CollisionGroups {
                        memberships: Group::all(),
                        filters: WORLD_COLLIDER_LAYER,
                    },
                }
            }
            6 => {
                let shape: Vec<(Vect, Rot, Collider)> =
                    vec![(Vec2::new(-6.0, 6.0), 0.0, Collider::cuboid(2.0, 2.0))];
                CollisionBundle {
                    name: Name::new("CollideCornerUL"), //upper left //FINISHED
                    rigidbody: RigidBody::Fixed,
                    collision_shape: Collider::compound(shape),
                    collision_group: CollisionGroups {
                        memberships: Group::all(),
                        filters: WORLD_COLLIDER_LAYER,
                    },
                }
            }
            7 => {
                let shape: Vec<(Vect, Rot, Collider)> =
                    vec![(Vec2::new(-6.0, -6.0), 0.0, Collider::cuboid(2.0, 2.0))];
                CollisionBundle {
                    name: Name::new("CollideCornerLL"), //lower left
                    rigidbody: RigidBody::Fixed,
                    collision_shape: Collider::compound(shape),
                    collision_group: CollisionGroups {
                        memberships: Group::all(),
                        filters: WORLD_COLLIDER_LAYER,
                    },
                }
            }
            8 => {
                let shape: Vec<(Vect, Rot, Collider)> =
                    vec![(Vec2::new(6.0, 6.0), 0.0, Collider::cuboid(2.0, 2.0))];
                CollisionBundle {
                    name: Name::new("CollideCornerUR"), //upper right   //done
                    rigidbody: RigidBody::Fixed,
                    collision_shape: Collider::compound(shape),
                    collision_group: CollisionGroups {
                        memberships: Group::all(),
                        filters: WORLD_COLLIDER_LAYER,
                    },
                }
            }
            9 => {
                let shape: Vec<(Vect, Rot, Collider)> =
                    vec![(Vec2::new(6.0, -6.0), 0.0, Collider::cuboid(2.0, 2.0))];
                CollisionBundle {
                    name: Name::new("CollideCornerLR"), //lower right
                    rigidbody: RigidBody::Fixed,
                    collision_shape: Collider::compound(shape),
                    collision_group: CollisionGroups {
                        memberships: Group::all(),
                        filters: WORLD_COLLIDER_LAYER,
                    },
                }
            }
            10 => {
                let shape: Vec<(Vect, Rot, Collider)> = vec![
                    (Vec2::new(-6.0, -2.0), ndgs, Collider::cuboid(6.0, 2.0)),
                    (Vec2::new(0.0, 6.0), 0.0, Collider::cuboid(8.0, 2.0)),
                ];
                CollisionBundle {
                    name: Name::new("CollideInnerUL"), //lower left inverted corner
                    rigidbody: RigidBody::Fixed,
                    collision_shape: Collider::compound(shape),
                    collision_group: CollisionGroups {
                        memberships: Group::all(),
                        filters: WORLD_COLLIDER_LAYER,
                    },
                }
            }
            11 => {
                let shape: Vec<(Vect, Rot, Collider)> = vec![
                    (Vec2::new(-6.0, 2.0), ndgs, Collider::cuboid(6.0, 2.0)),
                    (Vec2::new(0.0, -6.0), 0.0, Collider::cuboid(8.0, 2.0)),
                ];
                CollisionBundle {
                    name: Name::new("CollideInnerLL"), //lower left inverted corner
                    rigidbody: RigidBody::Fixed,
                    collision_shape: Collider::compound(shape),
                    collision_group: CollisionGroups {
                        memberships: Group::all(),
                        filters: WORLD_COLLIDER_LAYER,
                    },
                }
            }
            12 => {
                let shape: Vec<(Vect, Rot, Collider)> = vec![
                    (Vec2::new(6.0, -2.0), ndgs, Collider::cuboid(6.0, 2.0)),
                    (Vec2::new(0.0, 6.0), 0.0, Collider::cuboid(8.0, 2.0)),
                ];
                CollisionBundle {
                    name: Name::new("CollideInnerUR"), //upper right inverted corner
                    rigidbody: RigidBody::Fixed,
                    collision_shape: Collider::compound(shape),
                    collision_group: CollisionGroups {
                        memberships: Group::all(),
                        filters: WORLD_COLLIDER_LAYER,
                    },
                }
            }
            13 => {
                let shape: Vec<(Vect, Rot, Collider)> = vec![
                    (Vec2::new(6.0, 2.0), ndgs, Collider::cuboid(6.0, 2.0)),
                    (Vec2::new(0.0, -6.0), 0.0, Collider::cuboid(8.0, 2.0)),
                ];
                CollisionBundle {
                    name: Name::new("CollideInnerLR"), //lower right inverted corner
                    rigidbody: RigidBody::Fixed,
                    collision_shape: Collider::compound(shape),
                    collision_group: CollisionGroups {
                        memberships: Group::all(),
                        filters: WORLD_COLLIDER_LAYER,
                    },
                }
            }
            _ => CollisionBundle {
                name: Name::new("shouldnt_exist"),
                rigidbody: RigidBody::Fixed,
                collision_shape: Collider::cuboid(100.0, 100.0),
                collision_group: CollisionGroups {
                    memberships: Group::NONE,
                    filters: Group::NONE,
                },
            },
        }
    }
}

#[derive(Bundle, LdtkEntity)]
pub struct LdtkSensorBundle {
    #[with(sensor_bundle)]
    sensorbundle: SensorBundle,
}

#[derive(Clone, Debug, Bundle, LdtkEntity)]
pub struct SensorBundle {
    name: Name,
    sensor: Sensor,
    homeworldsensor: HomeWorldTeleportSensor,
    collision_shape: Collider,
    events: ActiveEvents,
}
fn sensor_bundle(_ent_instance: &EntityInstance) -> SensorBundle {
    SensorBundle {
        name: Name::new("SensorBundle"),
        collision_shape: Collider::cuboid(8., 8.),
        sensor: Sensor,
        events: ActiveEvents::COLLISION_EVENTS,
        homeworldsensor: HomeWorldTeleportSensor { active: true },
    }
    // ent_instance.field_instances.leak()
}

#[derive(Bundle, LdtkEntity)]
pub struct LdtkSpawnerBundle {
    #[with(spawner_bundle)]
    sensorbundle: SpawnerBundle,
}

#[derive(Clone, Debug, Bundle, LdtkEntity)]
pub struct SpawnerBundle {
    name: Name,
    state: Spawner,
    timer: SpawnerTimer,
}

fn spawner_bundle(_ent_instance: &EntityInstance) -> SpawnerBundle {
    SpawnerBundle {
        name: Name::new("spawnerbundle"),
        state: Spawner {
            enemytype: EnemyType::Skeleton,
            spawn_radius: 100.0,
            max_enemies: 7,
            randomenemy: true,
        },
        timer: SpawnerTimer(Timer::from_seconds(2.0, TimerMode::Repeating)),
    }
    // ent_instance.field_instances.leak()
}
